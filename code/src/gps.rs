use crate::{
    logging::error,
    math_stuff::{DEG_TO_RAD_FACTOR, RAD_TO_DEG_FACTOR},
};
use embassy_executor::SendSpawner;
use embassy_stm32::{
    mode::Async,
    usart::{RingBufferedUartRx, UartRx},
};
use embedded_io::ReadExactError;
use embedded_io_async::Read;
use zerocopy::{
    little_endian::{I16, I32, U16, U32},
    FromBytes, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned,
};

use crate::{hw_select::UartMaker, shared_state::SharedState};

const UBX_SYNC_1: u8 = 0xb5;
const UBX_SYNC_2: u8 = 0x62;

const PACKET_CLASS_TYPE_UBX_NAV_PVT: PacketClassAndType = PacketClassAndType::new(0x01, 0x07);

#[derive(Debug)]
enum GPSReceiveError {
    UartError(ReadExactError<embassy_stm32::usart::Error>),
    NoSyncBytes,
    FrameTooLong,
    UnknownFrameType,
    DeserializationError,
    BadCrc,
}

#[derive(IntoBytes, Immutable, TryFromBytes, KnownLayout, Unaligned, Clone, PartialEq)]
#[repr(C)]
struct PacketClassAndType {
    class: u8,
    packet_type: u8,
}

impl PacketClassAndType {
    const fn new(class: u8, packet_type: u8) -> Self {
        PacketClassAndType { class, packet_type }
    }
}

#[derive(IntoBytes, Immutable, TryFromBytes, KnownLayout, Unaligned, Clone, PartialEq)]
#[repr(C)]
struct UbxHeader {
    sync1: u8,
    sync2: u8,
    class_and_type: PacketClassAndType,
    length: U16,
}

#[derive(IntoBytes, Immutable, TryFromBytes, KnownLayout, Unaligned, Clone)]
#[repr(C)]
pub struct UbxNavPVTPacket {
    pub time_of_week: U32,
    pub year: U16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub validity: TimeValidityFlags,
    pub time_accuracy_ns: I32,
    pub nanosecond: I32,
    pub fix_type: FixType,
    pub fix_status_flags: FixStatusFlags,
    pub satelites_visible: u8,
    pub position: SpherePosition,
    pub height_ellipsoid: Altitude,
    pub height_mean_sea_level: Altitude,
    pub horizontal_accuracy: U32,
    pub vertical_accuracy: U32,
    pub velocity_north: Speed,
    pub velocity_east: Speed,
    pub velocity_down: Speed,
    pub ground_speed: Speed,
    pub motion_heading: Heading,
    pub speed_accuracy: U32,
    pub heading_accuracy: U32,
    pub position_dop: U16,
    pub flags3: AdditionalFlags,
    pub reserved: [u8; 4],
    pub vehicle_heading: Heading,
    pub magnetic_declination: I16,
    pub magnetic_declination_accuracy: U16,
}

impl UbxNavPVTPacket {
    pub fn gps_data_displayable(&self) -> bool {
        self.fix_type == FixType::Fix2D
            || self.fix_type == FixType::Fix3D
            || self.fix_type == FixType::GNSS
    }
}

#[derive(IntoBytes, Default, Immutable, FromBytes, KnownLayout, Unaligned, Clone)]
#[repr(C)]
pub struct TimeValidityFlags {
    validity: u8,
}

#[allow(dead_code)]
impl TimeValidityFlags {
    fn value(&self) -> u8 {
        self.validity
    }

    fn mag_valid(&self) -> bool {
        self.validity & 8 == 8
    }
}

#[derive(TryFromBytes, IntoBytes, Immutable, KnownLayout, Unaligned, Clone, Debug, PartialEq)]
#[repr(u8)]
#[allow(dead_code, clippy::upper_case_acronyms)]
pub enum FixType {
    NoFix = 0,
    DeadReckoning = 1,
    Fix2D = 2,
    Fix3D = 3,
    GNSS = 4,
    TimeOnly = 5,
}

#[derive(IntoBytes, Default, Immutable, FromBytes, KnownLayout, Unaligned, Clone)]
#[repr(C)]
pub struct FixStatusFlags {
    part1: u8,
    part2: u8,
}

#[derive(IntoBytes, Default, Immutable, FromBytes, KnownLayout, Unaligned, Clone)]
#[repr(C)]
pub struct AdditionalFlags {
    part1: U16,
}

#[derive(IntoBytes, Default, Immutable, FromBytes, KnownLayout, Unaligned, Clone)]
#[repr(C)]
pub struct AngularCoordinate {
    inner_deg_1e7: I32,
}

impl AngularCoordinate {
    pub fn as_1e7(&self) -> i32 {
        self.inner_deg_1e7.into()
    }

    pub fn as_f32(&self) -> f32 {
        (i32::from(self.inner_deg_1e7) as f32) / 1e7f32
    }
}

#[derive(IntoBytes, Default, Immutable, FromBytes, KnownLayout, Unaligned, Clone)]
#[repr(C)]
pub struct SpherePosition {
    pub longitude: AngularCoordinate,
    pub latitude: AngularCoordinate,
}

use num_traits::Float;
impl SpherePosition {
    pub fn heading_to(&self, other: &Self) -> Heading {
        // Returns heading needed to go from self to other

        // Convert all coordinates to radians
        let lat_a = self.latitude.as_f32();
        let lon_a = self.longitude.as_f32();

        let lat_b = other.latitude.as_f32();
        let lon_b = other.longitude.as_f32();

        let lat_a = lat_a * DEG_TO_RAD_FACTOR;
        let lon_a = lon_a * DEG_TO_RAD_FACTOR;
        let lat_b = lat_b * DEG_TO_RAD_FACTOR;
        let lon_b = lon_b * DEG_TO_RAD_FACTOR;

        // Haversine formula for calculating heading between two points
        let x = lat_b.cos() * (lon_b - lon_a).sin();
        let y = lat_a.cos() * lat_b.sin() - lat_a.sin() * lat_b.cos() * (lon_b - lon_a).cos();
        let heading_rad = x.atan2(y);
        
        // Convert back to degrees*1e5
        let heading_deg_1e5 = heading_rad * RAD_TO_DEG_FACTOR * 1e5f32;
        Heading {
            inner_deg_1e5: I32::from(heading_deg_1e5 as i32),
        }
    }
}

#[derive(IntoBytes, Default, Immutable, FromBytes, KnownLayout, Unaligned, Clone)]
#[repr(C)]
pub struct Speed {
    inner_mm_s: I32,
}

impl Speed {
    pub fn as_kmh_multiple(&self, multiple: i32) -> i16 {
        (i32::from(self.inner_mm_s) * multiple * 36 / 10000) as i16
    }
}

#[derive(IntoBytes, Default, Immutable, FromBytes, KnownLayout, Unaligned, Clone)]
#[repr(C)]
pub struct Altitude {
    inner_mm: I32,
}

impl Altitude {
    pub fn as_meters(&self) -> i16 {
        (i32::from(self.inner_mm) / 1000) as i16
    }
}

#[derive(IntoBytes, Default, Immutable, FromBytes, KnownLayout, Unaligned, Clone)]
#[repr(C)]
pub struct Heading {
    inner_deg_1e5: I32,
}

impl Heading {
    pub fn as_degrees_multiple(&self, multiple: i32) -> i16 {
        (i32::from(self.inner_deg_1e5) * multiple / 100000) as i16
    }

    pub fn as_degrees_0_360(&self) -> u16 {
        self.as_degrees_multiple(1).rem_euclid(360) as u16
    }
}

#[derive(Default, Clone)]
pub struct GPSState {
    pub gps_packet: Option<UbxNavPVTPacket>,
}

pub fn init_gps_receiver(uart: impl UartMaker, spawner: &SendSpawner, store: &'static SharedState) {
    let (rx, _) = uart.make_uart(
        115200,
        embassy_stm32::usart::Parity::ParityNone,
        embassy_stm32::usart::StopBits::STOP1,
        false,
    );

    spawner.spawn(gps_receiver_task(rx, store)).unwrap();
}

#[embassy_executor::task]
async fn gps_receiver_task(rx: UartRx<'static, Async>, store: &'static SharedState) {
    let mut rx_ring_buffer: [u8; 512] = [0u8; 512];

    let mut rx = rx.into_ring_buffered(&mut rx_ring_buffer);

    loop {
        receive_ubx_packet(&mut rx, store)
            .await
            .inspect_err(|err| {
                if let GPSReceiveError::UartError(err) = err {
                    error!("GPS receive UART error {}", err);
                    rx.start_uart();
                }
            })
            .ok();
    }
}

fn ubx_crc(buffer: &[u8]) -> (u8, u8) {
    let mut ck_a: u16 = 0;
    let mut ck_b: u16 = 0;

    for byte in buffer {
        ck_a = (ck_a + *byte as u16) & 0xff;
        ck_b = (ck_b + ck_a) & 0xff;
    }

    (ck_a as u8, ck_b as u8)
}

async fn receive_ubx_packet(
    uart: &mut RingBufferedUartRx<'_>,
    store: &SharedState,
) -> Result<(), GPSReceiveError> {
    let mut buffer = [0u8; 264];

    uart.read_exact(&mut buffer[0..1])
        .await
        .map_err(GPSReceiveError::UartError)?;

    if buffer[0] != UBX_SYNC_1 {
        return Err(GPSReceiveError::NoSyncBytes);
    }

    uart.read_exact(&mut buffer[1..2])
        .await
        .map_err(GPSReceiveError::UartError)?;

    if buffer[1] != UBX_SYNC_2 {
        return Err(GPSReceiveError::NoSyncBytes);
    }

    uart.read_exact(&mut buffer[2..6])
        .await
        .map_err(GPSReceiveError::UartError)?;

    let header = UbxHeader::try_ref_from_bytes(&buffer[0..6])
        .map_err(|_| GPSReceiveError::DeserializationError)?;

    let packet_len: usize = u16::from(header.length) as usize;

    if packet_len >= 256 {
        return Err(GPSReceiveError::FrameTooLong);
    }

    let class_and_type = header.class_and_type.clone();

    uart.read_exact(&mut buffer[6..packet_len + 8])
        .await
        .map_err(GPSReceiveError::UartError)?;

    let (crc_a, crc_b) = ubx_crc(&buffer[2..packet_len + 6]);

    if crc_a != buffer[packet_len + 6] || crc_b != buffer[packet_len + 7] {
        return Err(GPSReceiveError::BadCrc);
    }

    match class_and_type {
        PACKET_CLASS_TYPE_UBX_NAV_PVT => {
            receive_nav_pvt_packet(&buffer[6..packet_len + 6], store).await?;
        }
        _ => {
            return Err(GPSReceiveError::UnknownFrameType);
        }
    }

    Ok(())
}

async fn receive_nav_pvt_packet(buffer: &[u8], store: &SharedState) -> Result<(), GPSReceiveError> {
    if buffer.len() != 92 {
        return Err(GPSReceiveError::DeserializationError);
    }

    let nav_packet = UbxNavPVTPacket::try_ref_from_bytes(buffer)
        .map_err(|_| GPSReceiveError::DeserializationError)?;

    store
        .update_gps_state(GPSState {
            gps_packet: Some(nav_packet.clone()),
        })
        .await;

    Ok(())
}
