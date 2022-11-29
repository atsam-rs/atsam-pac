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
    #[doc = "0x20 - Oscillator 32 Control Register"]
    pub oscctrl32: OSCCTRL32,
    #[doc = "0x24 - 32 kHz RC Oscillator Control Register"]
    pub rc32kcr: RC32KCR,
    #[doc = "0x28 - 32kHz RC Oscillator Tuning Register"]
    pub rc32ktune: RC32KTUNE,
    #[doc = "0x2c - BOD33 Control Register"]
    pub bod33ctrl: BOD33CTRL,
    #[doc = "0x30 - BOD33 Level Register"]
    pub bod33level: BOD33LEVEL,
    #[doc = "0x34 - BOD33 Sampling Control Register"]
    pub bod33sampling: BOD33SAMPLING,
    #[doc = "0x38 - BOD18 Control Register"]
    pub bod18ctrl: BOD18CTRL,
    #[doc = "0x3c - BOD18 Level Register"]
    pub bod18level: BOD18LEVEL,
    _reserved16: [u8; 0x04],
    #[doc = "0x44 - Voltage Regulator Configuration Register"]
    pub vregcr: VREGCR,
    _reserved17: [u8; 0x04],
    #[doc = "0x4c - Normal Mode Control and Status Register"]
    pub vregncsr: VREGNCSR,
    #[doc = "0x50 - LP Mode Control and Status Register"]
    pub vreglpcsr: VREGLPCSR,
    _reserved19: [u8; 0x04],
    #[doc = "0x58 - 1MHz RC Clock Configuration Register"]
    pub rc1mcr: RC1MCR,
    #[doc = "0x5c - Bandgap Calibration Register"]
    pub bgcr: BGCR,
    #[doc = "0x60 - Bandgap Control Register"]
    pub bgctrl: BGCTRL,
    #[doc = "0x64 - Bandgap Status Register"]
    pub bgsr: BGSR,
    _reserved23: [u8; 0x10],
    #[doc = "0x78..0x88 - Backup Register"]
    pub br: [BR; 4],
    _reserved24: [u8; 0x035c],
    #[doc = "0x3e4 - Backup Register Interface Version Register"]
    pub brifbversion: BRIFBVERSION,
    #[doc = "0x3e8 - BGREFIFB Version Register"]
    pub bgrefifbversion: BGREFIFBVERSION,
    #[doc = "0x3ec - VREGIFA Version Register"]
    pub vregifgversion: VREGIFGVERSION,
    #[doc = "0x3f0 - BODIFC Version Register"]
    pub bodifcversion: BODIFCVERSION,
    #[doc = "0x3f4 - 32 kHz RC Oscillator Version Register"]
    pub rc32kifbversion: RC32KIFBVERSION,
    #[doc = "0x3f8 - 32 KHz Oscillator Version Register"]
    pub osc32ifaversion: OSC32IFAVERSION,
    #[doc = "0x3fc - BSCIF Version Register"]
    pub version: VERSION,
}
#[doc = "BGCR (rw) register accessor: an alias for `Reg<BGCR_SPEC>`"]
pub type BGCR = crate::Reg<bgcr::BGCR_SPEC>;
#[doc = "Bandgap Calibration Register"]
pub mod bgcr;
#[doc = "BGCTRL (rw) register accessor: an alias for `Reg<BGCTRL_SPEC>`"]
pub type BGCTRL = crate::Reg<bgctrl::BGCTRL_SPEC>;
#[doc = "Bandgap Control Register"]
pub mod bgctrl;
#[doc = "BGREFIFBVERSION (r) register accessor: an alias for `Reg<BGREFIFBVERSION_SPEC>`"]
pub type BGREFIFBVERSION = crate::Reg<bgrefifbversion::BGREFIFBVERSION_SPEC>;
#[doc = "BGREFIFB Version Register"]
pub mod bgrefifbversion;
#[doc = "BGSR (r) register accessor: an alias for `Reg<BGSR_SPEC>`"]
pub type BGSR = crate::Reg<bgsr::BGSR_SPEC>;
#[doc = "Bandgap Status Register"]
pub mod bgsr;
#[doc = "BODIFCVERSION (r) register accessor: an alias for `Reg<BODIFCVERSION_SPEC>`"]
pub type BODIFCVERSION = crate::Reg<bodifcversion::BODIFCVERSION_SPEC>;
#[doc = "BODIFC Version Register"]
pub mod bodifcversion;
#[doc = "BOD18CTRL (rw) register accessor: an alias for `Reg<BOD18CTRL_SPEC>`"]
pub type BOD18CTRL = crate::Reg<bod18ctrl::BOD18CTRL_SPEC>;
#[doc = "BOD18 Control Register"]
pub mod bod18ctrl;
#[doc = "BOD18LEVEL (rw) register accessor: an alias for `Reg<BOD18LEVEL_SPEC>`"]
pub type BOD18LEVEL = crate::Reg<bod18level::BOD18LEVEL_SPEC>;
#[doc = "BOD18 Level Register"]
pub mod bod18level;
#[doc = "BOD33CTRL (rw) register accessor: an alias for `Reg<BOD33CTRL_SPEC>`"]
pub type BOD33CTRL = crate::Reg<bod33ctrl::BOD33CTRL_SPEC>;
#[doc = "BOD33 Control Register"]
pub mod bod33ctrl;
#[doc = "BOD33LEVEL (rw) register accessor: an alias for `Reg<BOD33LEVEL_SPEC>`"]
pub type BOD33LEVEL = crate::Reg<bod33level::BOD33LEVEL_SPEC>;
#[doc = "BOD33 Level Register"]
pub mod bod33level;
#[doc = "BOD33SAMPLING (rw) register accessor: an alias for `Reg<BOD33SAMPLING_SPEC>`"]
pub type BOD33SAMPLING = crate::Reg<bod33sampling::BOD33SAMPLING_SPEC>;
#[doc = "BOD33 Sampling Control Register"]
pub mod bod33sampling;
#[doc = "BR (rw) register accessor: an alias for `Reg<BR_SPEC>`"]
pub type BR = crate::Reg<br::BR_SPEC>;
#[doc = "Backup Register"]
pub mod br;
#[doc = "BRIFBVERSION (r) register accessor: an alias for `Reg<BRIFBVERSION_SPEC>`"]
pub type BRIFBVERSION = crate::Reg<brifbversion::BRIFBVERSION_SPEC>;
#[doc = "Backup Register Interface Version Register"]
pub mod brifbversion;
#[doc = "CSCR (rw) register accessor: an alias for `Reg<CSCR_SPEC>`"]
pub type CSCR = crate::Reg<cscr::CSCR_SPEC>;
#[doc = "Chip Specific Configuration Register"]
pub mod cscr;
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
#[doc = "OSCCTRL32 (rw) register accessor: an alias for `Reg<OSCCTRL32_SPEC>`"]
pub type OSCCTRL32 = crate::Reg<oscctrl32::OSCCTRL32_SPEC>;
#[doc = "Oscillator 32 Control Register"]
pub mod oscctrl32;
#[doc = "OSC32IFAVERSION (r) register accessor: an alias for `Reg<OSC32IFAVERSION_SPEC>`"]
pub type OSC32IFAVERSION = crate::Reg<osc32ifaversion::OSC32IFAVERSION_SPEC>;
#[doc = "32 KHz Oscillator Version Register"]
pub mod osc32ifaversion;
#[doc = "PCLKSR (r) register accessor: an alias for `Reg<PCLKSR_SPEC>`"]
pub type PCLKSR = crate::Reg<pclksr::PCLKSR_SPEC>;
#[doc = "Power and Clocks Status Register"]
pub mod pclksr;
#[doc = "RC1MCR (rw) register accessor: an alias for `Reg<RC1MCR_SPEC>`"]
pub type RC1MCR = crate::Reg<rc1mcr::RC1MCR_SPEC>;
#[doc = "1MHz RC Clock Configuration Register"]
pub mod rc1mcr;
#[doc = "RC32KCR (rw) register accessor: an alias for `Reg<RC32KCR_SPEC>`"]
pub type RC32KCR = crate::Reg<rc32kcr::RC32KCR_SPEC>;
#[doc = "32 kHz RC Oscillator Control Register"]
pub mod rc32kcr;
#[doc = "RC32KIFBVERSION (r) register accessor: an alias for `Reg<RC32KIFBVERSION_SPEC>`"]
pub type RC32KIFBVERSION = crate::Reg<rc32kifbversion::RC32KIFBVERSION_SPEC>;
#[doc = "32 kHz RC Oscillator Version Register"]
pub mod rc32kifbversion;
#[doc = "RC32KTUNE (rw) register accessor: an alias for `Reg<RC32KTUNE_SPEC>`"]
pub type RC32KTUNE = crate::Reg<rc32ktune::RC32KTUNE_SPEC>;
#[doc = "32kHz RC Oscillator Tuning Register"]
pub mod rc32ktune;
#[doc = "UNLOCK (w) register accessor: an alias for `Reg<UNLOCK_SPEC>`"]
pub type UNLOCK = crate::Reg<unlock::UNLOCK_SPEC>;
#[doc = "Unlock Register"]
pub mod unlock;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "BSCIF Version Register"]
pub mod version;
#[doc = "VREGCR (rw) register accessor: an alias for `Reg<VREGCR_SPEC>`"]
pub type VREGCR = crate::Reg<vregcr::VREGCR_SPEC>;
#[doc = "Voltage Regulator Configuration Register"]
pub mod vregcr;
#[doc = "VREGIFGVERSION (r) register accessor: an alias for `Reg<VREGIFGVERSION_SPEC>`"]
pub type VREGIFGVERSION = crate::Reg<vregifgversion::VREGIFGVERSION_SPEC>;
#[doc = "VREGIFA Version Register"]
pub mod vregifgversion;
#[doc = "VREGLPCSR (rw) register accessor: an alias for `Reg<VREGLPCSR_SPEC>`"]
pub type VREGLPCSR = crate::Reg<vreglpcsr::VREGLPCSR_SPEC>;
#[doc = "LP Mode Control and Status Register"]
pub mod vreglpcsr;
#[doc = "VREGNCSR (rw) register accessor: an alias for `Reg<VREGNCSR_SPEC>`"]
pub type VREGNCSR = crate::Reg<vregncsr::VREGNCSR_SPEC>;
#[doc = "Normal Mode Control and Status Register"]
pub mod vregncsr;
