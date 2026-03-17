use embassy_stm32::pac::{
    common::{Reg, W},
    gpio::regs::Bsrr,
};

use cortex_m::interrupt::{free, CriticalSection};

use crate::hal::{
    dshot_delays::{dshot_delay_0, dshot_delay_0_to_1, dshot_delay_remainder},
    mcu_utils::{run_with_paused_icache, ICachePause},
};

fn add_telemetry_bit_and_checksum(value: u16) -> u16 {
    let telemetry = value < 48;

    // Shift the value 1 place to the left and (optionally) enable the telemetry bit
    let mut expanded = value << 1;
    if telemetry {
        expanded |= 1;
    }

    // Calculate the xor-based checksum of the 3 nibbles
    let xor = (expanded & 0xf) ^ ((expanded >> 4) & 0xf) ^ ((expanded >> 8) & 0xf);

    // Shift the value 4 places to the left and add the xor checksum
    (expanded << 4) | xor
}

/// Bit-bang given 16-bit dshot value through the given pin on the given port
/// port_bsrr - the bsrr register of the port
/// pin - number of the used pin on the given port
/// val - raw value being transmitted (without checksum and telemetry bit)
pub fn dshot_send_single(port_bsrr: Reg<Bsrr, W>, pin: usize, val: u16) {
    let sent = add_telemetry_bit_and_checksum(val);

    // The initial value to write to bsrr at the start of each bit - toggle the pin to high
    let mut start_bsrr = Bsrr(0);
    start_bsrr.set_bs(pin, true);

    // The end value to write to bsrr at the end of each bit - toggle the pin to low
    let mut end_bsrr = Bsrr(0);
    end_bsrr.set_br(pin, true);

    // An array of 16 bsrr register values
    // For each bit of the value:
    // - if it's a zero, we want to enable the reset pin in order to set the pin to low state
    // - if the bit is one, we leave the pin high for longer time
    let mut mid_bsrrs = [Bsrr(0); 16];
    let mut mask: u16 = 32768;
    for bsrr in &mut mid_bsrrs {
        if sent & mask == 0 {
            // If the bit is a zero, we enable the reset bit
            bsrr.set_br(pin, true);
        }
        mask >>= 1;
    }

    // Transmit the actual values
    free(|critical_section: &CriticalSection| {
        write_dshot_series(critical_section, port_bsrr, start_bsrr, end_bsrr, mid_bsrrs);
    });
}

/// Bit-bang 4 given 16-bit dshot values through the given pins on the given port
/// port_bsrr - the bsrr register of the port
/// pin - numbers of the used pins on the given port
/// val - raw values being transmitted (without checksum and telemetry bit)
pub fn dshot_send_parallel<const COUNT: usize>(
    port_bsrr: Reg<Bsrr, W>,
    pins: [usize; COUNT],
    mut values: [u16; COUNT],
) {
    for value in &mut values {
        *value = add_telemetry_bit_and_checksum(*value);
    }

    // The initial value to write to bsrr at the start of each bit - toggle the pin to high for each of the 4 outputs
    let mut start_bsrr = Bsrr(0);
    // The end value to write to bsrr at the end of each bit - toggle the pin to low for each of the 4 outputs
    let mut end_bsrr = Bsrr(0);

    for pin in pins {
        start_bsrr.set_bs(pin, true);
        end_bsrr.set_br(pin, true);
    }

    // An array of 16 bsrr register values
    // For each bit of the value:
    // - if it's a zero, we want to enable the reset pin in order to set the pin to low state
    // - if the bit is one, we leave the pin high for longer time
    // Do this for the bits of all 4 values - each value has its own pin
    let mut mid_bsrrs = [Bsrr(0); 16];
    let mut mask: u16 = 32768;

    for bsrr in &mut mid_bsrrs {
        for index in 0..COUNT {
            if values[index] & mask == 0 {
                bsrr.set_br(pins[index], true);
            }
        }
        mask >>= 1;
    }

    free(|critical_section: &CriticalSection| {
        write_dshot_series(critical_section, port_bsrr, start_bsrr, end_bsrr, mid_bsrrs);
    });
}

/// Writes the given series of values to the output pin
/// Repeat 16 times:
/// - Write the initial start_bsrr value to bsrr - toggle all pins to high
/// - Wait for zero-length time
/// - Write the N-th value from mid_bsrrs to bsrr - set to low the pins that corespond to outputs that have the zero bit in this cycle
/// - Wait for zero-length time
/// - Write the end_bsrr value to bsrr - toggle all pins to low
/// - Wait for remainder time
/// - Repeat
#[no_mangle]
#[inline(never)]
fn write_dshot_series(
    critical_section: &CriticalSection,
    port_bsrr: Reg<Bsrr, W>,
    start_bsrr: Bsrr,
    end_bsrr: Bsrr,
    mid_bsrrs: [Bsrr; 16],
) {
    // Icache causes unreliable timings, so we temporarily disable it
    // while sending dshot commands
    run_with_paused_icache(critical_section, |icache_pause: &ICachePause| {
        for bsrr in mid_bsrrs {
            port_bsrr.write_value(start_bsrr);
            dshot_delay_0(critical_section, icache_pause);
            port_bsrr.write_value(bsrr);
            dshot_delay_0_to_1(critical_section, icache_pause);
            port_bsrr.write_value(end_bsrr);
            dshot_delay_remainder(critical_section, icache_pause);
        }
    });
}
