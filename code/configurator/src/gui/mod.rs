use std::time::Instant;

use common::configurator_protocol::messages::FcStatus;
use fltk::{app::{App, Receiver, Sender}, button::Button, frame::Frame, group::Flex, menu::Choice};

use crate::drone_connection::{DroneConnection, UsbPort};

pub mod logic;
pub mod visual;

#[derive(Clone, Copy)]
enum Message {
    RefreshPorts,
    ConnectToPort,
    Tick,
    EnterConfigurator,
    ResetFc,
    Forward(u8),
    Backward(u8),
}

struct MotorRow {
    dropdown: Choice,
    forward: Button,
    backward: Button,
}

pub struct Gui {
    //motor_rows: Vec<MotorRow>,
    app: App,
    status_bar: StatusBar,
    side_bar: SideBar,
    main_box: MainBox,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
    connection: Option<DroneConnection>,
    last_fc_status: FcStatus,
    last_fc_status_update: Instant,
}

pub struct SideBar {
    bar: Flex,
}

pub struct StatusBar {
    status_label: Frame,
    serial_port_picker: SerialPortPicker,
    bar: Flex,
}

struct SerialPortPicker {
    serial_port_dropdown: Choice,
    available_ports: Vec<UsbPort>,
}

pub struct MainBox {
    motor_settings: MotorSettings,
}

pub struct MotorSettings {
    motor_settings: Flex,
    rows: Vec<MotorRow>,
}
