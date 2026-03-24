#![allow(dead_code, unused_variables)]
use crate::{ahrs_wrapper::AhrsWrapper, crsf::CRSFChannels, gps::{Heading, SpherePosition}, math_stuff::angle_sub};

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

fn x(inputs: &CRSFChannels, ahrs: &AhrsWrapper) {
    let euler = ahrs.read_ypr();
    let (yaw_target_angular_velocity, pitch_target_angular_velocity, roll_target_angular_velocity) = match inputs.mode() {
        crate::flight_control::FlightMode::Acro => {
            (inputs.yaw_expo(), inputs.pitch_expo(), inputs.roll_expo())
        }
        crate::flight_control::FlightMode::Angle => {
            (
                inputs.yaw_expo(),
                angle_mode_target_angular_velocity(
                    euler.pitch,
                    45f32,
                    10f32
                ),
                angle_mode_target_angular_velocity(
                    euler.roll,
                    inputs.roll_angle(),
                    10f32
                ),
            )
        }
    };
}