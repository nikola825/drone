use crate::{
    dshot::{dshot_send_parallel, dshot_send_single},
    motor::Motor,
};

#[derive(Clone, Copy)]
#[allow(dead_code, non_camel_case_types)]
pub enum DshotCommand {
    DSHOT_CMD_STOP = 0,
    DSHOT_CMD_BEEP1 = 1,
    DSHOT_CMD_BEEP2 = 2,
    DSHOT_CMD_BEEP3 = 3,
    DSHOT_CMD_BEEP4 = 4,
    DSHOT_CMD_BEEP5 = 5,
    DSHOT_CMD_SPIN_DIRECTION_1 = 7,
    DSHOT_CMD_SPIN_DIRECTION_2 = 8,
    DSHOT_CMD_3D_MODE_OFF = 9,
    DSHOT_CMD_SAVE_SETTINGS = 12,
    DSHOT_CMD_CUSTOM_DRIVE_SLOW_FOR_TEST = 80,
}

#[derive(Clone, Copy)]
pub enum BeepTone {
    Tone1,
    Tone2,
    Tone3,
    Tone4,
    Tone5,
}

impl BeepTone {
    pub fn next(self) -> Self {
        use BeepTone::*;
        match self {
            Tone1 => Tone2,
            Tone2 => Tone3,
            Tone3 => Tone4,
            Tone4 => Tone5,
            Tone5 => Tone1,
        }
    }
}

impl Motor {
    fn send_value(&self, value: u16) {
        dshot_send_single(self.port.bsrr(), self.pin as _, value);
    }

    fn send_command(&self, command: DshotCommand) {
        self.send_value(command as u16);
    }

    #[allow(dead_code)]
    fn set_throttle(&self, throttle: u16) {
        if throttle > 0 {
            self.send_value(48 + throttle);
        } else {
            self.send_value(0);
        }
    }

    pub fn beep(&self, tone: BeepTone) {
        use BeepTone::*;
        use DshotCommand::*;

        let command = match tone {
            Tone1 => DSHOT_CMD_BEEP1,
            Tone2 => DSHOT_CMD_BEEP2,
            Tone3 => DSHOT_CMD_BEEP3,
            Tone4 => DSHOT_CMD_BEEP4,
            Tone5 => DSHOT_CMD_BEEP5,
        };

        self.send_command(command);
    }

    pub fn multi_throttle<const COUNT: usize>(motors: [&Self; COUNT], mut throttles: [u16; COUNT]) {
        for throttle in &mut throttles {
            if *throttle > 0 {
                *throttle += 48;
            }
        }

        Self::multi_send(motors, throttles);
    }

    fn multi_send<const COUNT: usize>(motors: [&Self; COUNT], values: [u16; COUNT]) {
        let same_port = motors.iter().all(|motor| motor.port == motors[0].port);

        if same_port {
            dshot_send_parallel(
                motors[0].port.bsrr(),
                motors.map(|motor| motor.pin as usize),
                values,
            );
        } else {
            for (motor, value) in motors.iter().zip(values) {
                motor.send_value(value);
            }
        }
    }

    pub fn multi_send_command<const COUNT: usize>(
        motors: [&Self; COUNT],
        values: [DshotCommand; COUNT],
    ) {
        let values = values.map(|command| command as u16);
        Self::multi_send(motors, values);
    }
}
