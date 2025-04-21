use crate::{
    crsf::{CRSFChannels, CRSF_COMMAND_MAX, CRSF_COMMAND_MIN, CRSF_COMMAND_RANGE},
    expo_rates::map_crsf_to_expo,
};

const fn crsf_linear_transform(
    value: u16,
    out_low: i32,
    out_high: i32,
    out_deadpoint: i32,
    out_deadrange: i32,
) -> i32 {
    let value = if value > CRSF_COMMAND_MAX {
        CRSF_COMMAND_MAX
    } else if value < CRSF_COMMAND_MIN {
        CRSF_COMMAND_MIN
    } else {
        value
    };

    let offset: i32 = (value - CRSF_COMMAND_MIN) as i32;
    let out_range: i32 = out_high - out_low;

    let mut mapped = (out_low) + (offset * out_range) / (CRSF_COMMAND_RANGE as i32);
    if (mapped - out_deadpoint).abs() < out_deadrange {
        mapped = out_deadpoint
    }

    if mapped > out_high {
        out_high
    } else if mapped < out_low {
        out_low
    } else {
        mapped
    }
}

macro_rules! define_channel {
    ($name: ident, $index: expr) => {
        pub const fn $name(&self) -> u16 {
            self.unpacked_channels[$index]
        }
    };

    (bool, $name: ident, $index: expr) => {
        pub const fn $name(&self) -> bool {
            crsf_linear_transform(self.unpacked_channels[$index], 0, 2, 0, -1) > 0
        }
    };

    ($type: ident, $name: ident, $index: expr, $low: expr, $high: expr) => {
        define_channel!($type, $name, $index, $low, $high, 0, -1);
    };

    ($type: ident, $name: ident, $index: expr, $low: expr, $high: expr, $dead_point: expr, $dead_range: expr) => {
        pub const fn $name(&self) -> $type {
            crsf_linear_transform(
                self.unpacked_channels[$index],
                $low,
                $high,
                $dead_point,
                $dead_range,
            ) as $type
        }
    };

    ($type: ident, $name: ident, $index: expr, $transformer: ident) => {
        pub const fn $name(&self) -> $type {
            $transformer(self.unpacked_channels[$index])
        }
    };
}

const fn map_crsf_to_servo(value: u16) -> u16 {
    crsf_linear_transform(value, 1000, 2000, 0, -1) as u16
}

impl CRSFChannels {
    define_channel!(bool, armed, 4);

    define_channel!(u16, throttle, 2, 0, 6000, 0, 10);
    define_channel!(u8, throttle_percent, 2, 0, 100);

    define_channel!(f32, yaw_expo, 3, map_crsf_to_expo);
    define_channel!(f32, pitch_expo, 1, map_crsf_to_expo);
    define_channel!(f32, roll_expo, 0, map_crsf_to_expo);

    define_channel!(u16, yaw_servo, 3, map_crsf_to_servo);
    define_channel!(u16, pitch_servo, 1, map_crsf_to_servo);
    define_channel!(u16, roll_servo, 0, map_crsf_to_servo);
    define_channel!(u16, throttle_servo, 2, map_crsf_to_servo);

    define_channel!(u16, aux1, 5, 0, 128);
    define_channel!(u16, aux2, 6, 0, 128);
    define_channel!(bool, beep, 7);
}
