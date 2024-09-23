#![no_std]
#![no_main]

use core::arch::asm;
use core::cmp::min;

use crsf::{bluetooth_receiver_task, crsf_receiver_task};
use defmt::{error, info, println};
use dshot::dshot_send;
use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_stm32::pac::usart::Uart;
use embassy_stm32::pac::GPIOB;
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, i2c, peripherals, spi, Config};
use embassy_stm32::{
    gpio::{Input, Level, Output, Pull, Speed},
    pac::GPIOA,
    usart::{self},
};
use embassy_sync::lazy_lock::LazyLock;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::{Duration, Instant, Ticker, Timer};
use embedded_hal::spi::{Polarity, MODE_0, MODE_1, MODE_2, MODE_3};
use embedded_io::Write;
use embedded_io_async::Read;
use mpu6050::MPU6050;
use storage::Store;
use zerocopy::{big_endian, AsBytes, FromBytes, FromZeroes, Unaligned};
use {defmt_rtt as _, panic_probe as _};

mod crsf;
mod dshot;
mod icm42688;
mod mpu6050;
mod nopdelays;
mod storage;

bind_interrupts!(struct Irqs {
    USART2 => usart::InterruptHandler<peripherals::USART2>;
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

/*static COMMANDS_CHANNEL: Channel<ThreadModeRawMutex, CommandPacket, 10> = Channel::new();

#[derive(AsBytes, FromBytes, FromZeroes, Unaligned, Default)]
#[repr(C)]
struct Channels {
    pub vals: [u8; 22],
}

#[derive(AsBytes, FromBytes, FromZeroes, Unaligned, Default)]
#[repr(C)]
struct LinkStatistics {
    pub UplinkRssiAnt1: u8,
    pub UplinkRssiAnt2: u8,
    pub UplinkSuccessRate: u8,
    pub UplinkSNR: i8,
    pub ActiveAntenna: u8,
    pub RfMode: u8,
    pub TxPower: u8,
    pub UplinkRSSI: u8,
    pub DownlinkSuccessRate: u8,
    pub DownlinkSNR: i8,
}

impl defmt::Format for LinkStatistics {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "LinkStatistics RfMode:{} UplinkSuccessRate:{} UplinkSNR:{}",
            self.RfMode,
            self.UplinkSuccessRate,
            self.UplinkRssiAnt1
        );
    }
}

#[derive(AsBytes, FromBytes, FromZeroes, Unaligned, Default)]
#[repr(C)]
struct BatteryInfo {
    pub voltage: big_endian::I16,
    pub data:[u8;6]
}

#[derive(AsBytes, FromBytes, FromZeroes, Unaligned, Default)]
#[repr(C)]
struct BatPacket {
    sync:u8,
    len:u8,
    typ:u8,
    payload:[u8;8],
    crc8:u8
}

impl BatPacket{
    pub fn new(b:&BatteryInfo)->Self {
        let mut payload = [0u8;8];
        b.write_to(&mut payload).unwrap();
        let mut packet = BatPacket{
            sync:0xc8u8,
            len: (payload.len()+2) as u8,
            typ:0x08u8,
            payload:payload,
            crc8: 0u8
        };

        let buffer = packet.as_bytes_mut();
        let crc = crccalc(&buffer[2..buffer.len()-1]);
        buffer[buffer.len()-1]=crc;
        info!("SENDCRC {}", crc);

        return packet;
    }
}

impl Channels {
    pub fn get_channels(&self) -> [u16; 16] {
        let mut chanels = [0u16; 16];
        let mut src_idx = 0;
        let mut dst_idx = 0;
        let mut src_pos = 0;
        let mut dst_pos = 0;

        while dst_idx < 16 {
            let src = self.vals[src_idx];
            let bitcount = min(11 - dst_pos, 8 - src_pos);
            let bits = (src >> src_pos) as u16;
            let bits = bits & ((1 << bitcount) - 1);
            chanels[dst_idx] |= bits << dst_pos;

            src_pos += bitcount;
            dst_pos += bitcount;
            if src_pos == 8 {
                src_idx += 1;
                src_pos = 0;
            }
            if dst_pos == 11 {
                dst_idx += 1;
                dst_pos = 0;
            }
        }

        return chanels;
    }
}
*/


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    static STORE: LazyLock<Store> = LazyLock::new(|| {Store::new()});

    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(25_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV25,
            mul: PllMul::MUL400,
            divp: Some(PllPDiv::DIV4), // 25hz / 4 * 336 / 2 = 180Mhz.
            divq: Some(PllQDiv::DIV4),
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV2;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.sys = Sysclk::PLL1_P;
    }

    let peripherals = embassy_stm32::init(config);

    let (mut crsf_rx, mut crsf_tx) = crsf::make_uart_pair(
        GPIOA,
        3,
        peripherals.USART2,
        peripherals.PA3,
        peripherals.PA2,
        peripherals.DMA1_CH6,
        peripherals.DMA1_CH7,
        Irqs,
    );


    _spawner
            .spawn(crsf_receiver_task(crsf_rx, STORE.get()))
            .unwrap();
    /*let mut adc = Adc::new(peripherals.ADC1);
    adc.set_resolution(embassy_stm32::adc::Resolution::BITS10);
    let mut apin = peripherals.PA5;

    //let mut uartrx = bt_rx.into_ring_buffered(&mut ring_buffer);
    let mut cmdbuf = [0u8; 64];
    loop {
        if let Ok(_) = uart.read_exact(&mut cmdbuf[0..1]).await {
            if cmdbuf[0] == 0xc8u8 {
                if let Ok(_) = uart.read_exact(&mut cmdbuf[1..2]).await {
                    let rest_len: usize = cmdbuf[1].into();
                    if rest_len <= 62 {
                        if let Ok(_) = uart.read_exact(&mut cmdbuf[2..rest_len + 2]).await {
                            let c = crccalc(&cmdbuf[2..rest_len + 1]);
                            if c == cmdbuf[rest_len + 1] {
                                if cmdbuf[2] == 0x16u8 {
                                    let k = Channels::read_from(&cmdbuf[3..rest_len + 1]);
                                    /*info!(
                                        "REST {:x} {}",
                                        cmdbuf[2..rest_len + 2],
                                        k.unwrap().get_channels()
                                    );*/
                                } else {
                                    let k = LinkStatistics::read_from(&cmdbuf[3..rest_len + 1]);
                                    info!("{}", k.unwrap());
                                }
                            }
                        }
                    }
                }
            }
        }
        let x = adc.blocking_read(&mut apin) as f32;
        let vltg = x/1024f32*3.3f32;
        let r1=1.15f32;
        let r2=9.7f32;
        let vltg = vltg/r1*(r1+r2);
        info!("Analog {}", vltg);
        let v = (vltg*10f32) as i16;
        let mut bi = BatteryInfo::default();
        bi.voltage = v.into();
        let pack = BatPacket::new(&bi);
        let packbuf = pack.as_bytes();
        info!("SENDC {}", packbuf);
        tx.write(packbuf).await.unwrap();
    }

    /*
    Timer::after_millis(100).await;


    let mut led: Output = Output::new(peripherals.PC14, Level::Low, Speed::VeryHigh);
    let mut green: Output = Output::new(peripherals.PA4, Level::Low, Speed::VeryHigh);
    let mut blue: Output = Output::new(peripherals.PA11, Level::Low, Speed::VeryHigh);
    let mut yellow: Output = Output::new(peripherals.PA12, Level::Low, Speed::VeryHigh);

    let mut motor0: Output = Output::new(peripherals.PB0, Level::Low, Speed::VeryHigh);
    let mut motor1: Output = Output::new(peripherals.PB1, Level::Low, Speed::VeryHigh);
    let mut motor2: Output = Output::new(peripherals.PA6, Level::Low, Speed::VeryHigh);
    let mut motor3: Output = Output::new(peripherals.PA7, Level::Low, Speed::VeryHigh);



    let mut cs =  Output::new(peripherals.PB9, Level::High, Speed::VeryHigh);
    //let mut button = Input::new(peripherals.PA0, Pull::Up);
    led.set_low();

    let mut spic = spi::Config::default();
    spic.frequency = Hertz(10_000_000);
    spic.mode = MODE_3;

    //let mut sp = spi::Spi::new(peripherals.SPI3, peripherals.PB3, peripherals.PB5, peripherals.PB4, peripherals.DMA1_CH5, peripherals.DMA1_CH0, spic);
    // let mut mpu = mpu6050::MPU6050::new(peripherals.I2C1, peripherals.PB6, peripherals.PB7, Irqs, peripherals.DMA1_CH6, peripherals.DMA1_CH5);
    let bt_rx = bluetooth::make_bluetooth_uart(
        GPIOA,
        3,
        peripherals.USART2,
        peripherals.PA3,
        peripherals.PA2,
        peripherals.DMA1_CH7,
        Irqs,
    );


    Timer::after_millis(1000).await;
    let mut icm = icm42688::ICM42688::new(peripherals.SPI3, peripherals.PB3, peripherals.PB5, peripherals.PB4, peripherals.DMA1_CH5, peripherals.DMA1_CH0, cs);
    Timer::after_millis(1000).await;
    icm.init(icm42688::GyroRange::GYRO_CONFIG_FS_SEL_250);
    Timer::after_millis(1000).await;


    let mut bias_y = 0.0f32;
    let mut bias_p = -0.08169041;
    let mut bias_r = 0.0f32;

    let mut ticker = Ticker::every(Duration::from_micros(1250));
    let mut fsu = 0f32;

    for k1 in 1..=5 {
        for cnt in 1..=100 {
            let (y,p,r) = icm.get_ypr_deg();
            bias_y += (y-bias_y)/((cnt+1000) as f32);
            bias_p += (p-bias_p)/((cnt+1000) as f32);
            bias_r += (r-bias_r)/((cnt+1000) as f32);
            info!("{=f32:04} {} {}", y-bias_y, cnt, k1);
            ticker.next().await;
        }
    }
    loop {
        led.set_high();
        let (y,p,r) = icm.get_ypr_deg();
        fsu += (p-bias_p)*0.00125;
        info!("{=f32:04} {}", fsu, bias_p);
        led.set_low();

        ticker.next().await;
    }
    //GPIOB.bsrr().write(|w|{w.br(3)});

    /*Timer::after_millis(20).await;

    loop {
    led.set_high();
    Timer::after_millis(100).await;
    led.set_low();

    let mut rd = [0u8;1];

    Timer::after_millis(50).await;
    sp.blocking_write(&[0xacu8, 0x53u8]).unwrap();
    sp.blocking_transfer(&mut rd, &[0x00u8]).unwrap();
    sp.blocking_write(&[0x00u8]).unwrap();
    info!("SPI {}", rd);
    Timer::after_millis(500).await;


    }*/

    /*

    info!("MPUSTATUS {}", mpu.test_connection());
    mpu.init(mpu6050::GyroRange::GYRO_CONFIG_FS_SEL_250).await;

    _spawner
        .spawn(commands_task(COMMANDS_CHANNEL.receiver(), STORE.get()))
        .unwrap();

    _spawner
        .spawn(bluetooth_receiver_task(bt_rx, COMMANDS_CHANNEL.sender()))
        .unwrap();

    _spawner.spawn(tick_task(led, STORE.get(), mpu)).unwrap();*/

    */*/
}

#[embassy_executor::task]
async fn tick_task(mut led: Output<'static>, store: &'static Store, mut mpu: MPU6050) {
    let mut ticker = Ticker::every(Duration::from_micros(500));
    let mut up = false;
    let mut xp = 0f32;

    let mut y = 0f32;
    let mut p = 0f32;
    let mut r = 0f32;

    loop {
        let t1 = embassy_time::Instant::now();
        let ticks: f32 = t1.as_ticks() as f32;
        let t2 = ticks / 1000.0;
        let t3 = t2 as i32;
        let t4 = t2 - (t3 as f32);
        let x = store.snapshot().await.variables.Yaw_Kp();
        if x != xp {
            if t4 > 0.5 {
                info!("AAAAAAAAAAA {}", x);
            } else {
                info!("BBBBBBBBBB {}", x);
            }
            xp = x;
        } else {
        }

        led.set_high();
        (y, p, r) = mpu.get_ypr_deg().await;
        led.set_low();
        info!("GYRO {}", (y, p, r));
        ticker.next().await;
    }
}
