use crate::crsf::{CRSF_COMMAND_MAX, CRSF_COMMAND_MIN, CRSF_COMMAND_RANGE};

const RC: f32 = 1.2;
const RATE: f32 = 0.7;
const EXPO: f32 = 0.0;

const fn normalize_crsf_to_1(value: u16) -> f32 {
    const OUTPUT_LOW: f32 = -1.0;
    const OUTPUT_HIGH: f32 = 1.0;
    const OUTPUT_RANGE: f32 = OUTPUT_HIGH - OUTPUT_LOW;

    if value < CRSF_COMMAND_MIN {
        OUTPUT_LOW
    } else if value > CRSF_COMMAND_MAX {
        OUTPUT_HIGH
    } else {
        let offset: f32 = (value - CRSF_COMMAND_MIN) as f32;
        let result = offset / CRSF_COMMAND_RANGE * OUTPUT_RANGE + OUTPUT_LOW;

        result.clamp(OUTPUT_LOW, OUTPUT_HIGH)
    }
}

pub const fn map_crsf_to_expo(command: u16) -> f32 {
    let mut command = normalize_crsf_to_1(command);

    let command_abs = command.abs();

    if EXPO > 0.0 {
        command =
            command * (command_abs * command_abs * command_abs) * EXPO + command * (1.0 - EXPO);
    }

    let angle_rate = 200.0 * RC * command;

    let rc_factor = 1.0 - command_abs * RATE;
    let rc_factor = rc_factor.clamp(0.1, 1.0);
    let rc_factor = 1.0 / rc_factor;

    (angle_rate * rc_factor).clamp(-800.0, 800.0)
}
