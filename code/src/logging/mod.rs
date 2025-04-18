#[cfg(feature = "rtt-logging")]
mod rtt_logging;
#[cfg(feature = "rtt-logging")]
pub use rtt_logging::{error, info};

#[cfg(feature = "usb-logging")]
mod usb_logging;
#[cfg(feature = "usb-logging")]
pub use usb_logging::{error, info, init_usb_logging};

#[cfg(all(feature = "usb-logging", feature = "rtt-logging"))]
compile_error!("Can't do usb and rtt at the same time");

#[cfg(feature = "usb-logging")]
macro_rules! init_logging {
    ($hardware:ident, $spawner:ident) => {{
        use logging::init_usb_logging;
        init_usb_logging(
            $hardware.usb_peripheral,
            $hardware.usb_dp,
            $hardware.usb_dm,
            &$spawner,
        )
        .await;
    }};
}

#[cfg(feature = "rtt-logging")]
macro_rules! init_logging {
    ($hardware:ident, $spawner:ident) => {};
}

pub(crate) use init_logging;
