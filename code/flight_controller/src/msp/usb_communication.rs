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

use crate::four_way::four_way_esc::FourWayParameters;
use crate::{
    configurator::configurator_communication::handle_configurator_message,
    four_way::four_way_esc::four_way_loop,
    hal::{Disconnected, PacketHeaderType, UsbPeripheral, UsbSerialWrapper, ESC_COUNT},
    shared_state::SharedState,
};

const FC_VARIANT_BETAFLIGHT: &[u8; 4] = b"BTFL";

pub struct CommunicationContext {
    four_way_parameters: Option<FourWayParameters>,
    pub shared_state: &'static SharedState,
}

impl CommunicationContext {
    pub async fn get_four_way_parameters(&mut self) -> &mut FourWayParameters {
        if self.four_way_parameters.is_none() {
            self.four_way_parameters = Some(self.shared_state.request_four_way_mode().await);
        }

        self.four_way_parameters.as_mut().unwrap()
    }
}

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

pub async fn start_usb_communication(
    spawner: &SendSpawner,
    usb_peripheral: UsbPeripheral,
    shared_state: &'static SharedState,
) {
    spawner.must_spawn(msp_message_processor_task(usb_peripheral, shared_state));
}

#[embassy_executor::task]
pub async fn msp_message_processor_task(
    usb_peripheral: UsbPeripheral,
    shared_state: &'static SharedState,
) {
    let mut usb_port = usb_peripheral.into_usb_port(true);

    let usb_serial = UsbSerialWrapper::new(usb_port.cdc_acm);

    let usb_fut = usb_port.usb_device.run();
    let msp_fut = msp_listener_loop(usb_serial, shared_state);

    join(usb_fut, msp_fut).await;
}

async fn msp_listener_loop(mut usb_serial: UsbSerialWrapper, shared_state: &'static SharedState) {
    let mut communication_context = CommunicationContext {
        four_way_parameters: None,
        shared_state,
    };

    loop {
        //leds.all_off();

        usb_serial.wait_for_connection().await;

        //leds.blue_on();

        let _ = msp_processor_loop(&mut usb_serial, &mut communication_context).await;
    }
}

async fn msp_processor_loop(
    usb_serial: &mut UsbSerialWrapper,
    communication_context: &mut CommunicationContext,
) -> Result<(), Disconnected> {
    loop {
        let header: MSPHeader = usb_serial.advance_until_header().await?;

        handle_msp_message::<ESC_COUNT>(usb_serial, header, communication_context).await?;
    }
}

async fn handle_msp_message<const ESC_COUNT: usize>(
    usb: &mut UsbSerialWrapper,
    header: MSPHeader,
    communication_context: &mut CommunicationContext,
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
                    let response =
                        handle_configurator_message(communication_context, &result).await;
                    transmit_msp_message(usb, response).await?;
                }
                MSPMessageType::MSP_SET_PASSTHROUGH => {
                    transmit_msp_message(usb, SetPassthroughResponse::new(ESC_COUNT as u8).into())
                        .await?;

                    let params = communication_context.get_four_way_parameters().await;

                    four_way_loop(usb, params).await?;
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
