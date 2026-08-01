use std::time::{Duration, Instant};

use common::shared_objects::{MotorDirection, MotorPosition, StoredConfig};
use iced::Subscription;

use crate::{
    drone_connection::{
        DroneCommunicationError, DroneConnection, MockDroneConnection, UsbDroneConnection, UsbPort,
    },
    gui::{AvailablePort, CachedConfig, Gui, Message, MotorSettings, SharedState},
};

impl Gui {
    pub fn run_application() -> Result<(), Box<dyn std::error::Error>> {
        iced::application(Gui::default, Gui::update, Gui::view)
            .subscription(Gui::subscription)
            .title("DroneConfigurator")
            .run()?;

        Ok(())
    }

    fn show_message(&mut self, message: &str) {
        self.message_box_message = Some(message.into());
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) {
        self.handle_message_and_errors(message);
    }

    fn handle_message_and_errors(&mut self, message: Message) {
        let result = self.handle_message(message);

        if let Err(error) = result {
            self.disconnect(Some(&format!(
                "Communication failure. Disconnecting from drone {:?}",
                error
            )));
        }
    }

    fn handle_message(&mut self, message: Message) -> Result<(), DroneCommunicationError> {
        match message {
            Message::ChosePort(port) => {
                self.port_picker.chose_port(port);
            }
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
            Message::Tick => self.tick()?,
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
        }

        Ok(())
    }

    fn refresh_ports(&mut self) {
        let picker = &mut self.port_picker;

        picker.available_ports.clear();

        match UsbPort::enumerate() {
            Ok(available_ports) => {
                picker.available_ports = available_ports.into_iter().map(Into::into).collect();
                picker.available_ports.push(super::AvailablePort::Mock);
                if !picker.available_ports.is_empty() {
                    picker.selected = Some(picker.available_ports[0].clone());
                } else {
                    picker.selected = None;
                }
            }
            Err(error) => {
                self.show_message(&format!("Failed to enumerate USB ports {:?}", error));
            }
        }
    }

    fn disconnect(&mut self, error_message: Option<&str>) {
        self.connection = None;
        self.state = SharedState::default();
        if let Some(error_message) = error_message {
            self.show_message(error_message);
        }
    }

    fn tick(&mut self) -> Result<(), DroneCommunicationError> {
        self.refresh_status()?;

        Ok(())
    }

    fn connect(&mut self) {
        if let Some(port) = self.port_picker.selected.clone() {
            self.disconnect(None);
            let drone_connection = port.connect();

            match drone_connection {
                Ok(mut drone_connection) => {
                    if drone_connection.test_connection() {
                        println!("Connection successful, drone responding to MSP messages");
                        self.connection = Some(drone_connection);
                    } else {
                        self.show_message(
                            "USB serial port opened successfully but communication with FC could not be done",
                        );
                    }
                }
                Err(err) => {
                    self.show_message(&format!("Failed to communicate with drone {:?}", err));
                }
            }
        }
    }

    fn refresh_status(&mut self) -> Result<(), DroneCommunicationError> {
        if let Some(connection) = self.connection.as_mut() {
            self.state.battery_status = connection.query_battery()?;
            let status = connection.query_status()?;

            self.state.connected = true;

            if status.valid {
                self.state.last_fc_status_update = Instant::now();
                self.state.last_fc_status = status;
                status
            } else {
                self.state.last_fc_status
            };
        } else {
            self.state.connected = false;
            self.state.last_fc_status.valid = false;
        }

        Ok(())
    }

    fn enter_configurator(&mut self) -> Result<(), DroneCommunicationError> {
        if let Some(connection) = self.connection.as_mut() {
            connection.enter_configurator()?;
            self.read_config()?;
        }

        Ok(())
    }

    fn reset_fc(&mut self) -> Result<(), DroneCommunicationError> {
        if let Some(connection) = self.connection.as_mut() {
            connection.reset_fc()?;
        }

        self.disconnect(None);

        Ok(())
    }

    fn read_config(&mut self) -> Result<(), DroneCommunicationError> {
        let connection = match self.connection.as_mut() {
            Some(connection) => connection,
            None => return Ok(()),
        };

        let config = connection.read_config()?;
        let motor_count = connection.query_motor_count()?;

        self.motor_settings.new_config(&config);
        self.state.config = config.into();
        self.state.motor_count = motor_count;

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
        if let Some(connection) = self.connection.as_mut() {
            connection.drive_motor(motor, direction)?;
            self.state.config.mutate_if_loaded(|config, _| {
                config.set_direction_of(motor, direction);
            });
        }
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
        if let Some(connection) = self.connection.as_mut()
            && let Some(config) = self.state.config.inner.as_ref()
        {
            let mut config_to_store = config.current_config.clone();

            let ypr_offsets = self.motor_settings.try_parse_ypr_offsets();

            if let Ok((yaw_offset, pitch_offset, roll_offset)) = ypr_offsets {
                config_to_store.yaw_offset = yaw_offset.into();
                config_to_store.pitch_offset = pitch_offset.into();
                config_to_store.roll_offset = roll_offset.into();
                connection.save_config(config_to_store)?;
            } else if let Err(error) = ypr_offsets {
                self.show_message(error);
            }
        }

        self.read_config()?;

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
    fn connect(&self) -> Result<Box<dyn DroneConnection>, DroneCommunicationError> {
        match self {
            AvailablePort::Usb(usb_port) => Ok(Box::new(UsbDroneConnection::new(usb_port)?)),
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
