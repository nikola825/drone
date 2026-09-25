use common::msp_protocol::{
    messages::{
        ApiVersionMessage, BatteryStateMessage, FcVariantMessage, MotorConfigMessage,
        SetPassthroughResponse,
    },
    protocol::{transmit_msp_message, MSPHeader, MSPMessageType, ReceivedMspMessage},
};
use embassy_executor::SendSpawner;
use embassy_futures::join::join;
use zerocopy::TryFromBytes;

use crate::{
    configurator::configurator_communication::handle_configurator_message,
    four_way::four_way_esc::four_way_loop,
    hal::{Disconnected, PacketHeaderType, UsbPeripheral, UsbSerialWrapper, ESC_COUNT},
    shared_state::SHARED_STATE,
};

const FC_VARIANT_BETAFLIGHT: &[u8; 4] = b"BTFL";

impl PacketHeaderType for MSPHeader {
    fn try_extract_header(buffer: &[u8]) -> Option<Self> {
        if buffer.len() >= size_of::<Self>() {
            if let Ok((header, _)) = Self::try_read_from_prefix(buffer) {
                if header.valid() {
                    return Some(header);
                }
            }
        }

        None
    }
}

pub async fn start_usb_communication(spawner: &SendSpawner, usb_peripheral: UsbPeripheral) {
    spawner.must_spawn(msp_message_processor_task(usb_peripheral));
}

#[embassy_executor::task]
pub async fn msp_message_processor_task(usb_peripheral: UsbPeripheral) {
    let mut usb_port = usb_peripheral.into_usb_port(true);

    let usb_serial = UsbSerialWrapper::new(usb_port.cdc_acm);

    let usb_fut = usb_port.usb_device.run();
    let msp_fut = msp_listener_loop(usb_serial);

    join(usb_fut, msp_fut).await;
}

async fn msp_listener_loop(mut usb_serial: UsbSerialWrapper) {
    loop {
        //leds.all_off();

        usb_serial.wait_for_connection().await;

        //leds.blue_on();

        let _ = msp_processor_loop(&mut usb_serial).await;
    }
}

async fn msp_processor_loop(usb_serial: &mut UsbSerialWrapper) -> Result<(), Disconnected> {
    loop {
        let header: MSPHeader = usb_serial.advance_until_header().await?;

        handle_msp_message::<ESC_COUNT>(usb_serial, header).await?;
    }
}

async fn handle_msp_message<const ESC_COUNT: usize>(
    usb: &mut UsbSerialWrapper,
    header: MSPHeader,
) -> Result<(), Disconnected> {
    let mut buffer: [u8; 280] = [0u8; 280];
    usb.read_exact(&mut buffer[0..size_of::<MSPHeader>()])
        .await?;

    let remainder_len = (header.len + 1) as usize; // payload + xor
    usb.read_exact(&mut buffer[size_of::<MSPHeader>()..size_of::<MSPHeader>() + remainder_len])
        .await?;

    let received_message = ReceivedMspMessage::try_from_bytes(&mut buffer, header);

    match received_message {
        Ok(result) => {
            match result.header.message_type {
                MSPMessageType::MSP_API_VERSION => {
                    transmit_msp_message(usb, ApiVersionMessage::default().into()).await?;
                }
                MSPMessageType::FC_VARIANT => {
                    transmit_msp_message(
                        usb,
                        FcVariantMessage::new(FC_VARIANT_BETAFLIGHT).into(), // pretend to be Betaflight for compatibility with AM32 configurator
                    )
                    .await?;
                }
                MSPMessageType::MSP_BATTERY_STATE => {
                    transmit_msp_message(usb, BatteryStateMessage::default().into()).await?;
                }
                MSPMessageType::MSP_MOTOR_CONFIG => {
                    transmit_msp_message(usb, MotorConfigMessage::new(ESC_COUNT as u8).into())
                        .await?;
                }
                MSPMessageType::MSP_CUSTOM_CONFIGURATOR => {
                    let response = handle_configurator_message(&result).await;
                    transmit_msp_message(usb, response).await?;
                }
                MSPMessageType::MSP_SET_PASSTHROUGH => {
                    transmit_msp_message(usb, SetPassthroughResponse::new(ESC_COUNT as u8).into())
                        .await?;

                    let four_way_parameters = SHARED_STATE.request_four_way_mode().await;

                    four_way_loop(usb, four_way_parameters).await;
                }
                _ => {}
            };
        }
        Err(_) => {
            // ignore
        }
    }

    Ok(())
}
