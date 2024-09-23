use core::cmp::min;

use defmt::{error, info};
use embassy_stm32::interrupt;
use embassy_stm32::mode::Async;
use embassy_stm32::usart::{
    InterruptHandler, RingBufferedUartRx, RxDma, StopBits, TxDma, TxPin, Uart, UartRx, UartTx,
};
use embassy_stm32::{
    pac::gpio::{vals, Gpio},
    usart::{Instance, Parity, RxPin},
    Peripheral,
};
use embedded_io::ReadExactError;
use embedded_io_async::Read;
use zerocopy::{FromBytes, FromZeroes};

use crate::storage::Store;

const CRSF_FRAME_MAX_SIZE: usize = 64;
const CRSF_FRAME_SYNC_BYTE: u8 = 0xc8;
const CRSF_FRAME_TYPE_RC_CHANNELS_PACKED: u8 = 0x16;

#[derive(FromBytes, FromZeroes, Default)]
#[repr(C)]
pub struct CRSFFramePackedChannels {
    sync: u8,
    length: u8,
    frame_type: u8,
    packed: [u8; 22],
    crc8: u8,
}

impl CRSFFramePackedChannels {
    pub fn unpack(&self) -> CRSFChannels {
        let mut unpacked_channels = [0u16; 16];
        let mut src_idx = 0;
        let mut dst_idx = 0;
        let mut src_pos = 0;
        let mut dst_pos = 0;

        while dst_idx < 16 {
            let src = self.packed[src_idx];
            let bit_count = min(11 - dst_pos, 8 - src_pos);
            let bits = (src >> src_pos) as u16;
            let bits = bits & ((1 << bit_count) - 1);
            unpacked_channels[dst_idx] |= bits << dst_pos;

            src_pos += bit_count;
            dst_pos += bit_count;
            if src_pos == 8 {
                src_idx += 1;
                src_pos = 0;
            }
            if dst_pos == 11 {
                dst_idx += 1;
                dst_pos = 0;
            }
        }

        return CRSFChannels { unpacked_channels };
    }
}

#[derive(Default, Clone)]
pub struct CRSFChannels {
    pub unpacked_channels: [u16; 16],
}

fn crc8_process_byte(mut current: u8, byte: u8) -> u8 {
    current = current ^ byte;
    for _ in 0..8 {
        if current & 0x80 != 0 {
            current = (current << 1) ^ 0xD5;
        } else {
            current = current << 1;
        }
    }

    return current;
}

fn crc8_calculate(buf: &[u8]) -> u8 {
    let mut crc = 0u8;
    for b in buf {
        crc = crc8_process_byte(crc, *b);
    }

    return crc;
}

pub fn make_uart_pair<T: Instance>(
    gpio: Gpio,
    rx_pin_number: usize,
    uart_peripheral: impl Peripheral<P = T> + 'static,
    rx_pin: impl Peripheral<P = impl RxPin<T>> + 'static,
    tx_pin: impl Peripheral<P = impl TxPin<T>> + 'static,
    tx_dma: impl Peripheral<P = impl TxDma<T>> + 'static,
    rx_dma: impl Peripheral<P = impl RxDma<T>> + 'static,
    interrupt_handlers: impl interrupt::typelevel::Binding<T::Interrupt, InterruptHandler<T>> + 'static,
) -> (UartRx<'static, Async>, UartTx<'static, Async>) {
    let mut uart_config = embassy_stm32::usart::Config::default();
    uart_config.baudrate = 420000;
    uart_config.parity = Parity::ParityNone;
    uart_config.stop_bits = StopBits::STOP1;

    let uart = Uart::new(
        uart_peripheral,
        rx_pin,
        tx_pin,
        interrupt_handlers,
        tx_dma,
        rx_dma,
        uart_config,
    )
    .unwrap();

    gpio.pupdr()
        .modify(|w| w.set_pupdr(rx_pin_number, vals::Pupdr::PULLUP));

    let (tx, rx) = uart.split();
    return (rx, tx);
}

#[embassy_executor::task]
pub async fn crsf_receiver_task(rx: UartRx<'static, Async>, storage: &'static Store) {
    let mut ring_buffer = [0u8; 256];
    let mut rx = rx.into_ring_buffered(&mut ring_buffer);
    info!("BT receiver start");

    loop {
        {
            match read_next_command(&mut rx).await {
                Ok(channels) => {
                    if let Some(channels) = channels {
                        storage.update_channels(channels).await;
                    }
                }
                Err(e) => {
                    // log the error and reset UART
                    error!("CRSF receive UART error {}", e);
                    rx.start().unwrap();
                }
            }
        }
    }
}

async fn read_next_command<'a>(
    rx: &mut RingBufferedUartRx<'a>,
) -> Result<Option<CRSFChannels>, ReadExactError<embassy_stm32::usart::Error>> {
    let mut command_buffer = [0u8; CRSF_FRAME_MAX_SIZE];
    rx.read_exact(&mut command_buffer[0..1]).await?;
    if command_buffer[0] == CRSF_FRAME_SYNC_BYTE {
        rx.read_exact(&mut command_buffer[1..2]).await?;
        let remainder_length = command_buffer[1] as usize;
        if remainder_length + 2 <= CRSF_FRAME_MAX_SIZE {
            let total_len = 2 + remainder_length;
            rx.read_exact(&mut command_buffer[2..total_len]).await?;

            let crc = crc8_calculate(&command_buffer[2..total_len - 1]);
            if crc == command_buffer[total_len - 1] {
                let msg_type = command_buffer[2];
                if msg_type == CRSF_FRAME_TYPE_RC_CHANNELS_PACKED {
                    let packed = CRSFFramePackedChannels::read_from(&command_buffer).unwrap();
                    return Ok(Some(packed.unpack()));
                }
            }
        }
    }
    Ok(None)
}
