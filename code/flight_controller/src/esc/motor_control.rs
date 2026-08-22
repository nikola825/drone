use crate::{esc::EscMotorSet, hal::ESC_COUNT};

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
    DSHOT_CMD_CUSTOM_DRIVE_SLOW_FOR_TEST = 120,
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

impl EscMotorSet {
    pub async fn beep(&mut self, tone: BeepTone) {
        use BeepTone::*;
        use DshotCommand::*;

        let command = match tone {
            Tone1 => DSHOT_CMD_BEEP1,
            Tone2 => DSHOT_CMD_BEEP2,
            Tone3 => DSHOT_CMD_BEEP3,
            Tone4 => DSHOT_CMD_BEEP4,
            Tone5 => DSHOT_CMD_BEEP5,
        };

        self.multi_send_command([command; ESC_COUNT]).await
    }

    pub async fn multi_throttle(&mut self, mut throttles: [u16; ESC_COUNT]) {
        for throttle in &mut throttles {
            if *throttle > 0 {
                *throttle += 48;
            }
        }

        self.multi_send(throttles).await;
    }

    async fn multi_send(&mut self, values: [u16; ESC_COUNT]) {
        self.dshot_dma.dshot_send_parallel(self.pins, values).await;
    }

    pub async fn multi_send_command(&mut self, values: [DshotCommand; ESC_COUNT]) {
        let values = values.map(|command| command as u16);
        self.multi_send(values).await;
    }
}
