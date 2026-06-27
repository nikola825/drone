#![no_std]

pub mod msp_protocol;
pub mod shared_objects;
pub mod crc8;
pub mod configurator_protocol;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
