#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x04 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x08 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x0c - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x10 - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x14 - Power and Clocks Status Register"]
    pub pclksr: PCLKSR,
    #[doc = "0x18 - Unlock Register"]
    pub unlock: UNLOCK,
    #[doc = "0x1c - Chip Specific Configuration Register"]
    pub cscr: CSCR,
    #[doc = "0x20 - Oscillator Control Register"]
    pub oscctrl0: OSCCTRL0,
    #[doc = "0x24 - PLL0 Control Register"]
    pub pll: PLL,
    #[doc = "0x28 - DFLL0 Config Register"]
    pub dfll0conf: DFLL0CONF,
    #[doc = "0x2c - DFLL Value Register"]
    pub dfll0val: DFLL0VAL,
    #[doc = "0x30 - DFLL0 Multiplier Register"]
    pub dfll0mul: DFLL0MUL,
    #[doc = "0x34 - DFLL0 Step Register"]
    pub dfll0step: DFLL0STEP,
    #[doc = "0x38 - DFLL0 Spread Spectrum Generator Control Register"]
    pub dfll0ssg: DFLL0SSG,
    #[doc = "0x3c - DFLL0 Ratio Registe"]
    pub dfll0ratio: DFLL0RATIO,
    #[doc = "0x40 - DFLL0 Synchronization Register"]
    pub dfll0sync: DFLL0SYNC,
    #[doc = "0x44 - System RC Oscillator Calibration Register"]
    pub rccr: RCCR,
    #[doc = "0x48 - 4/8/12 MHz RC Oscillator Configuration Register"]
    pub rcfastcfg: RCFASTCFG,
    #[doc = "0x4c - 4/8/12 MHz RC Oscillator Status Register"]
    pub rcfastsr: RCFASTSR,
    #[doc = "0x50 - 80 MHz RC Oscillator Register"]
    pub rc80mcr: RC80MCR,
    _reserved21: [u8; 0x10],
    #[doc = "0x64 - High Resolution Prescaler Control Register"]
    pub hrpcr: HRPCR,
    #[doc = "0x68 - Fractional Prescaler Control Register"]
    pub fpcr: FPCR,
    #[doc = "0x6c - Fractional Prescaler Multiplier Register"]
    pub fpmul: FPMUL,
    #[doc = "0x70 - Fractional Prescaler DIVIDER Register"]
    pub fpdiv: FPDIV,
    #[doc = "0x74..0xa4 - Generic Clock Control"]
    pub gcctrl: [GCCTRL; 12],
    _reserved26: [u8; 0x0334],
    #[doc = "0x3d8 - 4/8/12 MHz RC Oscillator Version Register"]
    pub rcfastversion: RCFASTVERSION,
    #[doc = "0x3dc - Generic Clock Prescaler Version Register"]
    pub gclkprescversion: GCLKPRESCVERSION,
    #[doc = "0x3e0 - PLL Version Register"]
    pub pllifaversion: PLLIFAVERSION,
    #[doc = "0x3e4 - Oscillator 0 Version Register"]
    pub oscifaversion: OSCIFAVERSION,
    #[doc = "0x3e8 - DFLL Version Register"]
    pub dfllifbversion: DFLLIFBVERSION,
    #[doc = "0x3ec - System RC Oscillator Version Register"]
    pub rcoscifaversion: RCOSCIFAVERSION,
    #[doc = "0x3f0 - Frequency Locked Oscillator Version Register"]
    pub floversion: FLOVERSION,
    #[doc = "0x3f4 - 80MHz RC Oscillator Version Register"]
    pub rc80mversion: RC80MVERSION,
    #[doc = "0x3f8 - Generic Clock Version Register"]
    pub gclkifversion: GCLKIFVERSION,
    #[doc = "0x3fc - SCIF Version Register"]
    pub version: VERSION,
}
#[doc = "CSCR (rw) register accessor: an alias for `Reg<CSCR_SPEC>`"]
pub type CSCR = crate::Reg<cscr::CSCR_SPEC>;
#[doc = "Chip Specific Configuration Register"]
pub mod cscr;
#[doc = "DFLLIFBVERSION (r) register accessor: an alias for `Reg<DFLLIFBVERSION_SPEC>`"]
pub type DFLLIFBVERSION = crate::Reg<dfllifbversion::DFLLIFBVERSION_SPEC>;
#[doc = "DFLL Version Register"]
pub mod dfllifbversion;
#[doc = "DFLL0CONF (rw) register accessor: an alias for `Reg<DFLL0CONF_SPEC>`"]
pub type DFLL0CONF = crate::Reg<dfll0conf::DFLL0CONF_SPEC>;
#[doc = "DFLL0 Config Register"]
pub mod dfll0conf;
#[doc = "DFLL0MUL (rw) register accessor: an alias for `Reg<DFLL0MUL_SPEC>`"]
pub type DFLL0MUL = crate::Reg<dfll0mul::DFLL0MUL_SPEC>;
#[doc = "DFLL0 Multiplier Register"]
pub mod dfll0mul;
#[doc = "DFLL0RATIO (r) register accessor: an alias for `Reg<DFLL0RATIO_SPEC>`"]
pub type DFLL0RATIO = crate::Reg<dfll0ratio::DFLL0RATIO_SPEC>;
#[doc = "DFLL0 Ratio Registe"]
pub mod dfll0ratio;
#[doc = "DFLL0SSG (rw) register accessor: an alias for `Reg<DFLL0SSG_SPEC>`"]
pub type DFLL0SSG = crate::Reg<dfll0ssg::DFLL0SSG_SPEC>;
#[doc = "DFLL0 Spread Spectrum Generator Control Register"]
pub mod dfll0ssg;
#[doc = "DFLL0STEP (rw) register accessor: an alias for `Reg<DFLL0STEP_SPEC>`"]
pub type DFLL0STEP = crate::Reg<dfll0step::DFLL0STEP_SPEC>;
#[doc = "DFLL0 Step Register"]
pub mod dfll0step;
#[doc = "DFLL0SYNC (w) register accessor: an alias for `Reg<DFLL0SYNC_SPEC>`"]
pub type DFLL0SYNC = crate::Reg<dfll0sync::DFLL0SYNC_SPEC>;
#[doc = "DFLL0 Synchronization Register"]
pub mod dfll0sync;
#[doc = "DFLL0VAL (rw) register accessor: an alias for `Reg<DFLL0VAL_SPEC>`"]
pub type DFLL0VAL = crate::Reg<dfll0val::DFLL0VAL_SPEC>;
#[doc = "DFLL Value Register"]
pub mod dfll0val;
#[doc = "FLOVERSION (r) register accessor: an alias for `Reg<FLOVERSION_SPEC>`"]
pub type FLOVERSION = crate::Reg<floversion::FLOVERSION_SPEC>;
#[doc = "Frequency Locked Oscillator Version Register"]
pub mod floversion;
#[doc = "FPCR (rw) register accessor: an alias for `Reg<FPCR_SPEC>`"]
pub type FPCR = crate::Reg<fpcr::FPCR_SPEC>;
#[doc = "Fractional Prescaler Control Register"]
pub mod fpcr;
#[doc = "FPDIV (rw) register accessor: an alias for `Reg<FPDIV_SPEC>`"]
pub type FPDIV = crate::Reg<fpdiv::FPDIV_SPEC>;
#[doc = "Fractional Prescaler DIVIDER Register"]
pub mod fpdiv;
#[doc = "FPMUL (rw) register accessor: an alias for `Reg<FPMUL_SPEC>`"]
pub type FPMUL = crate::Reg<fpmul::FPMUL_SPEC>;
#[doc = "Fractional Prescaler Multiplier Register"]
pub mod fpmul;
#[doc = "GCCTRL (rw) register accessor: an alias for `Reg<GCCTRL_SPEC>`"]
pub type GCCTRL = crate::Reg<gcctrl::GCCTRL_SPEC>;
#[doc = "Generic Clock Control"]
pub mod gcctrl;
#[doc = "GCLKIFVERSION (r) register accessor: an alias for `Reg<GCLKIFVERSION_SPEC>`"]
pub type GCLKIFVERSION = crate::Reg<gclkifversion::GCLKIFVERSION_SPEC>;
#[doc = "Generic Clock Version Register"]
pub mod gclkifversion;
#[doc = "GCLKPRESCVERSION (r) register accessor: an alias for `Reg<GCLKPRESCVERSION_SPEC>`"]
pub type GCLKPRESCVERSION = crate::Reg<gclkprescversion::GCLKPRESCVERSION_SPEC>;
#[doc = "Generic Clock Prescaler Version Register"]
pub mod gclkprescversion;
#[doc = "HRPCR (rw) register accessor: an alias for `Reg<HRPCR_SPEC>`"]
pub type HRPCR = crate::Reg<hrpcr::HRPCR_SPEC>;
#[doc = "High Resolution Prescaler Control Register"]
pub mod hrpcr;
#[doc = "ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "OSCCTRL0 (rw) register accessor: an alias for `Reg<OSCCTRL0_SPEC>`"]
pub type OSCCTRL0 = crate::Reg<oscctrl0::OSCCTRL0_SPEC>;
#[doc = "Oscillator Control Register"]
pub mod oscctrl0;
#[doc = "OSCIFAVERSION (r) register accessor: an alias for `Reg<OSCIFAVERSION_SPEC>`"]
pub type OSCIFAVERSION = crate::Reg<oscifaversion::OSCIFAVERSION_SPEC>;
#[doc = "Oscillator 0 Version Register"]
pub mod oscifaversion;
#[doc = "PCLKSR (r) register accessor: an alias for `Reg<PCLKSR_SPEC>`"]
pub type PCLKSR = crate::Reg<pclksr::PCLKSR_SPEC>;
#[doc = "Power and Clocks Status Register"]
pub mod pclksr;
#[doc = "PLLIFAVERSION (r) register accessor: an alias for `Reg<PLLIFAVERSION_SPEC>`"]
pub type PLLIFAVERSION = crate::Reg<pllifaversion::PLLIFAVERSION_SPEC>;
#[doc = "PLL Version Register"]
pub mod pllifaversion;
#[doc = "PLL (rw) register accessor: an alias for `Reg<PLL_SPEC>`"]
pub type PLL = crate::Reg<pll::PLL_SPEC>;
#[doc = "PLL0 Control Register"]
pub mod pll;
#[doc = "RCCR (rw) register accessor: an alias for `Reg<RCCR_SPEC>`"]
pub type RCCR = crate::Reg<rccr::RCCR_SPEC>;
#[doc = "System RC Oscillator Calibration Register"]
pub mod rccr;
#[doc = "RCFASTCFG (rw) register accessor: an alias for `Reg<RCFASTCFG_SPEC>`"]
pub type RCFASTCFG = crate::Reg<rcfastcfg::RCFASTCFG_SPEC>;
#[doc = "4/8/12 MHz RC Oscillator Configuration Register"]
pub mod rcfastcfg;
#[doc = "RCFASTSR (rw) register accessor: an alias for `Reg<RCFASTSR_SPEC>`"]
pub type RCFASTSR = crate::Reg<rcfastsr::RCFASTSR_SPEC>;
#[doc = "4/8/12 MHz RC Oscillator Status Register"]
pub mod rcfastsr;
#[doc = "RCFASTVERSION (r) register accessor: an alias for `Reg<RCFASTVERSION_SPEC>`"]
pub type RCFASTVERSION = crate::Reg<rcfastversion::RCFASTVERSION_SPEC>;
#[doc = "4/8/12 MHz RC Oscillator Version Register"]
pub mod rcfastversion;
#[doc = "RCOSCIFAVERSION (r) register accessor: an alias for `Reg<RCOSCIFAVERSION_SPEC>`"]
pub type RCOSCIFAVERSION = crate::Reg<rcoscifaversion::RCOSCIFAVERSION_SPEC>;
#[doc = "System RC Oscillator Version Register"]
pub mod rcoscifaversion;
#[doc = "RC80MCR (rw) register accessor: an alias for `Reg<RC80MCR_SPEC>`"]
pub type RC80MCR = crate::Reg<rc80mcr::RC80MCR_SPEC>;
#[doc = "80 MHz RC Oscillator Register"]
pub mod rc80mcr;
#[doc = "RC80MVERSION (r) register accessor: an alias for `Reg<RC80MVERSION_SPEC>`"]
pub type RC80MVERSION = crate::Reg<rc80mversion::RC80MVERSION_SPEC>;
#[doc = "80MHz RC Oscillator Version Register"]
pub mod rc80mversion;
#[doc = "UNLOCK (w) register accessor: an alias for `Reg<UNLOCK_SPEC>`"]
pub type UNLOCK = crate::Reg<unlock::UNLOCK_SPEC>;
#[doc = "Unlock Register"]
pub mod unlock;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "SCIF Version Register"]
pub mod version;
