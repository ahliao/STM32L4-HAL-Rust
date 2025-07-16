#![no_main]
#![no_std]

//use core::sync::atomic::{AtomicUsize, Ordering};
//use defmt_brtt as _; // global logger

//use panic_probe as _;
//use panic_rtt_target as _;

use cortex_m::{self, delay::Delay};
pub use stm32l4::stm32l4x6 as pac;

pub mod gpio;
pub mod timer;
pub mod rcc;

/// Terminates the application and makes `probe-rs` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
