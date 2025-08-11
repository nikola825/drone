use embassy_executor::InterruptExecutor;
use embassy_stm32::{
    adc::AdcChannel,
    bind_interrupts,
    gpio::Pin,
    interrupt::{InterruptExt, Priority},
    peripherals::ADC1,
    time::Hertz,
    usart::{self},
    usb::{self},
    Config, Peripherals,
};

pub const FLASH_SIZE: u32 =
    embassy_stm32::flash::BANK1_REGION.size + embassy_stm32::flash::BANK2_REGION.size;
pub const STORED_CONFIG_START: u32 = FLASH_SIZE - STORED_CONFIG_STRUCT_SIZE;
pub const FLASH_ERASE_SIZE: u32 = embassy_stm32::flash::BANK2_REGION.erase_size;
pub const FLASH_ERASE_START: u32 = FLASH_SIZE - FLASH_ERASE_SIZE;

use embassy_stm32::interrupt;

pub use embassy_stm32::peripherals::PA11 as USB_DM;
pub use embassy_stm32::peripherals::PA12 as USB_DP;
pub use embassy_stm32::peripherals::USB_OTG_FS as USB_PERIPHERAL;

use crate::{
    config_storage::STORED_CONFIG_STRUCT_SIZE,
    generic_hardware_type,
    hw_select::{
        Hardware, OptionalOutput, SpiHardware, SpiMaker, UartHardware, UartMaker, VoltageReader,
    },
};

use super::Spawners;

bind_interrupts!(pub struct Irqs {
    USART2 => usart::InterruptHandler<embassy_stm32::peripherals::USART2>;
    UART4 => usart::InterruptHandler<embassy_stm32::peripherals::UART4>;
    UART7 => usart::InterruptHandler<embassy_stm32::peripherals::UART7>;
    OTG_FS => usb::InterruptHandler<USB_PERIPHERAL>;
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

pub type BatteryMeter = VoltageReader<ADC1>;

fn make_config() -> Config {
    let mut config = Config::default();

    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000), // 20 MHz HSE
            mode: HseMode::Oscillator,
        });
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV1,   // 8 MHz / 1 = 8 MHz
            mul: PllMul::MUL120,       // 8MHz * 120 = 960 MHz
            divp: Some(PllDiv::DIV2),  // P = 960 MHz / 2 = 480 MHz
            divq: Some(PllDiv::DIV2),  // Q = 960 MHz / 4 = 240 MHz
            divr: Some(PllDiv::DIV10), // R = 960 MHz / 10 = 96 MHz
        });

        config.rcc.pll2 = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV2,  // 8 MHz / 2 = 4 MHz
            mul: PllMul::MUL125,      // 4 MHz * 125 = 500 MHz
            divp: Some(PllDiv::DIV4), // P = 500 MHz / 4 = 125 MHz
            divq: Some(PllDiv::DIV4), // Q = 500 MHz / 4 = 125 MHz
            divr: Some(PllDiv::DIV5), // R = 500 MHz / 5 = 100 MHz
        });

        config.rcc.pll3 = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV1,  // 8 MHz / 1 = 8 MHz
            mul: PllMul::MUL25,       // 8 MHz * 25 = 200 MHz
            divp: Some(PllDiv::DIV1), // P = 200 MHz / 1 = 200 MHz
            divq: Some(PllDiv::DIV2), // Q = 200 MHz / 2 = 100 MHz
            divr: Some(PllDiv::DIV2), // R = 200 MHz / 2 = 100 MHz
        });

        config.rcc.sys = Sysclk::PLL1_P; // sysclk = P = 550MHz;
        config.rcc.d1c_pre = AHBPrescaler::DIV1; // D1C = sysclk / 1 = 480 MHZ
        config.rcc.ahb_pre = AHBPrescaler::DIV2; // AHB = 480 MHz / 2 = 240 MHz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // APB1 = 240 MHz / 2 = 120 MHz
        config.rcc.apb2_pre = APBPrescaler::DIV2; // APB2 = 240 MHz / 2 = 120 MHz
        config.rcc.apb3_pre = APBPrescaler::DIV2; // APB3 = 240 MHz / 2 = 120 MHz
        config.rcc.apb4_pre = APBPrescaler::DIV2; // APB4 = 240 MHz / 2 = 120 MHz

        config.rcc.mux.usbsel = mux::Usbsel::HSI48; // USB CLK = PLL3.Q = 48 MHz
        config.rcc.mux.adcsel = mux::Adcsel::PLL3_R; // USB CLK = PLL3.R = 100 MHz
        config.rcc.mux.spi123sel = mux::Saisel::PLL3_P; // SPI123 CLK = PLL3.P = 200 MHz;
        config.rcc.mux.usart234578sel = mux::Usart234578sel::PLL2_Q; // USART234578 CLK = PLL2.Q  = 125 MHz
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

    unsafe {
        let peripherals = cortex_m::Peripherals::steal();
        let mut scb = peripherals.SCB;
        scb.set_sleeponexit();
        scb.enable_icache();
        scb.enable_fpu();
    }

    peripherals
}

pub fn make_hardware() -> generic_hardware_type!() {
    let peripherals = make_peripherals();

    Hardware {
        blue_pin: peripherals.PE3,
        green_pin: peripherals.PE2,
        yellow_pin: peripherals.PE4,

        usb_peripheral: peripherals.USB_OTG_FS,
        usb_dp: peripherals.PA12,
        usb_dm: peripherals.PA11,

        battery_meter: BatteryMeter::new(peripherals.PA4.degrade_adc(), peripherals.ADC1),

        imu_spi: SpiHardware {
            peripheral: peripherals.SPI1,
            sck_pin: peripherals.PB3,
            miso_pin: peripherals.PB4,
            mosi_pin: peripherals.PB5,

            rx_dma: peripherals.DMA1_CH0,
            tx_dma: peripherals.DMA1_CH1,
            cs_pin: peripherals.PB7,
        },

        motor0_pin: peripherals.PE12,
        motor1_pin: peripherals.PE13,
        motor2_pin: peripherals.PE14,
        motor3_pin: peripherals.PE15,

        radio_uart: UartHardware {
            peripheral: peripherals.UART4,
            tx_pin: peripherals.PA0,
            rx_pin: peripherals.PA1,
            rx_dma: peripherals.DMA1_CH4,
            tx_dma: peripherals.DMA1_CH5,
            irqs: Irqs,
        },

        flash: peripherals.FLASH,

        vtx_power_toggle: OptionalOutput::new(peripherals.PD8, embassy_stm32::gpio::Level::Low),
        msp_uart: Some(UartHardware {
            peripheral: peripherals.UART7,
            tx_pin: peripherals.PE8,
            rx_pin: peripherals.PE7,
            rx_dma: peripherals.DMA1_CH6,
            tx_dma: peripherals.DMA1_CH7,
            irqs: Irqs,
        }),
        gps_uart: Some(UartHardware {
            peripheral: peripherals.USART2,
            rx_pin: peripherals.PA3,
            tx_pin: peripherals.PA2,
            tx_dma: peripherals.DMA1_CH2,
            rx_dma: peripherals.DMA1_CH3,
            irqs: Irqs,
        }),
    }
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
        nop250!();
    };
}

#[macro_export]
macro_rules! dshot_nop_0_to_1 {
    () => {
        nop250!();
    };
}

#[macro_export]
macro_rules! dshot_nop_remainder {
    () => {
        nop100!();
    };
}
