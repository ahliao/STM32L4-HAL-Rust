use crate::pac::RCC;

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
pub struct VRange1;
pub struct VRange2;

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

pub struct ClockManager<VRANGE, SOURCE, PLL> {
    pub sys_clock: u32,
    msi_range: MSIRange,
    vrange: VRANGE,
    source: SOURCE,
    pllenabled: PLL,
}

impl<VRANGE, SOURCE, PLL> ClockManager<VRANGE, SOURCE, PLL> {
    pub fn new() -> ClockManager<VRange1, SourceMSI, PLLDisabled> {
        // System clock starts at 4MHz with MSI on reset


        let result = ClockManager {
            sys_clock: 4_000_000,
            msi_range: MSIRange::Range4,
            vrange: VRange1,
            source: SourceMSI,
            pllenabled: PLLDisabled,
        };
        result
    }
}

impl<VRANGE, PLL> ClockManager<VRANGE, SourceMSI, PLL> {
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

        self.msi_range = new_range.clone();

        // Update the MSI range
        rcc.cr().modify(|_,w| unsafe { w.msirange().bits(new_range as u8) });
    }

    // Function to enable the PLL with the given config values
    // This probably can be a common function for all states
    pub fn enable_pll(self) {
        // Configure the PLL


        // Need to make sure the source clock is ready

        // Enable the PLL 
    }

    pub fn switch_to_hsi(self) -> ClockManager<VRANGE, SourceHSI16, PLL> {
        let result = ClockManager {
            sys_clock: 16_000_000,
            msi_range: self.msi_range,
            vrange: self.vrange,
            source: SourceHSI16,
            pllenabled: self.pllenabled,
        };
        result
    }
}