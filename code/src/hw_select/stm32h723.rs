use core::ops::RangeInclusive;

use embassy_executor::InterruptExecutor;
use embassy_stm32::{
    adc::{Adc, AnyAdcChannel},
    bind_interrupts,
    interrupt::{InterruptExt, Priority},
    pac::VREFBUF,
    peripherals::{
        ADC1, DMA1_CH2, DMA1_CH3, DMA1_CH4, DMA1_CH5, PA0, PA1, PA2, PA3, UART4, USART2,
    },
    time::Hertz,
    usart::{self},
    usb::{self},
    Config, Peripherals,
};

use embassy_stm32::interrupt;

pub use embassy_stm32::peripherals::PA11 as USB_DM;
pub use embassy_stm32::peripherals::PA12 as USB_DP;
pub use embassy_stm32::peripherals::USB_OTG_HS as USB_PERIPHERAL;

use super::{Spawners, UartHardware};

bind_interrupts!(pub struct Irqs {
    USART2 => usart::InterruptHandler<embassy_stm32::peripherals::USART2>;
    UART4 => usart::InterruptHandler<embassy_stm32::peripherals::UART4>;
    UART7 => usart::InterruptHandler<embassy_stm32::peripherals::UART7>;
    OTG_HS => usb::InterruptHandler<USB_PERIPHERAL>;
});

// High-priority executor used mainly for PID loop
pub static EXECUTOR_HIGH: InterruptExecutor = InterruptExecutor::new();
// Low-priority executor used for less important tasks
pub static EXECUTOR_LOW: InterruptExecutor = InterruptExecutor::new();

#[interrupt]
unsafe fn UART5() {
    EXECUTOR_HIGH.on_interrupt()
}

#[interrupt]
unsafe fn USART3() {
    EXECUTOR_LOW.on_interrupt()
}

#[allow(dead_code)]
pub struct ExtraHardware {
    pub msp_uart:
        UartHardware<UART4, embassy_stm32::peripherals::UART4, PA1, PA0, DMA1_CH4, DMA1_CH5, Irqs>,
    pub uart2: UartHardware<
        USART2,
        embassy_stm32::peripherals::USART2,
        PA3,
        PA2,
        DMA1_CH2,
        DMA1_CH3,
        Irqs,
    >,
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

        adc1.set_averaging(embassy_stm32::adc::Averaging::Samples16);
        adc1.set_sample_time(embassy_stm32::adc::SampleTime::CYCLES32_5);

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

fn make_config() -> Config {
    let mut config = Config::default();

    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(20_000_000), // 20 MHz HSE
            mode: HseMode::Oscillator,
        });
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV10, // 20 MHz / 10 = 2 MHz
            mul: PllMul::MUL240,      // 2MHz * 240 = 480 MHz
            divp: Some(PllDiv::DIV1), // P = 480 MHz / 1 = 480 MHz
            divq: Some(PllDiv::DIV2), // Q = 480 MHz / 2 = 240 MHz
            divr: Some(PllDiv::DIV2), // R = 480 MHz / 2 = 240 MHz
        });

        config.rcc.pll2 = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV2,  // 20 MHz / 2 = 10 MHz
            mul: PllMul::MUL24,       // 10MHz * 24 = 240 MHz
            divp: Some(PllDiv::DIV1), // P = 240 MHz / 1 = 240 MHz
            divq: Some(PllDiv::DIV2), // Q = 240 MHz / 2 = 120 MHz
            divr: Some(PllDiv::DIV1), // R = 240 MHz / 1 = 240 MHz
        });

        config.rcc.pll3 = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV2,   // 20 MHz / 2 = 10 MHz
            mul: PllMul::MUL48,        // 10MHz * 48 = 480 MHz
            divp: Some(PllDiv::DIV2),  // P = 480 MHz / 2  = 240 MHz
            divq: Some(PllDiv::DIV10), // Q = 480 MHz / 10 = 48 MHz
            divr: Some(PllDiv::DIV3),  // R = 480 MHz / 3  = 160 MHz
        });

        config.rcc.sys = Sysclk::PLL1_P; // sysclk = P = 550MHz;
        config.rcc.d1c_pre = AHBPrescaler::DIV1; // D1C = sysclk / 1 = 480 MHZ
        config.rcc.ahb_pre = AHBPrescaler::DIV2; // AHB = 480 MHz / 2 = 240 MHz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // APB1 = 240 MHz / 2 = 120 MHz
        config.rcc.apb2_pre = APBPrescaler::DIV2; // APB2 = 240 MHz / 2 = 120 MHz
        config.rcc.apb3_pre = APBPrescaler::DIV2; // APB3 = 240 MHz / 2 = 120 MHz
        config.rcc.apb4_pre = APBPrescaler::DIV2; // APB4 = 240 MHz / 2 = 120 MHz

        config.rcc.mux.usbsel = mux::Usbsel::HSI48; // USB CLK = PLL3.Q = 48 MHz
        config.rcc.mux.adcsel = mux::Adcsel::PLL3_R; // USB CLK = PLL3.R = 160 MHz
        config.rcc.mux.spi123sel = mux::Saisel::PLL2_P; // SPI123 CLK = PLL2.P = 240 MHz;
        config.rcc.mux.usart234578sel = mux::Usart234578sel::PLL2_Q; // USART234578 CLK = PLL2.Q  = 120 MHz
        config.rcc.hsi48 = Some(Hsi48Config {
            sync_from_usb: true,
        });

        //config.rcc.voltage_scale = VoltageScale::Scale0;
    }

    config
}
pub fn make_peripherals() -> Peripherals {
    let config = make_config();
    let peripherals = embassy_stm32::init(config);

    VREFBUF.csr().modify(|x| {
        x.set_envr(false);
        x.set_hiz(embassy_stm32::pac::vrefbuf::vals::Hiz::HIGH_Z);
    });

    unsafe {
        let peripherals = cortex_m::Peripherals::steal();
        let mut scb = peripherals.SCB;
        scb.set_sleeponexit();
        scb.enable_icache();
        scb.enable_fpu();
    }

    peripherals
}

#[macro_export]
macro_rules! get_hardware {
    () => {{
        use $crate::hw_select::stm32h723::*;
        use $crate::hw_select::*;
        let peripherals = make_peripherals();

        Hardware {
            blue_pin: peripherals.PE3,
            yellow_pin: peripherals.PE4,
            green_pin: peripherals.PE2,

            usb_dm: peripherals.PA11,
            usb_dp: peripherals.PA12,
            usb_peripheral: peripherals.USB_OTG_HS,

            imu_spi: SpiHardware {
                peripheral: peripherals.SPI1,
                sck_pin: peripherals.PB3,
                miso_pin: peripherals.PB4,
                mosi_pin: peripherals.PB5,

                rx_dma: peripherals.DMA1_CH0,
                tx_dma: peripherals.DMA1_CH1,
                cs_pin: peripherals.PB7,
            },

            adc_reader: AdcReader::new(peripherals.PA4.degrade_adc(), peripherals.ADC1),

            motor0_pin: peripherals.PE12,
            motor1_pin: peripherals.PE13,
            motor2_pin: peripherals.PE14,
            motor3_pin: peripherals.PE15,

            radio_uart: UartHardware {
                peripheral: peripherals.UART7,
                tx_pin: peripherals.PE8,
                rx_pin: peripherals.PE7,
                rx_dma: peripherals.DMA1_CH6,
                tx_dma: peripherals.DMA1_CH7,
                irqs: Irqs,
            },

            extra: ExtraHardware {
                msp_uart: UartHardware {
                    peripheral: peripherals.UART4,
                    tx_pin: peripherals.PA0,
                    rx_pin: peripherals.PA1,
                    rx_dma: peripherals.DMA1_CH4,
                    tx_dma: peripherals.DMA1_CH5,
                    irqs: Irqs,
                },
                uart2: UartHardware {
                    peripheral: peripherals.USART2,
                    rx_pin: peripherals.PA3,
                    tx_pin: peripherals.PA2,
                    tx_dma: peripherals.DMA1_CH3,
                    rx_dma: peripherals.DMA1_CH2,
                    irqs: Irqs,
                },
            },
        }
    }};
}

pub fn get_spawners() -> Spawners {
    interrupt::UART5.set_priority(Priority::P6);
    let spawner_high = EXECUTOR_HIGH.start(interrupt::UART5);

    interrupt::USART3.set_priority(Priority::P7);
    let spawner_low = EXECUTOR_LOW.start(interrupt::USART3);

    Spawners {
        spawner_high,
        spawner_low,
    }
}

#[macro_export]
macro_rules! dshot_nop_0 {
    () => {
        nop500!();
    };
}

#[macro_export]
macro_rules! dshot_nop_0_to_1 {
    () => {
        nop500!();
    };
}

#[macro_export]
macro_rules! dshot_nop_remainder {
    () => {
        nop200!();
    };
}
