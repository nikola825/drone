use embassy_executor::SendSpawner;
use embassy_sync::lazy_lock::LazyLock;
use embassy_time::{Duration, Instant, Ticker, Timer};

use crate::{
    battery_monitor::init_battery_monitor,
    crsf::init_crsf_communication,
    generic_hardware_type,
    gps::init_gps_receiver,
    hal::Leds,
    icm42688::ICM42688,
    logging::{info, init_logging},
    mixer::MotorMix,
    motors::{disarm, drive_motors, MotorsContext},
    osd::init_osd,
    pid::{do_pid_iteration, PidContext},
    shared_state::SharedState,
    stored_config::read_stored_config,
};

struct FlightContext {
    motor_context: MotorsContext,
    pid_context: PidContext,
}

pub async fn flight_main(
    spawner_low: SendSpawner,
    spawner_high: SendSpawner,
    mut hardware: generic_hardware_type!(),
) {
    static STORE: LazyLock<SharedState> = LazyLock::new(SharedState::new);

    let mut leds: Leds = hardware.led_pins.into();

    init_logging!(hardware, spawner_low);

    leds.green_on();

    let mut imu = ICM42688::new(hardware.imu_spi);
    imu.init().await;
    Timer::after_millis(10).await;

    leds.yellow_on();

    // stored_config::reconfigure_and_store(&mut hardware.config_store).await;
    // stored_config::dump_config(&mut hardware.config_store).await;

    let stored_config = read_stored_config(&mut hardware.config_store).await;

    // icm42688::calibrate_gyro_offsets(imu, &stored_config, true).await;

    init_crsf_communication(hardware.radio_uart, &spawner_low, STORE.get());
    init_battery_monitor(hardware.battery_meter, STORE.get(), &spawner_low);

    if let Some(gps_uart) = hardware.gps_uart {
        init_gps_receiver(gps_uart, &spawner_low, STORE.get());
    }

    if let Some(msp_uart) = hardware.msp_uart {
        hardware.vtx_power_toggle.set_high();
        init_osd(
            msp_uart,
            hardware.vtx_power_toggle,
            &spawner_low,
            STORE.get(),
        );
    }

    leds.blue_on();

    let motor_mix: MotorMix;
    #[cfg(feature = "quad")]
    {
        use crate::mixer::QuadcopterMix;

        let mut motors = hardware.motor_layout.get_motors().map(Some);
        // motors::do_motor_mapping(motors, &stored_config).await;

        let front_left = motors[stored_config.front_left_motor as usize]
            .take()
            .unwrap();
        let front_right = motors[stored_config.front_right_motor as usize]
            .take()
            .unwrap();
        let rear_left = motors[stored_config.rear_left_motor as usize]
            .take()
            .unwrap();
        let rear_right = motors[stored_config.rear_right_motor as usize]
            .take()
            .unwrap();

        // motors::apply_motor_directions(&front_left, &front_right, &rear_left, &rear_right, &stored_config).await;

        motor_mix = QuadcopterMix::new(front_left, front_right, rear_left, rear_right);
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

    spawner_high.must_spawn(flight_control_task(leds, imu, context, STORE.get()));
}

#[embassy_executor::task]
pub async fn flight_control_task(
    mut leds: Leds,
    mut imu: ICM42688,
    mut context: FlightContext,
    store: &'static SharedState,
) {
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

    loop {
        let t1 = Instant::now();
        let command_state = store.command_snapshot().await;

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

        let t2 = Instant::now();
        print_counter += 1;
        let motor_inputs =
            do_pid_iteration(&mut imu, &mut context.pid_context, &command_state.commands).await;

        if armed {
            drive_motors(&mut context.motor_context, &motor_inputs);
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
            max_measured_period = 0;
            min_measured_period = 10000;
        }
        previous_t1 = t1;
        ticker.next().await;
    }
}
