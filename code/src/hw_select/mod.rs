use core::ops::RangeInclusive;

use embassy_executor::SendSpawner;
use embassy_stm32::{
    adc::{Adc, AnyAdcChannel},
    gpio::{Output, Pin},
    interrupt,
    mode::Async,
    pac::gpio::vals::Pupdr,
    spi::{self, BitOrder, MisoPin, Mode, MosiPin, SckPin, Spi},
    time::Hertz,
    usart::{InterruptHandler, Parity, StopBits, Uart, UartRx, UartTx},
    usb::{DmPin, DpPin},
    Peripheral,
};

#[cfg(feature = "stm32h723")]
mod stm32h723;
#[cfg(feature = "stm32h723")]
use stm32h723 as hardware_module;

#[cfg(feature = "stm32h743")]
mod stm32h743;
#[cfg(feature = "stm32h743")]
use stm32h743 as hardware_module;

#[cfg(feature = "stm32f411")]
mod stm32f411;
#[cfg(feature = "stm32f411")]
use stm32f411 as hardware_module;

pub use hardware_module::{
    get_spawners, make_hardware, BatteryMeter, Irqs, USB_DM, USB_DP, USB_PERIPHERAL,
};

#[cfg(feature = "flash_storage")]
pub use hardware_module::{FLASH_ERASE_START, FLASH_SIZE, STORED_CONFIG_START};

pub fn get_pin_gpio<T: Pin>(pin: &T) -> embassy_stm32::pac::gpio::Gpio {
    {
        unsafe {
            {
                embassy_stm32::pac::gpio::Gpio::from_ptr(
                    (1476526080usize + 1024usize * (pin.port() as usize)) as _,
                )
            }
        }
    }
}

pub struct UartHardware<
    UartType: embassy_stm32::usart::Instance + 'static,
    UartPeripheral: Peripheral<P = UartType> + 'static,
    UartRxPin: embassy_stm32::usart::RxPin<UartType> + 'static,
    UartTxPin: embassy_stm32::usart::TxPin<UartType> + 'static,
    RxDma: embassy_stm32::usart::RxDma<UartType> + 'static,
    TxDma: embassy_stm32::usart::TxDma<UartType> + 'static,
    IrqType: interrupt::typelevel::Binding<UartType::Interrupt, InterruptHandler<UartType>> + 'static,
> {
    pub peripheral: UartPeripheral,
    pub rx_pin: UartRxPin,
    pub tx_pin: UartTxPin,
    pub rx_dma: RxDma,
    pub tx_dma: TxDma,
    pub irqs: IrqType,
}

pub trait UartMaker {
    fn make_uart(
        self,
        baudrate: u32,
        parity: Parity,
        stop_bits: StopBits,
        rx_pullup: bool,
    ) -> (UartRx<'static, Async>, UartTx<'static, Async>);
}

impl<
        UartType: embassy_stm32::usart::Instance + 'static,
        UartPeripheral: Peripheral<P = UartType> + 'static,
        UartRxPin: embassy_stm32::usart::RxPin<UartType> + 'static,
        UartTxPin: embassy_stm32::usart::TxPin<UartType> + 'static,
        RxDma: embassy_stm32::usart::RxDma<UartType> + 'static,
        TxDma: embassy_stm32::usart::TxDma<UartType> + 'static,
        IrqType: interrupt::typelevel::Binding<UartType::Interrupt, InterruptHandler<UartType>> + 'static,
    > UartMaker
    for UartHardware<UartType, UartPeripheral, UartRxPin, UartTxPin, RxDma, TxDma, IrqType>
{
    fn make_uart(
        self,
        baudrate: u32,
        parity: Parity,
        stop_bits: StopBits,
        rx_pullup: bool,
    ) -> (UartRx<'static, Async>, UartTx<'static, Async>) {
        let mut uart_config = embassy_stm32::usart::Config::default();
        uart_config.baudrate = baudrate;
        uart_config.parity = parity;
        uart_config.stop_bits = stop_bits;

        let rx_gpio = get_pin_gpio(&self.rx_pin);
        let rx_pin_number = self.rx_pin.pin();

        let uart = Uart::new(
            self.peripheral,
            self.rx_pin,
            self.tx_pin,
            self.irqs,
            self.tx_dma,
            self.rx_dma,
            uart_config,
        )
        .unwrap();

        if rx_pullup {
            rx_gpio
                .pupdr()
                .modify(|w| w.set_pupdr(rx_pin_number as usize, Pupdr::PULL_UP));
        }

        let (tx, rx) = uart.split();

        (rx, tx)
    }
}

pub trait SpiMaker {
    fn make_spi(
        self,
        frequency: Hertz,
        mode: Mode,
        bit_order: BitOrder,
    ) -> (Output<'static>, Spi<'static, Async>);
}

pub struct SpiHardware<
    SpiType: embassy_stm32::spi::Instance + 'static,
    SpiPeripheral: Peripheral<P = SpiType> + 'static,
    SpiSckPin: SckPin<SpiType> + 'static,
    SpiMosiPin: MosiPin<SpiType> + 'static,
    SpiMisoPin: MisoPin<SpiType> + 'static,
    SpiTxDma: embassy_stm32::spi::TxDma<SpiType> + 'static,
    SpiRxDma: embassy_stm32::spi::RxDma<SpiType> + 'static,
    CsPin: Pin + 'static,
> {
    pub peripheral: SpiPeripheral,
    pub sck_pin: SpiSckPin,
    pub mosi_pin: SpiMosiPin,
    pub miso_pin: SpiMisoPin,
    pub rx_dma: SpiRxDma,
    pub tx_dma: SpiTxDma,
    pub cs_pin: CsPin,
}

impl<
        SpiType: embassy_stm32::spi::Instance + 'static,
        SpiPeripheral: Peripheral<P = SpiType> + 'static,
        SpiSckPin: SckPin<SpiType> + 'static,
        SpiMosiPin: MosiPin<SpiType> + 'static,
        SpiMisoPin: MisoPin<SpiType> + 'static,
        SpiTxDma: embassy_stm32::spi::TxDma<SpiType> + 'static,
        SpiRxDma: embassy_stm32::spi::RxDma<SpiType> + 'static,
        CsPin: Pin + 'static,
    > SpiMaker
    for SpiHardware<
        SpiType,
        SpiPeripheral,
        SpiSckPin,
        SpiMosiPin,
        SpiMisoPin,
        SpiTxDma,
        SpiRxDma,
        CsPin,
    >
{
    fn make_spi(
        self,
        frequency: Hertz,
        mode: Mode,
        bit_order: BitOrder,
    ) -> (Output<'static>, Spi<'static, embassy_stm32::mode::Async>) {
        let mut spi_config = spi::Config::default();
        spi_config.frequency = frequency;
        spi_config.mode = mode;
        spi_config.bit_order = bit_order;

        (
            Output::new(
                self.cs_pin,
                embassy_stm32::gpio::Level::High,
                embassy_stm32::gpio::Speed::VeryHigh,
            ),
            spi::Spi::new(
                self.peripheral,
                self.sck_pin,
                self.mosi_pin,
                self.miso_pin,
                self.tx_dma,
                self.rx_dma,
                spi_config,
            ),
        )
    }
}

pub struct OptionalOutput {
    toggle_output: Option<Output<'static>>,
}

impl OptionalOutput {
    #[allow(dead_code)]
    pub fn new<PinType: Pin + 'static>(
        pin: PinType,
        initial_level: embassy_stm32::gpio::Level,
    ) -> Self {
        Self {
            toggle_output: Some(Output::new(
                pin,
                initial_level,
                embassy_stm32::gpio::Speed::High,
            )),
        }
    }

    #[allow(dead_code)]
    pub fn unimplemented() -> Self {
        Self {
            toggle_output: None,
        }
    }

    pub fn set_high(&mut self) {
        if let Some(toggle) = self.toggle_output.as_mut() {
            toggle.set_high();
        }
    }

    #[allow(dead_code)]
    pub fn set_low(&mut self) {
        if let Some(toggle) = self.toggle_output.as_mut() {
            toggle.set_low();
        }
    }
}

pub struct VoltageReader<AdcType: embassy_stm32::adc::Instance> {
    pub adc: Adc<'static, AdcType>,
    pub pin: AnyAdcChannel<AdcType>,
    pub adc_range_max: u16,
    pub resistor_divider_factor: f32,
    pub acceptable_voltage_range: RangeInclusive<f32>,
    pub voltage_reference: f32,
}

impl<AdcType: embassy_stm32::adc::Instance> VoltageReader<AdcType> {
    pub fn new(pin: AnyAdcChannel<AdcType>, adc: AdcType) -> Self {
        let mut adc = Adc::new(adc);

        adc.set_averaging(embassy_stm32::adc::Averaging::Samples16);
        adc.set_sample_time(embassy_stm32::adc::SampleTime::CYCLES32_5);

        adc.set_resolution(embassy_stm32::adc::Resolution::BITS12);

        Self {
            adc,
            pin,
            adc_range_max: 4096u16,
            resistor_divider_factor: 11f32,
            acceptable_voltage_range: 0f32..=28f32,
            voltage_reference: 3.3f32,
        }
    }

    pub fn get_voltage(&mut self) -> f32 {
        let measurement = self.adc.blocking_read(&mut self.pin);

        let measured_voltage = ((measurement as f32) * self.voltage_reference)
            * self.resistor_divider_factor
            / (self.adc_range_max as f32);

        if self.acceptable_voltage_range.contains(&measured_voltage) {
            measured_voltage
        } else {
            0f32
        }
    }
}

#[allow(dead_code)]
pub struct Hardware<
    BluePin: Pin,
    GreenPin: Pin,
    YellowPin: Pin,
    UsbDp: DpPin<USB_PERIPHERAL>,
    UsbDm: DmPin<USB_PERIPHERAL>,
    ImuSpiMaker: SpiMaker,
    Motor0Pin: Pin,
    Motor1Pin: Pin,
    Motor2Pin: Pin,
    Motor3Pin: Pin,
    RadioUartMaker: UartMaker,
    MspUartMaker: UartMaker,
    GpsUartMaker: UartMaker,
> {
    pub blue_pin: BluePin,
    pub green_pin: GreenPin,
    pub yellow_pin: YellowPin,
    pub usb_peripheral: USB_PERIPHERAL,
    pub usb_dp: UsbDp,
    pub usb_dm: UsbDm,

    pub battery_meter: BatteryMeter,

    pub imu_spi: ImuSpiMaker,

    pub motor0_pin: Motor0Pin,
    pub motor1_pin: Motor1Pin,
    pub motor2_pin: Motor2Pin,
    pub motor3_pin: Motor3Pin,

    pub radio_uart: RadioUartMaker,

    pub flash: embassy_stm32::peripherals::FLASH,

    pub vtx_power_toggle: OptionalOutput,

    pub msp_uart: Option<MspUartMaker>,
    pub gps_uart: Option<GpsUartMaker>,
}

// Used to indicate a generic type of the make_hardware methods until we have
// generic type aliases
// https://github.com/rust-lang/rust/issues/63063
#[macro_export]
macro_rules! generic_hardware_type {
    () => {
        Hardware<
            impl Pin,
            impl Pin,
            impl Pin,
            USB_DP,
            USB_DM,
            impl SpiMaker,
            impl Pin,
            impl Pin,
            impl Pin,
            impl Pin,
            impl UartMaker,
            impl UartMaker,
            impl UartMaker,
        >
    };
}

pub struct Spawners {
    pub spawner_high: SendSpawner,
    pub spawner_low: SendSpawner,
}
