use crate::nopdelays::*;
use core::arch::asm;
use embassy_stm32::{
    gpio::{Input, Level, Output, Pull, Speed},
    pac::{
        self,
        common::{Reg, W},
        gpio::{regs::Bsrr, vals},
        GPIOA, GPIOC,
    },
    peripherals::PC13,
    timer,
    usart::{Parity, Uart},
    Peripheral,
};

#[macro_export]
macro_rules! dshot_bitbang_bit {
    ($bsrr: expr, $x: expr, $val: expr, $bit: expr) => {
        ($bsrr).write(|w| w.set_bs(($bit), true));
        if ($val) & ($x) == ($x) {
            nop112!();
            ($bsrr).write(|w| w.set_br(($bit), true));
            nop38!();
        } else {
            nop58!();
            ($bsrr).write(|w| w.set_br(($bit), true));
            nop93!();
        }
    };
}

#[no_mangle]
fn dshot_bitbang(bsrr: Reg<Bsrr, W>, bit: usize, val: u16) {
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
pub fn dshot_send(bsrr: Reg<Bsrr, W>, bit: usize, val: u16) {
    let val = val << 1;
    let sent = (val & 0xf) ^ ((val >> 4) & 0xf) ^ ((val >> 8) & 0xf) | (val << 4);
    dshot_bitbang(bsrr, bit, sent);
}
