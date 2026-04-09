#![allow(dead_code, unused_variables)]
use crate::{
    ahrs_wrapper::{AhrsAngles, AhrsWrapper},
    crsf::CRSFChannels,
    flight_control::TargetAngularVelocities,
    gps::{Heading, SpherePosition},
    math_stuff::angle_sub,
};

fn angle_mode_target_angular_velocity(angle: f32, target_angle: f32, velocity_scale: f32) -> f32 {
    let angle_error = angle_sub(target_angle, angle);

    if angle_error == 0f32 {
        0f32
    } else if angle_error.abs() < 0.1 {
        1.0 * angle_error.signum()
    } else {
        angle_error * velocity_scale
    }
}

fn nav_target_yaw(
    heading: Option<Heading>,
    position: Option<SpherePosition>,
    home: Option<SpherePosition>,
    inputs: &CRSFChannels,
) -> f32 {
    if let (Some(heading), Some(position), Some(home)) = (heading, position, home) {
        let home_heading = position.heading_to(&home);
        let home_heading_offset = heading.offset_to(&home_heading).as_degrees();

        if home_heading_offset != 0f32 {
            home_heading_offset.abs().clamp(2f32, 120f32) * home_heading_offset.signum()
        } else {
            0f32
        }
    } else {
        inputs.yaw_expo()
    }
}

pub fn get_target_velocities(inputs: &CRSFChannels, ahrs: &AhrsWrapper) -> TargetAngularVelocities {
    let euler = ahrs.read_ypr();

    let (pitch, roll) =
        calculate_angle_mode_pr_velocities(inputs.pitch_angle(), inputs.roll_angle(), euler);

    TargetAngularVelocities {
        yaw: inputs.yaw_expo(),
        pitch,
        roll,
    }
}

pub fn calculate_angle_mode_pr_velocities(
    target_pitch: f32,
    target_roll: f32,
    euler: AhrsAngles,
) -> (f32, f32) {
    (
        angle_mode_target_angular_velocity(euler.pitch, target_pitch, 10f32),
        angle_mode_target_angular_velocity(euler.roll, target_roll, 10f32),
    )
}
