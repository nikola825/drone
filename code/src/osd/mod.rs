#[cfg(feature = "msp_osd")]
pub mod msp_osd;

#[cfg(feature = "msp_osd")]
macro_rules! init_osd {
    ($hardware:ident, $spawner:ident, $store: expr) => {{
        use osd::msp_osd::init_msp_osd;
        init_msp_osd($hardware.extra.msp_uart, &$spawner, $store);
    }};
}

#[cfg(feature = "dummy_osd")]
macro_rules! init_osd {
    ($hardware:ident, $spawner:ident, $store: expr) => {};
}

pub(crate) use init_osd;

pub mod char_map_hdzero_inav;
