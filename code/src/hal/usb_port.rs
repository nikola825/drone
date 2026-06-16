#[cfg(feature = "serial-passthrough")]
use core::cmp::min;

use crate::{
    hal::{Irqs, USB_DEVICE_PRODUCT, USB_DM, USB_DP, USB_PERIPHERAL},
    make_static_buffer, make_static_object,
};
use embassy_stm32::{usb::Driver, Peri};
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
    Builder, UsbDevice,
};

const USB_DEVICE_MANUFACTURER: &str = "nikola825";
const USB_DEVICE_SERIAL: &str = "12345678";

const USB_DEVICE_VID: u16 = 0xdead;
const USB_DEVICE_VID_STM: u16 = 0x0483;
const USB_DEVICE_PID: u16 = 0xbeef;

const DEVICE_CLASS_MISCELANEOUS: u8 = 0xEF;
const DEVICE_SUBCLASS_INTERFACE_ASSOCIATION_DESCRIPTOR: u8 = 0x02;
const DEVICE_PROTOCOL_INTERFACE_ASSOCIATION_DESCRIPTOR: u8 = 0x01;

#[derive(Debug)]
pub struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}

pub type UsbCdcAcmPort = CdcAcmClass<'static, Driver<'static, USB_PERIPHERAL>>;

pub struct UsbPort {
    pub cdc_acm: UsbCdcAcmPort,
    pub usb_device: UsbDevice<'static, Driver<'static, USB_PERIPHERAL>>,
}

pub struct UsbPeripheral {
    pub peripheral: Peri<'static, USB_PERIPHERAL>,
    pub dp_pin: Peri<'static, USB_DP>,
    pub dm_pin: Peri<'static, USB_DM>,
}

impl UsbPeripheral {
    pub fn into_usb_port(self, use_stm_vid: bool) -> UsbPort {
        let usb_buffer = make_static_buffer!(256);
        let config_descriptor = make_static_buffer!(256);
        let bos_descriptor = make_static_buffer!(256);
        let control_buf = make_static_buffer!(64);
        let state = make_static_object!(State, State::new());

        let mut config = embassy_stm32::usb::Config::default();

        config.vbus_detection = false;
        let driver = Driver::new_fs(
            self.peripheral,
            Irqs,
            self.dp_pin,
            self.dm_pin,
            usb_buffer,
            config,
        );

        let vid = if use_stm_vid {
            USB_DEVICE_VID_STM
        } else {
            USB_DEVICE_VID
        };

        let mut config = embassy_usb::Config::new(vid, USB_DEVICE_PID);
        config.manufacturer = Some(USB_DEVICE_MANUFACTURER);
        config.product = Some(USB_DEVICE_PRODUCT);
        config.serial_number = Some(USB_DEVICE_SERIAL);

        config.device_class = DEVICE_CLASS_MISCELANEOUS;
        config.device_sub_class = DEVICE_SUBCLASS_INTERFACE_ASSOCIATION_DESCRIPTOR;
        config.device_protocol = DEVICE_PROTOCOL_INTERFACE_ASSOCIATION_DESCRIPTOR;
        config.composite_with_iads = true;
        config.self_powered = true;
        config.max_power = 0;

        // Create embassy-usb DeviceBuilder using the driver and config.
        // It needs some buffers for building the descriptors.
        let mut builder = Builder::new(
            driver,
            config,
            config_descriptor,
            bos_descriptor,
            &mut [], // no msos descriptors
            control_buf,
        );

        // Create classes on the builder.
        let cdc_acm = CdcAcmClass::new(&mut builder, state, 64);

        // Build the builder.
        let usb_device = builder.build();

        UsbPort {
            cdc_acm,
            usb_device,
        }
    }
}

#[cfg(feature = "serial-passthrough")]
pub struct UsbSerialWrapper {
    cdc_acm: UsbCdcAcmPort,
    current_packet: [u8; 64],
    cursor: Option<usize>,
    packet_len: usize,
}

#[cfg(feature = "serial-passthrough")]
pub trait PacketHeaderType: Sized {
    fn try_extract_header(buffer: &[u8]) -> Option<Self>;
}

#[cfg(feature = "serial-passthrough")]
impl UsbSerialWrapper {
    pub fn new(cdc_acm: UsbCdcAcmPort) -> Self {
        Self {
            cdc_acm,
            current_packet: [0u8; 64],
            cursor: None,
            packet_len: 0,
        }
    }
    pub async fn write_chunked(&mut self, data: &[u8]) -> Result<(), Disconnected> {
        let mut index = 0;
        while index < data.len() {
            let end = core::cmp::min(index + 32, data.len());
            self.cdc_acm.write_packet(&data[index..end]).await?;
            index += 32;
        }

        Ok(())
    }

    async fn fill_packet(&mut self) -> Result<(), Disconnected> {
        loop {
            self.packet_len = self.cdc_acm.read_packet(&mut self.current_packet).await?;
            if self.packet_len > 0 {
                self.cursor = Some(0);
                return Ok(());
            }
        }
    }

    async fn get_current_cursor(&mut self) -> Result<usize, Disconnected> {
        if let Some(cursor) = self.cursor {
            Ok(cursor)
        } else {
            self.fill_packet().await?;
            Ok(0)
        }
    }

    fn advance_cursor(&mut self, by: usize) {
        let cursor = if let Some(cursor) = self.cursor {
            cursor
        } else {
            panic!("Cursor advanced attempted with None existing cursor");
        };

        let next_cursor = cursor + by;

        match next_cursor.cmp(&self.packet_len) {
            core::cmp::Ordering::Less => {
                self.cursor = Some(next_cursor);
            }
            core::cmp::Ordering::Equal => {
                self.cursor = None;
            }
            core::cmp::Ordering::Greater => {
                panic!(
                    "Cursor advanced to value {} beyond packet len {}",
                    next_cursor, self.packet_len
                );
            }
        }
    }

    pub async fn advance_until_header<HeaderType>(&mut self) -> Result<HeaderType, Disconnected>
    where
        HeaderType: PacketHeaderType,
    {
        let mut cursor = self.get_current_cursor().await?;
        loop {
            if let Some(header) = HeaderType::try_extract_header(&self.current_packet[cursor..]) {
                return Ok(header);
            }
            self.advance_cursor(1);
            cursor = self.get_current_cursor().await?;
        }
    }

    pub async fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), Disconnected> {
        let mut bytes_remaining_to_read = buffer.len();
        let mut out_index = 0usize;

        while bytes_remaining_to_read > 0 {
            let cursor = self.get_current_cursor().await?;
            let bytes_remaining_in_packet = self.packet_len - cursor;

            let bytes_to_copy = min(bytes_remaining_in_packet, bytes_remaining_to_read);

            buffer[out_index..out_index + bytes_to_copy]
                .copy_from_slice(&self.current_packet[cursor..cursor + bytes_to_copy]);

            bytes_remaining_to_read -= bytes_to_copy;
            out_index += bytes_to_copy;
            self.advance_cursor(bytes_to_copy);
        }

        Ok(())
    }

    pub async fn wait_for_connection(&mut self) {
        self.cdc_acm.wait_connection().await
    }
}
