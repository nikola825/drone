use common::command::CommandPacket;
use defmt::{error, info};
use embassy_stm32::interrupt;
use embassy_stm32::mode::Async;
use embassy_stm32::usart::{InterruptHandler, RxDma, StopBits, TxPin, UartRx};
use embassy_stm32::{
    pac::gpio::{vals, Gpio},
    usart::{Instance, Parity, RxPin},
    Peripheral,
};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Sender;
use embedded_io_async::Read;
use zerocopy::FromBytes;

pub fn make_bluetooth_uart<T: Instance>(
    gpio: Gpio,
    rx_pin_number: usize,
    uart_peripheral: impl Peripheral<P = T> + 'static,
    rx_pin: impl Peripheral<P = impl RxPin<T>> + 'static,
    tx_pin: impl Peripheral<P = impl TxPin<T>> + 'static,
    rx_dma: impl Peripheral<P = impl RxDma<T>> + 'static,
    irqs: impl interrupt::typelevel::Binding<T::Interrupt, InterruptHandler<T>> + 'static,
) -> UartRx<'static, Async> {
    let mut uart_config = embassy_stm32::usart::Config::default();
    uart_config.baudrate = 420000;
    uart_config.parity = Parity::ParityNone;
    uart_config.stop_bits = StopBits::STOP1;

    let rx = UartRx::new(uart_peripheral, irqs, rx_pin, rx_dma, uart_config).unwrap();
    gpio.pupdr()
        .modify(|w| w.set_pupdr(rx_pin_number, vals::Pupdr::PULLUP));

    return rx;
}

#[embassy_executor::task]
pub async fn bluetooth_receiver_task(
    uart: UartRx<'static, Async>,
    sender: Sender<'static, ThreadModeRawMutex, CommandPacket, 10>
) {
    let mut ring_buffer = [0u8; 256];
    let mut uart = uart.into_ring_buffered(&mut ring_buffer);
    info!("BT receiver start");

    let mut command_buffer = [0u8; size_of::<CommandPacket>()];
    let mut next_read_start: usize = 0;

    loop {
        {
            match uart
                .read_exact(&mut command_buffer[next_read_start..])
                .await
            {
                Ok(_) => {
                    if command_buffer[0] == 0x42 {
                        let packet = CommandPacket::read_from(&command_buffer).unwrap();
                        sender.send(packet).await;
                        next_read_start = 0;
                    } else {
                        next_read_start = 0;
                        for i in 0..size_of::<CommandPacket>() {
                            if command_buffer[i] == 0x42 {
                                for j in i..size_of::<CommandPacket>() {
                                    command_buffer[j - i] = command_buffer[j];
                                    next_read_start = j - i + 1;
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    next_read_start = 0;
                    error!("Bt ERR {}", e);
                }
            }
        }
    }
}