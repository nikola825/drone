use std::time::Duration;

use log::{error, warn};
use wasm_bindgen::{JsValue, convert::TryFromJsValue};
use web_sys::{
    ReadableStreamDefaultReader, ReadableStreamReadResult, SerialOptions, SerialPort,
    WritableStreamDefaultWriter,
    js_sys::{Reflect, Uint8Array},
};

use crate::drone_connection::serial_connection::SerialError;

#[derive(PartialEq, Debug, Clone)]
pub struct WebSerialPort {
    port: SerialPort,
}

impl std::fmt::Display for WebSerialPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl WebSerialPort {
    pub async fn enumerate() -> Result<Vec<WebSerialPort>, SerialError> {
        let serial = web_sys::window()
            .map(|window| window.navigator())
            .and_then(|navigator| {
                if Reflect::has(&navigator, &JsValue::from_str("serial")).unwrap_or(false) {
                    Some(navigator.serial())
                } else {
                    error!("window.navigator has no serial field");
                    None
                }
            })
            .ok_or(SerialError::WebSerialNotSupported)?;

        let port = serial.request_port().await;
        match port {
            Ok(port) => Ok(vec![WebSerialPort { port }]),
            Err(err) => {
                warn!("request_port did not return any ports, error:{:?}", err);
                Ok(Vec::new())
            }
        }
    }

    pub fn description(&self) -> String {
        let vendor_id = self
            .port
            .get_info()
            .get_usb_vendor_id()
            .map(|x| format!("0x{:04x}", x))
            .unwrap_or("N/A".into());

        let product_id = self
            .port
            .get_info()
            .get_usb_product_id()
            .map(|x| format!("0x{:04x}", x))
            .unwrap_or("N/A".into());
        format!("USB:{}:{}", vendor_id, product_id)
    }

    pub async fn connect(&self) -> Result<WebSerialConnection, SerialError> {
        let s: SerialOptions = SerialOptions::new(115200);
        self.port
            .open(&s)
            .await
            .map_err(SerialError::CannotOpenPort)?;
        let readable_stream = self.port.readable();
        let writable_stream = self.port.writable();

        let stream_reader =
            ReadableStreamDefaultReader::try_from_js_value(readable_stream.get_reader().into())
                .map_err(SerialError::CannotOpenPort)?;
        let stream_writer = writable_stream
            .get_writer()
            .map_err(SerialError::CannotOpenPort)?;

        Ok(WebSerialConnection {
            reader: stream_reader,
            writer: stream_writer,
        })
    }
}

impl From<JsValue> for SerialError {
    fn from(value: JsValue) -> Self {
        let string = format!("Bad JS response: {:?}", value);
        Self::IOError(string)
    }
}
pub struct WebSerialConnection {
    reader: ReadableStreamDefaultReader,
    writer: WritableStreamDefaultWriter,
}

impl WebSerialConnection {
    pub async fn read_to_end(&mut self) -> Result<(), SerialError> {
        // Not supported in JS/Web
        Ok(())
    }

    pub async fn write_all(&mut self, buf: &[u8]) -> Result<(), SerialError> {
        self.writer
            .write_with_chunk(&Uint8Array::new_from_slice(buf))
            .await?;

        Ok(())
    }
    pub async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), SerialError> {
        let mut out_index = 0;
        while out_index < buf.len() {
            let read_result: ReadableStreamReadResult = self.reader.read().await?.into();
            let done = read_result.get_done().ok_or(SerialError::IOError(
                "ReadableStreamReadResult has no `done` field".into(),
            ))?;

            if done {
                return Err(SerialError::IOError(
                    "Serial stream aborted, done=true".into(),
                ));
            }

            let value: Uint8Array = read_result.get_value().into();
            if (value.length() as usize + out_index) <= buf.len() {
                value.copy_to(&mut buf[out_index..out_index + value.length() as usize]);
                out_index += value.length() as usize;
            } else {
                return Err(SerialError::IOError(
                    "Too many bytes read from the serial port".into(),
                ));
            }
        }
        Ok(())
    }
    pub async fn set_timeout(&mut self, _timeout: Duration) -> Result<(), SerialError> {
        // Timeouts not supported in JS/Web
        Ok(())
    }
}
