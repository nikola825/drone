use crate::nopdelays::*;
use core::arch::asm;
use embassy_stm32::pac::{
    common::{Reg, W},
    gpio::regs::Bsrr,
};

use cortex_m::interrupt::{free, CriticalSection};

#[cfg(feature = "stm32f411")]
#[macro_export]
macro_rules! dshot_bitbang_bit {
    ($bsrr: ident, $x: expr, $val: expr, $bit: ident) => {
        ($bsrr).write(|w| w.set_bs(($bit), true));
        if ($val) & ($x) == ($x) {
            dshot_bitbang_bit!(1, 1200, $bsrr, $bit);
        } else {
            dshot_bitbang_bit!(0, 1200, $bsrr, $bit);
        }
    };

    (1, 600, $bsrr: ident, $bit: ident) => {
        nop112!();
        ($bsrr).write(|w| w.set_br(($bit), true));
        nop38!();
    };

    (0, 600, $bsrr: ident, $bit: ident) => {
        nop58!();
        ($bsrr).write(|w| w.set_br(($bit), true));
        nop93!();
    };

    (1, 1200, $bsrr: ident, $bit: ident) => {
        nop56!();
        ($bsrr).write(|w| w.set_br(($bit), true));
        nop19!();
    };

    (0, 1200, $bsrr: ident, $bit: ident) => {
        nop29!();
        ($bsrr).write(|w| w.set_br(($bit), true));
        nop47!();
    };

    (1, 2400, $bsrr: ident, $bit: ident) => {
        nop28!();
        ($bsrr).write(|w| w.set_br(($bit), true));
        nop9!();
    };

    (0, 2400, $bsrr: ident, $bit: ident) => {
        nop15!();
        ($bsrr).write(|w| w.set_br(($bit), true));
        nop24!();
    };
}

#[cfg(feature = "stm32h723")]
#[macro_export]
macro_rules! dshot_bitbang_bit {
    ($bsrr: ident, $x: expr, $val: expr, $bit: ident) => {
        ($bsrr).write(|w| w.set_bs(($bit), true));
        if ($val) & ($x) == 0 {
            dshot_bitbang_bit!(0, 1200, $bsrr, $bit);
        } else {
            dshot_bitbang_bit!(1, 1200, $bsrr, $bit);
        }
    };

    (1, 1200, $bsrr: ident, $bit: ident) => {
        nop254!();
        ($bsrr).write(|w| w.set_br(($bit), true));
        nop70!();
    };

    (0, 1200, $bsrr: ident, $bit: ident) => {
        nop127!();
        ($bsrr).write(|w| w.set_br(($bit), true));
        nop197!();
    };
}

macro_rules! dshot_nop_0 {
    () => {
        nop105!();
    };
}

macro_rules! dshot_nop_0_to_1 {
    () => {
        nop105!();
    };
}

macro_rules! dshot_nop_remainder {
    () => {
        nop70!();
    };
}

#[cfg(feature = "stm32h723")]
#[macro_export]
macro_rules! dshot_bitbang_bits {
    (0, $bsrr0: expr, $bit0: expr, $bsrr1: expr, $bit1: expr, $bsrr2: expr, $bit2: expr, $bsrr3: expr, $bit3: expr) => {
        ($bsrr0).write(|w| w.set_bs(($bit0), true));
        ($bsrr1).write(|w| w.set_bs(($bit1), true));
        ($bsrr2).write(|w| w.set_bs(($bit2), true));
        ($bsrr3).write(|w| w.set_bs(($bit3), true));

        dshot_nop_0!();
        ($bsrr0).write(|w| w.set_br(($bit0), true));
        ($bsrr1).write(|w| w.set_br(($bit1), true));
        ($bsrr2).write(|w| w.set_br(($bit2), true));
        ($bsrr3).write(|w| w.set_br(($bit3), true));

        dshot_nop_0_to_1!();

        dshot_nop_remainder!();
    };

    (1, $bsrr0: expr, $bit0: expr, $bsrr1: expr, $bit1: expr, $bsrr2: expr, $bit2: expr, $bsrr3: expr, $bit3: expr) => {
        ($bsrr0).write(|w| w.set_bs(($bit0), true));
        ($bsrr1).write(|w| w.set_bs(($bit1), true));
        ($bsrr2).write(|w| w.set_bs(($bit2), true));
        ($bsrr3).write(|w| w.set_bs(($bit3), true));

        dshot_nop_0!();
        ($bsrr1).write(|w| w.set_br(($bit1), true));
        ($bsrr2).write(|w| w.set_br(($bit2), true));
        ($bsrr3).write(|w| w.set_br(($bit3), true));

        dshot_nop_0_to_1!();
        ($bsrr0).write(|w| w.set_br(($bit0), true));

        dshot_nop_remainder!();
    };

    (2, $bsrr0: expr, $bit0: expr, $bsrr1: expr, $bit1: expr, $bsrr2: expr, $bit2: expr, $bsrr3: expr, $bit3: expr) => {
        ($bsrr0).write(|w| w.set_bs(($bit0), true));
        ($bsrr1).write(|w| w.set_bs(($bit1), true));
        ($bsrr2).write(|w| w.set_bs(($bit2), true));
        ($bsrr3).write(|w| w.set_bs(($bit3), true));

        dshot_nop_0!();
        ($bsrr0).write(|w| w.set_br(($bit0), true));
        ($bsrr2).write(|w| w.set_br(($bit2), true));
        ($bsrr3).write(|w| w.set_br(($bit3), true));

        dshot_nop_0_to_1!();
        ($bsrr1).write(|w| w.set_br(($bit1), true));

        dshot_nop_remainder!();
    };

    (3, $bsrr0: expr, $bit0: expr, $bsrr1: expr, $bit1: expr, $bsrr2: expr, $bit2: expr, $bsrr3: expr, $bit3: expr) => {
        ($bsrr0).write(|w| w.set_bs(($bit0), true));
        ($bsrr1).write(|w| w.set_bs(($bit1), true));
        ($bsrr2).write(|w| w.set_bs(($bit2), true));
        ($bsrr3).write(|w| w.set_bs(($bit3), true));

        dshot_nop_0!();
        ($bsrr2).write(|w| w.set_br(($bit2), true));
        ($bsrr3).write(|w| w.set_br(($bit3), true));

        dshot_nop_0_to_1!();
        ($bsrr0).write(|w| w.set_br(($bit0), true));
        ($bsrr1).write(|w| w.set_br(($bit1), true));

        dshot_nop_remainder!();
    };

    (4, $bsrr0: expr, $bit0: expr, $bsrr1: expr, $bit1: expr, $bsrr2: expr, $bit2: expr, $bsrr3: expr, $bit3: expr) => {
        ($bsrr0).write(|w| w.set_bs(($bit0), true));
        ($bsrr1).write(|w| w.set_bs(($bit1), true));
        ($bsrr2).write(|w| w.set_bs(($bit2), true));
        ($bsrr3).write(|w| w.set_bs(($bit3), true));

        dshot_nop_0!();
        ($bsrr0).write(|w| w.set_br(($bit0), true));
        ($bsrr1).write(|w| w.set_br(($bit1), true));
        ($bsrr3).write(|w| w.set_br(($bit3), true));

        dshot_nop_0_to_1!();
        ($bsrr2).write(|w| w.set_br(($bit2), true));

        dshot_nop_remainder!();
    };

    (5, $bsrr0: expr, $bit0: expr, $bsrr1: expr, $bit1: expr, $bsrr2: expr, $bit2: expr, $bsrr3: expr, $bit3: expr) => {
        ($bsrr0).write(|w| w.set_bs(($bit0), true));
        ($bsrr1).write(|w| w.set_bs(($bit1), true));
        ($bsrr2).write(|w| w.set_bs(($bit2), true));
        ($bsrr3).write(|w| w.set_bs(($bit3), true));

        dshot_nop_0!();
        ($bsrr1).write(|w| w.set_br(($bit1), true));
        ($bsrr3).write(|w| w.set_br(($bit3), true));

        dshot_nop_0_to_1!();
        ($bsrr0).write(|w| w.set_br(($bit0), true));
        ($bsrr2).write(|w| w.set_br(($bit2), true));

        dshot_nop_remainder!();
    };

    (6, $bsrr0: expr, $bit0: expr, $bsrr1: expr, $bit1: expr, $bsrr2: expr, $bit2: expr, $bsrr3: expr, $bit3: expr) => {
        ($bsrr0).write(|w| w.set_bs(($bit0), true));
        ($bsrr1).write(|w| w.set_bs(($bit1), true));
        ($bsrr2).write(|w| w.set_bs(($bit2), true));
        ($bsrr3).write(|w| w.set_bs(($bit3), true));

        dshot_nop_0!();
        ($bsrr0).write(|w| w.set_br(($bit0), true));
        ($bsrr3).write(|w| w.set_br(($bit3), true));

        dshot_nop_0_to_1!();
        ($bsrr1).write(|w| w.set_br(($bit1), true));
        ($bsrr2).write(|w| w.set_br(($bit2), true));

        dshot_nop_remainder!();
    };

    (7, $bsrr0: expr, $bit0: expr, $bsrr1: expr, $bit1: expr, $bsrr2: expr, $bit2: expr, $bsrr3: expr, $bit3: expr) => {
        ($bsrr0).write(|w| w.set_bs(($bit0), true));
        ($bsrr1).write(|w| w.set_bs(($bit1), true));
        ($bsrr2).write(|w| w.set_bs(($bit2), true));
        ($bsrr3).write(|w| w.set_bs(($bit3), true));

        dshot_nop_0!();
        ($bsrr3).write(|w| w.set_br(($bit3), true));

        dshot_nop_0_to_1!();
        ($bsrr0).write(|w| w.set_br(($bit0), true));
        ($bsrr1).write(|w| w.set_br(($bit1), true));
        ($bsrr2).write(|w| w.set_br(($bit2), true));

        dshot_nop_remainder!();
    };

    (8, $bsrr0: expr, $bit0: expr, $bsrr1: expr, $bit1: expr, $bsrr2: expr, $bit2: expr, $bsrr3: expr, $bit3: expr) => {
        ($bsrr0).write(|w| w.set_bs(($bit0), true));
        ($bsrr1).write(|w| w.set_bs(($bit1), true));
        ($bsrr2).write(|w| w.set_bs(($bit2), true));
        ($bsrr3).write(|w| w.set_bs(($bit3), true));

        dshot_nop_0!();
        ($bsrr0).write(|w| w.set_br(($bit0), true));
        ($bsrr1).write(|w| w.set_br(($bit1), true));
        ($bsrr2).write(|w| w.set_br(($bit2), true));

        dshot_nop_0_to_1!();
        ($bsrr3).write(|w| w.set_br(($bit3), true));

        dshot_nop_remainder!();
    };

    (9, $bsrr0: expr, $bit0: expr, $bsrr1: expr, $bit1: expr, $bsrr2: expr, $bit2: expr, $bsrr3: expr, $bit3: expr) => {
        ($bsrr0).write(|w| w.set_bs(($bit0), true));
        ($bsrr1).write(|w| w.set_bs(($bit1), true));
        ($bsrr2).write(|w| w.set_bs(($bit2), true));
        ($bsrr3).write(|w| w.set_bs(($bit3), true));

        dshot_nop_0!();
        ($bsrr1).write(|w| w.set_br(($bit1), true));
        ($bsrr2).write(|w| w.set_br(($bit2), true));

        dshot_nop_0_to_1!();
        ($bsrr0).write(|w| w.set_br(($bit0), true));
        ($bsrr3).write(|w| w.set_br(($bit3), true));

        dshot_nop_remainder!();
    };

    (10, $bsrr0: expr, $bit0: expr, $bsrr1: expr, $bit1: expr, $bsrr2: expr, $bit2: expr, $bsrr3: expr, $bit3: expr) => {
        ($bsrr0).write(|w| w.set_bs(($bit0), true));
        ($bsrr1).write(|w| w.set_bs(($bit1), true));
        ($bsrr2).write(|w| w.set_bs(($bit2), true));
        ($bsrr3).write(|w| w.set_bs(($bit3), true));

        dshot_nop_0!();
        ($bsrr0).write(|w| w.set_br(($bit0), true));
        ($bsrr2).write(|w| w.set_br(($bit2), true));

        dshot_nop_0_to_1!();
        ($bsrr1).write(|w| w.set_br(($bit1), true));
        ($bsrr3).write(|w| w.set_br(($bit3), true));

        dshot_nop_remainder!();
    };

    (11, $bsrr0: expr, $bit0: expr, $bsrr1: expr, $bit1: expr, $bsrr2: expr, $bit2: expr, $bsrr3: expr, $bit3: expr) => {
        ($bsrr0).write(|w| w.set_bs(($bit0), true));
        ($bsrr1).write(|w| w.set_bs(($bit1), true));
        ($bsrr2).write(|w| w.set_bs(($bit2), true));
        ($bsrr3).write(|w| w.set_bs(($bit3), true));

        dshot_nop_0!();
        ($bsrr2).write(|w| w.set_br(($bit2), true));

        dshot_nop_0_to_1!();
        ($bsrr0).write(|w| w.set_br(($bit0), true));
        ($bsrr1).write(|w| w.set_br(($bit1), true));
        ($bsrr3).write(|w| w.set_br(($bit3), true));

        dshot_nop_remainder!();
    };

    (12, $bsrr0: expr, $bit0: expr, $bsrr1: expr, $bit1: expr, $bsrr2: expr, $bit2: expr, $bsrr3: expr, $bit3: expr) => {
        ($bsrr0).write(|w| w.set_bs(($bit0), true));
        ($bsrr1).write(|w| w.set_bs(($bit1), true));
        ($bsrr2).write(|w| w.set_bs(($bit2), true));
        ($bsrr3).write(|w| w.set_bs(($bit3), true));

        dshot_nop_0!();
        ($bsrr0).write(|w| w.set_br(($bit0), true));
        ($bsrr1).write(|w| w.set_br(($bit1), true));

        dshot_nop_0_to_1!();
        ($bsrr2).write(|w| w.set_br(($bit2), true));
        ($bsrr3).write(|w| w.set_br(($bit3), true));

        dshot_nop_remainder!();
    };

    (13, $bsrr0: expr, $bit0: expr, $bsrr1: expr, $bit1: expr, $bsrr2: expr, $bit2: expr, $bsrr3: expr, $bit3: expr) => {
        ($bsrr0).write(|w| w.set_bs(($bit0), true));
        ($bsrr1).write(|w| w.set_bs(($bit1), true));
        ($bsrr2).write(|w| w.set_bs(($bit2), true));
        ($bsrr3).write(|w| w.set_bs(($bit3), true));

        dshot_nop_0!();
        ($bsrr1).write(|w| w.set_br(($bit1), true));

        dshot_nop_0_to_1!();
        ($bsrr0).write(|w| w.set_br(($bit0), true));
        ($bsrr2).write(|w| w.set_br(($bit2), true));
        ($bsrr3).write(|w| w.set_br(($bit3), true));

        dshot_nop_remainder!();
    };

    (14, $bsrr0: expr, $bit0: expr, $bsrr1: expr, $bit1: expr, $bsrr2: expr, $bit2: expr, $bsrr3: expr, $bit3: expr) => {
        ($bsrr0).write(|w| w.set_bs(($bit0), true));
        ($bsrr1).write(|w| w.set_bs(($bit1), true));
        ($bsrr2).write(|w| w.set_bs(($bit2), true));
        ($bsrr3).write(|w| w.set_bs(($bit3), true));

        dshot_nop_0!();
        ($bsrr0).write(|w| w.set_br(($bit0), true));

        dshot_nop_0_to_1!();
        ($bsrr1).write(|w| w.set_br(($bit1), true));
        ($bsrr2).write(|w| w.set_br(($bit2), true));
        ($bsrr3).write(|w| w.set_br(($bit3), true));

        dshot_nop_remainder!();
    };

    (15, $bsrr0: expr, $bit0: expr, $bsrr1: expr, $bit1: expr, $bsrr2: expr, $bit2: expr, $bsrr3: expr, $bit3: expr) => {
        ($bsrr0).write(|w| w.set_bs(($bit0), true));
        ($bsrr1).write(|w| w.set_bs(($bit1), true));
        ($bsrr2).write(|w| w.set_bs(($bit2), true));
        ($bsrr3).write(|w| w.set_bs(($bit3), true));

        dshot_nop_0!();

        dshot_nop_0_to_1!();
        ($bsrr0).write(|w| w.set_br(($bit0), true));
        ($bsrr1).write(|w| w.set_br(($bit1), true));
        ($bsrr2).write(|w| w.set_br(($bit2), true));
        ($bsrr3).write(|w| w.set_br(($bit3), true));

        dshot_nop_remainder!();
    };
}

#[no_mangle]
#[inline(never)]
fn dshot_bitbang(_: &CriticalSection, bsrr: Reg<Bsrr, W>, bit: usize, val: u16) {
    unsafe {
        dshot_bitbang_bit!(bsrr, 32768, val, bit);
        dshot_bitbang_bit!(bsrr, 16384, val, bit);
        dshot_bitbang_bit!(bsrr, 8192, val, bit);
        dshot_bitbang_bit!(bsrr, 4096, val, bit);
        dshot_bitbang_bit!(bsrr, 2048, val, bit);
        dshot_bitbang_bit!(bsrr, 1024, val, bit);
        dshot_bitbang_bit!(bsrr, 512, val, bit);
        dshot_bitbang_bit!(bsrr, 256, val, bit);
        dshot_bitbang_bit!(bsrr, 128, val, bit);
        dshot_bitbang_bit!(bsrr, 64, val, bit);
        dshot_bitbang_bit!(bsrr, 32, val, bit);
        dshot_bitbang_bit!(bsrr, 16, val, bit);
        dshot_bitbang_bit!(bsrr, 8, val, bit);
        dshot_bitbang_bit!(bsrr, 4, val, bit);
        dshot_bitbang_bit!(bsrr, 2, val, bit);
        dshot_bitbang_bit!(bsrr, 1, val, bit);
    }
}

#[no_mangle]
#[inline(never)]
pub fn dshot_send(bsrr: Reg<Bsrr, W>, bit: usize, val: u16) {
    let telemetry = val < 48;
    let mut val = val << 1;
    if telemetry {
        val |= 1;
    }
    let sent = (val & 0xf) ^ ((val >> 4) & 0xf) ^ ((val >> 8) & 0xf) | (val << 4);

    free(|critical_section: &CriticalSection| {
        dshot_bitbang(critical_section, bsrr, bit, sent);
    });
}

pub fn dshot_send_values(bsrrs: [Reg<Bsrr, W>; 4], bits: [usize; 4], mut values: [u16; 4]) {
    for i in 0..4 {
        let telemetry = values[i] < 48;
        let mut val = values[i] << 1;
        if telemetry {
            val |= 1;
        }
        let sent = (val & 0xf) ^ ((val >> 4) & 0xf) ^ ((val >> 8) & 0xf) | (val << 4);
        values[i] = sent;
    }

    let mut value = 0u64;
    let mut mask = 32768u16;
    let mut cursor = 1;
    for _ in 0..16 {
        for i in 0..4 {
            if values[i] & mask != 0 {
                value |= cursor;
            }
            cursor <<= 1;
        }
        mask >>= 1;
    }
    dshot_send_multi(bsrrs, bits, value);
}

pub fn dshot_send_multi(bsrrs: [Reg<Bsrr, W>; 4], bits: [usize; 4], mut value: u64) {
    unsafe {
        for _ in 0..16 {
            match value & 15 {
                0 => {
                    dshot_bitbang_bits!(
                        0, bsrrs[0], bits[0], bsrrs[1], bits[1], bsrrs[2], bits[2], bsrrs[3],
                        bits[3]
                    );
                }
                1 => {
                    dshot_bitbang_bits!(
                        1, bsrrs[0], bits[0], bsrrs[1], bits[1], bsrrs[2], bits[2], bsrrs[3],
                        bits[3]
                    );
                }
                2 => {
                    dshot_bitbang_bits!(
                        2, bsrrs[0], bits[0], bsrrs[1], bits[1], bsrrs[2], bits[2], bsrrs[3],
                        bits[3]
                    );
                }
                3 => {
                    dshot_bitbang_bits!(
                        3, bsrrs[0], bits[0], bsrrs[1], bits[1], bsrrs[2], bits[2], bsrrs[3],
                        bits[3]
                    );
                }
                4 => {
                    dshot_bitbang_bits!(
                        4, bsrrs[0], bits[0], bsrrs[1], bits[1], bsrrs[2], bits[2], bsrrs[3],
                        bits[3]
                    );
                }
                5 => {
                    dshot_bitbang_bits!(
                        5, bsrrs[0], bits[0], bsrrs[1], bits[1], bsrrs[2], bits[2], bsrrs[3],
                        bits[3]
                    );
                }
                6 => {
                    dshot_bitbang_bits!(
                        6, bsrrs[0], bits[0], bsrrs[1], bits[1], bsrrs[2], bits[2], bsrrs[3],
                        bits[3]
                    );
                }
                7 => {
                    dshot_bitbang_bits!(
                        7, bsrrs[0], bits[0], bsrrs[1], bits[1], bsrrs[2], bits[2], bsrrs[3],
                        bits[3]
                    );
                }
                8 => {
                    dshot_bitbang_bits!(
                        8, bsrrs[0], bits[0], bsrrs[1], bits[1], bsrrs[2], bits[2], bsrrs[3],
                        bits[3]
                    );
                }
                9 => {
                    dshot_bitbang_bits!(
                        9, bsrrs[0], bits[0], bsrrs[1], bits[1], bsrrs[2], bits[2], bsrrs[3],
                        bits[3]
                    );
                }
                10 => {
                    dshot_bitbang_bits!(
                        10, bsrrs[0], bits[0], bsrrs[1], bits[1], bsrrs[2], bits[2], bsrrs[3],
                        bits[3]
                    );
                }
                11 => {
                    dshot_bitbang_bits!(
                        11, bsrrs[0], bits[0], bsrrs[1], bits[1], bsrrs[2], bits[2], bsrrs[3],
                        bits[3]
                    );
                }
                12 => {
                    dshot_bitbang_bits!(
                        12, bsrrs[0], bits[0], bsrrs[1], bits[1], bsrrs[2], bits[2], bsrrs[3],
                        bits[3]
                    );
                }
                13 => {
                    dshot_bitbang_bits!(
                        13, bsrrs[0], bits[0], bsrrs[1], bits[1], bsrrs[2], bits[2], bsrrs[3],
                        bits[3]
                    );
                }
                14 => {
                    dshot_bitbang_bits!(
                        14, bsrrs[0], bits[0], bsrrs[1], bits[1], bsrrs[2], bits[2], bsrrs[3],
                        bits[3]
                    );
                }
                15 => {
                    dshot_bitbang_bits!(
                        15, bsrrs[0], bits[0], bsrrs[1], bits[1], bsrrs[2], bits[2], bsrrs[3],
                        bits[3]
                    );
                }
                _ => todo!(),
            }
            value >>= 4;
        }
    }
}
