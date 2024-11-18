use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};

use crate::crsf::CRSFChannels;
pub struct Store {
    pub state: Mutex<ThreadModeRawMutex, StoreState>,
}

#[derive(Clone, Default)]
pub struct StoreState {
    pub channels: CRSFChannels,
}

impl Store {
    pub fn new() -> Self {
        Store {
            state: Mutex::new(StoreState::default()),
        }
    }

    pub async fn snapshot(&self) -> StoreState {
        let guard = self.state.lock().await;
        guard.clone()
    }

    pub async fn update_channels(&self, channels: CRSFChannels) {
        let mut guard = self.state.lock().await;
        guard.channels = channels;
    }
}
