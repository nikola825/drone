// This module implements the DShot protocol by generating a waveform on GPIO pins using DMA.
// We assume that all ESCs are attached to pins of the same GPIO port.
//
// The DMA transfers are driven by a timer that is running at 3x the DShot bitrate
// For example, for DShot 600, the timer runs at 1800 KHz
// The DMA is fed with an array of 32bit words and writes one word into Bsrr register at every timer tick
//
// To transmit a single DShot bit, we need to do 3 writes to Bsrr
// The first write pulls the pin(s) high for each ESC
// The second write pulls the pin(s) low for outputs where the current bit is zero
// The third write pulls all the pins(s) low
//
// For DShot 600, these writes happen at 1800 KHz
// This way, when transmiting a zero, the pin stays on for ~556 nS - it gets pulled up by the first write to Bsrr, and pulled down by the second write
// And when transmitting a one, the pin stays on for ~1.1 uS - it gets pulled up by the first write, second write does nothing, third write pulls it down
// We repeat these 3 transfers 16 times
// Because we're writing to the whole Bsrr port, we can toggle multiple pins at a time allowing us to control up to 16 ESCs through a single GPIO port
//
// A single DShot bit transfer is represented by the DshotBit struct, it contains the 3 Bsrr values for 3 writes explained above
// A whole DShot burst is represented by the DshotDmaPacketInner struct, it contains an array of 16 DhotBit elements
// The DshotDmaBuffer type represents the same data as DshotDmaPacketInner, just as an array of u32 words.
// `DshotDmaPacket` is a union of `DshotDmaPacketInner` and `DshotDmaBuffer`
// This gives us two "views" to the same piece of memory - we use the DshotDmaPacketInner when generating the Bsrr values
// and we use DshotDmaBuffer when writing to DMA because DMA API expect an &[u32] as input
//
// To ensure union safety, this is checked at compile time in the `_ensure_transmuatability` function:
// sizeof(DshotDmaPacket) == sizeof(DshotDmaBuffer) == sizeof(DshotDmaPacketInner)
//
// When initiating a DMA transfer, DMA will write the first word immediately, even before the timer ticks
// For this reason, DshotDmaPacket also contains a padding word at the start, to make sure the length of the first bit stays consisent
//
// A single DShot burst goes like this
// 1. timer is stopped and reset to zero
// 2. DshotDmaPacketInner is filled with proper values for each write to Bsrr
// 3. DMA transfer is initiated, using the packet as a memory source, first word (the padding) gets written to Bsrr - padding has a value of zero so nothing happens
// 4. Timer is started, it ticks down and the DMA writes the first actual byte to Bsrr and pulls the pins high
// 5. We let the whole thing go, and wait for DMA to transfer the whole buffer
//
// Without the padding word, the write in step #3 would have pulled the pins high immediately because it would read the first Bsrr value
// and any delay between steps #3 and #4 (by an interrupt for example) would have affected the length of the first high-period
// For this reason, we use the dummy padding word as the first write - the dummy word does not toggle any pins, so the delay between #3 and #4 will not affect anything
// Once the timer in step #4 runs, it will keep the proper tempo regardles of any delays in the code execution

use embassy_stm32::{
    dma::{Burst, FifoThreshold, Transfer, TransferOptions},
    pac::gpio::regs::Bsrr,
    time::khz,
    timer::UpDma,
    Peri,
};

use crate::hal::{DSHOT_DMA, DSHOT_TIMER};

const DSHOT_BITRATE: u32 = 600;
const TIMER_FREQUENCY: u32 = DSHOT_BITRATE * 3;

const DSHOT_PACKET_BITS: usize = 16;

const DSHOT_DMA_START_PADDING_WORDS: usize = 1;
const DSHOT_DMA_BUFFER_WORDS_PER_DSHOT_BIT: usize = 3;
const DSHOT_DMA_BUFFER_WORDS: usize =
    DSHOT_PACKET_BITS * DSHOT_DMA_BUFFER_WORDS_PER_DSHOT_BIT + DSHOT_DMA_START_PADDING_WORDS;

// This is the same data as DshotDmaPacket represented as an array of u32s for the DMA
type DshotDmaBuffer = [u32; DSHOT_DMA_BUFFER_WORDS];

// This is the array of 32-bit words that gets sent into GPIOx_Bsrr at every tick of the timer
// The padding segment is a zero that gets written to DMA and immediately sent to Bsrr before the timer starts
#[derive(Clone, Copy)]
#[repr(C, packed(4))]
struct DshotDmaPacketInner {
    start_padding: [u32; DSHOT_DMA_START_PADDING_WORDS],
    dshot_bits: [DshotBit; DSHOT_PACKET_BITS],
}

// When generating the Bsrr values before the write, we access the packet field
// When sending words to DMA, we access the dma_buffer fields
// union allows us to have two "views" of the same memory without manually copying data
#[repr(C, packed(4))]
union DshotDmaPacket {
    packet: DshotDmaPacketInner,
    dma_buffer: DshotDmaBuffer,
}

impl DshotDmaPacket {
    fn inner_packet(&mut self) -> &mut DshotDmaPacketInner {
        unsafe { &mut self.packet }
    }

    fn as_dma_buffer(&self) -> &DshotDmaBuffer {
        unsafe { &self.dma_buffer }
    }

    // This unused function serves as a compile-time check to ensure sizeof(DshotDmaPacket)==sizeof(DshotDmaBuffer)==sizeof(DshotDmaPacketInner)
    // This allows us to safely access both parts of the union and to know they have the same size and no padding
    // It serves to prevent compilation in case any mistakes are made in the definition of `DSHOT_DMA_BUFFER_WORDS` and other related constants
    fn _ensure_transmuatability(
        packet: &DshotDmaPacket,
        packet_inner: &DshotDmaPacketInner,
        buffer: &DshotDmaBuffer,
    ) {
        /*
        Portions of this function have been copied from static-assertions crate
        https://github.com/nvzqz/static-assertions

        MIT License

        Copyright (c) 2017 Nikolai Vazquez

        Permission is hereby granted, free of charge, to any person obtaining a copy
        of this software and associated documentation files (the "Software"), to deal
        in the Software without restriction, including without limitation the rights
        to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
        copies of the Software, and to permit persons to whom the Software is
        furnished to do so, subject to the following conditions:

        The above copyright notice and this permission notice shall be included in all
        copies or substantial portions of the Software.

        THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
        IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
        FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
        AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
        LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
        OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
        SOFTWARE.

        */
        #[allow(
            unknown_lints,
            unsafe_code,
            forget_copy,
            clippy::useless_transmute,
            clippy::forget_non_drop,
            forgetting_copy_types
        )]
        unsafe {
            use core::mem;
            use core::ptr;

            let mut copy = ptr::read(packet);
            ptr::write(
                &mut copy,
                mem::transmute::<DshotDmaPacketInner, DshotDmaPacket>(ptr::read(packet_inner)),
            );
            mem::forget(copy);

            let mut copy = ptr::read(packet_inner);
            ptr::write(
                &mut copy,
                mem::transmute::<DshotDmaBuffer, DshotDmaPacketInner>(ptr::read(buffer)),
            );
            mem::forget(copy);
        }
    }
}

// `DshotBit` repesents the sequence of writes to Bsrr to transmit one bit over DShot
// The `initial` write pulls all the outputs to one
// The `middle` pulls down the outputs for which we are outputting a zero bit
// The `end` pulls down all the outputs
#[derive(Clone, Copy)]
#[repr(C, packed(4))]
struct DshotBit {
    initial: Bsrr,
    middle: Bsrr,
    end: Bsrr,
}

impl DshotBit {
    // We keep a static buffer used as a source DMA transfers
    // It is initialized to represent a burst of zero bits
    pub fn zero_bit(pins: &[u8]) -> Self {
        let mut initial = Bsrr::default();
        let mut middle = Bsrr::default();
        let mut end = Bsrr::default();

        for pin in pins {
            initial.set_bs(*pin as usize, true);
            middle.set_br(*pin as usize, true);
            end.set_br(*pin as usize, true);
        }

        Self {
            initial,
            middle,
            end,
        }
    }

    // To turn a bit into a one, do not pull the pin down on the second timer tick
    // We do not touch the initial and end Bsrr values- those were populated at the start and stay the same
    pub fn make_one(&mut self, pin: usize) {
        self.middle.set_br(pin, false);
    }

    // To turn a bit into a zero, pull the pin down on the second timer tick
    // We do not touch the initial and end Bsrr values- those were populated at the start and stay the same
    pub fn make_zero(&mut self, pin: usize) {
        self.middle.set_br(pin, true);
    }
}

pub struct DshotDma {
    timer: embassy_stm32::timer::low_level::Timer<'static, DSHOT_TIMER>,
    gpio: embassy_stm32::pac::gpio::Gpio,
    dma: Peri<'static, DSHOT_DMA>,
    packet: DshotDmaPacket,
    dma_transfer_options: TransferOptions,
    //channel: embassy_stm32::dma::Channel<'static>,
}

impl DshotDma {
    // Initiate the DMA, set up the timer, and generate the initial value for the buffer
    // The pins parameter tells us which pins of the GPIO port have ESCs attached to them
    pub fn new(
        timer: Peri<'static, DSHOT_TIMER>,
        dma: Peri<'static, DSHOT_DMA>,
        gpio: embassy_stm32::pac::gpio::Gpio,
        pins: &[u8],
    ) -> Self {
        let mut dma_transfer_options = TransferOptions::default();

        // Use the DMA fifo to make it spend less time on the bus when reading memory
        dma_transfer_options.fifo_threshold = Some(FifoThreshold::Full);
        dma_transfer_options.mburst = Burst::Incr4;

        Self {
            timer: embassy_stm32::timer::low_level::Timer::new(timer),
            gpio,
            dma,
            packet: DshotDmaPacket {
                packet: DshotDmaPacketInner {
                    start_padding: [0; DSHOT_DMA_START_PADDING_WORDS],
                    dshot_bits: [DshotBit::zero_bit(pins); DSHOT_PACKET_BITS],
                },
            },
            dma_transfer_options,
        }
    }

    pub fn init(&self) {
        // Counting down means that doing timer.reset(); timer.start(); will generate a tick immediately
        // This saves us a few nS of latency
        self.timer
            .set_counting_mode(embassy_stm32::timer::low_level::CountingMode::EdgeAlignedDown);

        self.timer.set_frequency(khz(TIMER_FREQUENCY));
        self.timer.enable_update_dma(true);

        // Timer is stopped when not transferring
        self.timer.stop();
    }

    fn request<Dma: UpDma<DSHOT_TIMER>>(dma: &Peri<'static, Dma>) -> u8 {
        // Get the request number
        // This number is given to DMA to tell it to use the given timer to dictate the tempo
        dma.request()
    }

    pub fn stop(&self) {
        self.timer.stop();
        self.timer.enable_update_dma(false);
    }

    fn fill_packet<const MOTOR_COUNT: usize>(
        &mut self,
        values: [u16; MOTOR_COUNT],
        pins: [u8; MOTOR_COUNT],
    ) {
        // For every transfer, we update the buffer in place
        let mut mask: u16 = 1 << 15;

        let packet = self.packet.inner_packet();

        for bit in &mut packet.dshot_bits {
            for index in 0..MOTOR_COUNT {
                if values[index] & mask == 0 {
                    bit.make_zero(pins[index] as usize);
                } else {
                    bit.make_one(pins[index] as usize);
                }
            }
            mask >>= 1;
        }
    }

    async fn dma_send_packet(&mut self) {
        let request = Self::request(&self.dma);

        // Keep the timer off while we set up things
        self.timer.stop();
        self.timer.reset(); // this sets the counter to zero

        unsafe {
            // This will start the DMA and write the first word (the padding) to Bsrr
            let transfer = Transfer::new_write(
                self.dma.reborrow(),
                request,
                self.packet.as_dma_buffer(),
                self.gpio.bsrr().as_ptr() as *mut u32,
                self.dma_transfer_options,
            );

            // At this point, the DMA is doing nothing - it's waiting for a tick from the timer

            // Timer starts, rolls over from zero, and triggers an update event
            // This makes the DMA write the first actual non-padding word into the Bsrr
            self.timer.start();

            // We let it tick until the whole transfer is done
            transfer.await;
            self.timer.stop();
        }
    }

    async fn send_packet<const MOTOR_COUNT: usize>(
        &mut self,
        pins: [u8; MOTOR_COUNT],
        values: [u16; MOTOR_COUNT],
    ) {
        // Fill the DMA packet buffer in place before sending it
        self.fill_packet(values, pins);

        self.dma_send_packet().await
    }

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

    /// Bit-bang the given 16-bit dshot values through the given pins on the GPIO port
    /// pins - numbers of the used pins on the given port
    /// values - raw values being transmitted (without checksum and telemetry bit)
    pub async fn dshot_send_parallel<const COUNT: usize>(
        &mut self,
        pins: [u8; COUNT],
        mut values: [u16; COUNT],
    ) {
        // Modify the values to append telemetry bit and checksum before sending them
        for value in &mut values {
            *value = Self::add_telemetry_bit_and_checksum(*value);
        }

        self.send_packet(pins, values).await;
    }
}
