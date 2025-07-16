#![no_std]
#![no_main]

use core::cell::{Cell, RefCell};
use cortex_m_rt::entry;
use cortex_m::interrupt::Mutex;
use panic_halt as _;
//use crate::pac as _;
use stm32l4_hal::gpio::{Pin, PinMode, Port};
use stm32l4_hal::rcc::ClockManager;
use stm32l4_hal::timer::{Timer};
//use stm32l4_hal::rcc::{VRange1, SourceMSI, PLLDisabled};
use stm32l4_hal::rcc::MSIRange;

#[entry]
fn main() -> ! {
    //let _cp = cortex_m::Peripherals::take().unwrap();
    //let dp = pac::Peripherals::take().unwrap();
    let mut cm= ClockManager::new();
    cm.update_msi_range(MSIRange::Range11);

    let mut led = Pin::new(Port::B, 7, PinMode::Output);

    let tim6 = Timer::new();
    tim6.start();

    loop {
        for _ in 0..10000 {
            led.set_low();
        }
        for _ in 0..10000 {
            led.set_high();
        }
    }
}