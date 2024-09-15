use common::storedvars::StoredVariables;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
pub struct Store {
    pub state: Mutex<ThreadModeRawMutex, StoreState>,
}

#[derive(Clone, Default)]
pub struct StoreState {
    pub variables: StoredVariables,
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
}
