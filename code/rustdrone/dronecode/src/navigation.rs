use crate::{crsf::CRSFChannels, DroneContext};

pub struct NavigationContext {
    pub motor_thrust: u16
}

impl NavigationContext {
    pub fn new() -> Self {
        NavigationContext {
            motor_thrust: 0
        }
    }
}

pub fn navigate(context: &mut DroneContext, inputs: CRSFChannels) {
    context.navigation_context.motor_thrust = inputs.throttle();
}

