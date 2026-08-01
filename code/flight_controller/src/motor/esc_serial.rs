use embassy_time::{Duration, Instant};

use crate::motor::Motor;

pub enum EscCommunicationError {
    #[allow(dead_code)]
    NoSuccessCode(u8),
    #[allow(dead_code)]
    ReceiveTooShort(usize, usize),
    #[allow(dead_code)]
    BadCrc(u16, u16),
}

const BAUD_RATE: u64 = 19200;
const BAUD_BIT_TIME_US: u64 = 1000000 / BAUD_RATE;
const BIT_DURATION: Duration = Duration::from_micros(BAUD_BIT_TIME_US);
const START_BIT_DURATION: Duration = Duration::from_micros((BAUD_BIT_TIME_US * 3) / 4);

const ESC_SUCCESS: u8 = 0x30;

#[no_mangle]
#[inline(never)]
fn delay_until(instant: Instant) {
    #[allow(clippy::unit_arg)]
    core::hint::black_box(while Instant::now() < instant {});
}

impl Motor {
    #[no_mangle]
    #[inline(never)]
    fn read_byte16(&mut self, timeout: Instant) -> Option<u16> {
        #[allow(clippy::unit_arg)]
        core::hint::black_box(while self.flex.is_high() {
            if Instant::now() > timeout {
                return None;
            }
        });

        let mut next_bit = Instant::now() + START_BIT_DURATION;
        let mut data: u16 = 0;

        for position in 0..10 {
            delay_until(next_bit);
            if self.flex.is_high() {
                data |= 1 << position;
            }

            next_bit += BIT_DURATION;
        }

        Some(data)
    }

    #[no_mangle]
    #[inline(never)]
    pub fn read(
        &mut self,
        buffer: &mut [u8],
        expected_size: usize,
        test_crc: bool,
        timeout_ms: u64,
    ) -> Result<(), EscCommunicationError> {
        let timeout = Instant::now() + Duration::from_millis(timeout_ms);
        let payload_size = expected_size;
        let expected_size = if test_crc {
            expected_size + 3 // CRC (2 bytes) + ACK (1 byte)
        } else {
            expected_size + 1 // ACK (1 byte)
        };

        let mut count: usize = 0;
        let mut inner_buffer = [0u16; 256];

        for _ in 0..expected_size {
            let byte = core::hint::black_box(self.read_byte16(timeout));
            if let Some(byte) = byte {
                inner_buffer[count] = byte;
                count += 1;
            } else {
                break;
            }
        }

        let max_len = core::cmp::min(buffer.len(), 256);

        for i in 0..max_len {
            buffer[i] = ((inner_buffer[i] >> 1) & 0xff) as u8;
        }

        if count < expected_size {
            return Err(EscCommunicationError::ReceiveTooShort(expected_size, count));
        }

        if buffer[expected_size - 1] != ESC_SUCCESS {
            return Err(EscCommunicationError::NoSuccessCode(
                buffer[expected_size - 1],
            ));
        }

        if test_crc {
            let mut calculated_crc = 0u16;

            for x in &buffer[..payload_size] {
                calculated_crc = byte_crc(*x, calculated_crc);
            }

            let low = buffer[expected_size - 3];
            let high = buffer[expected_size - 2];

            let received_crc: u16 = ((high as u16) << 8) | (low as u16);

            if received_crc != calculated_crc {
                return Err(EscCommunicationError::BadCrc(calculated_crc, received_crc));
            }
        }

        Ok(())
    }

    #[no_mangle]
    #[inline(never)]
    fn send_byte(&mut self, byte: u8) {
        // append stop and start bit to either sides
        let mut expanded: u16 = ((byte as u16) << 2) | 1 | (1 << 10);

        let mut to_send = 11;
        let mut next_bit_time = Instant::now() + BIT_DURATION;

        while to_send != 0 {
            if expanded & 1 == 1 {
                self.flex.set_high();
            } else {
                self.flex.set_low();
            }

            next_bit_time += BIT_DURATION;
            expanded >>= 1;
            to_send -= 1;
            if to_send == 0 {
                break;
            }
            delay_until(next_bit_time);
        }

        self.flex.set_high();
    }

    pub fn send(&mut self, data: &[u8], crc: bool) {
        let crc = if crc {
            let mut crc: u16 = 0;
            for x in data {
                crc = byte_crc(*x, crc);
            }

            let low = (crc & 0xff) as u8;
            let high = ((crc >> 8) & 0xff) as u8;

            Some((low, high))
        } else {
            None
        };

        for byte in data {
            self.send_byte(*byte);
        }

        if let Some((low, high)) = crc {
            self.send_byte(low);
            self.send_byte(high);
        }
    }
}

fn byte_crc(data: u8, mut crc: u16) -> u16 {
    let mut xb = data as u16;
    for _ in 0..8 {
        if ((xb & 0x01) ^ (crc & 0x0001)) != 0 {
            crc >>= 1;
            crc ^= 0xA001;
        } else {
            crc >>= 1;
        }
        xb >>= 1;
    }
    crc
}
