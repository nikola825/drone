use embassy_stm32::{
    bind_interrupts,
    peripherals::{DMA1_CH4, DMA1_CH5, DMA1_CH6, DMA1_CH7, PA0, PA1, PE7, PE8, UART4, UART7},
    time::Hertz,
    usart::{self},
    usb::{self},
    Config, Peripherals,
};

pub use embassy_stm32::peripherals::PA11 as USB_DM;
pub use embassy_stm32::peripherals::PA12 as USB_DP;
pub use embassy_stm32::peripherals::USB_OTG_HS as USB_PERIPHERAL;

use super::UartHardware;

bind_interrupts!(pub struct Irqs {
    USART2 => usart::InterruptHandler<embassy_stm32::peripherals::USART2>;
    UART4 => usart::InterruptHandler<embassy_stm32::peripherals::UART4>;
    UART7 => usart::InterruptHandler<embassy_stm32::peripherals::UART7>;
    OTG_HS => usb::InterruptHandler<USB_PERIPHERAL>;
});

#[allow(dead_code)]
pub struct ExtraHardware {
    pub uart4: UartHardware<UART4, embassy_stm32::peripherals::UART4, PA1, PA0, DMA1_CH4, DMA1_CH5>,
    pub uart7: UartHardware<UART7, embassy_stm32::peripherals::UART7, PE7, PE8, DMA1_CH6, DMA1_CH7>,
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
            mul: PllMul::MUL275,      // 2MHz * 275 = 550 MHz
            divp: Some(PllDiv::DIV1), // P = 550 MHz / 1 = 550 MHz
            divq: Some(PllDiv::DIV2), // Q = 550 MHz / 2 = 275 MHz
            divr: Some(PllDiv::DIV2), // R = 550 MHz / 2 = 275 MHz
        });

        config.rcc.pll2 = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV2,  // 20 MHz / 2 = 10 MHz
            mul: PllMul::MUL25,       // 10MHz * 25 = 250 MHz
            divp: Some(PllDiv::DIV1), // P = 250 MHz / 1 = 250 MHz
            divq: Some(PllDiv::DIV1), // Q = 250 MHz / 1 = 250 MHz
            divr: Some(PllDiv::DIV1), // R = 250 MHz / 1 = 250 MHz
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
        config.rcc.d1c_pre = AHBPrescaler::DIV1; // D1C = sysclk / 1 = 550 MHZ
        config.rcc.ahb_pre = AHBPrescaler::DIV2; // AHB = 550 MHz / 2 = 275 MHz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // APB1 = 275 MHz / 2 = 137.5 MHz
        config.rcc.apb2_pre = APBPrescaler::DIV2; // APB2 = 275 MHz / 2 = 137.5 MHz
        config.rcc.apb3_pre = APBPrescaler::DIV2; // APB3 = 275 MHz / 2 = 137.5 MHz
        config.rcc.apb4_pre = APBPrescaler::DIV2; // APB4 = 275 MHz / 2 = 137.5 MHz

        config.rcc.mux.usbsel = mux::Usbsel::PLL3_Q; // USB CLK = PLL3.Q = 48 MHz
        config.rcc.mux.adcsel = mux::Adcsel::PLL3_R; // USB CLK = PLL3.R = 160 MHz
        config.rcc.mux.spi123sel = mux::Saisel::PLL2_P; // SPI123 CLK = PLL2.P = 250MHz;
        config.rcc.mux.usart234578sel = mux::Usart234578sel::PCLK1;
        /*config.rcc.hsi48 = Some(Hsi48Config {
            sync_from_usb: true,
        });*/
    }

    return config;
}
pub fn make_peripherals() -> Peripherals {
    
    let config = make_config();
    let peripherals = embassy_stm32::init(config);
    return peripherals;
}

#[macro_export]
macro_rules! get_hardware {
    () => {{
        use crate::hw_select::stm32h723::*;
        use crate::hw_select::*;
        let peripherals = make_peripherals();

        Hardware {
            blue_pin: peripherals.PC0,
            yellow_pin: peripherals.PC1,
            green_pin: peripherals.PC13,

            usb_dm: peripherals.PA11,
            usb_dp: peripherals.PA12,
            usb_peripheral: peripherals.USB_OTG_HS,

            imu_sck: peripherals.PB3,
            imu_miso: peripherals.PB4,
            imu_mosi: peripherals.PB5,
            imu_spi: peripherals.SPI1,
            imu_rx_dma: peripherals.DMA1_CH0,
            imu_tx_dma: peripherals.DMA1_CH1,
            imu_cs_pin: peripherals.PB7,

            bat_adc: peripherals.ADC1,
            bat_adc_pin: peripherals.PA4.degrade_adc(),

            motor0_pin: peripherals.PE12,
            motor1_pin: peripherals.PE13,
            motor2_pin: peripherals.PE14,
            motor3_pin: peripherals.PE15,

            radio_uart: UartHardware {
                peripheral: peripherals.USART2,
                rx_pin: peripherals.PA3,
                tx_pin: peripherals.PA2,
                tx_dma: peripherals.DMA1_CH2,
                rx_dma: peripherals.DMA1_CH3,
            },

            extra: ExtraHardware {
                uart4: UartHardware {
                    peripheral: peripherals.UART4,
                    tx_pin: peripherals.PA0,
                    rx_pin: peripherals.PA1,
                    rx_dma: peripherals.DMA1_CH4,
                    tx_dma: peripherals.DMA1_CH5,
                },
                uart7: UartHardware {
                    peripheral: peripherals.UART7,
                    tx_pin: peripherals.PE8,
                    rx_pin: peripherals.PE7,
                    rx_dma: peripherals.DMA1_CH6,
                    tx_dma: peripherals.DMA1_CH7,
                },
            },
        }
    }};
}
