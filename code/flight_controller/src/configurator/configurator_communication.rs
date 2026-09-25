use common::{
    configurator_protocol::{
        messages::{BatteryResponse, MotorDirectionSetting},
        protocol::{ConfiguratorMessage, Dummy},
    },
    msp_protocol::protocol::{MSPMessage, ReceivedMspMessage},
    shared_objects::StoredConfig,
};
use zerocopy::TryFromBytes;

use crate::{hal::mcu_utils::reset_fc, shared_state::SHARED_STATE};

pub async fn handle_configurator_message(
    received_msp_message: &ReceivedMspMessage,
) -> MSPMessage<ConfiguratorMessage> {
    let parsed_message = ConfiguratorMessage::try_read_from_prefix(&received_msp_message.payload);

    let configurator_message = if let Ok((message, _)) = parsed_message {
        message
    } else {
        return ConfiguratorMessage::FailedDeserializationResponse(Dummy::default()).into();
    };

    let response = match configurator_message {
        ConfiguratorMessage::StartConfigurator(_) => start_configurator().await,
        ConfiguratorMessage::QueryConfig(_) => query_config().await,
        ConfiguratorMessage::SetConfig(config) => set_config(config).await,
        ConfiguratorMessage::QueryBattery(_) => query_battery(),
        ConfiguratorMessage::QueryFcStatus(_) => query_fc_status(),
        ConfiguratorMessage::ResetFc(_) => {
            reset_fc();
        }
        ConfiguratorMessage::SetMotorDirection(direction_setting) => {
            set_motor_direction(direction_setting.inner)
        }
        ConfiguratorMessage::QueryLog(_) => query_log().await,
        _ => ConfiguratorMessage::InvalidCommandResponse(Dummy::default()).into(),
    };

    response
}

async fn start_configurator() -> MSPMessage<ConfiguratorMessage> {
    SHARED_STATE.request_configurator_mode();
    ConfiguratorMessage::EmptyOkResponse(Dummy::default()).into()
}

async fn query_config() -> MSPMessage<ConfiguratorMessage> {
    ConfiguratorMessage::QueryConfigResponse(SHARED_STATE.read_config().await).into()
}

fn query_battery() -> MSPMessage<ConfiguratorMessage> {
    let battery_info = SHARED_STATE.get_battery_information();

    ConfiguratorMessage::QueryBatteryResponse(
        BatteryResponse {
            cell_count: battery_info.get_cell_count(),
            voltage: battery_info.get_total_voltage().into(),
        }
        .into(),
    )
    .into()
}

fn query_fc_status() -> MSPMessage<ConfiguratorMessage> {
    let status = SHARED_STATE.query_status();

    ConfiguratorMessage::QueryFcStatusResponse(status.into()).into()
}

fn set_motor_direction(
    direction_setting: MotorDirectionSetting,
) -> MSPMessage<ConfiguratorMessage> {
    SHARED_STATE.push_motor_direction_setting(direction_setting);

    ConfiguratorMessage::EmptyOkResponse(Dummy::default()).into()
}

async fn set_config(config: StoredConfig) -> MSPMessage<ConfiguratorMessage> {
    SHARED_STATE.store_config(config).await;

    ConfiguratorMessage::EmptyOkResponse(Dummy::default()).into()
}

#[cfg(feature = "debug-logging")]
async fn query_log() -> MSPMessage<ConfiguratorMessage> {
    use common::shared_objects::DebugLog;

    let log = SHARED_STATE.get_log().await;

    let log = log.unwrap_or(DebugLog::new());
    ConfiguratorMessage::LogResponse(log).into()
}

#[cfg(not(feature = "debug-logging"))]
async fn query_log() -> MSPMessage<ConfiguratorMessage> {
    ConfiguratorMessage::LoggingDisabledResponse(Dummy::default()).into()
}
