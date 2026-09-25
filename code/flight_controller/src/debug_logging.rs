#[cfg(feature = "debug-logging")]
use core::fmt::Write;

#[cfg(feature = "debug-logging")]
use common::shared_objects::{DebugLog, LOG_LINE_SIZE};

#[cfg(feature = "debug-logging")]
const LOG_CACHE_SIZE: usize = 512;

#[cfg(feature = "debug-logging")]
#[derive(Clone, Copy)]
pub struct LoggingState {
    buffer: [u8; LOG_CACHE_SIZE],
    head: usize,
    tail: usize,
    count: usize,
}

#[cfg(feature = "debug-logging")]
impl LoggingState {
    pub const fn new() -> Self {
        Self {
            buffer: [0; LOG_CACHE_SIZE],
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    pub fn take_log(&mut self) -> Option<DebugLog> {
        if self.count == 0 {
            return None;
        }

        let mut log = DebugLog::new();
        for index in 0..LOG_LINE_SIZE {
            if self.count == 0 {
                break;
            }
            log.line[index] = self.getchar();
        }

        Some(log)
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.as_bytes() {
            if self.count == LOG_CACHE_SIZE {
                return;
            }

            self.putchar(*byte);
        }
    }

    fn putchar(&mut self, character: u8) {
        if self.count == LOG_CACHE_SIZE {
            return;
        }

        self.buffer[self.head] = character;

        self.head = (self.head + 1) % LOG_CACHE_SIZE;
        self.count += 1;
    }

    fn getchar(&mut self) -> u8 {
        if self.count == 0 {
            return 0;
        }

        let character = self.buffer[self.tail];
        self.tail = (self.tail + 1) % LOG_CACHE_SIZE;
        self.count -= 1;

        character
    }

    pub fn write_log_fmt(&mut self, args: core::fmt::Arguments<'_>) {
        let _ = self.write_fmt(args);
        self.write_string("\n");
    }
}

#[cfg(feature = "debug-logging")]
impl Write for LoggingState {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);

        Ok(())
    }
}

#[cfg(feature = "debug-logging")]
#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {{
        use $crate::shared_state::SHARED_STATE;
        SHARED_STATE.write_log_fmt(format_args!($($arg)*)).await;
    }};
}

#[cfg(not(feature = "debug-logging"))]
#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {{}};
}
