use std::{sync::mpsc::Sender, time::Duration};

use common::shared_objects::{MotorDirection, MotorPosition};
use eframe::App;
use egui::{
    Color32, Id, Modal, RichText, Spinner,
    epaint::text::{FontInsert, InsertFontFamily},
};

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
            .show(ui, |ui| self.top_bar(ui));

        let mut bottom_expanded = self.bottom_bar_expanded;
        egui::Panel::bottom("bottom")
            .resizable(false)
            .show_collapsible(ui, &mut bottom_expanded, |ui| self.bottom_status_bar(ui));
        self.bottom_bar_expanded = bottom_expanded;

        let mut left_expanded = self.left_bar_expanded;
        egui::Panel::left("left")
            .resizable(true)
            .show_collapsible(ui, &mut left_expanded, |ui| self.side_bar(ui));
        self.left_bar_expanded = left_expanded;

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
        self.motor_settings.view(ui, &self.state, &self.sender);
    }

    fn bottom_status_bar(&mut self, ui: &mut egui::Ui) {
        if !self.state.connected {
            ui.horizontal_centered(|ui| {
                ui.style_mut().spacing.item_spacing.x = 5f32;
                ui.style_mut().spacing.item_spacing.y = 0f32;
                ui.label("\n");
                ui.separator();
                ui.label(RichText::new("\u{e560}").color(Color32::RED))
                    .on_hover_text("Disconnected");
                ui.separator();
            });
        } else if !self.state.last_fc_status.valid {
            ui.horizontal_centered(|ui| {
                ui.style_mut().spacing.item_spacing.x = 5f32;
                ui.style_mut().spacing.item_spacing.y = 0f32;
                ui.label("\n");
                ui.separator();
                ui.label(RichText::new("\u{e55d}").color(Color32::YELLOW))
                    .on_hover_text("Connected but no drone status");
                ui.separator();
            });
        } else {
            ui.style_mut().spacing.item_spacing.x = 5f32;
            ui.style_mut().spacing.item_spacing.y = 0f32;

            let status_age = now() - self.state.last_fc_status_update;
            let status_age = status_age.as_secs();

            let voltage: f32 = self.state.battery_status.voltage.into();
            let cell_voltage = voltage / (self.state.battery_status.cell_count as f32);

            let (status_emoji, status_color, status_tooltip) =
                match self.state.last_fc_status.fc_phase {
                    common::configurator_protocol::messages::FcPhase::Disarmed => {
                        ("\u{f637}", Color32::RED, "Connected, disarmed")
                    }
                    common::configurator_protocol::messages::FcPhase::Armed => {
                        ("\u{f637}", Color32::GREEN, "Connected, armed")
                    }
                    common::configurator_protocol::messages::FcPhase::Config => {
                        ("\u{f7d9}", Color32::BLUE, "Connected, in config mode")
                    }
                    common::configurator_protocol::messages::FcPhase::IMUInitFail => {
                        ("\u{f06a}", Color32::RED, "Connected, IMU init fail")
                    }
                };

            let status_label =
                RichText::new(format!("{}\n{}s", status_emoji, status_age)).color(status_color);

            ui.horizontal(|ui| {
                ui.style_mut().spacing.item_spacing.x = 5f32;
                ui.style_mut().spacing.item_spacing.y = 0f32;
                ui.label("\n");
                ui.separator();
                ui.add(egui::Label::new(status_label).halign(egui::Align::Center))
                    .on_hover_text(status_tooltip);
                ui.separator();
                ui.add(
                    egui::Label::new(format!(
                        "\u{f242}\n{}S",
                        self.state.battery_status.cell_count
                    ))
                    .halign(egui::Align::Center),
                )
                .on_hover_text("Battery cell count");
                ui.separator();
                ui.add(
                    egui::Label::new(format!("\u{f242}V\n{:.1}", voltage))
                        .halign(egui::Align::Center),
                )
                .on_hover_text("Battery voltage");
                ui.separator();
                ui.add(
                    egui::Label::new(format!("\u{f242}V/C\n{:.2}", cell_voltage))
                        .halign(egui::Align::Center),
                )
                .on_hover_text("Cell voltage");
                ui.separator();
                ui.add(
                    egui::Label::new(format!(
                        "\u{f863}\n{:.1}",
                        self.state.last_fc_status.inner_duration
                    ))
                    .halign(egui::Align::Center),
                )
                .on_hover_text("Dshot frame transmit time (us)");
                ui.separator();
                ui.add(
                    egui::Label::new(format!(
                        "\u{f2f2}\n{:.1}",
                        self.state.last_fc_status.total_duration
                    ))
                    .halign(egui::Align::Center),
                )
                .on_hover_text("Total PID execution time (us)");
                ui.separator();
                ui.add(
                    egui::Label::new(format!(
                        "\u{f363}\u{f063}\n{:.1}-{:.1}",
                        self.state.last_fc_status.min_measured_period,
                        self.state.last_fc_status.max_measured_period,
                    ))
                    .halign(egui::Align::Center),
                )
                .on_hover_text("Loop period(us)");
                ui.separator();
            });
        };
    }

    pub(super) fn use_fontawesome(ctx: &egui::Context) {
        ctx.add_font(FontInsert::new(
            "my_font",
            egui::FontData::from_static(include_bytes!("../../assets/fa.otf")),
            vec![
                InsertFontFamily {
                    family: egui::FontFamily::Proportional,
                    priority: egui::epaint::text::FontPriority::Lowest,
                },
                InsertFontFamily {
                    family: egui::FontFamily::Monospace,
                    priority: egui::epaint::text::FontPriority::Lowest,
                },
            ],
        ));
    }

    fn top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_top(|ui| {
            self.port_picker.view(ui, &self.sender);
            ui.separator();
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
        ui.vertical_centered_justified(|ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                let dirty_text = state
                    .config
                    .read_if_loaded(|config, dirty| {
                        for motor_index in 0..state.motor_count {
                            let position =
                                config.motor_positions.get_position_of(motor_index).unwrap();
                            let mut new_position = position;
                            let direction = config.get_direction_of(motor_index);
                            let mut new_direction = direction;
                            let available_width = ui.available_width();
                            ui.horizontal(|ui| {
                                ui.set_width(available_width);
                                egui::ComboBox::from_id_salt(format!(
                                    "MotorPosition{}",
                                    motor_index
                                ))
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
                                        .send(Message::MotorPositionChosen(
                                            motor_index,
                                            new_position,
                                        ))
                                        .unwrap();
                                }

                                ui.radio_value(
                                    &mut new_direction,
                                    MotorDirection::Forward,
                                    "Forward",
                                );
                                ui.radio_value(
                                    &mut new_direction,
                                    MotorDirection::Backward,
                                    "Backward",
                                );

                                if new_direction != direction {
                                    sender
                                        .send(Message::MotorDirectionChosen(
                                            motor_index,
                                            new_direction,
                                        ))
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
        });
    }
}
