use std::{
    thread,
    time::{Duration, Instant},
};

use common::{configurator_protocol::messages::FcStatus, shared_objects::StoredConfig};
use fltk::{
    app::{self, Sender},
    button::Button,
    dialog,
    enums::Event,
    frame::Frame,
    group::Flex,
    menu::Choice,
    prelude::{GroupExt, MenuExt, WidgetBase, WidgetExt},
    window::Window,
};
use fltk_theme::WidgetTheme;

use crate::{
    drone_connection::{DroneCommunicationError, DroneConnection, UsbPort},
    gui::{Gui, MainBox, Message, MotorRow, MotorSettings, SerialPortPicker, SideBar, StatusBar},
};

impl Gui {
    pub fn new() -> Self {
        let app = app::App::default();

        let (sender, receiver) = app::channel::<Message>();
        let widget_theme = WidgetTheme::new(fltk_theme::ThemeType::HighContrast);
        widget_theme.apply();

        let mut window = Window::new(100, 100, 1200, 800, "DroneConfigurator");
        window.make_resizable(true);

        let mut root_grid = Flex::default_fill().center_of_parent().column();

        let status_bar = StatusBar::new(sender);
        root_grid.fixed(&status_bar.bar, 30);

        let mut main_layout = Flex::default().center_of_parent().row();

        let side_bar = SideBar::new(sender);

        main_layout.fixed(&side_bar.bar, 150);

        let main_box = MainBox::new(sender);

        main_layout.end();

        root_grid.end();

        window.end();
        window.show();

        Self {
            app,
            status_bar,
            sender,
            receiver,
            side_bar,
            main_box,
            connection: None,
            last_fc_status_update: Instant::now(),
            last_fc_status: FcStatus::default(),
        }
    }

    pub fn tick(&mut self) -> Result<(), DroneCommunicationError> {
        if let Some(connection) = self.connection.as_mut() {
            let battery = connection.query_battery()?;

            let status = connection.query_status()?;

            let status = if status.valid {
                self.last_fc_status_update = Instant::now();
                self.last_fc_status = status;
                status
            } else {
                self.last_fc_status
            };

            if !status.valid {
                self.status_bar
                    .status_label
                    .set_label("No status update recived");
                return Ok(());
            }

            let voltage: f32 = battery.voltage.into();
            let cell_voltage = voltage / (battery.cell_count as f32);

            let phase_string = match status.fc_phase {
                common::configurator_protocol::messages::FcPhase::IMUInitFail => "IMU",
                common::configurator_protocol::messages::FcPhase::Disarmed => "DIS",
                common::configurator_protocol::messages::FcPhase::Armed => "ARM",
                common::configurator_protocol::messages::FcPhase::Config => "CFG",
            };

            let status_age = (Instant::now() - self.last_fc_status_update).as_secs();

            let status = format!(
                "{:2}s    {}    Bat {}S {:.1}V {:.2}V/cell    Dshot time: {:.1}us PID time: {:.1}us Min delta {}us Max delta {}us",
                status_age,
                phase_string,
                battery.cell_count,
                voltage,
                cell_voltage,
                status.inner_duration,
                status.total_duration,
                status.min_measured_period,
                status.max_measured_period
            );

            self.status_bar.status_label.set_label(&status);
        } else {
            self.status_bar.status_label.set_label("Disconnected");
        }

        Ok(())
    }

    pub fn reset_fc(&mut self) -> Result<(), DroneCommunicationError> {
        if let Some(connection) = self.connection.as_mut() {
            connection.reset_fc()?;
        }

        self.connection = None;

        Ok(())
    }

    pub fn drive_forward(&mut self, motor: u8) -> Result<(), DroneCommunicationError> {
        if let Some(connection) = self.connection.as_mut() {
            connection.drive_forward(motor)?;
        }
        Ok(())
    }

    pub fn drive_backward(&mut self, motor: u8) -> Result<(), DroneCommunicationError> {
        if let Some(connection) = self.connection.as_mut() {
            connection.drive_backward(motor)?;
        }
        Ok(())
    }
}

impl StatusBar {
    fn new(sender: Sender<Message>) -> Self {
        let mut bar = Flex::default_fill().row();

        let serial_port_picker = SerialPortPicker::new(sender, &mut bar);

        let status_label = Frame::default_fill().with_label("Perica");

        bar.end();

        StatusBar {
            bar,
            status_label,
            serial_port_picker,
        }
    }
}

impl SerialPortPicker {
    pub fn new(sender: Sender<Message>, status_bar: &mut Flex) -> Self {
        let mut refresh_button = Button::default_fill().with_label("Refresh");
        refresh_button.emit(sender, Message::RefreshPorts);
        let serial_port_dropdown = Choice::default_fill();
        let mut connect_button = Button::default_fill().with_label("Connect");
        connect_button.emit(sender, Message::ConnectToPort);

        status_bar.fixed(&refresh_button, 75);
        status_bar.fixed(&serial_port_dropdown, 150);
        status_bar.fixed(&connect_button, 75);

        Self {
            serial_port_dropdown,
            available_ports: Vec::new(),
        }
    }

    pub fn refresh_ports(&mut self) {
        self.serial_port_dropdown.clear();
        self.available_ports.clear();

        match UsbPort::enumerate() {
            Ok(available_ports) => {
                self.available_ports = available_ports;
                for port in &self.available_ports {
                    self.serial_port_dropdown.add_choice(&port.description());
                }

                if !self.available_ports.is_empty() {
                    self.serial_port_dropdown.set_value(0);
                }
            }
            Err(error) => {
                dialog::message_default(&format!("Failed to enumerate USB ports {:?}", error));
            }
        }
    }

    pub fn get_chosen_port(&mut self) -> Option<&UsbPort> {
        let selected = self.serial_port_dropdown.value();
        if selected > 0 - 1 {
            let port = &self.available_ports[selected as usize];

            Some(port)
        } else {
            None
        }
    }
}

impl SideBar {
    fn new(sender: Sender<Message>) -> Self {
        let mut bar = Flex::default_fill().column();
        let mut btn1 = Button::default_fill().with_label("Start CFG");
        let mut btn2 = Button::default_fill().with_label("Reset FC");

        btn1.emit(sender, Message::EnterConfigurator);
        btn2.emit(sender, Message::ResetFc);
        bar.fixed(&btn1, 50);
        bar.fixed(&btn2, 50);

        bar.make_resizable(true);

        bar.end();

        Self { bar }
    }
}

impl MainBox {
    fn new(sender: Sender<Message>) -> Self {
        let motor_settings = MotorSettings::new(sender);

        Self { motor_settings }
    }
}

impl MotorSettings {
    fn new(sender: Sender<Message>) -> Self {
        let motor_settings = Flex::default_fill().center_of_parent().column();

        let motor_rows: Vec<MotorRow> = Vec::new();

        motor_settings.end();

        Self {
            motor_settings,
            rows: motor_rows,
        }
    }

    pub fn redraw(&mut self, motor_count: u8, config: &StoredConfig, sender: Sender<Message>) {
        let motor_choices = "Front left| Front right| Rear left| Rear right";

        self.motor_settings.clear();
        self.rows.clear();

        self.motor_settings.begin();

        for motor_index in 0..motor_count {
            let mut row = Flex::default_fill()
                .with_size(400, 100)
                .center_of_parent()
                .row();

            let mut choice = Choice::default_fill();
            choice.add_choice(motor_choices);

            let mut button1 = Button::default_fill().with_label("Forward");
            let mut button2 = Button::default_fill().with_label("Backward");
            row.fixed(&button1, 100);
            row.fixed(&button2, 100);

            button1.handle(move |_, event| -> bool {
                match event {
                    Event::Push => {
                        sender.send(Message::Forward(motor_index));
                        true
                    }
                    _ => false,
                }
            });

            button2.handle(move |_, event| -> bool {
                match event {
                    Event::Push => {
                        sender.send(Message::Backward(motor_index));
                        true
                    }
                    _ => false,
                }
            });

            row.end();

            self.motor_settings.fixed(&row, 100);

            self.rows.push(MotorRow {
                dropdown: choice,
                forward: button1,
                backward: button2,
            });
        }

        self.rows[config.front_left_motor as usize]
            .dropdown
            .set_value(0);
        self.rows[config.front_right_motor as usize]
            .dropdown
            .set_value(1);
        self.rows[config.rear_left_motor as usize]
            .dropdown
            .set_value(2);
        self.rows[config.rear_right_motor as usize]
            .dropdown
            .set_value(3);

        self.motor_settings.end();
    }
}
