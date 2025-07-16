use crate::pac::{RCC, PWR, FLASH};
use core::marker::PhantomData;

// System Clock type states
pub struct SourceHSI16;
pub struct SourceMSI;
pub struct SourceHSE;

pub struct PLLDisabled;
pub struct PLLEnabled;

// Voltage Range
// Range   |  MSI  | HSI16 |  HSE  | PLL/PLLSAI1/PLLSAI2
// Range 1 | 48MHz | 16MHz | 48MHz | 80MHz
// Range 2 | 24MHz | 16MHz | 26MHz | 26MHz
#[derive(PartialEq)]
pub enum VoltageRange {
    VRange1Boost = 0b00,
    VRange1 = 0b01,
    VRange2 = 0b10,
}

pub enum FlashLatency {
    Latency0 = 0b000,
    Latency1 = 0b001,
    Latency2 = 0b010,
    Latency3 = 0b011,
    Latency4 = 0b100,
}

// MSI Range
#[derive(Clone)]
pub enum MSIRange {
    Range0 = 0b0000,     // 100kHz
    Range1 = 0b0001,     // 200kHz
    Range2 = 0b0010,     // 400kHz
    Range3 = 0b0011,     // 800kHz
    Range4 = 0b0100,     // 1MHz
    Range5 = 0b0101,     // 2MHz
    Range6 = 0b0110,     // 4MHz
    Range7 = 0b0111,     // 8MHz
    Range8 = 0b1000,     // 16MHz
    Range9 = 0b1001,     // 24MHz
    Range10 = 0b1010,    // 32MHz
    Range11 = 0b1011,    // 48MHz
}

// PLL Configure
pub struct PLLConfig {
    PLLPDIV: u8,
    PLLR: u8,
    PLLQ: u8,
    PLLQEN: bool,
    PLLP: bool,
    PLLPEN: bool,
    PLLN: u8,
    PLLM: u8,
}

pub struct ClockManager<SOURCE, PLL> {
    pub sys_clock: u32,
    msi_range: MSIRange,
    source: SOURCE,
    pllenabled: PLL,
    //vrange: PhantomData<VRANGE>,
    //source: PhantomData<SOURCE>,
    //pllenabled: PhantomData<PLL>,
}

impl ClockManager<SourceMSI, PLLDisabled> {
    pub fn new() -> ClockManager<SourceMSI, PLLDisabled> {
        // System clock starts at 4MHz with MSI on reset

        ClockManager { 
            sys_clock: 4_000_000, 
            msi_range: MSIRange::Range4,  
            source: SourceMSI, 
            pllenabled: PLLDisabled 
        }
        // let result = ClockManager {
        //     sys_clock: 4_000_000,
        //     msi_range: MSIRange::Range4,
        //     //vrange: VRange1,
        //     //source: SourceMSI,
        //     //pllenabled: PLLDisabled,
        // };
        // result
    }
}

impl<PLL> ClockManager<SourceMSI, PLL> {
    pub fn update_msi_range(&mut self, new_range: MSIRange) {
        let rcc = unsafe { &(*RCC::ptr()) };
        // NOTE: MSIRANGE can only be modified when MSI is OFF or when MSI is ready
        // Not when MSI is ON but not ready

        // Confirm that MSI is OFF or (MSI is ON and MSI is RDY)
        let msion = rcc.cr().read().msion().bit();
        let msirdy = rcc.cr().read().msirdy().bit();
        if msion && !msirdy {
            // Can't change MSI range right now
            // TODO: Figure out what to do here...
            // Maybe return a Result here
        }

        // Need to update the flash wait states
        // Enable the PWR clock if not already
        if rcc.apb1enr1().read().pwren().bit_is_clear() {
            rcc.apb1enr1().modify(|_,w| w.pwren().set_bit());
            // Need a delay after enabling 
            while rcc.apb1enr1().read().pwren().bit_is_clear() {

            }
        }
        let pwr = unsafe { &(*PWR::ptr()) };
        let curr_vos;
        if pwr.cr1().read().vos() == VoltageRange::VRange2 as u8 {
            curr_vos = VoltageRange::VRange2;
        } else {
            curr_vos = VoltageRange::VRange1;
        }
        // L496 doesn't have boost range
        // Set flash states based on the MSI range
        let new_latency: FlashLatency;
        if curr_vos == VoltageRange::VRange1 {
            match new_range {
                MSIRange::Range0 => new_latency = FlashLatency::Latency0,
                MSIRange::Range1 => new_latency = FlashLatency::Latency0,
                MSIRange::Range2 => new_latency = FlashLatency::Latency0,
                MSIRange::Range3 => new_latency = FlashLatency::Latency0,
                MSIRange::Range4 => new_latency = FlashLatency::Latency0,
                MSIRange::Range5 => new_latency = FlashLatency::Latency0,
                MSIRange::Range6 => new_latency = FlashLatency::Latency0,
                MSIRange::Range7 => new_latency = FlashLatency::Latency0,
                MSIRange::Range8 => new_latency = FlashLatency::Latency0,
                MSIRange::Range9 => new_latency = FlashLatency::Latency1,
                MSIRange::Range10 => new_latency = FlashLatency::Latency1,
                MSIRange::Range11 => new_latency = FlashLatency::Latency2,
            };
        } else {
            match new_range {
                MSIRange::Range0 => new_latency = FlashLatency::Latency0,
                MSIRange::Range1 => new_latency = FlashLatency::Latency0,
                MSIRange::Range2 => new_latency = FlashLatency::Latency0,
                MSIRange::Range3 => new_latency = FlashLatency::Latency0,
                MSIRange::Range4 => new_latency = FlashLatency::Latency0,
                MSIRange::Range5 => new_latency = FlashLatency::Latency0,
                MSIRange::Range6 => new_latency = FlashLatency::Latency0,
                MSIRange::Range7 => new_latency = FlashLatency::Latency1,
                MSIRange::Range8 => new_latency = FlashLatency::Latency2,
                MSIRange::Range9 => new_latency = FlashLatency::Latency2,
                MSIRange::Range10 => new_latency = FlashLatency::Latency2,
                MSIRange::Range11 => new_latency = FlashLatency::Latency2,
            };
        }
        let flash = unsafe { &(*FLASH::ptr()) };
        flash.acr().modify(|_,w| unsafe { w.latency().bits(new_latency as u8) });

        self.msi_range = new_range.clone();

        // Set the MSIRGSEL to set range based on the CR value
        rcc.cr().modify(|_,w| w.msirgsel().set_bit());

        // Update the MSI range
        rcc.cr().modify(|_,w| unsafe { w.msirange().bits(new_range as u8) });

        while rcc.cr().read().msirdy().bit_is_clear() {

        }
    }

    // Function to enable the PLL with the given config values
    // This probably can be a common function for all states
    pub fn enable_pll(self) {
        // Configure the PLL


        // Need to make sure the source clock is ready

        // Enable the PLL 
    }

    pub fn switch_to_hsi(self) -> ClockManager<SourceHSI16, PLL> {
        let rcc = unsafe { &(*RCC::ptr()) };

        // First turn on the HSI16
        rcc.cr().modify(|_,w| w.hsion().set_bit());

        // Wait for HSI ready
        while rcc.cr().read().hsirdy().bit_is_clear() {
            // TODO: Consider adding a timeout here
        }

        // Switch to the HSI
        rcc.cfgr().modify( unsafe { |_,w| w.sw().bits(0b01) } );

        // Wait for system clock switch status
        while rcc.cfgr().read().sws().bits() != 0b01 {
            // TODO: Consider adding a timeout here
        }

        // Turn off the MSI if not used
        // TODO: Put the clock sources in a counting ref or something
        rcc.cr().modify(|_,w| w.msion().clear_bit());

        let result = ClockManager {
            sys_clock: 16_000_000,
            msi_range: self.msi_range,
            // vrange: self.vrange,
            source: SourceHSI16,
            pllenabled: self.pllenabled,
        };
        result
    }
}