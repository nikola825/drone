#[cfg(not(target_arch = "wasm32"))]
mod native_usb;

#[cfg(target_arch = "wasm32")]
mod web_serial;

#[cfg(not(target_arch = "wasm32"))]
pub use native_usb::{UsbPort as SerialPort, UsbSerialConection as SerialConnection};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
pub use web_serial::{WebSerialConnection as SerialConnection, WebSerialPort as SerialPort};

#[allow(dead_code)]
#[derive(Debug)]
pub enum SerialError {
    #[cfg(not(target_arch = "wasm32"))]
    CannotOpenPort(serialport::Error),
    #[cfg(target_arch = "wasm32")]
    CannotOpenPort(JsValue),
    #[cfg(not(target_arch = "wasm32"))]
    IOError(std::io::Error),
    #[cfg(target_arch = "wasm32")]
    IOError(String),
    WebSerialNotSupported,
}
