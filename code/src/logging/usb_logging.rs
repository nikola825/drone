use core::panic::PanicInfo;

use crate::hal::UsbPeripheral;
use embassy_executor::SendSpawner;
use embassy_futures::join::join;
pub use log::{error, info};

#[panic_handler]
fn custom_panic(_: &PanicInfo) -> ! {
    loop {}
}

pub async fn init_usb_logging(peripheral: UsbPeripheral, spawner: &SendSpawner) {
    spawner.must_spawn(usb_task(peripheral));
}

#[embassy_executor::task]
async fn usb_task(peripheral: UsbPeripheral) {
    let mut usb_port = peripheral.into_usb_port(false);

    // Run the USB device.
    let usb_fut = usb_port.usb_device.run();
    let cdc_acm = usb_port.cdc_acm;

    let log_fut = embassy_usb_logger::with_custom_style!(
        1024,
        log::LevelFilter::Info,
        cdc_acm,
        |record, writer| {
            use core::fmt::Write;
            let level = record.level().as_str();
            let file = record.file_static().unwrap_or("NA");
            let line = record.line().unwrap_or(0);

            let _ = write!(writer, "[{level}] @{file}:{line} {}\r\n", record.args());
        }
    );

    join(usb_fut, log_fut).await;
}
