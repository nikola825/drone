mod angle_mode;

use crate::{
    ahrs_wrapper::AhrsWrapper,
    crsf::{CRSFChannels, CRSF_COMMAND_MAX, CRSF_COMMAND_MIN},
    icm42688::{MotionData, ICM42688},
};

#[derive(Debug)]
pub enum FlightMode {
    Acro,
    Angle,
}

impl FlightMode {
    pub const fn from_crsf(value: u16) -> FlightMode {
        const MODE_WIDTH: u16 = (CRSF_COMMAND_MAX - CRSF_COMMAND_MIN) / 6;
        let offset = value - CRSF_COMMAND_MIN;
        let mode_index = offset / MODE_WIDTH;

        match mode_index {
            0 => FlightMode::Acro,
            1 => FlightMode::Angle,
            _ => FlightMode::Acro,
        }
    }

    pub fn needs_ahrs(&self) -> bool {
        matches!(*self, FlightMode::Angle)
    }
}

pub struct FlightControlContext {
    imu: ICM42688,
    ahrs: Option<AhrsWrapper>,
}

impl FlightControlContext {
    pub fn new(imu: ICM42688) -> Self {
        Self { imu, ahrs: None }
    }

    pub fn update_motion(&mut self) -> MotionData {
        let motion_data = self.imu.get_motion_data();
        let ahrs = self.ahrs.get_or_insert_with(|| AhrsWrapper::new());
        ahrs.update(&motion_data);
        motion_data
    }

    pub fn reset(&mut self) {
        self.ahrs = None;
    }
}

pub struct PidInputs {
    pub target_yaw: f32,
    pub target_pitch: f32,
    pub target_roll: f32,
    pub motion_data: MotionData,
}

impl PidInputs {
    pub fn zero() -> Self {
        PidInputs { target_yaw: 0, target_pitch: 0, target_roll: (), motion_data: () }
    }
}

pub fn control_flight(context: &mut FlightControlContext, inputs: &CRSFChannels) -> PidInputs {
    let motion_data = context.imu.get_motion_data();

    if inputs.armed() {
        let ahrs = context.ahrs.get_or_insert_with(|| AhrsWrapper::new());

        ahrs.update(&motion_data);
    } else {
    }
    PidInputs {
        target_yaw: inputs.yaw_expo(),
        target_pitch: inputs.pitch_expo(),
        target_roll: inputs.roll_expo(),
        motion_data: context.imu.get_motion_data(),
    }
}
