use core::cmp::{max, min};
use num_traits::float::FloatCore;

use crate::{logging::info, storage::Store};
use embassy_stm32::{
    interrupt,
    mode::Async,
    usart::{
        Instance, InterruptHandler, Parity, RxDma, RxPin, StopBits, TxDma, TxPin, Uart, UartRx,
        UartTx,
    },
    Peripheral,
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

pub async fn make_msp_uart_pair<T: Instance>(
    uart_peripheral: impl Peripheral<P = T> + 'static,
    rx_pin: impl RxPin<T> + 'static,
    tx_pin: impl TxPin<T> + 'static,
    tx_dma: impl TxDma<T> + 'static,
    rx_dma: impl RxDma<T> + 'static,
    interrupt_handlers: impl interrupt::typelevel::Binding<T::Interrupt, InterruptHandler<T>> + 'static,
) -> (UartRx<'static, Async>, UartTx<'static, Async>) {
    let mut uart_config = embassy_stm32::usart::Config::default();
    uart_config.baudrate = 115200;
    uart_config.parity = Parity::ParityNone;
    uart_config.stop_bits = StopBits::STOP1;

    let uart = Uart::new(
        uart_peripheral,
        rx_pin,
        tx_pin,
        interrupt_handlers,
        tx_dma,
        rx_dma,
        uart_config,
    )
    .unwrap();

    let (tx, rx) = uart.split();

    (rx, tx)
}

async fn draw_status_osd(
    tx: &mut UartTx<'static, Async>,
    bat_voltage: f32,
) -> Result<(), embassy_stm32::usart::Error> {
    let mut bat_string = *b"00.00";

    let bat_voltage = (bat_voltage * 100f32).round() as i32;
    let bat_voltage = min(bat_voltage, 9999);
    let mut bat_voltage = max(0, bat_voltage);

    bat_string[4] = 48u8 + (bat_voltage % 10) as u8;
    bat_voltage /= 10;
    bat_string[3] = 48u8 + (bat_voltage % 10) as u8;
    bat_voltage /= 10;
    bat_string[1] = 48u8 + (bat_voltage % 10) as u8;
    bat_voltage /= 10;
    bat_string[0] = 48u8 + (bat_voltage % 10) as u8;

    clear_display(tx).await?;
    write_string(tx, 0, 0, &bat_string).await?;
    write_string(tx, 1, 0, b"WORLD").await?;
    draw_display(tx).await
}

#[allow(dead_code)]
#[embassy_executor::task]
pub async fn osd_refresh_task(mut tx: UartTx<'static, Async>, store: &'static Store) {
    info!("OSD task start");
    loop {
        let (throttle, yaw, pitch, roll) = (1500u16, 1500u16, 1500u16, 1500u16);
        let armed = false;
        let battery_voltage = store.get_voltage().await;

        let _ = set_resolution(&mut tx, HDZeroResolution::HD_5018).await;
        let _ = transmit_fc_variant(&mut tx).await;
        let _ = transmit_sticks(&mut tx, throttle, yaw, pitch, roll).await;
        let _ = transmit_status(&mut tx, armed).await;
        let _ = draw_status_osd(&mut tx, battery_voltage).await;
        Timer::after_millis(100).await;
    }
}
