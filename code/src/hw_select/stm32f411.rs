use embassy_stm32::{
    bind_interrupts,
    usart::{self},
    usb::{self},
};

pub use embassy_stm32::peripherals::PA11 as USB_DM;
pub use embassy_stm32::peripherals::PA12 as USB_DP;
pub use embassy_stm32::peripherals::USB_OTG_FS as USB_PERIPHERAL;

bind_interrupts!(pub struct Irqs {
    USART2 => usart::InterruptHandler<embassy_stm32::peripherals::USART2>;
    OTG_FS => usb::InterruptHandler<USB_PERIPHERAL>;
});

pub struct ExtraHardware {}

#[macro_export]
macro_rules! get_hardware {
    () => {{
        use crate::hw_select::stm32f411::*;
        use crate::hw_select::*;
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
        Hardware {
            blue_pin: peripherals.PA14,
            green_pin: peripherals.PA4,
            yellow_pin: peripherals.PA13,

            usb_dm: peripherals.PA11,
            usb_dp: peripherals.PA12,
            usb_peripheral: peripherals.USB_OTG_FS,

            bat_adc: peripherals.ADC1,
            bat_adc_pin: peripherals.PA5.degrade_adc(),

            imu_sck: peripherals.PB3,
            imu_mosi: peripherals.PB5,
            imu_miso: peripherals.PB4,
            imu_spi: peripherals.SPI3,
            imu_rx_dma: peripherals.DMA1_CH0,
            imu_tx_dma: peripherals.DMA1_CH5,
            imu_cs_pin: peripherals.PB9,

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
            },

            extra: ExtraHardware {},
        }
    }};
}
