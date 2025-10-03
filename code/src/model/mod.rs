mod five_inch;
mod seven_inch;

#[cfg(feature = "five-inch")]
use five_inch as model_parameters;
#[cfg(feature = "seven-inch")]
use seven_inch as model_parameters;

pub use model_parameters::{
    PITCH_KD, PITCH_KI, PITCH_KP, ROLL_KD, ROLL_KI, ROLL_KP, YAW_KD, YAW_KI, YAW_KP,
};
