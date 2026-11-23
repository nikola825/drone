use std::time::Duration;

use serialport::{SerialPort, SerialPortInfo, SerialPortType};

use crate::drone_connection::serial_connection::SerialError;

#[derive(PartialEq, Debug, Clone)]
pub struct UsbPort {
    port_info: SerialPortInfo,
}

impl std::fmt::Display for UsbPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

const ALLOWED_VIDS: [u16; 2] = [0x0483, 0xdead];
const ALLOWED_PIDS: [u16; 1] = [0xbeef];

impl UsbPort {
    pub async fn enumerate() -> Result<Vec<UsbPort>, SerialError> {
        let available_ports = serialport::available_ports()?;

        let filtered = available_ports.into_iter().filter_map(|port| {
            if let SerialPortType::UsbPort(usb_info) = &port.port_type
                && ALLOWED_VIDS.contains(&usb_info.vid)
                && ALLOWED_PIDS.contains(&usb_info.pid)
            {
                Some(UsbPort { port_info: port })
            } else {
                None
            }
        });

        Ok(filtered.collect())
    }

    pub fn description(&self) -> String {
        self.port_info.port_name.clone()
    }

    pub async fn connect(&self) -> Result<UsbSerialConection, SerialError> {
        println!("Connecting to {:?}", self.description());

        let connection = serialport::new(self.port_info.port_name.clone(), 115200)
            .timeout(Duration::from_secs(1))
            .open()?;

        Ok(UsbSerialConection {
            serial_port: connection,
        })
    }
}

pub struct UsbSerialConection {
    serial_port: Box<dyn SerialPort>,
}

impl From<std::io::Error> for SerialError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<serialport::Error> for SerialError {
    fn from(error: serialport::Error) -> Self {
        Self::CannotOpenPort(error)
    }
}

impl UsbSerialConection {
    pub async fn read_to_end(&mut self) -> Result<(), SerialError> {
        let mut dummy = Vec::new();
        self.serial_port.read_to_end(&mut dummy)?;

        Ok(())
    }

    pub async fn write_all(&mut self, buf: &[u8]) -> Result<(), SerialError> {
        self.serial_port.write_all(buf)?;

        Ok(())
    }
    pub async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), SerialError> {
        self.serial_port.read_exact(buf)?;

        Ok(())
    }
    pub async fn set_timeout(&mut self, timeout: Duration) -> Result<(), SerialError> {
        self.serial_port.set_timeout(timeout)?;

        Ok(())
    }
}
