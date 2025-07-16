use core::convert::Infallible;

use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};
use stm32l4::stm32l4x6::gpioa::ospeedr;

use crate::pac::{self, EXTI, RCC};
//use crate::util::rcc_en_reset;

use paste::paste;

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum PinMode {
    Input,
    Output,
    Alt(u8),
    Analog,
}

impl PinMode {
    fn val(&self) -> u8 {
        match self {
            Self::Input => 0b00,
            Self::Output => 0b01,
            Self::Alt(_) => 0b10,
            Self::Analog => 0b11,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum OutputType {
    PushPull = 0,
    OpenDrain = 1,
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum OutputSpeed {
    Low = 0b00,
    Medium = 0b01,
    High = 0b10,
    VeryHigh = 0b11,
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Pull {
    Floating = 0b00,
    Up = 0b01,
    Down = 0b10,
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum PinState {
    High = 1,
    Low = 0,
}

#[derive(Copy, Clone)]
#[repr(u8)]
/// Values for `GPIOx_LCKR`.
pub enum CfgLock {
    NotLocked = 0,
    Locked = 1,
}

#[derive(Copy, Clone)]
#[repr(u8)]
/// Values for `GPIOx_BRR`.
pub enum ResetState {
    NoAction = 0,
    Reset = 1,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Port {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
}

impl Port {
    fn cr_val(&self) -> u8 {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
            Self::E => 4,
            Self::F => 5,
            Self::G => 6,
            Self::H => 7,
            Self::I => 8,
        }
    }
}

#[derive(Copy, Clone, Debug)]
/// The pulse edge used to trigger interrupts. Either rising, falling, or either.
pub enum Edge {
    /// Interrupts trigger on rising pin edge.
    Rising,
    /// Interrupts trigger on falling pin edge.
    Falling,
    /// Interrupts trigger on either rising or falling pin edges.
    Either,
}

// Macro to set the relavant pin field for a register field
macro_rules! set_field {
    ($regs:expr, $pin:expr, $reg:ident, $field:ident, $bit:ident, $val:expr, [$($num:expr),+]) => {
        paste! {
            unsafe {
                match $pin {
                    $(
                        $num => (*$regs).$reg().modify(|_, w| w.[<$field $num>]().$bit($val)),
                    )+
                    _ => panic!("GPIO pins must be 0 - 15."),
                }
            }
        }
    }
}

macro_rules! set_state {
    ($regs:expr, $pin:expr, $offset:expr, [$($num:expr),+]) => {
        paste! {
            unsafe {
                match $pin {
                    $(
                        $num => (*$regs).bsrr().write(|w| w.bits(1 << ($offset + $num))),
                    )+
                    _ => panic!("GPIO pins must be 0 - 15."),
                }
            }
        }
    };
}

macro_rules! get_input_data {
    ($regs: expr, $pin:expr, [$($num:expr),+]) => {
        paste! {
            unsafe {
                match $pin {
                    $(
                        $num => (*$regs).idr().read().[<idr $num>]().bit_is_set(),
                    )+
                    _ => panic!("GPIO pins must be 0 - 15."),
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Pin {
    pub port: Port,
    pub pin: u8,
}

/*macro_rules! gpio_rcc_en {
    ($port:expr, $rcc:expr) => {
        if $rcc.ahb2enr().read().$port().bit_is_clear() {
            //$rcc.ahb2enr().write(|w| w.$port().set_bit());
        }
    };
}*/

impl Pin {
    pub fn new(port: Port, pin: u8, mode: PinMode) -> Self {
        assert!(pin <= 15, "Pin must be 0 - 15.");

        // Sets the clock
        let rcc = unsafe { &(*RCC::ptr()) };
        match port {
            Port::A => {
                // Set the AHB2ENR GPIOA Enable
                //gpio_rcc_en!(gpioaen, rcc);
                if rcc.ahb2enr().read().gpioaen().bit_is_clear() {
                    rcc.ahb2enr().write(|w| w.gpioaen().set_bit());
                }
            }
            Port::B => {
                if rcc.ahb2enr().read().gpioben().bit_is_clear() {
                    rcc.ahb2enr().write(|w| w.gpioben().set_bit());
                }
            }
            Port::C => {
                if rcc.ahb2enr().read().gpiocen().bit_is_clear() {
                    rcc.ahb2enr().write(|w| w.gpiocen().set_bit());
                }
            }
            Port::D => {
                if rcc.ahb2enr().read().gpioden().bit_is_clear() {
                    rcc.ahb2enr().write(|w| w.gpioden().set_bit());
                }
            }
            Port::E => {
                if rcc.ahb2enr().read().gpioeen().bit_is_clear() {
                    rcc.ahb2enr().write(|w| w.gpioeen().set_bit());
                }
            }
            Port::F => {
                if rcc.ahb2enr().read().gpiofen().bit_is_clear() {
                    rcc.ahb2enr().write(|w| w.gpiofen().set_bit());
                }
            }
            Port::G => {
                // Port G needs to enable the power bit
                if rcc.ahb2enr().read().gpiogen().bit_is_clear() {
                    rcc.ahb2enr().write(|w| w.gpiogen().set_bit());

                    // Set pwr bit and iobank2
                    let pwr = unsafe { &(*pac::PWR::ptr()) };
                    rcc.apb1enr1().modify(|_, w| w.pwren().set_bit());
                    pwr.cr2().modify(|_, w| w.iosv().set_bit());
                }
            }
            Port::H => {
                if rcc.ahb2enr().read().gpiohen().bit_is_clear() {
                    rcc.ahb2enr().write(|w| w.gpiohen().set_bit());
                }
            }
            Port::I => {
                if rcc.ahb2enr().read().gpioien().bit_is_clear() {
                    //rcc.ahb2enr().write(|w| w.gpiohen().set_bit());
                    rcc.ahb2enr().modify(|_,w| w.gpioien().set_bit());
                    rcc.ahb2rstr().modify(|_,w| w.gpioirst().set_bit());
                    rcc.ahb2rstr().modify(|_,w| w.gpioirst().clear_bit());
                }
            }
        }

        let mut result = Self { port, pin };
        result.mode(mode);

        result
    }

    

    pub fn mode(&mut self, value: PinMode) {
        let pinnum = self.pin;
        /*match self.port {
            Port::A => {
                paste! {
                    let gpioa = unsafe { &(*crate::pac::GPIOA::ptr()) };
                    gpioa.moder().write(|w| unsafe {w.[<moder pinnum>]().bits(value.val()) });
                }
            }
            _ => panic!("TODO")
        }*/
        set_field!(
            self.regs(),
            self.pin,
            moder,
            moder,
            bits,
            value.val(),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );

        //if let PinMode::Alt(alt) = value {
        //    self.alt_fn(alt);
        //}
    }

    // Output Type, Sets GPIOx_OTYPER
    pub fn output_type(&mut self, value: OutputType) {
        // This does regs().otyper().modify(|_,w| w.ot{self.pin}.bit(0 or 1))
        set_field!(
            self.regs(),
            self.pin,
            otyper,
            ot,
            bit,
            value as u8 != 0,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
    }

    pub fn output_speed(&mut self, value: OutputSpeed) {
        //unsafe {
        //(*(self.regs())).ospeedr().modify(|_,w| w.ospeedr0().bits(value as u8));
        //}
        set_field!(
            self.regs(),
            self.pin,
            ospeedr,
            ospeedr,
            bits,
            value as u8,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
    }

    // PUPDR
    pub fn pull(&mut self, value: Pull) {
        set_field!(
            self.regs(),
            self.pin,
            pupdr,
            pupdr,
            bits,
            value as u8,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
    }

    /// Lock or unlock a port configuration. Sets the `LCKR` register.
    pub fn cfg_lock(&mut self, value: CfgLock) {
        set_field!(
            self.regs(),
            self.pin,
            lckr,
            lck,
            bit,
            value as u8 != 0,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
    }

    pub fn get_state(&mut self) -> PinState {
        let val = get_input_data!(
            self.regs(),
            self.pin,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
        if val { PinState::High } else { PinState::Low }
    }

    pub fn set_state(&mut self, value: PinState) {
        let offset = match value {
            PinState::Low => 16,
            PinState::High => 0,
        };

        set_state!(
            self.regs(),
            self.pin,
            offset,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
    }

    pub fn is_high(&self) -> bool {
        get_input_data!(
            self.regs(),
            self.pin,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        )
    }

    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    pub fn set_high(&mut self) {
        self.set_state(PinState::High);
    }

    pub fn set_low(&mut self) {
        self.set_state(PinState::Low);
    }

    /// Toggle output voltage between low and high. Sets the `BSRR` register. Atomic.
    pub fn toggle(&mut self) {
        // if self.is_high() {
        if Pin::is_high(self) {
            Pin::set_low(self);
            // self.set_low();
        } else {
            // self.set_high();
            Pin::set_high(self);
        }
    }

    const fn regs(&self) -> *const pac::gpioa::RegisterBlock {
        // Note that we use this `const` fn and pointer casting since not all ports actually
        // deref to GPIOA in PAC.
        regs(self.port)
    }

    
}


impl ErrorType for Pin {
    type Error = Infallible;
}

impl InputPin for Pin {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Pin::is_high(self))
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Pin::is_low(self))
    }
}

const fn regs(port: Port) -> *const pac::gpioa::RegisterBlock {
    match port {
        Port::A => crate::pac::GPIOA::ptr(),
        Port::B => crate::pac::GPIOB::ptr() as _,
        Port::C => crate::pac::GPIOC::ptr() as _,
        Port::D => crate::pac::GPIOD::ptr() as _,
        Port::E => crate::pac::GPIOE::ptr() as _,
        Port::F => crate::pac::GPIOF::ptr() as _,
        Port::G => crate::pac::GPIOG::ptr() as _,
        Port::H => crate::pac::GPIOH::ptr() as _,
        Port::I => crate::pac::GPIOI::ptr() as _,
    }
}