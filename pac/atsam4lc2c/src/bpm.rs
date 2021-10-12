#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x04 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x08 - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x0c - Interrupt Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x10 - Interrupt Clear Register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x14 - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x18 - Unlock Register"]
    pub unlock: crate::Reg<unlock::UNLOCK_SPEC>,
    #[doc = "0x1c - Power Mode Control Register"]
    pub pmcon: crate::Reg<pmcon::PMCON_SPEC>,
    _reserved8: [u8; 0x08],
    #[doc = "0x28 - Backup Wake up Cause Register"]
    pub bkupwcause: crate::Reg<bkupwcause::BKUPWCAUSE_SPEC>,
    #[doc = "0x2c - Backup Wake up Enable Register"]
    pub bkupwen: crate::Reg<bkupwen::BKUPWEN_SPEC>,
    #[doc = "0x30 - Backup Pin Muxing Register"]
    pub bkuppmux: crate::Reg<bkuppmux::BKUPPMUX_SPEC>,
    #[doc = "0x34 - Input Output Retention Register"]
    pub ioret: crate::Reg<ioret::IORET_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x40 - Bypass Register"]
    pub bpr: crate::Reg<bpr::BPR_SPEC>,
    #[doc = "0x44 - Factory Word Run PS Register"]
    pub fwrunps: crate::Reg<fwrunps::FWRUNPS_SPEC>,
    #[doc = "0x48 - Factory Word Power Save PS Register"]
    pub fwpsaveps: crate::Reg<fwpsaveps::FWPSAVEPS_SPEC>,
    _reserved15: [u8; 0xb0],
    #[doc = "0xfc - Version Register"]
    pub version: crate::Reg<version::VERSION_SPEC>,
}
#[doc = "BKUPPMUX register accessor: an alias for `Reg<BKUPPMUX_SPEC>`"]
pub type BKUPPMUX = crate::Reg<bkuppmux::BKUPPMUX_SPEC>;
#[doc = "Backup Pin Muxing Register"]
pub mod bkuppmux;
#[doc = "BKUPWCAUSE register accessor: an alias for `Reg<BKUPWCAUSE_SPEC>`"]
pub type BKUPWCAUSE = crate::Reg<bkupwcause::BKUPWCAUSE_SPEC>;
#[doc = "Backup Wake up Cause Register"]
pub mod bkupwcause;
#[doc = "BKUPWEN register accessor: an alias for `Reg<BKUPWEN_SPEC>`"]
pub type BKUPWEN = crate::Reg<bkupwen::BKUPWEN_SPEC>;
#[doc = "Backup Wake up Enable Register"]
pub mod bkupwen;
#[doc = "BPR register accessor: an alias for `Reg<BPR_SPEC>`"]
pub type BPR = crate::Reg<bpr::BPR_SPEC>;
#[doc = "Bypass Register"]
pub mod bpr;
#[doc = "FWPSAVEPS register accessor: an alias for `Reg<FWPSAVEPS_SPEC>`"]
pub type FWPSAVEPS = crate::Reg<fwpsaveps::FWPSAVEPS_SPEC>;
#[doc = "Factory Word Power Save PS Register"]
pub mod fwpsaveps;
#[doc = "FWRUNPS register accessor: an alias for `Reg<FWRUNPS_SPEC>`"]
pub type FWRUNPS = crate::Reg<fwrunps::FWRUNPS_SPEC>;
#[doc = "Factory Word Run PS Register"]
pub mod fwrunps;
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
#[doc = "IORET register accessor: an alias for `Reg<IORET_SPEC>`"]
pub type IORET = crate::Reg<ioret::IORET_SPEC>;
#[doc = "Input Output Retention Register"]
pub mod ioret;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "PMCON register accessor: an alias for `Reg<PMCON_SPEC>`"]
pub type PMCON = crate::Reg<pmcon::PMCON_SPEC>;
#[doc = "Power Mode Control Register"]
pub mod pmcon;
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
