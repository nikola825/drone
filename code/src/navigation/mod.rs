pub mod math;

use crate::{
    gps::{Altitude, Heading, Speed, SpherePosition, UbxNavPVTPacket}, navigation::math::HeadingOffset, shared_state::SharedState
};

#[derive(Default, Clone)]
pub struct PVTData {
    pub position: SpherePosition,
    pub altitude_msl: Altitude,
    pub heading: Heading,
    pub ground_speed: Speed,
}

#[derive(Default, Clone)]
pub struct GPSData {
    pub pvt_data: Option<PVTData>,
    pub satelites_visible: u8,
}

impl From<&UbxNavPVTPacket> for PVTData {
    fn from(gps_packet: &UbxNavPVTPacket) -> Self {
        Self {
            position: gps_packet.position,
            altitude_msl: gps_packet.height_mean_sea_level,
            heading: gps_packet.vehicle_heading,
            ground_speed: gps_packet.ground_speed,
        }
    }
}

impl From<UbxNavPVTPacket> for GPSData {
    fn from(gps_packet: UbxNavPVTPacket) -> Self {
        let pvt_data: Option<PVTData> = if gps_packet.gps_data_displayable() {
            Some((&gps_packet).into())
        } else {
            None
        };

        Self {
            pvt_data,
            satelites_visible: gps_packet.satelites_visible,
        }
    }
}

#[derive(Default, Clone)]
pub struct NavigationState {
    gps_data: Option<GPSData>,
    home: Option<Home>,
}

#[derive(Clone)]
pub struct Home {
    pub position: SpherePosition,
    pub heading_to: Heading,
    pub heading_offset_to: HeadingOffset,
}

impl Home {
    pub fn new(position: SpherePosition) -> Self {
        Self {
            position,
            heading_to: Heading::default(),
            heading_offset_to: HeadingOffset::Clockwise(0),
        }
    }

    pub fn update_headings(&mut self, gps_data: &PVTData) {
        self.heading_to = gps_data.position.heading_to(&self.position);
        self.heading_offset_to = gps_data.heading.offset_to(&self.heading_to);
    }
}

impl NavigationState {
    pub fn update(&mut self, packet: UbxNavPVTPacket, armed: bool) {
        let gps_data: GPSData = packet.into();
        if let Some(pvt_data) = &gps_data.pvt_data {
            if !armed || self.home.is_none() {
                self.home = Some(Home::new(pvt_data.position));
            }

            if let Some(home) = &mut self.home {
                home.update_headings(pvt_data);
            }
        }
        self.gps_data = Some(gps_data);
    }

    pub fn try_read_navigation_data(&self) -> Option<(&PVTData, &Home)> {
        if let (Some(gps_data), Some(home)) = (&self.gps_data, &self.home) {
            if let Some(pvt_data) = &gps_data.pvt_data {
                return Some((pvt_data, home));
            }
        }
        None
    }

    pub fn try_read_gps_data(&self) -> Option<&GPSData> {
        self.gps_data.as_ref()
    }
}

pub async fn update_navigation(packet: UbxNavPVTPacket, store: &SharedState) {
    let mut current_state = store.get_navigation_state().await;
    let armed = store.is_armed().await;
    current_state.update(packet, armed);

    store.update_navigation_state(current_state).await;
}
