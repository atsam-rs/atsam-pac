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
    #[doc = "0x14 - Status Register"]
    pub sr: SR,
    #[doc = "0x18 - Unlock Register"]
    pub unlock: UNLOCK,
    #[doc = "0x1c - Power Mode Control Register"]
    pub pmcon: PMCON,
    _reserved8: [u8; 0x08],
    #[doc = "0x28 - Backup Wake up Cause Register"]
    pub bkupwcause: BKUPWCAUSE,
    #[doc = "0x2c - Backup Wake up Enable Register"]
    pub bkupwen: BKUPWEN,
    #[doc = "0x30 - Backup Pin Muxing Register"]
    pub bkuppmux: BKUPPMUX,
    #[doc = "0x34 - Input Output Retention Register"]
    pub ioret: IORET,
    _reserved12: [u8; 0x08],
    #[doc = "0x40 - Bypass Register"]
    pub bpr: BPR,
    #[doc = "0x44 - Factory Word Run PS Register"]
    pub fwrunps: FWRUNPS,
    #[doc = "0x48 - Factory Word Power Save PS Register"]
    pub fwpsaveps: FWPSAVEPS,
    _reserved15: [u8; 0xb0],
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
#[doc = "BKUPPMUX (rw) register accessor: an alias for `Reg<BKUPPMUX_SPEC>`"]
pub type BKUPPMUX = crate::Reg<bkuppmux::BKUPPMUX_SPEC>;
#[doc = "Backup Pin Muxing Register"]
pub mod bkuppmux;
#[doc = "BKUPWCAUSE (r) register accessor: an alias for `Reg<BKUPWCAUSE_SPEC>`"]
pub type BKUPWCAUSE = crate::Reg<bkupwcause::BKUPWCAUSE_SPEC>;
#[doc = "Backup Wake up Cause Register"]
pub mod bkupwcause;
#[doc = "BKUPWEN (rw) register accessor: an alias for `Reg<BKUPWEN_SPEC>`"]
pub type BKUPWEN = crate::Reg<bkupwen::BKUPWEN_SPEC>;
#[doc = "Backup Wake up Enable Register"]
pub mod bkupwen;
#[doc = "BPR (rw) register accessor: an alias for `Reg<BPR_SPEC>`"]
pub type BPR = crate::Reg<bpr::BPR_SPEC>;
#[doc = "Bypass Register"]
pub mod bpr;
#[doc = "FWPSAVEPS (r) register accessor: an alias for `Reg<FWPSAVEPS_SPEC>`"]
pub type FWPSAVEPS = crate::Reg<fwpsaveps::FWPSAVEPS_SPEC>;
#[doc = "Factory Word Power Save PS Register"]
pub mod fwpsaveps;
#[doc = "FWRUNPS (r) register accessor: an alias for `Reg<FWRUNPS_SPEC>`"]
pub type FWRUNPS = crate::Reg<fwrunps::FWRUNPS_SPEC>;
#[doc = "Factory Word Run PS Register"]
pub mod fwrunps;
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
#[doc = "IORET (rw) register accessor: an alias for `Reg<IORET_SPEC>`"]
pub type IORET = crate::Reg<ioret::IORET_SPEC>;
#[doc = "Input Output Retention Register"]
pub mod ioret;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "PMCON (rw) register accessor: an alias for `Reg<PMCON_SPEC>`"]
pub type PMCON = crate::Reg<pmcon::PMCON_SPEC>;
#[doc = "Power Mode Control Register"]
pub mod pmcon;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "UNLOCK (w) register accessor: an alias for `Reg<UNLOCK_SPEC>`"]
pub type UNLOCK = crate::Reg<unlock::UNLOCK_SPEC>;
#[doc = "Unlock Register"]
pub mod unlock;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
