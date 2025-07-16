use core::cell::{Cell, RefCell};
use cortex_m::interrupt::Mutex;
use stm32l4::stm32l4x6::interrupt;

use crate::pac::tim1::{arr, cr1, psc, sr};
use crate::pac::{RCC};

//static G_TIM: Mutex<RefCell<Option<Timer>>> = Mutex::new(RefCell::new(None));

pub struct Timer {
    timeout: u32,
}

impl Timer {
    pub fn new() -> Self
    {
        // TODO: Get the system clock

        unsafe {
            // Enable the Interrupt NVIC
            cortex_m::peripheral::NVIC::unmask(stm32l4::stm32l4x6::interrupt::TIM6_DACUNDER);
        }

        // Enable the RCC peripheral clock
        let rcc = unsafe { &(*RCC::ptr()) };
        if rcc.apb1enr1().read().tim6en().bit_is_clear() {
            rcc.apb1enr1().write(|w| w.tim6en().set_bit());
        }

        let tim6 = unsafe{ &(*crate::pac::TIM6::ptr()) };
        unsafe {
            // Write the prescaler
            tim6.psc().write(|w| w.bits(16));

            // Write the auto-reload
            tim6.arr().write(|w| w.bits(0xFFFF));

            // Update generation
            tim6.egr().write(|w| w.bits(1));

            // Clear the interrupt
            tim6.sr().write(|w| w.uif().clear());

            // Enable the timer interrupt in the peripheral
            tim6.dier().modify(|_,w| w.uie().set_bit());
        }

        let result: Timer = Self { timeout: 0 };
        result
    }

    pub fn start(&self) {
        let tim6 = unsafe{ &(*crate::pac::TIM6::ptr()) };
        tim6.cr1().modify(|_,w| w.cen().bit(true));
    }
}

#[interrupt]
fn TIM6_DACUNDER()
{
    // Clear the timer interrupt
    let tim6 = unsafe{ &(*crate::pac::TIM6::ptr()) };
    tim6.sr().write(|w| w.uif().clear());
}