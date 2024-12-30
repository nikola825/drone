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
