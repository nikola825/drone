#![no_std]
#![no_main]

use battery_monitor::battery_monitor_task;
use crsf::{crsf_receiver_task, crsf_telemetry_task, CRSFChannels};
use embassy_executor::Spawner;
use embassy_stm32::adc::AdcChannel;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_sync::lazy_lock::LazyLock;
use embassy_time::{Duration, Instant, Ticker, Timer};
use icm42688::ICM42688;
use logging::{info, init_logging};
use motors::{disarm, drive, Motor, MotorInputs, MotorsContext};
use msp_osd::osd_refresh_task;
use navigation::{navigate, NavigationContext};
use storage::Store;

mod battery_monitor;
mod crsf;
mod dshot;
mod hw_select;
mod icm42688;
mod logging;
mod motors;
mod msp_osd;
mod navigation;
mod nopdelays;
mod storage;

struct DroneContext {
    armed: bool,
    motor_context: MotorsContext,
    navigation_context: NavigationContext,
}

impl DroneContext {
    fn update_armed(&mut self, commands: &CRSFChannels) {
        let stay_armed = self.armed & commands.armed();
        let arm_at_zero = commands.armed() && commands.throttle() < 10;
        self.armed = commands.is_fresh() && (stay_armed || arm_at_zero);
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    static STORE: LazyLock<Store> = LazyLock::new(Store::new);

    let hardware = get_hardware!();

    let mut blue = Output::new(hardware.blue_pin, Level::Low, Speed::VeryHigh);
    let mut green = Output::new(hardware.green_pin, Level::Low, Speed::VeryHigh);
    let mut yellow = Output::new(hardware.yellow_pin, Level::Low, Speed::VeryHigh);

    init_logging!(hardware, _spawner);

    green.set_high();

    let mut imu = ICM42688::new(hardware.imu_spi);
    imu.init().await;
    Timer::after_millis(10).await;

    yellow.set_high();

    let front_left = Motor::new(hardware.motor0_pin);
    let front_right = Motor::new(hardware.motor3_pin);
    let rear_left = Motor::new(hardware.motor2_pin);
    let rear_right = Motor::new(hardware.motor1_pin);

    let (crsf_rx, crsf_tx) = crsf::make_uart_pair(hardware.extra.uart7);

    let msp_uart = hardware.extra.uart4;
    let (_, msp_tx) = msp_osd::make_msp_uart_pair(msp_uart).await;

    blue.set_high();

    _spawner
        .spawn(crsf_receiver_task(crsf_rx, STORE.get()))
        .unwrap();
    _spawner
        .spawn(crsf_telemetry_task(crsf_tx, STORE.get()))
        .unwrap();
    _spawner
        .spawn(battery_monitor_task(hardware.adc_reader, STORE.get()))
        .unwrap();
    _spawner
        .spawn(osd_refresh_task(msp_tx, STORE.get()))
        .unwrap();

    let context = DroneContext {
        armed: false,
        motor_context: MotorsContext::new(front_left, front_right, rear_left, rear_right),
        navigation_context: NavigationContext::new(),
    };
    _spawner
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
    store: &'static Store,
) {
    const PID_PERIOD_US: u64 = 1000;
    let mut ticker = Ticker::every(Duration::from_micros(PID_PERIOD_US));

    let mut print_counter = 0;

    blue_led.set_low();
    green_led.set_low();
    yellow_led.set_high();

    let mut duration = 0f32;
    loop {
        let t1 = Instant::now();
        let snapshot = store.channel_snapshot().await;
        let command_inputs = &snapshot.channels;

        context.update_armed(command_inputs);

        green_led.set_level(match context.armed {
            true => Level::High,
            false => Level::Low,
        });

        yellow_led.set_level(match context.armed {
            true => Level::Low,
            false => Level::High,
        });

        blue_led.set_level(match snapshot.channels.is_fresh() {
            true => Level::High,
            false => Level::Low,
        });

        let motor_inputs = navigate(&mut imu, &mut context.navigation_context, command_inputs);

        if context.armed {
            drive(&mut context.motor_context, &motor_inputs);
        } else {
            disarm(
                &mut context.motor_context,
                &motor_inputs,
                command_inputs.beep(),
            )
            .await;
        }

        let t2 = Instant::now();

        duration = (t2 - t1).as_micros() as f32 * 0.5 + duration * 0.5;
        print_counter += 1;
        if print_counter > 800 * (1000 / PID_PERIOD_US) {
            print_counter = 0;
            info!("TICK {} {:?} {} {} {} {}", duration, imu.get_ypr_deg(), motor_inputs.motor_thrust, motor_inputs.pitch_input, motor_inputs.roll_input, motor_inputs.yaw_input);
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
    loop {
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
            .set_direction(motors::MotorDirection::Forward)
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
