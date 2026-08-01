use std::time::Instant;

use common::shared_objects::{MotorDirection, MotorPosition};
use iced::{
    Alignment::Center,
    Background, Color, Element, Font,
    Length::{self, Fill, Shrink},
    color,
    widget::{Column, Row, button, container, pick_list, radio, rule, text, text_input},
};

use crate::gui::{AvailablePort, Gui, Message, MotorSettings, SerialPortPicker, SharedState};

use iced::widget::{column, row};

impl Gui {
    pub(super) fn view(&self) -> Element<'_, Message> {
        if let Some(message) = self.message_box_message.as_ref() {
            MessageBox::show_message(message, true)
        } else {
            column![
                self.status_bar().height(Shrink).padding(5),
                rule::horizontal(1),
                row![
                    self.side_bar().padding(5),
                    rule::vertical(1),
                    self.main_view().padding(5).width(Fill)
                ]
                .height(Fill)
                .width(Fill)
            ]
            .into()
        }
    }

    fn side_bar(&self) -> Column<'_, Message> {
        column![
            button("Start CFG")
                .on_press(Message::EnterConfigurator)
                .width(Fill),
            button("Reset FC").on_press(Message::ResetFc).width(Fill),
        ]
        .width(150)
        .spacing(10)
    }

    fn main_view(&self) -> Column<'_, Message> {
        self.motor_settings.view(&self.state)
    }

    fn status_bar(&self) -> Row<'_, Message> {
        let status: String = if !self.state.connected {
            "Disconnected".into()
        } else if !self.state.last_fc_status.valid {
            "No status update recived".into()
        } else {
            let voltage: f32 = self.state.battery_status.voltage.into();
            let cell_voltage = voltage / (self.state.battery_status.cell_count as f32);

            let phase_string = match self.state.last_fc_status.fc_phase {
                common::configurator_protocol::messages::FcPhase::IMUInitFail => "IMU",
                common::configurator_protocol::messages::FcPhase::Disarmed => "DIS",
                common::configurator_protocol::messages::FcPhase::Armed => "ARM",
                common::configurator_protocol::messages::FcPhase::Config => "CFG",
            };

            let status_age = Instant::now() - self.state.last_fc_status_update;
            let status_age = status_age.as_secs();

            format!(
                "{:2}s    {}    Bat {}S {:.1}V {:.2}V/cell    Dshot time: {:.1}us PID time: {:.1}us Min delta {}us Max delta {}us",
                status_age,
                phase_string,
                self.state.battery_status.cell_count,
                voltage,
                cell_voltage,
                self.state.last_fc_status.inner_duration,
                self.state.last_fc_status.total_duration,
                self.state.last_fc_status.min_measured_period,
                self.state.last_fc_status.max_measured_period
            )
        };

        row![
            self.port_picker.view().width(Shrink),
            text(status).width(Fill).center()
        ]
        .align_y(Center)
        .spacing(10)
    }
}

struct MessageBox {}

impl<'a> MessageBox {
    fn show_message(message_text: &'a str, show_dismiss_button: bool) -> Element<'a, Message> {
        let mut inner_content = column![text(message_text).center().width(Fill),].align_x(Center);

        if show_dismiss_button {
            inner_content =
                inner_content.push(button("Dismiss").on_press(Message::DismissMessageBox));
        }

        container(
            container(inner_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(Background::Color(Color::BLACK)),
            ..Default::default()
        })
        .into()
    }
}

impl SerialPortPicker {
    fn view(&self) -> Row<'_, Message> {
        row![
            button("Refresh").on_press(Message::RefreshPorts),
            pick_list(
                self.available_ports.clone(),
                self.selected.clone(),
                Message::ChosePort
            )
            .placeholder("Chose FC serial port"),
            button("Connect").on_press(Message::ConnectToPort),
        ]
        .spacing(10)
    }

    pub(super) fn chose_port(&mut self, port: AvailablePort) {
        self.selected = Some(port);
    }
}

impl MotorSettings {
    fn view(&self, state: &SharedState) -> Column<'_, Message> {
        match state.last_fc_status.fc_phase {
            common::configurator_protocol::messages::FcPhase::Config => self.view_actual(state),
            _ => column![MessageBox::show_message(
                "Flight controller not in config mode",
                false
            )],
        }
    }

    fn view_actual(&self, state: &SharedState) -> Column<'_, Message> {
        let (mut column, dirty_text) = state
            .config
            .read_if_loaded(|config, dirty| {
                let mut column = column![].spacing(10).width(Fill).height(Fill);

                for motor_index in 0..state.motor_count {
                    let position = config.motor_positions.get_position_of(motor_index).unwrap();
                    let direction = config.get_direction_of(motor_index);

                    column = column.push(
                        row![
                            pick_list(
                                MotorPosition::all_values(),
                                Some(position),
                                move |position: MotorPosition| {
                                    Message::MotorPositionChosen(motor_index, position)
                                }
                            )
                            .width(Fill),
                            radio(
                                "Forward",
                                MotorDirection::Forward,
                                Some(direction),
                                |direction| {
                                    Message::MotorDirectionChosen(motor_index, direction)
                                }
                            ),
                            radio(
                                "Backward",
                                MotorDirection::Backward,
                                Some(direction),
                                |direction| {
                                    Message::MotorDirectionChosen(motor_index, direction)
                                }
                            )
                        ]
                        .spacing(10)
                        .align_y(Center)
                        .width(Fill),
                    );
                }

                column = column.extend([
                    row![
                        text("Yaw offset:").width(150),
                        text_input("Yaw offset", &self.yaw_offset_string)
                            .on_input(Message::YawOffsetchanged)
                    ]
                    .spacing(10)
                    .align_y(Center)
                    .into(),
                    row![
                        text("Pitch offset:").width(150),
                        text_input("Pitch offset", &self.pitch_offset_string)
                            .on_input(Message::PitchOffsetChanged)
                    ]
                    .spacing(10)
                    .align_y(Center)
                    .into(),
                    row![
                        text("Roll offset:").width(150),
                        text_input("Roll offset", &self.roll_offset_string)
                            .on_input(Message::RollOffsetChanged)
                    ]
                    .spacing(10)
                    .align_y(Center)
                    .into(),
                ]);

                let mut dirty_font = Font::MONOSPACE;
                dirty_font.weight = iced::font::Weight::Bold;

                let dirty_text = if dirty {
                    text("DIRTY").color(color!(0xff0000))
                } else {
                    text("STORED").color(color!(0x00ff00))
                };

                let dirty_text = dirty_text.size(16).font(dirty_font);

                (column, dirty_text)
            })
            .unwrap_or((column![], text("")));

        column = column.push(
            row![
                button("Read config").on_press(Message::ReadConfig),
                button("Store config").on_press(Message::SaveConfig),
                dirty_text.width(Fill),
            ]
            .spacing(10)
            .align_y(Center),
        );

        column
    }
}
