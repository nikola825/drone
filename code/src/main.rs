#![no_std]
#![no_main]

use crsf::{crsf_receiver_task, crsf_telemetry_task, CRSFChannels};
use embassy_executor::Spawner;
use embassy_stm32::adc::{Adc, AdcChannel};
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, i2c, peripherals, Config};
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    usart::{self},
};
use embassy_sync::lazy_lock::LazyLock;
use embassy_time::{Duration, Instant, Ticker, Timer};
use icm42688::ICM42688;
use logging::info;
use motors::{disarm, drive, Motor, MotorsContext};
use navigation::{navigate, NavigationContext};
use storage::Store;

mod crsf;
mod dshot;
mod icm42688;
mod logging;
mod motors;
mod navigation;
mod nopdelays;
mod storage;

bind_interrupts!(struct Irqs {
    USART2 => usart::InterruptHandler<peripherals::USART2>;
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

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

    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(20_000_000), // 20 MHz HSE
            mode: HseMode::Oscillator,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV20,  // 20 MHz / 20 = 1MHz
            mul: PllMul::MUL384,       // 1MHz * 384 = 384 MHz
            divp: Some(PllPDiv::DIV4), // P = 384 MHz / 4 = 96 MHz
            divq: Some(PllQDiv::DIV8), // Q = 384 MHz / 8 = 48 MHz
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1; // AHB = 96 MHz / 1 = 96 MHz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // APB1 = 96 MHz / 2 = 48 MHz
        config.rcc.apb2_pre = APBPrescaler::DIV1; // APB2 = 96 MHz / 2 = 48 MHz
        config.rcc.sys = Sysclk::PLL1_P; // sysclk = P = 96 MHz
        config.rcc.mux.clk48sel = mux::Clk48sel::PLL1_Q
    }

    let peripherals = embassy_stm32::init(config);
    let mut blue: Output = Output::new(peripherals.PA14, Level::Low, Speed::VeryHigh);
    let mut green: Output = Output::new(peripherals.PA4, Level::Low, Speed::VeryHigh);
    let mut yellow: Output = Output::new(peripherals.PA13, Level::Low, Speed::VeryHigh);

    #[cfg(feature = "usb-logging")]
    {
        use logging::init_usb_logging;
        init_usb_logging(
            peripherals.USB_OTG_FS,
            peripherals.PA12,
            peripherals.PA11,
            &_spawner,
        )
        .await;
    }
    #[cfg(feature = "rtt-logging")]
    {}

    green.set_high();

    let mut battery_adc = Adc::new(peripherals.ADC1);
    battery_adc.set_resolution(embassy_stm32::adc::Resolution::BITS12);

    let mut imu = ICM42688::new(
        peripherals.SPI3,
        peripherals.PB3,
        peripherals.PB5,
        peripherals.PB4,
        peripherals.DMA1_CH5,
        peripherals.DMA1_CH0,
        peripherals.PB9,
    );
    imu.init().await;
    Timer::after_millis(10).await;

    yellow.set_high();

    let front_left = Motor::new(peripherals.PB0);
    let front_right = Motor::new(peripherals.PB1);
    let rear_left = Motor::new(peripherals.PA7);
    let rear_right = Motor::new(peripherals.PA6);

    let (crsf_rx, crsf_tx) = crsf::make_uart_pair(
        peripherals.USART2,
        peripherals.PA3,
        peripherals.PA2,
        peripherals.DMA1_CH6,
        peripherals.DMA1_CH7,
        Irqs,
    );

    blue.set_high();

    _spawner
        .spawn(crsf_receiver_task(crsf_rx, STORE.get()))
        .unwrap();
    _spawner
        .spawn(crsf_telemetry_task(
            battery_adc,
            peripherals.PA5.degrade_adc(),
            crsf_tx,
        ))
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
        let snapshot = store.snapshot().await;
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
            info!("TICK {}", duration);
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

    #[allow(clippy::empty_loop)]
    loop {}
}
