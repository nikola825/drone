use core::ops::RangeInclusive;

use embassy_executor::InterruptExecutor;
use embassy_stm32::{
    adc::{Adc, AnyAdcChannel},
    bind_interrupts,
    interrupt::{InterruptExt, Priority},
    peripherals::ADC1,
    time::Hertz,
    usart::{self},
    usb::{self},
    Config, Peripherals,
};

use embassy_stm32::interrupt;

pub use embassy_stm32::peripherals::PA11 as USB_DM;
pub use embassy_stm32::peripherals::PA12 as USB_DP;
pub use embassy_stm32::peripherals::USB_OTG_FS as USB_PERIPHERAL;

use super::Spawners;

bind_interrupts!(pub struct Irqs {
    USART2 => usart::InterruptHandler<embassy_stm32::peripherals::USART2>;
    OTG_FS => usb::InterruptHandler<USB_PERIPHERAL>;
});

// High-priority executor used mainly for PID loop
pub static EXECUTOR_HIGH: InterruptExecutor = InterruptExecutor::new();
// Low-priority executor used for less important tasks
pub static EXECUTOR_LOW: InterruptExecutor = InterruptExecutor::new();

#[interrupt]
unsafe fn USART1() {
    EXECUTOR_HIGH.on_interrupt()
}

#[interrupt]
unsafe fn USART6() {
    EXECUTOR_LOW.on_interrupt()
}

pub struct AdcReader {
    adc: Adc<'static, ADC1>,
    bat_pin: AnyAdcChannel<ADC1>,
    adc_range_max: u16,
    resistor_divider_factor: f32,
    acceptable_voltage_range: RangeInclusive<f32>,
    voltage_reference: f32,
}

impl AdcReader {
    pub fn new(bat_pin: AnyAdcChannel<ADC1>, adc1: ADC1) -> Self {
        let mut adc1 = Adc::new(adc1);

        adc1.set_sample_time(embassy_stm32::adc::SampleTime::CYCLES28);

        adc1.set_resolution(embassy_stm32::adc::Resolution::BITS12);

        AdcReader {
            adc: adc1,
            bat_pin,
            adc_range_max: 4096u16,
            resistor_divider_factor: 11f32,
            acceptable_voltage_range: 0f32..=20f32,
            voltage_reference: 3.3f32,
        }
    }

    pub fn get_bat(&mut self) -> f32 {
        let measurement = self.adc.blocking_read(&mut self.bat_pin);

        let measured_voltage = ((measurement as f32) * self.voltage_reference)
            / (self.adc_range_max as f32)
            * self.resistor_divider_factor;

        if self.acceptable_voltage_range.contains(&measured_voltage) {
            measured_voltage
        } else {
            0f32
        }
    }
}

pub struct ExtraHardware {}

fn make_config() -> Config {
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
        config.rcc.mux.clk48sel = mux::Clk48sel::PLL1_Q;
    }

    config
}

pub fn make_peripherals() -> Peripherals {
    let config = make_config();

    embassy_stm32::init(config)
}

#[macro_export]
macro_rules! get_hardware {
    () => {{
        use $crate::hw_select::stm32f411::*;
        use $crate::hw_select::*;

        let peripherals = make_peripherals();

        Hardware {
            blue_pin: peripherals.PA14,
            green_pin: peripherals.PA4,
            yellow_pin: peripherals.PA13,

            usb_dm: peripherals.PA11,
            usb_dp: peripherals.PA12,
            usb_peripheral: peripherals.USB_OTG_FS,

            adc_reader: AdcReader::new(peripherals.PA5.degrade_adc(), peripherals.ADC1),

            imu_spi: SpiHardware {
                peripheral: peripherals.SPI3,
                sck_pin: peripherals.PB3,
                miso_pin: peripherals.PB4,
                mosi_pin: peripherals.PB5,

                rx_dma: peripherals.DMA1_CH0,
                tx_dma: peripherals.DMA1_CH5,
                cs_pin: peripherals.PB9,
            },

            motor0_pin: peripherals.PB1,
            motor1_pin: peripherals.PB0,
            motor2_pin: peripherals.PA7,
            motor3_pin: peripherals.PA6,

            radio_uart: UartHardware {
                peripheral: peripherals.USART2,
                rx_pin: peripherals.PA3,
                tx_pin: peripherals.PA2,
                tx_dma: peripherals.DMA1_CH6,
                rx_dma: peripherals.DMA1_CH7,
                irqs: Irqs,
            },

            extra: ExtraHardware {},
        }
    }};
}

pub fn get_spawners() -> Spawners {
    interrupt::USART1.set_priority(Priority::P6);
    let spawner_high = EXECUTOR_HIGH.start(interrupt::USART1);

    interrupt::USART6.set_priority(Priority::P7);
    let spawner_low = EXECUTOR_LOW.start(interrupt::USART6);

    Spawners {
        spawner_high,
        spawner_low,
    }
}

#[macro_export]
macro_rules! dshot_nop_0 {
    () => {
        nop29!();
    };
}

#[macro_export]
macro_rules! dshot_nop_0_to_1 {
    () => {
        nop29!();
    };
}

#[macro_export]
macro_rules! dshot_nop_remainder {
    () => {
        nop19!();
    };
}
