use cortex_m::{interrupt::CriticalSection, peripheral::SCB};

// The icache toggle bit is the 17th bit of SCB CCR
pub const SCB_CCR_IC_MASK: u32 = 1 << 17;

// icache is disabled during the lifetime of this object
// once dropped, it reenables icache if it was previously enabled
// has reference to a critical section to ensure it does not live longer than a critical section
pub struct ICachePause<'a> {
    _critical_section: &'a CriticalSection,
    icache_was_enabled: bool,
}

impl<'a> ICachePause<'a> {
    unsafe fn new(critical_section: &'a CriticalSection) -> Self {
        // As soon as object is created, disable icache
        // it will remain disabled until drop
        // Keep a reference to a critical section to ensure this
        // object does not exist outside the lifetime of a critical section

        let icache_was_enabled = SCB::icache_enabled();

        let peripherals = cortex_m::Peripherals::steal();
        peripherals.SCB.ccr.modify(|r| r & !SCB_CCR_IC_MASK);

        cortex_m::asm::isb(); // flush pipeline

        ICachePause {
            _critical_section: critical_section,
            icache_was_enabled,
        }
    }
}

impl Drop for ICachePause<'_> {
    fn drop(&mut self) {
        unsafe {
            // On drop, if cache was enabled at creation, we reenable the cache
            if self.icache_was_enabled {
                let peripherals = cortex_m::Peripherals::steal();
                peripherals.SCB.ccr.modify(|r| r | SCB_CCR_IC_MASK);

                cortex_m::asm::isb(); // flush pipeline
            }
        }
    }
}

// Disables icache, runs the callable, reenables icache if it was previously enabled
// Must be called in a critical section
pub fn run_with_paused_icache(
    critical_section: &CriticalSection,
    callable: impl FnOnce(&ICachePause<'_>),
) {
    unsafe {
        let icache_pause = ICachePause::new(critical_section);
        callable(&icache_pause);
    }
}
