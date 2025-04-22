use embassy_stm32::{
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
pub mod stm32h723;
#[cfg(feature = "stm32h723")]
pub use stm32h723::{AdcReader, ExtraHardware, Irqs, USB_DM, USB_DP, USB_PERIPHERAL};

#[cfg(feature = "stm32h743")]
pub mod stm32h743;
#[cfg(feature = "stm32h743")]
pub use stm32h743::{AdcReader, ExtraHardware, Irqs, USB_DM, USB_DP, USB_PERIPHERAL};

#[cfg(feature = "stm32f411")]
pub mod stm32f411;
#[cfg(feature = "stm32f411")]
pub use stm32f411::{AdcReader, ExtraHardware, Irqs, USB_DM, USB_DP, USB_PERIPHERAL};

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

#[allow(dead_code)]
pub struct Hardware<
    BluePin: Pin,
    GreenPin: Pin,
    YellowPin: Pin,
    UsbDp: DpPin<USB_PERIPHERAL>,
    UsbDm: DmPin<USB_PERIPHERAL>,
    IMUSpiType: embassy_stm32::spi::Instance + 'static,
    IMUSpiPeripheral: Peripheral<P = IMUSpiType> + 'static,
    IMUSck: SckPin<IMUSpiType> + 'static,
    IMUMosi: MosiPin<IMUSpiType> + 'static,
    IMUMiso: MisoPin<IMUSpiType> + 'static,
    IMUTxDma: embassy_stm32::spi::TxDma<IMUSpiType> + 'static,
    IMURxDma: embassy_stm32::spi::RxDma<IMUSpiType> + 'static,
    IMUCsPin: Pin + 'static,
    Motor0Pin: Pin,
    Motor1Pin: Pin,
    Motor2Pin: Pin,
    Motor3Pin: Pin,
    RadioUartType: embassy_stm32::usart::Instance + 'static,
    RadioUartPeripheral: Peripheral<P = RadioUartType> + 'static,
    RadioUartRxPin: embassy_stm32::usart::RxPin<RadioUartType> + 'static,
    RadioUartTxPin: embassy_stm32::usart::TxPin<RadioUartType> + 'static,
    RadioUartRxDma: embassy_stm32::usart::RxDma<RadioUartType> + 'static,
    RadioUartTxDma: embassy_stm32::usart::TxDma<RadioUartType> + 'static,
    RadioIrqType: interrupt::typelevel::Binding<RadioUartType::Interrupt, InterruptHandler<RadioUartType>>
        + 'static,
> {
    pub blue_pin: BluePin,
    pub green_pin: GreenPin,
    pub yellow_pin: YellowPin,
    pub usb_peripheral: USB_PERIPHERAL,
    pub usb_dp: UsbDp,
    pub usb_dm: UsbDm,

    pub adc_reader: AdcReader,

    pub imu_spi: SpiHardware<
        IMUSpiType,
        IMUSpiPeripheral,
        IMUSck,
        IMUMosi,
        IMUMiso,
        IMUTxDma,
        IMURxDma,
        IMUCsPin,
    >,

    pub motor0_pin: Motor0Pin,
    pub motor1_pin: Motor1Pin,
    pub motor2_pin: Motor2Pin,
    pub motor3_pin: Motor3Pin,

    pub radio_uart: UartHardware<
        RadioUartType,
        RadioUartPeripheral,
        RadioUartRxPin,
        RadioUartTxPin,
        RadioUartRxDma,
        RadioUartTxDma,
        RadioIrqType,
    >,

    pub extra: ExtraHardware,
}
