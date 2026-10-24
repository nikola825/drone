#![cfg(feature = "gyro-dumping")]

use common::shared_objects::{GyroData, GyroDataPacket};
use embassy_futures::join::join3;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{Duration, Instant, Ticker, Timer};
use zerocopy::IntoBytes;

use crate::{
    esc::EscMotorSet,
    filtering::GyroFilter,
    generic_hardware_type,
    hal::{Leds, UsbSerialWrapper},
    icm42688::ICM42688,
};

static GYRO_DATA: Signal<CriticalSectionRawMutex, GyroData> = Signal::new();

const LOOP_FREQUENCY: f32 = 1000f32;
const LOOP_PERIOD_SECS: f32 = 1f32 / LOOP_FREQUENCY;
const LOOP_PERIOD_MICROS: u64 = (LOOP_PERIOD_SECS * 1e6) as u64;
const LOOP_PERIOD_DURATION: Duration = Duration::from_micros(LOOP_PERIOD_MICROS);

pub async fn usb_send_loop(mut usb: UsbSerialWrapper) -> ! {
    usb.wait_for_connection().await;
    loop {
        let data = GyroDataPacket {
            header: 0xdeadbeef.into(),
            data: GYRO_DATA.wait().await,
        };
        usb.write_chunked(data.as_bytes()).await.unwrap();
    }
}

pub async fn imu_read_loop(mut imu: ICM42688, mut leds: Leds, mut motor_set: EscMotorSet) -> ! {
    let mut ticker = Ticker::every(LOOP_PERIOD_DURATION + Duration::from_micros(5));
    let mut last_print = Instant::now();
    let mut ledson = false;

    let mut gyro_filter = GyroFilter::new(100f32, LOOP_PERIOD_SECS);

    loop {
        if Instant::now() - last_print > Duration::from_millis(200) {
            last_print = Instant::now();
            if ledson {
                leds.blue_on();
            } else {
                leds.blue_off();
            }
            ledson = !ledson;
        }

        let imudata = gyro_filter.apply(imu.get_motion_data());
        motor_set.multi_throttle([0, 0, 0, 0]).await;

        GYRO_DATA.signal(GyroData {
            y: imudata.gyro_yaw.into(),
            p: imudata.gyro_pitch.into(),
            r: imudata.gyro_roll.into(),
        });
        ticker.next().await;
    }
}

pub async fn gyro_dumper_main(hardware: generic_hardware_type!()) -> ! {
    let motor_set = hardware.motor_layout.get_motors();

    let leds: Leds = hardware.led_pins.into();

    let mut imu = ICM42688::new(hardware.imu_spi);
    imu.init().await.unwrap();

    Timer::after_millis(10).await;

    let mut usb_port = hardware.usb.into_usb_port(true);

    let usb_serial = UsbSerialWrapper::new(usb_port.cdc_acm);

    let usb_fut = usb_port.usb_device.run();
    let msp_fut = usb_send_loop(usb_serial);
    let imu_fut = imu_read_loop(imu, leds, motor_set);

    join3(usb_fut, msp_fut, imu_fut).await;

    loop {}
}
