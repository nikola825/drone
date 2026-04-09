mod angle_mode;
mod waypoint_nav;

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
}

pub struct FlightControlContext {
    imu: ICM42688,
    ahrs: Option<AhrsWrapper>,
}

impl FlightControlContext {
    pub fn new(imu: ICM42688) -> Self {
        Self { imu, ahrs: None }
    }

    pub fn update_motion(&mut self, armed: bool) -> MotionData {
        let motion_data = self.imu.get_motion_data();

        if armed {
            let ahrs = self.ahrs.get_or_insert_with(AhrsWrapper::new);
            ahrs.update(&motion_data);
        }

        motion_data
    }

    pub fn reset(&mut self) {
        self.ahrs = None;
    }
}

pub struct TargetAngularVelocities {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}
pub struct PidInputs {
    pub target_angular_velocities: TargetAngularVelocities,
    pub motion_data: MotionData,
}

impl TargetAngularVelocities {
    pub fn zero() -> Self {
        Self {
            yaw: 0f32,
            pitch: 0f32,
            roll: 0f32,
        }
    }
}

impl PidInputs {
    pub fn disarmed(motion_data: MotionData) -> Self {
        PidInputs {
            target_angular_velocities: TargetAngularVelocities::zero(),
            motion_data,
        }
    }
}

fn get_target_angular_velocities(
    context: &mut FlightControlContext,
    inputs: &CRSFChannels,
) -> TargetAngularVelocities {
    match inputs.mode() {
        FlightMode::Acro => TargetAngularVelocities {
            yaw: inputs.yaw_expo(),
            pitch: inputs.pitch_expo(),
            roll: inputs.roll_expo(),
        },
        FlightMode::Angle => {
            if inputs.return_to_home() {
                waypoint_nav::get_target_angular_velocities()
            } else {
                angle_mode::get_target_velocities(inputs, context.ahrs.as_ref().unwrap())
            }
        }
    }
}

pub fn control_flight(
    armed: bool,
    context: &mut FlightControlContext,
    inputs: &CRSFChannels,
) -> PidInputs {
    let motion_data = context.update_motion(armed);

    if armed {
        let target_angular_velocities = get_target_angular_velocities(context, inputs);

        PidInputs {
            target_angular_velocities,
            motion_data,
        }
    } else {
        context.reset();

        PidInputs::disarmed(motion_data)
    }
}
