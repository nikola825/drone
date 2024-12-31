use embassy_stm32::{
    adc::AnyAdcChannel,
    gpio::Pin,
    peripherals::ADC1,
    spi::{MisoPin, MosiPin, SckPin},
    usb::{DmPin, DpPin},
    Peripheral,
};

#[cfg(feature = "stm32h723")]
pub mod stm32h723;
#[cfg(feature = "stm32h723")]
pub use stm32h723::{ExtraHardware, Irqs, USB_DM, USB_DP, USB_PERIPHERAL};

#[cfg(feature = "stm32f411")]
pub mod stm32f411;
#[cfg(feature = "stm32f411")]
pub use stm32f411::{ExtraHardware, Irqs, USB_DM, USB_DP, USB_PERIPHERAL};

pub struct UartHardware<
    UartType: embassy_stm32::usart::Instance,
    UartPeripheral: Peripheral<P = UartType>,
    UartRxPin: embassy_stm32::usart::RxPin<UartType>,
    UartTxPin: embassy_stm32::usart::TxPin<UartType>,
    RxDma: embassy_stm32::usart::RxDma<UartType>,
    TxDma: embassy_stm32::usart::TxDma<UartType>,
> {
    pub peripheral: UartPeripheral,
    pub rx_pin: UartRxPin,
    pub tx_pin: UartTxPin,
    pub rx_dma: RxDma,
    pub tx_dma: TxDma,
}

#[allow(dead_code)]
pub struct Hardware<
    BluePin: Pin,
    GreenPin: Pin,
    YellowPin: Pin,
    UsbDp: DpPin<USB_PERIPHERAL>,
    UsbDm: DmPin<USB_PERIPHERAL>,
    SpiType: embassy_stm32::spi::Instance,
    IMUSpi: Peripheral<P = SpiType>,
    IMUSck: SckPin<SpiType>,
    IMUMosi: MosiPin<SpiType>,
    IMUMiso: MisoPin<SpiType>,
    IMUTxDma: embassy_stm32::spi::TxDma<SpiType>,
    IMURxDma: embassy_stm32::spi::RxDma<SpiType>,
    IMUCsPin: Pin,
    Motor0Pin: Pin,
    Motor1Pin: Pin,
    Motor2Pin: Pin,
    Motor3Pin: Pin,
    RadioUartType: embassy_stm32::usart::Instance,
    RadioUartPeripheral: Peripheral<P = RadioUartType>,
    RadioUartRxPin: embassy_stm32::usart::RxPin<RadioUartType>,
    RadioUartTxPin: embassy_stm32::usart::TxPin<RadioUartType>,
    RadioUartRxDma: embassy_stm32::usart::RxDma<RadioUartType>,
    RadioUartTxDma: embassy_stm32::usart::TxDma<RadioUartType>,
> {
    pub blue_pin: BluePin,
    pub green_pin: GreenPin,
    pub yellow_pin: YellowPin,
    pub usb_peripheral: USB_PERIPHERAL,
    pub usb_dp: UsbDp,
    pub usb_dm: UsbDm,

    pub bat_adc: ADC1,
    pub bat_adc_pin: AnyAdcChannel<ADC1>,

    pub imu_spi: IMUSpi,
    pub imu_sck: IMUSck,
    pub imu_mosi: IMUMosi,
    pub imu_miso: IMUMiso,
    pub imu_rx_dma: IMURxDma,
    pub imu_tx_dma: IMUTxDma,
    pub imu_cs_pin: IMUCsPin,

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
    >,

    pub extra: ExtraHardware,
}
