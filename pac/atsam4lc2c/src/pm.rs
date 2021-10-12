#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Main Clock Control"]
    pub mcctrl: crate::Reg<mcctrl::MCCTRL_SPEC>,
    #[doc = "0x04 - CPU Clock Select"]
    pub cpusel: crate::Reg<cpusel::CPUSEL_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - PBA Clock Select"]
    pub pbasel: crate::Reg<pbasel::PBASEL_SPEC>,
    #[doc = "0x10 - PBB Clock Select"]
    pub pbbsel: crate::Reg<pbbsel::PBBSEL_SPEC>,
    #[doc = "0x14 - PBC Clock Select"]
    pub pbcsel: crate::Reg<pbcsel::PBCSEL_SPEC>,
    #[doc = "0x18 - PBD Clock Select"]
    pub pbdsel: crate::Reg<pbdsel::PBDSEL_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - CPU Mask"]
    pub cpumask: crate::Reg<cpumask::CPUMASK_SPEC>,
    #[doc = "0x24 - HSB Mask"]
    pub hsbmask: crate::Reg<hsbmask::HSBMASK_SPEC>,
    #[doc = "0x28 - PBA Mask"]
    pub pbamask: crate::Reg<pbamask::PBAMASK_SPEC>,
    #[doc = "0x2c - PBB Mask"]
    pub pbbmask: crate::Reg<pbbmask::PBBMASK_SPEC>,
    #[doc = "0x30 - PBC Mask"]
    pub pbcmask: crate::Reg<pbcmask::PBCMASK_SPEC>,
    #[doc = "0x34 - PBD Mask"]
    pub pbdmask: crate::Reg<pbdmask::PBDMASK_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x40 - PBA Divided Clock Mask"]
    pub pbadivmask: crate::Reg<pbadivmask::PBADIVMASK_SPEC>,
    _reserved13: [u8; 0x10],
    #[doc = "0x54 - Clock Failure Detector Control"]
    pub cfdctrl: crate::Reg<cfdctrl::CFDCTRL_SPEC>,
    #[doc = "0x58 - Unlock Register"]
    pub unlock: crate::Reg<unlock::UNLOCK_SPEC>,
    _reserved15: [u8; 0x64],
    #[doc = "0xc0 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0xc4 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0xc8 - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0xcc - Interrupt Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0xd0 - Interrupt Clear Register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0xd4 - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    _reserved21: [u8; 0x88],
    #[doc = "0x160 - Peripheral Power Control Register"]
    pub ppcr: crate::Reg<ppcr::PPCR_SPEC>,
    _reserved22: [u8; 0x1c],
    #[doc = "0x180 - Reset Cause Register"]
    pub rcause: crate::Reg<rcause::RCAUSE_SPEC>,
    #[doc = "0x184 - Wake Cause Register"]
    pub wcause: crate::Reg<wcause::WCAUSE_SPEC>,
    #[doc = "0x188 - Asynchronous Wake Enable"]
    pub awen: crate::Reg<awen::AWEN_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0x190 - Obsvervability"]
    pub obs: crate::Reg<obs::OBS_SPEC>,
    #[doc = "0x194 - Fast Sleep Register"]
    pub fastsleep: crate::Reg<fastsleep::FASTSLEEP_SPEC>,
    _reserved27: [u8; 0x0260],
    #[doc = "0x3f8 - Configuration Register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    #[doc = "0x3fc - Version Register"]
    pub version: crate::Reg<version::VERSION_SPEC>,
}
#[doc = "AWEN register accessor: an alias for `Reg<AWEN_SPEC>`"]
pub type AWEN = crate::Reg<awen::AWEN_SPEC>;
#[doc = "Asynchronous Wake Enable"]
pub mod awen;
#[doc = "CFDCTRL register accessor: an alias for `Reg<CFDCTRL_SPEC>`"]
pub type CFDCTRL = crate::Reg<cfdctrl::CFDCTRL_SPEC>;
#[doc = "Clock Failure Detector Control"]
pub mod cfdctrl;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration Register"]
pub mod config;
#[doc = "CPUMASK register accessor: an alias for `Reg<CPUMASK_SPEC>`"]
pub type CPUMASK = crate::Reg<cpumask::CPUMASK_SPEC>;
#[doc = "CPU Mask"]
pub mod cpumask;
#[doc = "CPUSEL register accessor: an alias for `Reg<CPUSEL_SPEC>`"]
pub type CPUSEL = crate::Reg<cpusel::CPUSEL_SPEC>;
#[doc = "CPU Clock Select"]
pub mod cpusel;
#[doc = "FASTSLEEP register accessor: an alias for `Reg<FASTSLEEP_SPEC>`"]
pub type FASTSLEEP = crate::Reg<fastsleep::FASTSLEEP_SPEC>;
#[doc = "Fast Sleep Register"]
pub mod fastsleep;
#[doc = "HSBMASK register accessor: an alias for `Reg<HSBMASK_SPEC>`"]
pub type HSBMASK = crate::Reg<hsbmask::HSBMASK_SPEC>;
#[doc = "HSB Mask"]
pub mod hsbmask;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "MCCTRL register accessor: an alias for `Reg<MCCTRL_SPEC>`"]
pub type MCCTRL = crate::Reg<mcctrl::MCCTRL_SPEC>;
#[doc = "Main Clock Control"]
pub mod mcctrl;
#[doc = "OBS register accessor: an alias for `Reg<OBS_SPEC>`"]
pub type OBS = crate::Reg<obs::OBS_SPEC>;
#[doc = "Obsvervability"]
pub mod obs;
#[doc = "PBADIVMASK register accessor: an alias for `Reg<PBADIVMASK_SPEC>`"]
pub type PBADIVMASK = crate::Reg<pbadivmask::PBADIVMASK_SPEC>;
#[doc = "PBA Divided Clock Mask"]
pub mod pbadivmask;
#[doc = "PBAMASK register accessor: an alias for `Reg<PBAMASK_SPEC>`"]
pub type PBAMASK = crate::Reg<pbamask::PBAMASK_SPEC>;
#[doc = "PBA Mask"]
pub mod pbamask;
#[doc = "PBASEL register accessor: an alias for `Reg<PBASEL_SPEC>`"]
pub type PBASEL = crate::Reg<pbasel::PBASEL_SPEC>;
#[doc = "PBA Clock Select"]
pub mod pbasel;
#[doc = "PBBMASK register accessor: an alias for `Reg<PBBMASK_SPEC>`"]
pub type PBBMASK = crate::Reg<pbbmask::PBBMASK_SPEC>;
#[doc = "PBB Mask"]
pub mod pbbmask;
#[doc = "PBBSEL register accessor: an alias for `Reg<PBBSEL_SPEC>`"]
pub type PBBSEL = crate::Reg<pbbsel::PBBSEL_SPEC>;
#[doc = "PBB Clock Select"]
pub mod pbbsel;
#[doc = "PBCMASK register accessor: an alias for `Reg<PBCMASK_SPEC>`"]
pub type PBCMASK = crate::Reg<pbcmask::PBCMASK_SPEC>;
#[doc = "PBC Mask"]
pub mod pbcmask;
#[doc = "PBCSEL register accessor: an alias for `Reg<PBCSEL_SPEC>`"]
pub type PBCSEL = crate::Reg<pbcsel::PBCSEL_SPEC>;
#[doc = "PBC Clock Select"]
pub mod pbcsel;
#[doc = "PBDMASK register accessor: an alias for `Reg<PBDMASK_SPEC>`"]
pub type PBDMASK = crate::Reg<pbdmask::PBDMASK_SPEC>;
#[doc = "PBD Mask"]
pub mod pbdmask;
#[doc = "PBDSEL register accessor: an alias for `Reg<PBDSEL_SPEC>`"]
pub type PBDSEL = crate::Reg<pbdsel::PBDSEL_SPEC>;
#[doc = "PBD Clock Select"]
pub mod pbdsel;
#[doc = "PPCR register accessor: an alias for `Reg<PPCR_SPEC>`"]
pub type PPCR = crate::Reg<ppcr::PPCR_SPEC>;
#[doc = "Peripheral Power Control Register"]
pub mod ppcr;
#[doc = "RCAUSE register accessor: an alias for `Reg<RCAUSE_SPEC>`"]
pub type RCAUSE = crate::Reg<rcause::RCAUSE_SPEC>;
#[doc = "Reset Cause Register"]
pub mod rcause;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "UNLOCK register accessor: an alias for `Reg<UNLOCK_SPEC>`"]
pub type UNLOCK = crate::Reg<unlock::UNLOCK_SPEC>;
#[doc = "Unlock Register"]
pub mod unlock;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
#[doc = "WCAUSE register accessor: an alias for `Reg<WCAUSE_SPEC>`"]
pub type WCAUSE = crate::Reg<wcause::WCAUSE_SPEC>;
#[doc = "Wake Cause Register"]
pub mod wcause;
