use common::configurator_protocol::messages::{FcPhase, FcStatus};
use embassy_executor::SendSpawner;
use embassy_time::{Duration, Instant, Ticker, Timer};
use static_cell::StaticCell;

use crate::{
    battery_monitor::init_battery_monitor,
    crsf::init_crsf_communication,
    esc::EscMotorSet,
    generic_hardware_type,
    gps::init_gps_receiver,
    hal::Leds,
    icm42688::ICM42688,
    logging::info,
    mixer::MotorMix,
    motors::{disarm, drive_motors, MotorInputs, MotorsContext},
    msp::usb_communication::start_usb_communication,
    osd::init_osd,
    pid::{do_pid_iteration, PidContext},
    shared_state::SharedState,
};

struct FlightContext {
    motor_context: MotorsContext,
    pid_context: PidContext,
}

pub struct FlightModeExitData {
    pub motor_set: EscMotorSet,
    pub shared_state: &'static SharedState,
    pub leds: Leds,
    pub phase: FcPhase,
}

pub enum FlightModeExit {
    Configurator(FlightModeExitData),
    FourWayEsc(FlightModeExitData),
}

pub async fn flight_main(
    spawner_low: SendSpawner,
    mut hardware: generic_hardware_type!(),
) -> FlightModeExit {
    static SHARED_STATE: StaticCell<SharedState> = StaticCell::new();

    let motor_set = hardware.motor_layout.get_motors();

    let shared_state = SHARED_STATE.init(SharedState::new(hardware.config_store));

    let mut leds: Leds = hardware.led_pins.into();

    start_usb_communication(&spawner_low, hardware.usb, shared_state).await;

    leds.green_on();

    let mut imu = ICM42688::new(hardware.imu_spi);
    let imu_result = imu.init().await;

    if imu_result.is_err() {
        return FlightModeExit::Configurator(FlightModeExitData {
            motor_set,
            shared_state,
            leds,
            phase: FcPhase::IMUInitFail,
        });
    }

    Timer::after_millis(10).await;

    leds.yellow_on();

    // stored_config::reconfigure_and_store(&mut hardware.config_store).await;
    // stored_config::dump_config(&mut hardware.config_store).await;

    let stored_config = shared_state.read_config().await;

    // icm42688::calibrate_gyro_offsets(imu, &stored_config, true).await;

    init_crsf_communication(hardware.radio_uart, &spawner_low, shared_state);
    init_battery_monitor(hardware.battery_meter, shared_state, &spawner_low);

    if let Some(gps_uart) = hardware.gps_uart {
        init_gps_receiver(gps_uart, &spawner_low, shared_state);
    }

    if let Some(msp_uart) = hardware.msp_uart {
        hardware.vtx_power_toggle.set_high();
        init_osd(
            msp_uart,
            hardware.vtx_power_toggle,
            &spawner_low,
            shared_state,
        );
    }

    leds.blue_on();

    let motor_mix: MotorMix;
    #[cfg(feature = "quad")]
    {
        use crate::mixer::QuadcopterMix;
        motor_mix = QuadcopterMix::new(motor_set, &stored_config);
    }
    #[cfg(feature = "wing")]
    {
        use crate::mixer::WingMix;

        motor_mix = WingMix::new(
            hardware.motor_layout.left_winglet_servo,
            hardware.motor_layout.right_winglet_servo,
            hardware.motor_layout.thrust_motor,
        );
    }

    let context = FlightContext {
        motor_context: MotorsContext::new(motor_mix),
        pid_context: PidContext::new(&stored_config),
    };

    flight_loop(leds, imu, context, shared_state).await
}

async fn flight_loop(
    mut leds: Leds,
    mut imu: ICM42688,
    mut context: FlightContext,
    shared_state: &'static SharedState,
) -> FlightModeExit {
    const PID_PERIOD_US: u64 = 1005;
    let mut ticker = Ticker::every(Duration::from_micros(PID_PERIOD_US));

    let mut print_counter = 0;

    leds.green_off();
    leds.blue_off();
    leds.yellow_on();

    let mut total_duration = 0f32;
    let mut inner_duration = 0f32;
    let mut previous_t1 = Instant::now();
    let mut max_measured_period = 0;
    let mut min_measured_period = 10000;

    let mut motor_inputs: MotorInputs = MotorInputs::default();

    loop {
        let t1 = Instant::now();
        let command_state = shared_state.command_snapshot();

        let armed = command_state.arming_tracker.is_armed();

        match armed {
            true => {
                leds.green_on();
                leds.yellow_off();
            }
            false => {
                leds.green_off();
                leds.yellow_on();
            }
        };

        match command_state.commands.is_fresh() {
            true => leds.blue_on(),
            false => leds.blue_off(),
        };

        if shared_state.is_four_way_mode_requested() {
            disarm(
                &mut context.motor_context,
                &motor_inputs,
                command_state.commands.beep(),
            )
            .await;

            let mix = context.motor_context.into_mix();
            let motor_set = mix.into_motors();

            return FlightModeExit::FourWayEsc(FlightModeExitData {
                motor_set,
                shared_state,
                leds,
                phase: FcPhase::Disarmed,
            });
        }

        if shared_state.is_configurator_mode_requested() {
            disarm(
                &mut context.motor_context,
                &motor_inputs,
                command_state.commands.beep(),
            )
            .await;

            let motor_set = context.motor_context.into_mix().into_motors();

            return FlightModeExit::Configurator(FlightModeExitData {
                motor_set,
                shared_state,
                leds,
                phase: FcPhase::Disarmed,
            });
        }

        print_counter += 1;
        motor_inputs =
            do_pid_iteration(&mut imu, &mut context.pid_context, &command_state.commands).await;

        let t2 = Instant::now();
        if armed {
            drive_motors(&mut context.motor_context, &motor_inputs).await;
        } else {
            disarm(
                &mut context.motor_context,
                &motor_inputs,
                command_state.commands.beep(),
            )
            .await;
        }

        let t3 = Instant::now();

        total_duration = (t3 - t1).as_micros() as f32 * 0.5 + total_duration * 0.5;
        inner_duration = (t3 - t2).as_micros() as f32 * 0.5 + inner_duration * 0.5;
        let measured_period: u64 = (t1 - previous_t1).as_micros();
        if measured_period > max_measured_period {
            max_measured_period = measured_period;
        }
        if measured_period < min_measured_period {
            min_measured_period = measured_period;
        }

        if print_counter > 1000 * (2000 / PID_PERIOD_US) {
            print_counter = 0;
            info!(
                "TICK {} {} {} {}",
                total_duration, inner_duration, min_measured_period, max_measured_period,
            );
            shared_state.publish_status(FcStatus {
                inner_duration: inner_duration.into(),
                total_duration: total_duration.into(),
                min_measured_period: min_measured_period.into(),
                max_measured_period: max_measured_period.into(),
                fc_phase: if armed {
                    FcPhase::Armed
                } else {
                    FcPhase::Disarmed
                },
                valid: true,
            });
            max_measured_period = 0;
            min_measured_period = 10000;
        }
        previous_t1 = t1;
        ticker.next().await;
    }
}
