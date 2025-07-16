#![no_std]
#![no_main]

use core::cell::{Cell, RefCell};
use cortex_m_rt::entry;
use cortex_m::interrupt::Mutex;
use panic_halt as _;
//use crate::pac as _;
use pac_test::gpio::{Pin, PinMode, Port};
use pac_test::timer::{Timer};

#[entry]
fn main() -> ! {
    //let _cp = cortex_m::Peripherals::take().unwrap();
    //let dp = pac::Peripherals::take().unwrap();
    let mut led = Pin::new(Port::B, 7, PinMode::Output);

    let tim6 = Timer::new();
    tim6.start();

    loop {
        led.set_high();
    }
}