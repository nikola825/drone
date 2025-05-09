#![no_std]
#![no_main]

use battery_monitor::init_battery_monitor;
use crsf::init_crsf_communication;
use embassy_executor::Spawner;
use embassy_stm32::adc::AdcChannel;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_sync::lazy_lock::LazyLock;
use embassy_time::{Duration, Instant, Ticker, Timer};
use icm42688::ICM42688;
use logging::{info, init_logging};
use motors::{disarm, drive_motors, Motor, MotorsContext};
use osd::init_osd;
use pid::{do_pid_iteration, PidContext};
use shared_state::SharedState;

mod arming;
mod battery_monitor;
mod channel_mapping;
mod crc8;
mod crsf;
mod dshot;
mod expo_rates;
mod hw_select;
mod icm42688;
mod logging;
mod motors;
mod nopdelays;
mod osd;
mod pid;
mod shared_state;

struct DroneContext {
    motor_context: MotorsContext,
    pid_context: PidContext,
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    static STORE: LazyLock<SharedState> = LazyLock::new(SharedState::new);

    let hardware = get_hardware!();

    let mut blue = Output::new(hardware.blue_pin, Level::Low, Speed::VeryHigh);
    let mut green = Output::new(hardware.green_pin, Level::Low, Speed::VeryHigh);
    let mut yellow = Output::new(hardware.yellow_pin, Level::Low, Speed::VeryHigh);

    init_logging!(hardware, spawner);

    green.set_high();

    let mut imu = ICM42688::new(hardware.imu_spi);
    imu.init().await;
    Timer::after_millis(10).await;

    yellow.set_high();

    let front_left = Motor::new(hardware.motor2_pin);
    let front_right = Motor::new(hardware.motor3_pin);
    let rear_left = Motor::new(hardware.motor1_pin);
    let rear_right = Motor::new(hardware.motor0_pin);

    init_crsf_communication(hardware.radio_uart, &spawner, STORE.get());
    init_battery_monitor(hardware.adc_reader, STORE.get(), &spawner);
    init_osd!(hardware, spawner, STORE.get());

    blue.set_high();

    // motor_reset(&front_left, &front_right, &rear_left, &rear_right).await;

    let context = DroneContext {
        motor_context: MotorsContext::new(front_left, front_right, rear_left, rear_right),
        pid_context: PidContext::new(),
    };

    spawner
        .spawn(tick_task(blue, green, yellow, imu, context, STORE.get()))
        .unwrap();
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
    const PID_PERIOD_US: u64 = 1000;
    let mut ticker = Ticker::every(Duration::from_micros(PID_PERIOD_US));

    let mut print_counter = 0;

    blue_led.set_low();
    green_led.set_low();
    yellow_led.set_high();

    let mut total_duration = 0f32;
    let mut inner_duration = 0f32;

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
            do_pid_iteration(&mut imu, &mut context.pid_context, &command_state.commands);

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

        if print_counter > 800 * (1000 / PID_PERIOD_US) {
            print_counter = 0;
            info!("TICK {} {}", total_duration, inner_duration);
        }

        ticker.next().await;
    }
}

#[allow(dead_code)]
async fn motor_reset(
    front_left: &Motor,
    front_right: &Motor,
    rear_left: &Motor,
    rear_right: &Motor,
) {
    for _ in 0..5 {
        info!("BEGINNING RESET");
        info!("Front left");
        front_left.disable_3d_mode().await;
        front_left
            .set_direction(motors::MotorDirection::Forward)
            .await;

        info!("Front right");
        front_right.disable_3d_mode().await;
        front_right
            .set_direction(motors::MotorDirection::Backward)
            .await;

        info!("Rear left");
        rear_left.disable_3d_mode().await;
        rear_left
            .set_direction(motors::MotorDirection::Backward)
            .await;

        info!("Rear right");
        rear_right.disable_3d_mode().await;
        rear_right
            .set_direction(motors::MotorDirection::Forward)
            .await;

        info!("RESET END");
        Timer::after_millis(100).await;
    }
}
