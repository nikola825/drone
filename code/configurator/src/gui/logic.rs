use std::{thread, time::Duration};

use fltk::{button::Button, dialog, enums::Event, group::Flex, menu::Choice, prelude::{GroupExt, MenuExt, WidgetBase, WidgetExt}};

use crate::{drone_connection::{DroneCommunicationError, DroneConnection}, gui::{Gui, Message, MotorRow, MotorSettings}};

impl Gui {
    pub fn run(&mut self) {
        let sender = self.sender;

        thread::spawn(move || {
            loop {
                sender.send(Message::Tick);
                thread::sleep(Duration::from_millis(500));
            }
        });

        while self.app.wait() {
            if let Some(message) = self.receiver.recv() {
                self.handle_message_and_errors(message);
            }
        }
    }

    fn handle_message_and_errors(&mut self, message: Message) {
        let result = self.handle_message(message);

        if let Err(error) = result {
            self.connection = None;
            dialog::message_default(&format!(
                "Communication failure. Disconnecting from drone {:?}",
                error
            ));
        }
    }

    fn handle_message(&mut self, message: Message) -> Result<(), DroneCommunicationError> {
        match message {
            Message::RefreshPorts => self.status_bar.serial_port_picker.refresh_ports(),
            Message::ConnectToPort => self.connect(),
            Message::Tick => self.tick()?,
            Message::EnterConfigurator => self.enter_configurator()?,
            Message::ResetFc => self.reset_fc()?,
            Message::Forward(motor) => self.drive_forward(motor)?,
            Message::Backward(motor) => self.drive_backward(motor)?,
        };

        Ok(())
    }

    fn connect(&mut self) {
        if let Some(port) = self.status_bar.serial_port_picker.get_chosen_port() {
            let drone_connection = DroneConnection::new(port);

            match drone_connection {
                Ok(mut drone_connection) => {
                    if drone_connection.test_connection() {
                        println!("Connection successful, drone responding to MSP messages");
                        self.connection = Some(drone_connection);
                    } else {
                        dialog::message_default(
                            "USB serial port opened successfully but communication with FC could not be done",
                        );
                    }
                }
                Err(err) => {
                    dialog::message_default(&format!("Failed to communicate with drone {:?}", err));
                }
            }
        }
    }

    pub fn enter_configurator(&mut self) -> Result<(), DroneCommunicationError> {
        if let Some(connection) = self.connection.as_mut() {
            connection.enter_configurator()?;
            self.refresh_motor_list()?;
        }

        Ok(())
    }

    fn refresh_motor_list(&mut self) -> Result<(), DroneCommunicationError> {
        let connection = match self.connection.as_mut() {
            Some(connection) => connection,
            None => return Ok(())
        };

        let config = connection.read_config()?;
        let motor_count = connection.query_motor_count()?;

        self.main_box.motor_settings.redraw(motor_count, &config, self.sender);

        Ok(())
    }
}
