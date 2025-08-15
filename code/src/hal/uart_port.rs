use embassy_stm32::{
    interrupt,
    mode::Async,
    pac::gpio::vals::Pupdr,
    usart::{InterruptHandler, Parity, StopBits, Uart, UartRx, UartTx},
    Peripheral as Stm32Peripheral,
};

use crate::hal::mcu_utils::get_pin_gpio;

pub struct UartPort<
    Stm32UartInstance: embassy_stm32::usart::Instance + 'static,
    Peripheral: Stm32Peripheral<P = Stm32UartInstance> + 'static,
    RxPin: embassy_stm32::usart::RxPin<Stm32UartInstance> + 'static,
    TxPin: embassy_stm32::usart::TxPin<Stm32UartInstance> + 'static,
    RxDma: embassy_stm32::usart::RxDma<Stm32UartInstance> + 'static,
    TxDma: embassy_stm32::usart::TxDma<Stm32UartInstance> + 'static,
    IrqType: interrupt::typelevel::Binding<
            Stm32UartInstance::Interrupt,
            InterruptHandler<Stm32UartInstance>,
        > + 'static,
> {
    pub peripheral: Peripheral,
    pub rx_pin: RxPin,
    pub tx_pin: TxPin,
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
        Stm32UartInstance: embassy_stm32::usart::Instance + 'static,
        Peripheral: Stm32Peripheral<P = Stm32UartInstance> + 'static,
        RxPin: embassy_stm32::usart::RxPin<Stm32UartInstance> + 'static,
        TxPin: embassy_stm32::usart::TxPin<Stm32UartInstance> + 'static,
        RxDma: embassy_stm32::usart::RxDma<Stm32UartInstance> + 'static,
        TxDma: embassy_stm32::usart::TxDma<Stm32UartInstance> + 'static,
        IrqType: interrupt::typelevel::Binding<
                Stm32UartInstance::Interrupt,
                InterruptHandler<Stm32UartInstance>,
            > + 'static,
    > UartMaker for UartPort<Stm32UartInstance, Peripheral, RxPin, TxPin, RxDma, TxDma, IrqType>
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

// To be used only as None type for Option<UartMaker>
#[allow(dead_code)]
pub struct UnimplementedUartMaker {
    _prevent_construction: i32,
}

impl UartMaker for UnimplementedUartMaker {
    fn make_uart(
        self,
        _: u32,
        _: Parity,
        _: StopBits,
        _: bool,
    ) -> (UartRx<'static, Async>, UartTx<'static, Async>) {
        unimplemented!("make_uart call made on UnimplementedUartMaker");
    }
}
