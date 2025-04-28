use embassy_time::{Duration, Instant};

use crate::crsf::CRSFChannels;

const ARMING_THROTTHE_THRESHOLD: u16 = 50;
const FAILSAFE_DISARM_TIMEOUT: Duration = Duration::from_millis(100);

const DISARM_REASON_NO_SIGNAL: [u8; 3] = *b"SIG";
const DISARM_REASON_THROTTLE: [u8; 3] = *b"THR";
const DISARM_REASON_SWITCH: [u8; 3] = *b"SWI";
const ARMING_MESSAGE_ARMED: [u8; 3] = *b"   ";

#[derive(Clone)]
pub struct ArmingTracker {
    arming_conditions_satisfied: bool,
    timestamp: Instant,
    disarm_reason: &'static [u8; 3],
}

impl Default for ArmingTracker {
    fn default() -> Self {
        Self {
            arming_conditions_satisfied: false,
            timestamp: Instant::MIN,
            disarm_reason: &DISARM_REASON_NO_SIGNAL,
        }
    }
}

impl ArmingTracker {
    pub fn update(&mut self, commands: &CRSFChannels) {
        if !commands.armed() {
            self.disarm_reason = &DISARM_REASON_SWITCH;
        } else if commands.throttle() >= ARMING_THROTTHE_THRESHOLD {
            self.disarm_reason = &DISARM_REASON_THROTTLE;
        }
        let stay_armed = self.is_armed() & commands.armed();
        let arm_at_zero = commands.armed() && commands.throttle() < ARMING_THROTTHE_THRESHOLD;
        self.arming_conditions_satisfied = stay_armed || arm_at_zero;
        self.timestamp = commands.timestamp
    }

    pub fn is_armed(&self) -> bool {
        self.arming_conditions_satisfied && self.is_fresh()
    }

    fn is_fresh(&self) -> bool {
        let now = Instant::now();
        let age = now - self.timestamp;

        age < FAILSAFE_DISARM_TIMEOUT && self.timestamp != Instant::MIN
    }

    pub fn arming_message(&self) -> &'static [u8; 3] {
        if self.is_armed() {
            &ARMING_MESSAGE_ARMED
        } else if !self.is_fresh() {
            &DISARM_REASON_NO_SIGNAL
        } else {
            self.disarm_reason
        }
    }
}
