use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

use crate::{
    configurator_protocol::messages::{BatteryResponse, FcStatus, MotorOverride},
    msp_protocol::protocol::{MSPMessagePayload, MSPReceivedPayload},
    shared_objects::StoredConfig,
};

#[derive(IntoBytes, TryFromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(u8)]
pub enum ConfiguratorMessage {
    EmptyOkResponse(Dummy),
    FailedDeserializationResponse(Dummy),
    InvalidCommandResponse(Dummy),
    StartConfigurator(Dummy),
    QueryConfig(Dummy),
    SetConfig(Dummy),
    QueryConfigResponse(StoredConfig),
    QueryBattery(Dummy),
    QueryBatteryResponse(PaddedStruct<BatteryResponse, 59>),
    QueryFcStatus(Dummy),
    QueryFcStatusResponse(PaddedStruct<FcStatus, 38>),
    ResetFc(Dummy),
    OverrideMotor(PaddedStruct<MotorOverride, 62>)
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned, FromBytes)]
#[repr(C)]
pub struct Dummy {
    pub _unused: [u8; 64],
}

impl Default for Dummy {
    fn default() -> Self {
        Self { _unused: [0u8; 64] }
    }
}

pub trait Paddable: IntoBytes + Immutable + KnownLayout + Unaligned + Sized + TryFromBytes {}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned, TryFromBytes)]
#[repr(C)]
pub struct PaddedStruct<Inner: Paddable, const PADDING: usize> {
    pub inner: Inner,
    _padding: [u8; PADDING],
}

impl<Inner: Paddable, const PADDING: usize> From<Inner> for PaddedStruct<Inner, PADDING> {
    fn from(inner: Inner) -> Self {
        Self {
            inner,
            _padding: [0u8; PADDING],
        }
    }
}

impl MSPMessagePayload for ConfiguratorMessage {
    fn message_type() -> crate::msp_protocol::protocol::MSPMessageType {
        crate::msp_protocol::protocol::MSPMessageType::MSP_CUSTOM_CONFIGURATOR
    }
}

impl MSPReceivedPayload for ConfiguratorMessage {}
