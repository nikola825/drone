use std::{sync::mpsc::Sender, time::Duration};

use common::shared_objects::{MotorDirection, MotorPosition};
use eframe::App;
use egui::{Color32, Id, Layout, Modal, RichText, Spinner};

use crate::{
    gui::{Gui, Message, MotorSettings, SerialPortPicker, SharedState},
    time_wrapper::now,
};

impl App for Gui {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Some(message) = &self.message_box_message.as_ref() {
            let modal = Modal::new(Id::new("MessageBox")).show(ui.ctx(), |ui| {
                ui.set_width(300f32);
                ui.set_height(100f32);
                ui.vertical(|ui| {
                    ui.horizontal_centered(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.label(*message);
                        });
                    });
                    ui.vertical_centered_justified(|ui| {
                        if ui.button("OK").clicked() {
                            self.sender.send(Message::DismissMessageBox).unwrap();
                        }
                    })
                });
            });
            if modal.should_close() {
                self.sender.send(Message::DismissMessageBox).unwrap();
            }
        } else if let Some(taken_time) = self.connection.lock().unwrap().taken_time() {
            let age = web_time::Instant::now() - taken_time;
            if age.as_millis() > 100 {
                Modal::new(Id::new("BusyModal")).show(ui.ctx(), |ui| {
                    ui.set_width(300f32);
                    ui.set_height(100f32);
                    ui.vertical(|ui| {
                        ui.horizontal_centered(|ui| {
                            ui.centered_and_justified(|ui| {
                                ui.label("Busy");
                            });
                        });
                        ui.vertical_centered_justified(|ui| {
                            ui.add(Spinner::new());
                        });
                    });
                });
            }
        }
        egui::Panel::top("top")
            .resizable(false)
            .show(ui, |ui| self.status_bar(ui));
        egui::Panel::left("left")
            .resizable(true)
            .show(ui, |ui| self.side_bar(ui));
        egui::CentralPanel::default_margins().show(ui, |ui| self.main_view(ui));
        ui.ctx().request_repaint_after_secs(1f32);
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if (now() - self.last_tick) > Duration::from_secs(1) {
            self.sender.send(Message::Tick).unwrap();
        }
        while let Ok(message) = self.receiver.try_recv() {
            self.update(message, ctx);
        }
    }
}
impl Gui {
    fn side_bar(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.add_space(10f32);
            if ui.button("Start CFG").clicked() {
                self.sender.send(Message::EnterConfigurator).unwrap();
            }
            if ui.button("Reset FC").clicked() {
                self.sender.send(Message::ResetFc).unwrap();
            }
        });
    }

    fn main_view(&mut self, ui: &mut egui::Ui) {
        self.motor_settings.view(ui, &self.state, &self.sender)
    }

    fn status_bar(&mut self, ui: &mut egui::Ui) {
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

            let status_age = now() - self.state.last_fc_status_update;
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

        ui.horizontal_top(|ui| {
            self.port_picker.view(ui, &self.sender);
            ui.separator();
            ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                ui.label(status);
            });
        });
    }
}

impl SerialPortPicker {
    fn view(&mut self, ui: &mut egui::Ui, sender: &Sender<Message>) {
        ui.horizontal_top(|ui| {
            if ui.button("Refresh").clicked() {
                sender.send(Message::RefreshPorts).unwrap();
            }

            let selected_text = if let Some(selected) = self.selected.as_ref() {
                &selected.to_string()
            } else {
                "Select port"
            };
            egui::ComboBox::from_id_salt("Select port")
                .selected_text(selected_text)
                .show_ui(ui, |ui| {
                    for port in self.available_ports.iter().as_ref() {
                        ui.selectable_value(
                            &mut self.selected,
                            Some(port.clone()),
                            port.to_string(),
                        );
                    }
                });
            if ui.button("Connect").clicked() {
                sender.send(Message::ConnectToPort).unwrap();
            }
        });
    }
}

impl MotorSettings {
    fn view(&self, ui: &mut egui::Ui, state: &SharedState, sender: &Sender<Message>) {
        match state.last_fc_status.fc_phase {
            common::configurator_protocol::messages::FcPhase::Config => {
                self.view_actual(ui, state, sender)
            }
            _ => {
                ui.horizontal_centered(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("Flight controller not in config mode");
                    })
                });
            }
        }
    }

    fn view_actual(&self, ui: &mut egui::Ui, state: &SharedState, sender: &Sender<Message>) {
        ui.vertical(|ui| {
            let dirty_text = state
                .config
                .read_if_loaded(|config, dirty| {
                    for motor_index in 0..state.motor_count {
                        let position = config.motor_positions.get_position_of(motor_index).unwrap();
                        let mut new_position = position;
                        let direction = config.get_direction_of(motor_index);
                        let mut new_direction = direction;

                        ui.horizontal(|ui| {
                            egui::ComboBox::from_id_salt(format!("MotorPosition{}", motor_index))
                                .selected_text(position.to_string())
                                .width(200f32)
                                .show_ui(ui, |ui| {
                                    for possible_position in MotorPosition::all_values() {
                                        ui.selectable_value(
                                            &mut new_position,
                                            possible_position,
                                            possible_position.to_string(),
                                        );
                                    }
                                });
                            if new_position != position {
                                sender
                                    .send(Message::MotorPositionChosen(motor_index, new_position))
                                    .unwrap();
                            }

                            ui.radio_value(&mut new_direction, MotorDirection::Forward, "Forward");
                            ui.radio_value(
                                &mut new_direction,
                                MotorDirection::Backward,
                                "Backward",
                            );

                            if new_direction != direction {
                                sender
                                    .send(Message::MotorDirectionChosen(motor_index, new_direction))
                                    .unwrap();
                            }
                        });
                    }
                    egui::Grid::new("YPRConfig").num_columns(2).show(ui, |ui| {
                        let mut yaw_offset_string = self.yaw_offset_string.clone();
                        ui.label("Yaw offset:");
                        ui.text_edit_singleline(&mut yaw_offset_string);

                        if yaw_offset_string != self.yaw_offset_string {
                            sender
                                .send(Message::YawOffsetchanged(yaw_offset_string))
                                .unwrap();
                        }

                        ui.end_row();

                        let mut pitch_offset_string = self.pitch_offset_string.clone();
                        ui.label("Pitch offset:");
                        ui.text_edit_singleline(&mut pitch_offset_string);

                        if pitch_offset_string != self.pitch_offset_string {
                            sender
                                .send(Message::PitchOffsetChanged(pitch_offset_string))
                                .unwrap();
                        }

                        ui.end_row();

                        let mut roll_offset_string = self.roll_offset_string.clone();
                        ui.label("Roll offset:");
                        ui.text_edit_singleline(&mut roll_offset_string);

                        if roll_offset_string != self.roll_offset_string {
                            sender
                                .send(Message::RollOffsetChanged(roll_offset_string))
                                .unwrap();
                        }

                        ui.end_row();
                    });
                    if dirty {
                        RichText::new("DIRTY").color(Color32::RED).strong()
                    } else {
                        RichText::new("STORED").color(Color32::GREEN).strong()
                    }
                })
                .unwrap_or(RichText::new(""));
            ui.horizontal(|ui| {
                if ui.button("Read config").clicked() {
                    sender.send(Message::ReadConfig).unwrap();
                }
                if ui.button("Store config").clicked() {
                    sender.send(Message::SaveConfig).unwrap();
                }
                ui.label(dirty_text);
            })
        });
    }
}
