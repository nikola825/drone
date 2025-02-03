use core::panic::PanicInfo;

use crate::hw_select::{Irqs, USB_DM, USB_DP, USB_PERIPHERAL};
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_stm32::{
    peripherals::{PA11, PA12},
    usb::Driver,
};
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    Builder,
};
pub use log::{error, info};

const USB_DEVICE_MANUFACTURER: &str = "nikola825";
const USB_DEVICE_PRODUCT: &str = "STM32F4 flight controller";
const USB_DEVICE_SERIAL: &str = "12345678";

const USB_DEVICE_VID: u16 = 0xdead;
const USB_DEVICE_PID: u16 = 0xbeef;

const DEVICE_CLASS_MISCELANEOUS: u8 = 0xEF;
const DEVICE_SUBCLASS_INTERFACE_ASSOCIATION_DESCRIPTOR: u8 = 0x02;
const DEVICE_PROTOCOL_INTERFACE_ASSOCIATION_DESCRIPTOR: u8 = 0x01;

#[panic_handler]
fn custom_panic(_: &PanicInfo) -> ! {
    loop {}
}

pub async fn init_usb_logging(
    peripheral: USB_PERIPHERAL,
    dp_pin: PA12,
    dm_pin: PA11,
    spawner: &Spawner,
) {
    spawner.spawn(usb_task(peripheral, dp_pin, dm_pin)).unwrap();
}

#[embassy_executor::task]
async fn usb_task(peripheral: USB_PERIPHERAL, dp_pin: USB_DP, dm_pin: USB_DM) {
    let mut config = embassy_stm32::usb::Config::default();
    config.vbus_detection = false;
    let mut usb_buffer = [0u8; 256];
    let driver = Driver::new_fs(peripheral, Irqs, dp_pin, dm_pin, &mut usb_buffer, config);

    let mut config = embassy_usb::Config::new(USB_DEVICE_VID, USB_DEVICE_PID);
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
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];

    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // no msos descriptors
        &mut control_buf,
    );

    // Create classes on the builder.
    let class = CdcAcmClass::new(&mut builder, &mut state, 64);

    // Build the builder.
    let mut usb = builder.build();

    // Run the USB device.
    let usb_fut = usb.run();

    let log_fut = embassy_usb_logger::with_custom_style!(
        1024,
        log::LevelFilter::Info,
        class,
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
