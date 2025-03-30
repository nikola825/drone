use core::cmp::{max, min};
use num_traits::float::FloatCore;

use crate::{hw_select::UartMaker, logging::info, storage::Store, ArmingContext};
use embassy_stm32::{
    mode::Async,
    usart::{Parity, StopBits, UartRx, UartTx},
};
use embassy_time::Timer;

const FC_VARIANT: &[u8] = b"BTFL";

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Clone, Copy)]
enum MSPMessageType {
    FC_VARIANT = 0x02,  // 2 - FC variant string
    STATUS = 0x65,      // 101 - FC status - arming state mainly
    RC = 0x69,          // 105 - Stick positions
    DISPLAYPORT = 0xb6, // 182 - Display commands
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Clone, Copy)]
enum MSPDisplayportMessageType {
    CLEAR = 0x01,  // 1 - clear display
    WRITE = 0x03,  // 3 - write string to display
    DRAW = 0x04,   // 4 - draw written strings on the display
    CONFIG = 0x05, // 5 - configure display resolution
}

#[allow(non_camel_case_types, dead_code)]
#[derive(Clone, Copy)]
enum HDZeroResolution {
    SD_3016 = 0x00,
    HD_5018 = 0x01,
    HD_3016 = 0x02,
}

async fn transmit_msp_message(
    tx: &mut UartTx<'static, Async>,
    message_type: MSPMessageType,
    data: &[u8],
) -> Result<(), embassy_stm32::usart::Error> {
    let header = [b'$', b'M', b'>', data.len() as u8, message_type as u8];

    let xor: u8 = (data.len() as u8) ^ (message_type as u8);
    let xor = data
        .iter()
        .fold(xor, |accumulator, element| accumulator ^ *element);

    tx.write(&header).await?;
    tx.write(data).await?;
    tx.write(&[xor]).await
}

async fn transmit_fc_variant(
    tx: &mut UartTx<'static, Async>,
) -> Result<(), embassy_stm32::usart::Error> {
    transmit_msp_message(tx, MSPMessageType::FC_VARIANT, FC_VARIANT).await
}

async fn transmit_status(
    tx: &mut UartTx<'static, Async>,
    armed: bool,
) -> Result<(), embassy_stm32::usart::Error> {
    const DISARMED_STATUS: [u8; 22] = [0u8; 22];
    const ARMED_STATUS: [u8; 22] = [
        0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    if armed {
        transmit_msp_message(tx, MSPMessageType::STATUS, &ARMED_STATUS).await
    } else {
        transmit_msp_message(tx, MSPMessageType::STATUS, &DISARMED_STATUS).await
    }
}

async fn transmit_sticks(
    tx: &mut UartTx<'static, Async>,
    throttle: u16,
    yaw: u16,
    pitch: u16,
    roll: u16,
) -> Result<(), embassy_stm32::usart::Error> {
    const fn split_number(x: u16) -> (u8, u8) {
        let low = (x & 0xff) as u8;
        let high = ((x >> 8) & 0xff) as u8;

        (low, high)
    }

    let (throttle_low, throttle_high) = split_number(throttle);
    let (yaw_low, yaw_high) = split_number(yaw);
    let (pitch_low, pitch_high) = split_number(pitch);
    let (roll_low, roll_high) = split_number(roll);

    transmit_msp_message(
        tx,
        MSPMessageType::RC,
        &[
            roll_low,
            roll_high,
            pitch_low,
            pitch_high,
            yaw_low,
            yaw_high,
            throttle_low,
            throttle_high,
        ],
    )
    .await
}

async fn clear_display(tx: &mut UartTx<'static, Async>) -> Result<(), embassy_stm32::usart::Error> {
    transmit_msp_message(
        tx,
        MSPMessageType::DISPLAYPORT,
        &[MSPDisplayportMessageType::CLEAR as u8],
    )
    .await
}

async fn draw_display(tx: &mut UartTx<'static, Async>) -> Result<(), embassy_stm32::usart::Error> {
    transmit_msp_message(
        tx,
        MSPMessageType::DISPLAYPORT,
        &[MSPDisplayportMessageType::DRAW as u8],
    )
    .await
}

async fn set_resolution(
    tx: &mut UartTx<'static, Async>,
    resolution: HDZeroResolution,
) -> Result<(), embassy_stm32::usart::Error> {
    transmit_msp_message(
        tx,
        MSPMessageType::DISPLAYPORT,
        &[MSPDisplayportMessageType::CONFIG as u8, 0, resolution as u8],
    )
    .await
}

async fn write_string(
    tx: &mut UartTx<'static, Async>,
    row: u8,
    col: u8,
    string: &[u8],
) -> Result<(), embassy_stm32::usart::Error> {
    const MAX_STRING_SIZE: usize = 50;
    const HEADER_LEN: usize = 4;
    const BUFFER_CAPACITY: usize = HEADER_LEN + MAX_STRING_SIZE;
    let string_len = min(MAX_STRING_SIZE, string.len());
    let used_buffer_len = HEADER_LEN + string_len;

    let header: [u8; HEADER_LEN] = [MSPDisplayportMessageType::WRITE as u8, row, col, 0];

    let mut buffer = [0u8; BUFFER_CAPACITY];

    buffer[..HEADER_LEN].clone_from_slice(&header);
    buffer[HEADER_LEN..used_buffer_len].clone_from_slice(&string[..string_len]);

    transmit_msp_message(tx, MSPMessageType::DISPLAYPORT, &buffer[..used_buffer_len]).await
}

pub async fn make_msp_uart_pair(
    uart_getter: impl UartMaker,
) -> (UartRx<'static, Async>, UartTx<'static, Async>) {
    let (rx, tx) = uart_getter.make_uart(115200, Parity::ParityNone, StopBits::STOP1, false);

    (rx, tx)
}

fn float_to_byte_string(value: f32, byte_string: &mut [u8]) {
    let value = (value * 100f32).round() as i32;
    let value = min(value, 9999);
    let mut value = max(0, value);

    byte_string[4] = b'0' + (value % 10) as u8;
    value /= 10;
    byte_string[3] = b'0' + (value % 10) as u8;
    value /= 10;
    byte_string[2] = b'.';
    byte_string[1] = b'0' + (value % 10) as u8;
    value /= 10;
    byte_string[0] = b'0' + (value % 10) as u8;
}

fn uint_to_byte_string(mut value: u8, byte_string: &mut [u8]) {
    byte_string[2] = b'0' + (value % 10);
    value /= 10;
    byte_string[1] = b'0' + (value % 10);
    value /= 10;
    byte_string[0] = b'0' + (value % 10);

    for character in &mut byte_string[0..2] {
        if *character == b'0' {
            *character = b' ';
        } else {
            break;
        }
    }
}

async fn draw_status_osd(
    tx: &mut UartTx<'static, Async>,
    bat_voltage: f32,
    link_quality: u8,
    rssi: u8,
) -> Result<(), embassy_stm32::usart::Error> {
    let mut bat_string = *b"\x9000.00\x06";
    let mut link_quality_string = *b"\x90000\x06";
    let mut rssi_string = *b"\x90000\x06";

    float_to_byte_string(bat_voltage, &mut bat_string[1..6]);
    uint_to_byte_string(link_quality, &mut link_quality_string[1..4]);
    uint_to_byte_string(rssi, &mut rssi_string[1..4]);

    clear_display(tx).await?;
    write_string(tx, 0, 0, &bat_string).await?;
    write_string(tx, 1, 0, &link_quality_string).await?;
    write_string(tx, 2, 0, &rssi_string).await?;
    write_string(tx, 3, 0, b"WORLD").await?;
    draw_display(tx).await
}

#[allow(dead_code)]
#[embassy_executor::task]
pub async fn osd_refresh_task(mut tx: UartTx<'static, Async>, store: &'static Store) {
    info!("OSD task start");
    let mut arming_context = ArmingContext::default();
    loop {
        let link_stats = store.get_link_state().await;
        let command_inputs = store.channel_snapshot().await;
        let battery_voltage = store.get_voltage().await;
        arming_context.update(&command_inputs);

        let (throttle, yaw, pitch, roll) = (1500u16, 1500u16, 1500u16, 1500u16);
        /*info!(
            "LS {} {} {}",
            battery_voltage,
            arming_context.is_armed(),
            link_stats.rssi1
        );*/

        let _ = set_resolution(&mut tx, HDZeroResolution::HD_5018).await;
        let _ = transmit_fc_variant(&mut tx).await;
        let _ = transmit_sticks(&mut tx, throttle, yaw, pitch, roll).await;
        let _ = transmit_status(&mut tx, arming_context.is_armed()).await;
        let _ = draw_status_osd(
            &mut tx,
            battery_voltage,
            link_stats.link_quality,
            max(link_stats.rssi1, link_stats.rssi2),
        )
        .await;
        Timer::after_millis(100).await;
    }
}
