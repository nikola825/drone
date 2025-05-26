#![no_std]
#![no_main]

use battery_monitor::init_battery_monitor;
use config_storage::{read_stored_config, StoredConfig};
use cortex_m_rt::entry;
use crsf::init_crsf_communication;
use embassy_executor::SendSpawner;
use embassy_stm32::adc::AdcChannel;
use embassy_stm32::flash::Flash;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_sync::lazy_lock::LazyLock;
use embassy_time::{Duration, Instant, Ticker, Timer};
use hw_select::get_spawners;
use icm42688::ICM42688;
use logging::{info, init_logging};
use motors::{disarm, drive_motors, Motor, MotorsContext};
use osd::init_osd;
use pid::{do_pid_iteration, PidContext};
use shared_state::SharedState;

mod ahrs_wrapper;
mod arming;
mod battery_monitor;
mod channel_mapping;
mod config_storage;
mod crc8;
mod crsf;
mod dshot;
mod expo_rates;
mod hw_select;
mod icm42688;
mod logging;
mod math_stuff;
mod motors;
mod nopdelays;
mod osd;
mod pid;
mod shared_state;
mod static_buffer;

struct DroneContext {
    motor_context: MotorsContext,
    pid_context: PidContext,
}

#[entry]
fn main() -> ! {
    let spawners = get_spawners();

    spawners
        .spawner_low
        .must_spawn(async_main(spawners.spawner_low, spawners.spawner_high));

    loop {
        cortex_m::asm::wfi()
    }
}

#[embassy_executor::task]
async fn async_main(spawner_low: SendSpawner, spawner_high: SendSpawner) {
    static STORE: LazyLock<SharedState> = LazyLock::new(SharedState::new);

    let hardware = get_hardware!();

    let mut blue = Output::new(hardware.blue_pin, Level::Low, Speed::VeryHigh);
    let mut green = Output::new(hardware.green_pin, Level::Low, Speed::VeryHigh);
    let mut yellow = Output::new(hardware.yellow_pin, Level::Low, Speed::VeryHigh);

    let mut flash = Flash::new_blocking(hardware.flash);

    init_logging!(hardware, spawner_low);

    green.set_high();

    let mut imu = ICM42688::new(hardware.imu_spi);
    imu.init().await;
    Timer::after_millis(10).await;

    yellow.set_high();

    // reconfigure_and_store(&mut flash).await;
    // dump_config(&mut flash).await;

    let stored_config = read_stored_config(&mut flash).await;

    let mut motors: [Option<Motor>; 4] = [
        Some(Motor::new(hardware.motor0_pin)),
        Some(Motor::new(hardware.motor1_pin)),
        Some(Motor::new(hardware.motor2_pin)),
        Some(Motor::new(hardware.motor3_pin)),
    ];

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

    init_crsf_communication(hardware.radio_uart, &spawner_low, STORE.get());
    init_battery_monitor(hardware.adc_reader, STORE.get(), &spawner_low);
    init_osd!(hardware, spawner_low, STORE.get());

    blue.set_high();

    // motor_reset(&front_left, &front_right, &rear_left, &rear_right).await;

    let context = DroneContext {
        motor_context: MotorsContext::new(front_left, front_right, rear_left, rear_right),
        pid_context: PidContext::new(&stored_config),
    };

    spawner_high.must_spawn(tick_task(blue, green, yellow, imu, context, STORE.get()));
}

#[embassy_executor::task]
async fn tick_task(
    mut blue_led: Output<'static>,
    mut green_led: Output<'static>,
    mut yellow_led: Output<'static>,
    mut imu: ICM42688,
    mut context: DroneContext,
    store: &'static SharedState,
) {
    const PID_PERIOD_US: u64 = 1005;
    let mut ticker = Ticker::every(Duration::from_micros(PID_PERIOD_US));

    let mut print_counter = 0;

    blue_led.set_low();
    green_led.set_low();
    yellow_led.set_high();

    let mut total_duration = 0f32;
    let mut inner_duration = 0f32;
    let mut previous_t1 = Instant::now();
    let mut max_measured_period = 0;
    let mut min_measured_period = 10000;

    loop {
        let t1 = Instant::now();
        let command_state = store.command_snapshot().await;

        let armed = command_state.arming_tracker.is_armed();

        green_led.set_level(match armed {
            true => Level::High,
            false => Level::Low,
        });

        yellow_led.set_level(match armed {
            true => Level::Low,
            false => Level::High,
        });

        blue_led.set_level(match command_state.commands.is_fresh() {
            true => Level::High,
            false => Level::Low,
        });

        let t2 = Instant::now();
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
        print_counter += 1;
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
                total_duration, inner_duration, min_measured_period, max_measured_period
            );
            max_measured_period = 0;
            min_measured_period = 10000;
        }
        previous_t1 = t1;
        ticker.next().await;
    }
}

#[allow(dead_code)]
async fn reapply_motor_configuration(
    front_left: &Motor,
    front_right: &Motor,
    rear_left: &Motor,
    rear_right: &Motor,
    config: &StoredConfig,
) {
    for _ in 0..5 {
        info!("BEGINNING RESET");
        info!("Front left");
        front_left.disable_3d_mode().await;
        front_left.set_direction(config.front_left_direction).await;

        info!("Front right");
        front_right.disable_3d_mode().await;
        front_right
            .set_direction(config.front_right_direction)
            .await;

        info!("Rear left");
        rear_left.disable_3d_mode().await;
        rear_left.set_direction(config.rear_left_direction).await;

        info!("Rear right");
        rear_right.disable_3d_mode().await;
        rear_right.set_direction(config.rear_right_direction).await;

        info!("RESET END");
        Timer::after_millis(100).await;
    }
}
