#![no_std]
#![no_main]

use crsf::{crsf_receiver_task, crsf_telemetry_task, CRSFChannels};
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, i2c, peripherals, Config};
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    usart::{self},
};
use embassy_sync::lazy_lock::LazyLock;
use embassy_time::{Duration, Instant, Ticker, Timer};
use icm42688::ICM42688;
use motors::{disarm, drive, Motor, MotorsContext};
use navigation::{navigate, NavigationContext};
use storage::Store;
use {defmt_rtt as _, panic_probe as _};

mod crsf;
mod dshot;
mod icm42688;
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
    pub armed: bool,
    pub motor_context: MotorsContext,
    pub navigation_context: NavigationContext,
}

impl DroneContext {
    pub fn update_armed(&mut self, commands: &CRSFChannels) {
        let stay_armed = self.armed & commands.armed();
        let arm_at_zero = commands.armed() && commands.throttle() < 10;
        self.armed = commands.is_fresh() && (stay_armed || arm_at_zero);
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    static STORE: LazyLock<Store> = LazyLock::new(|| Store::new());

    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(20_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV20,
            mul: PllMul::MUL400,
            divp: Some(PllPDiv::DIV4), // 20MHz / 20 * 400 / 4 = 100Mhz.
            divq: Some(PllQDiv::DIV4),
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV2;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.sys = Sysclk::PLL1_P;
    }

    let peripherals = embassy_stm32::init(config);

    let mut blue: Output = Output::new(peripherals.PA11, Level::Low, Speed::VeryHigh);
    let mut green: Output = Output::new(peripherals.PA4, Level::Low, Speed::VeryHigh);
    let mut yellow: Output = Output::new(peripherals.PA12, Level::Low, Speed::VeryHigh);

    green.set_high();

    let mut battery_adc = Adc::new(peripherals.ADC1);
    battery_adc.set_resolution(embassy_stm32::adc::Resolution::BITS10);

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

    yellow.set_high();

    let motor0 = Motor::new(peripherals.PB0);
    let motor1 = Motor::new(peripherals.PB1);
    let motor2 = Motor::new(peripherals.PA6);
    let motor3 = Motor::new(peripherals.PA7);

    Timer::after_millis(10).await;

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
        .spawn(crsf_telemetry_task(battery_adc, peripherals.PA5, crsf_tx))
        .unwrap();

    let context = DroneContext {
        armed: false,
        motor_context: MotorsContext::new(motor3, motor1, motor0, motor2),
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
    let mut ticker = Ticker::every(Duration::from_micros(1000));
    let mut x = 0;

    blue_led.set_low();
    green_led.set_low();
    yellow_led.set_high();

    let mut duration = 0f32;
    loop {
        let t1=Instant::now();
        let snapshot = store.snapshot().await;

        context.update_armed(&snapshot.channels);

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

        navigate(&mut imu, &mut context, &snapshot.channels);

        if context.armed {
            drive(&mut context);
        } else {
            disarm(&mut context).await;
        }

        let t2 = Instant::now();

        duration = (t2-t1).as_micros() as f32 *0.5 + duration*0.5;
        x = x + 1;
        if x > 800 {
            x = 0;
            info!("TICK {} {}", duration, context.navigation_context.yaw_input);
        }

        ticker.next().await;
    }
}
