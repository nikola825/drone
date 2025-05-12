use core::cmp::min;
use embassy_executor::SendSpawner;
use num_traits::float::FloatCore;

use crate::crc8::crc8_calculate;
use crate::hw_select::UartMaker;
use crate::logging::{error, info};
use embassy_stm32::mode::Async;
use embassy_stm32::usart::Parity;
use embassy_stm32::usart::{RingBufferedUartRx, StopBits, UartRx, UartTx};

use embassy_time::{Duration, Instant, Ticker};
use embedded_io::ReadExactError;
use embedded_io_async::{Read, Write};
use zerocopy::{big_endian, FromBytes, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

use crate::shared_state::SharedState;

const CRSF_COMMUNICATION_TIMEOUT: Duration = Duration::from_millis(100);

const CRSF_FRAME_MAX_SIZE: usize = 64;
const CRSF_FRAME_SYNC_BYTE: u8 = 0xc8;

pub const CRSF_COMMAND_MIN: u16 = 172;
pub const CRSF_COMMAND_MAX: u16 = 1811;
pub const CRSF_COMMAND_RANGE: f32 = (CRSF_COMMAND_MAX - CRSF_COMMAND_MIN) as f32;

#[allow(non_camel_case_types)]
#[derive(TryFromBytes, IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(u8)]
enum CRSFFrameType {
    CRSF_FRAMETYPE_BATTERY_SENSOR = 0x08,
    #[allow(dead_code)]
    CRSF_FRAMETYPE_LINK_STATISTICS = 0x14,
    #[allow(dead_code)]
    CRSF_FRAME_TYPE_RC_CHANNELS_PACKED = 0x16,
}

#[derive(TryFromBytes, IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
struct CRSFPacket<Payload>
where
    Payload: TryFromBytes + IntoBytes + Immutable + KnownLayout + Unaligned + Clone,
{
    sync: u8,
    len: u8,
    frame_type: CRSFFrameType,
    payload: Payload,
    crc8: u8,
}

enum CRSFReceiveError {
    UartError(ReadExactError<embassy_stm32::usart::Error>),
    NoSyncByte,
    FrameTooLong,
    UnknownFrameType,
    DeserializationError,
    BadCrc,
}

impl<Payload> CRSFPacket<Payload>
where
    Payload: TryFromBytes + IntoBytes + Immutable + KnownLayout + Unaligned + Clone,
{
    fn new(frame_type: CRSFFrameType, payload: Payload) -> Self {
        let mut packet = CRSFPacket {
            sync: CRSF_FRAME_SYNC_BYTE,
            len: (size_of::<Payload>() + 2) as u8,
            frame_type,
            payload,
            crc8: 0,
        };
        packet.update_crc();

        packet
    }

    fn update_crc(&mut self) {
        let bytes = self.as_bytes();
        let crc = crc8_calculate(&bytes[2..bytes.len() - 1]);
        self.crc8 = crc;
    }

    fn valid_crc(&self) -> bool {
        let bytes = self.as_bytes();
        let crc = crc8_calculate(&bytes[2..bytes.len() - 1]);
        self.crc8 == crc
    }

    fn deserialize(bytes: &[u8]) -> Result<Payload, CRSFReceiveError> {
        Self::try_ref_from_prefix(bytes)
            .map_err(|_| CRSFReceiveError::DeserializationError)
            .and_then(|deserialized| {
                if deserialized.0.valid_crc() {
                    Ok(deserialized.0.payload.clone())
                } else {
                    Err(CRSFReceiveError::BadCrc)
                }
            })
    }
}

#[derive(IntoBytes, Default, Immutable, FromBytes, KnownLayout, Unaligned, Clone)]
#[repr(C)]
struct BatteryInfo {
    voltage: big_endian::I16,
    other_data: [u8; 6],
}

impl BatteryInfo {
    fn new(voltage: f32) -> CRSFPacket<Self> {
        CRSFPacket::new(
            CRSFFrameType::CRSF_FRAMETYPE_BATTERY_SENSOR,
            BatteryInfo {
                voltage: big_endian::I16::new((voltage * 10.0f32).round() as i16),
                other_data: Default::default(),
            },
        )
    }
}

#[derive(FromBytes, IntoBytes, Default, Immutable, KnownLayout, Unaligned, Clone)]
#[repr(C)]
struct CRSFFramePackedChannels {
    packed: [u8; 22],
}

#[derive(FromBytes, IntoBytes, Default, Clone, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct CRSFFrameLinkStatistics {
    pub rssi1: u8,
    pub rssi2: u8,
    pub link_quality: u8,
    pub snr: i8,
    pub active_antenna: u8,
    pub rf_mode: u8,
    pub tx_power: u8,
    pub telemetry_rssi: u8,
    pub telemetry_lq: u8,
    pub telemetry_snr: i8,
}

impl CRSFFramePackedChannels {
    fn unpack(&self) -> CRSFChannels {
        let mut channels = CRSFChannels {
            populated: true,
            timestamp: Instant::now(),
            ..Default::default()
        };
        let mut src_idx = 0;
        let mut dst_idx = 0;
        let mut src_pos = 0;
        let mut dst_pos = 0;

        while dst_idx < 16 {
            let src = self.packed[src_idx];
            let bit_count = min(11 - dst_pos, 8 - src_pos);
            let bits = (src >> src_pos) as u16;
            let bits = bits & ((1 << bit_count) - 1);
            channels.unpacked_channels[dst_idx] |= bits << dst_pos;

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

        channels
    }
}

#[derive(Clone)]
pub struct CRSFChannels {
    pub unpacked_channels: [u16; 16],
    populated: bool,
    pub timestamp: Instant,
}

impl Default for CRSFChannels {
    fn default() -> Self {
        Self {
            unpacked_channels: Default::default(),
            populated: false,
            timestamp: Instant::MIN,
        }
    }
}

impl CRSFChannels {
    pub fn is_fresh(&self) -> bool {
        let now = Instant::now();
        let age = now - self.timestamp;

        self.populated && now > self.timestamp && age < CRSF_COMMUNICATION_TIMEOUT
    }
}

fn make_uart_pair(uart_maker: impl UartMaker) -> (UartRx<'static, Async>, UartTx<'static, Async>) {
    let (rx, tx) = uart_maker.make_uart(420000, Parity::ParityNone, StopBits::STOP1, false);

    (rx, tx)
}

#[embassy_executor::task]
async fn crsf_receiver_task(rx: UartRx<'static, Async>, shared_state: &'static SharedState) {
    let mut ring_buffer = [0u8; 1024];
    let mut rx = rx.into_ring_buffered(&mut ring_buffer);
    info!("CRSF receiver start");

    loop {
        {
            process_crsf_packet(&mut rx, shared_state)
                .await
                .inspect_err(|err| {
                    if let CRSFReceiveError::UartError(err) = err {
                        // log the error and reset UART
                        error!("CRSF receive UART error {}", err);
                        rx.start_uart();
                    }
                })
                .ok();
        }
    }
}

async fn process_crsf_packet(
    rx: &mut RingBufferedUartRx<'_>,
    shared_state: &SharedState,
) -> Result<(), CRSFReceiveError> {
    let mut command_buffer: [u8; CRSF_FRAME_MAX_SIZE] = [0u8; CRSF_FRAME_MAX_SIZE];
    rx.read_exact(&mut command_buffer[0..1])
        .await
        .map_err(CRSFReceiveError::UartError)?;

    if command_buffer[0] != CRSF_FRAME_SYNC_BYTE {
        return Err(CRSFReceiveError::NoSyncByte);
    }

    rx.read_exact(&mut command_buffer[1..2])
        .await
        .map_err(CRSFReceiveError::UartError)?;

    let remainder_length = command_buffer[1] as usize;
    if remainder_length > CRSF_FRAME_MAX_SIZE {
        return Err(CRSFReceiveError::FrameTooLong);
    }

    let total_len = 2 + remainder_length;
    rx.read_exact(&mut command_buffer[2..total_len])
        .await
        .map_err(CRSFReceiveError::UartError)?;

    let frame_type = CRSFFrameType::try_read_from_bytes(&command_buffer[2..3])
        .map_err(|_| CRSFReceiveError::UnknownFrameType)?;

    match frame_type {
        CRSFFrameType::CRSF_FRAMETYPE_BATTERY_SENSOR => {}
        CRSFFrameType::CRSF_FRAMETYPE_LINK_STATISTICS => {
            process_link_statistics(CRSFPacket::deserialize(&command_buffer)?, shared_state).await
        }
        CRSFFrameType::CRSF_FRAME_TYPE_RC_CHANNELS_PACKED => {
            process_received_channels(CRSFPacket::deserialize(&command_buffer)?, shared_state).await
        }
    }

    Ok(())
}

async fn process_received_channels(packed: CRSFFramePackedChannels, shared_state: &SharedState) {
    let unpacked = packed.unpack();
    shared_state.update_channels(unpacked).await;
}

async fn process_link_statistics(stats: CRSFFrameLinkStatistics, shared_state: &SharedState) {
    shared_state.update_link_state(stats).await;
}

#[embassy_executor::task]
async fn crsf_telemetry_task(mut tx: UartTx<'static, Async>, shared_state: &'static SharedState) {
    info!("CRSF telemetry start");
    let mut ticker = Ticker::every(Duration::from_millis(200));

    loop {
        let measured_battery_voltage = shared_state.get_voltage().await;

        let packet = BatteryInfo::new(measured_battery_voltage);

        let _ = tx.write_all(packet.as_bytes()).await;

        ticker.next().await;
    }
}

pub fn init_crsf_communication(
    uart: impl UartMaker,
    spawner: &SendSpawner,
    store: &'static SharedState,
) {
    let (crsf_rx, crsf_tx) = make_uart_pair(uart);

    spawner.must_spawn(crsf_receiver_task(crsf_rx, store));
    spawner.must_spawn(crsf_telemetry_task(crsf_tx, store));
}
