use common::{
    configurator_protocol::{
        messages::{BatteryResponse, MotorOverride},
        protocol::{ConfiguratorMessage, Dummy},
    },
    msp_protocol::protocol::{MSPMessage, ReceivedMspMessage},
};
use cortex_m::peripheral::SCB;
use zerocopy::TryFromBytes;

use crate::msp::usb_communication::CommunicationContext;

pub async fn handle_configurator_message(
    communication_context: &mut CommunicationContext,
    received_msp_message: &ReceivedMspMessage,
) -> MSPMessage<ConfiguratorMessage> {
    let parsed_message = ConfiguratorMessage::try_read_from_prefix(&received_msp_message.payload);

    let configurator_message = if let Ok((message, _)) = parsed_message {
        message
    } else {
        return ConfiguratorMessage::FailedDeserializationResponse(Dummy::default()).into();
    };

    let response = match configurator_message {
        ConfiguratorMessage::StartConfigurator(_) => {
            start_configurator(communication_context).await
        }
        ConfiguratorMessage::QueryConfig(_) => query_config(communication_context).await,
        ConfiguratorMessage::QueryBattery(_) => query_battery(communication_context).await,
        ConfiguratorMessage::QueryFcStatus(_) => query_fc_status(communication_context).await,
        ConfiguratorMessage::ResetFc(_) => {
            reset_fc();
            unreachable!("Reset happens before this");
        }
        ConfiguratorMessage::OverrideMotor(motor_override) => {
            override_motor(communication_context, motor_override.inner)
        }
        _ => ConfiguratorMessage::InvalidCommandResponse(Dummy::default()).into(),
    };

    response
}

async fn start_configurator(
    communication_context: &CommunicationContext,
) -> MSPMessage<ConfiguratorMessage> {
    communication_context
        .shared_state
        .request_configurator_mode();
    ConfiguratorMessage::EmptyOkResponse(Dummy::default()).into()
}

async fn query_config(
    communication_context: &CommunicationContext,
) -> MSPMessage<ConfiguratorMessage> {
    ConfiguratorMessage::QueryConfigResponse(communication_context.shared_state.read_config().await)
        .into()
}

async fn query_battery(
    communication_context: &CommunicationContext,
) -> MSPMessage<ConfiguratorMessage> {
    let battery_info = communication_context
        .shared_state
        .get_battery_information()
        .await;

    ConfiguratorMessage::QueryBatteryResponse(
        BatteryResponse {
            cell_count: battery_info.get_cell_count(),
            voltage: battery_info.get_total_voltage().into(),
        }
        .into(),
    )
    .into()
}

async fn query_fc_status(
    communication_context: &CommunicationContext,
) -> MSPMessage<ConfiguratorMessage> {
    let status = communication_context.shared_state.query_status().await;

    ConfiguratorMessage::QueryFcStatusResponse(status.into()).into()
}

fn reset_fc() {
    SCB::sys_reset();
}

fn override_motor(
    communication_context: &CommunicationContext,
    motor_override: MotorOverride,
) -> MSPMessage<ConfiguratorMessage> {
    communication_context
        .shared_state
        .push_motor_override(motor_override);

    ConfiguratorMessage::EmptyOkResponse(Dummy::default()).into()
}
