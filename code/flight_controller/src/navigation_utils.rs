use num_traits::Float;

use crate::gps::{Heading, SpherePosition};

const EARTH_RADIUS_IN_METERS: f32 = 6371e3;

impl SpherePosition {
    pub fn heading_to(&self, other: &Self) -> Heading {
        // Returns heading needed to go from self to other

        let lat_a = self.latitude.as_radians();
        let lon_a = self.longitude.as_radians();
        let lat_b = other.latitude.as_radians();
        let lon_b = other.longitude.as_radians();

        // Haversine formula for calculating heading between two points
        let x = lat_b.cos() * (lon_b - lon_a).sin();
        let y = lat_a.cos() * lat_b.sin() - lat_a.sin() * lat_b.cos() * (lon_b - lon_a).cos();
        let heading_rad = x.atan2(y);

        Heading::from_radians(heading_rad)
    }

    pub fn distance_to_in_meters(&self, other: &Self) -> u32 {
        // Returns distance in meters from self to other

        let lat_a = self.latitude.as_radians();
        let lon_a = self.longitude.as_radians();
        let lat_b = other.latitude.as_radians();
        let lon_b = other.longitude.as_radians();

        let delta_lon_half = (lon_a - lon_b) / 2f32;
        let delta_lat_half = (lat_a - lat_b) / 2f32;

        // Haversine formula for calculating distance between two points
        let a =
            delta_lat_half.sin().powi(2) + lat_a.cos() * lat_b.cos() * delta_lon_half.sin().powi(2);
        let c = 2f32 * a.sqrt().atan2((1f32 - a).sqrt());
        let distance = EARTH_RADIUS_IN_METERS * c;

        distance.abs().round() as u32
    }
}

pub enum HeadingOffset {
    CounterClockwise(u16),
    Clockwise(u16),
}

impl Heading {
    pub fn offset_to(&self, other: &Self) -> HeadingOffset {
        let our_heading = self.as_degrees_0_360() as i16;
        let other_heading = other.as_degrees_0_360() as i16;

        let delta_ccw = (our_heading - other_heading).rem_euclid(360) as u16;
        let delta_cw = (other_heading - our_heading).rem_euclid(360) as u16;

        if delta_ccw < delta_cw {
            HeadingOffset::CounterClockwise(delta_ccw)
        } else {
            HeadingOffset::Clockwise(delta_cw)
        }
    }
}
