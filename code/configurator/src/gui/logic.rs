use std::{
    sync::{Arc, Mutex, mpsc::Sender},
    time::Duration,
};

use common::shared_objects::{MotorDirection, MotorPosition, StoredConfig};
use eframe::CreationContext;
use egui::TextStyle;

use crate::{
    drone_connection::{
        DroneCommunicationError, DroneConnection, MockDroneConnection, SerialMspConnection,
        serial_connection::SerialPort,
    },
    gui::{
        AvailablePort, CachedConfig, ConnectionOwnership, ConnectionState, ExecuteOnGuiCallback,
        Gui, Message, MotorSettings, SerialPortPicker, SharedState,
    },
    time_wrapper::now,
};

impl Gui {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        Self::style(&cc.egui_ctx);
        let (sender, receiver) = std::sync::mpsc::channel();
        Gui {
            connection: Arc::new(Mutex::new(ConnectionState::Disconnected)),
            motor_settings: MotorSettings::default(),
            port_picker: SerialPortPicker::default(),
            state: SharedState::default(),
            sender,
            receiver,
            last_tick: now(),
            message_box_message: None,
            loading_screen: false,
        }
    }

    fn style(ctx: &egui::Context) {
        ctx.all_styles_mut(|style| {
            style.text_styles.get_mut(&TextStyle::Body).unwrap().size = 18f32;
            style.text_styles.get_mut(&TextStyle::Button).unwrap().size = 18f32;
            style.spacing.item_spacing.x = 10f32;
            style.spacing.item_spacing.y = 10f32;
        });
    }

    fn show_message(&mut self, message: &str) {
        self.message_box_message = Some(message.into());
    }

    pub(super) fn update(&mut self, message: Message, ctx: &egui::Context) {
        self.handle_message_and_errors(message, ctx);
    }

    fn handle_message_and_errors(&mut self, message: Message, ctx: &egui::Context) {
        let result = self.handle_message(message, ctx);

        if let Err(error) = result {
            self.disconnect(Some(&format!(
                "Communication failure. Disconnecting from drone {:?}",
                error
            )));
        }
    }

    fn handle_message(
        &mut self,
        message: Message,
        ctx: &egui::Context,
    ) -> Result<(), DroneCommunicationError> {
        match message {
            Message::RefreshPorts => {
                self.refresh_ports();
            }
            Message::DismissMessageBox => {
                self.message_box_message = None;
            }
            Message::ConnectToPort => {
                self.connect();
                self.refresh_status()?;
            }
            Message::Tick => self.tick(ctx)?,
            Message::EnterConfigurator => self.enter_configurator()?,
            Message::ResetFc => self.reset_fc()?,
            Message::ReadConfig => self.read_config()?,
            Message::MotorPositionChosen(motor, position) => {
                self.motor_position_chosen(motor, position)
            }
            Message::MotorDirectionChosen(motor, direction) => {
                self.motor_direction_chosen(motor, direction)?
            }
            Message::YawOffsetchanged(offset_string) => {
                self.motor_settings.update_yaw(offset_string);
                self.try_apply_ypr_strings();
            }
            Message::PitchOffsetChanged(offset_string) => {
                self.motor_settings.update_pitch(offset_string);
                self.try_apply_ypr_strings();
            }
            Message::RollOffsetChanged(offset_string) => {
                self.motor_settings.update_roll(offset_string);
                self.try_apply_ypr_strings();
            }
            Message::SaveConfig => self.save_config()?,
            Message::ExecuteOnGui(mesage) => {
                self.loading_screen = false;
                let callback = mesage.callback;
                callback(self)?;
            }
        }

        Ok(())
    }

    fn execute_on_gui(sender: &Sender<Message>, callback: ExecuteOnGuiCallback) {
        sender
            .send(Message::ExecuteOnGui(super::ExecuteOnGuiCallbackWrapper {
                callback,
            }))
            .unwrap();
    }

    fn refresh_ports(&mut self) {
        let picker = &mut self.port_picker;

        picker.available_ports.clear();

        let sender = self.sender.clone();
        self.loading_screen = true;

        spawn_async(async move {
            let enumeration_result = SerialPort::enumerate().await;
            Self::execute_on_gui(
                &sender,
                Box::new(|gui| {
                    match enumeration_result {
                        Ok(available_ports) => {
                            gui.loading_screen = false;
                            gui.port_picker.available_ports =
                                available_ports.into_iter().map(Into::into).collect();
                            gui.port_picker
                                .available_ports
                                .push(super::AvailablePort::Mock);
                            if !gui.port_picker.available_ports.is_empty() {
                                gui.port_picker.selected =
                                    Some(gui.port_picker.available_ports[0].clone());
                            } else {
                                gui.port_picker.selected = None;
                            }
                        }
                        Err(error) => {
                            gui.show_message(&format!("Failed to enumerate USB ports {:?}", error));
                        }
                    };
                    Ok(())
                }),
            );
        });
    }

    fn disconnect(&mut self, error_message: Option<&str>) {
        *self.connection.lock().unwrap() = ConnectionState::Disconnected;
        self.state = SharedState::default();
        if let Some(error_message) = error_message {
            self.show_message(error_message);
        }
    }

    fn tick(&mut self, ctx: &egui::Context) -> Result<(), DroneCommunicationError> {
        self.last_tick = now();
        if !self.loading_screen {
            self.refresh_status()?;
        }
        ctx.request_repaint_after(Duration::from_secs(1));

        Ok(())
    }

    fn connect(&mut self) {
        if let Some(port) = self.port_picker.selected.clone() {
            self.disconnect(None);
            let sender = self.sender.clone();
            spawn_async(async move {
                let mut drone_connection = port.connect().await;
                let test_success = if let Ok(connection) = drone_connection.as_mut() {
                    connection.test_connection().await
                } else {
                    false
                };

                Self::execute_on_gui(
                    &sender,
                    Box::new(move |gui| {
                        match drone_connection {
                            Ok(drone_connection) => {
                                if test_success {
                                    println!(
                                        "Connection successful, drone responding to MSP messages"
                                    );
                                    gui.connection.lock().unwrap().release(drone_connection);
                                } else {
                                    gui.show_message("USB serial port opened successfully but communication with FC could not be done");
                                }
                            }
                            Err(err) => {
                                gui.show_message(&format!(
                                    "Failed to communicate with drone {:?}",
                                    err
                                ));
                            }
                        };
                        Ok(())
                    }),
                );
            });
        }
    }

    fn take_connection(&mut self, f: impl FnOnce(ConnectionOwnership, Sender<Message>)) {
        let mut conenction = self.connection.lock().unwrap();
        let owned_connection = conenction.take();

        if let Some(owned_connection) = owned_connection {
            let ownership = ConnectionOwnership::new(self.connection.clone(), owned_connection);
            f(ownership, self.sender.clone());
        }
    }

    fn refresh_status(&mut self) -> Result<(), DroneCommunicationError> {
        self.take_connection(|mut connection, sender| {
            spawn_async(async move {
                let battery_status = connection.query_battery().await;
                let status = connection.query_status().await;

                Self::execute_on_gui(
                    &sender,
                    Box::new(move |gui| {
                        gui.state.battery_status = battery_status?;
                        let status = status?;
                        gui.state.connected = true;

                        if status.valid {
                            gui.state.last_fc_status_update = now();
                            gui.state.last_fc_status = status;
                            status
                        } else {
                            gui.state.last_fc_status
                        };
                        Ok(())
                    }),
                );
            });
        });

        Ok(())
    }

    fn enter_configurator(&mut self) -> Result<(), DroneCommunicationError> {
        self.take_connection(|mut connection, sender| {
            spawn_async(async move {
                let enter_configurator_result = connection.enter_configurator().await;
                Self::execute_on_gui(
                    &sender,
                    Box::new(move |gui| {
                        enter_configurator_result?;
                        gui.read_config()
                    }),
                );
            });
        });

        Ok(())
    }

    fn reset_fc(&mut self) -> Result<(), DroneCommunicationError> {
        self.take_connection(|mut connection, sender| {
            spawn_async(async move {
                let result = connection.reset_fc().await;
                if result.is_ok() {
                    connection.disconnect();
                }

                Self::execute_on_gui(
                    &sender,
                    Box::new(|gui| {
                        result?;
                        gui.disconnect(None);
                        Ok(())
                    }),
                );
            });
        });

        Ok(())
    }

    fn read_config(&mut self) -> Result<(), DroneCommunicationError> {
        self.take_connection(|mut connection, sender| {
            spawn_async(async move {
                let config = connection.read_config().await;
                let motor_count = connection.query_motor_count().await;

                Self::execute_on_gui(
                    &sender,
                    Box::new(move |gui| {
                        let config = config?;
                        gui.motor_settings.new_config(&config);
                        gui.state.config = config.into();
                        gui.state.motor_count = motor_count?;

                        Ok(())
                    }),
                );
            });
        });

        Ok(())
    }

    fn motor_position_chosen(&mut self, motor: u8, new_position: MotorPosition) {
        self.state.config.mutate_if_loaded(|config, _| {
            let old_motor_at_position = config.motor_positions[new_position];

            if old_motor_at_position == motor {
                return;
            }

            let current_position_of_this_motor =
                config.motor_positions.get_position_of(motor).unwrap();

            config.motor_positions[current_position_of_this_motor] = old_motor_at_position;
            config.motor_positions[new_position] = motor;
        });
    }

    fn motor_direction_chosen(
        &mut self,
        motor: u8,
        direction: MotorDirection,
    ) -> Result<(), DroneCommunicationError> {
        self.take_connection(|mut connection, sender| {
            spawn_async(async move {
                let drive_result = connection.drive_motor(motor, direction).await;

                Self::execute_on_gui(
                    &sender,
                    Box::new(move |gui| {
                        drive_result?;
                        gui.state.config.mutate_if_loaded(|config, _| {
                            config.set_direction_of(motor, direction);
                        });
                        Ok(())
                    }),
                );
            });
        });
        Ok(())
    }

    fn try_apply_ypr_strings(&mut self) {
        if let Ok((yaw, pitch, roll)) = self.motor_settings.try_parse_ypr_offsets() {
            self.state.config.mutate_if_loaded(|config, _| {
                config.yaw_offset = yaw.into();
                config.pitch_offset = pitch.into();
                config.roll_offset = roll.into();
            });
        }
    }

    fn save_config(&mut self) -> Result<(), DroneCommunicationError> {
        if let Some(config) = self.state.config.inner.as_ref() {
            let mut config_to_store = config.current_config.clone();

            let ypr_offsets = self.motor_settings.try_parse_ypr_offsets();

            if let Ok((yaw_offset, pitch_offset, roll_offset)) = ypr_offsets {
                config_to_store.yaw_offset = yaw_offset.into();
                config_to_store.pitch_offset = pitch_offset.into();
                config_to_store.roll_offset = roll_offset.into();
                self.take_connection(|mut connection, sender| {
                    spawn_async(async move {
                        let result = connection.save_config(config_to_store).await;
                        Self::execute_on_gui(
                            &sender,
                            Box::new(move |gui| {
                                result?;
                                gui.read_config()
                            }),
                        );
                    });
                });
            } else if let Err(error) = ypr_offsets {
                self.show_message(error);
            }
        }

        Ok(())
    }
}

impl MotorSettings {
    fn new_config(&mut self, config: &StoredConfig) {
        self.yaw_offset_string = format!("{}", config.yaw_offset);
        self.pitch_offset_string = format!("{}", config.pitch_offset);
        self.roll_offset_string = format!("{}", config.roll_offset);
    }

    fn update_yaw(&mut self, yaw: String) {
        self.yaw_offset_string = yaw;
    }

    fn update_pitch(&mut self, yaw: String) {
        self.pitch_offset_string = yaw;
    }

    fn update_roll(&mut self, yaw: String) {
        self.roll_offset_string = yaw;
    }

    fn try_parse_ypr_offsets(&self) -> Result<(f32, f32, f32), &'static str> {
        let yaw_offset = match self.yaw_offset_string.parse::<f32>() {
            Ok(yaw) => yaw,
            Err(_) => {
                return Err("Yaw offset is not a valid float");
            }
        };

        let pitch_offset = match self.pitch_offset_string.parse::<f32>() {
            Ok(yaw) => yaw,
            Err(_) => {
                return Err("Pitch offset is not a valid float");
            }
        };

        let roll_offset = match self.roll_offset_string.parse::<f32>() {
            Ok(yaw) => yaw,
            Err(_) => {
                return Err("Roll offset is not a valid float");
            }
        };

        Ok((yaw_offset, pitch_offset, roll_offset))
    }
}

impl AvailablePort {
    async fn connect(&self) -> Result<Box<dyn DroneConnection>, DroneCommunicationError> {
        match self {
            AvailablePort::Serial(port) => {
                Ok(Box::new(SerialMspConnection::new(port.connect().await?)))
            }
            AvailablePort::Mock => Ok(Box::new(MockDroneConnection::new())),
        }
    }
}

impl CachedConfig {
    pub(super) fn mutate_if_loaded<T: FnOnce(&mut StoredConfig, bool)>(&mut self, mutator: T) {
        if let Some(inner) = self.inner.as_mut() {
            mutator(&mut inner.current_config, inner.dirty);

            inner.current_config.validate_values().unwrap();

            inner.current_config.update_checksum();

            inner.dirty = (inner.current_config != inner.read_config)
                || (!inner.read_config.valid_checksum());
        }
    }

    pub(super) fn read_if_loaded<R, T: FnOnce(&StoredConfig, bool) -> R>(
        &self,
        reader: T,
    ) -> Option<R> {
        if let Some(inner) = self.inner.as_ref() {
            return Some(reader(&inner.current_config, inner.dirty));
        }

        None
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_async<F>(future: F) -> tokio::task::JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    tokio::spawn(future)
}

#[cfg(target_arch = "wasm32")]
pub fn spawn_async<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future)
}
