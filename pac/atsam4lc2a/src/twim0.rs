#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Clock Waveform Generator Register"]
    pub cwgr: crate::Reg<cwgr::CWGR_SPEC>,
    #[doc = "0x08 - SMBus Timing Register"]
    pub smbtr: crate::Reg<smbtr::SMBTR_SPEC>,
    #[doc = "0x0c - Command Register"]
    pub cmdr: crate::Reg<cmdr::CMDR_SPEC>,
    #[doc = "0x10 - Next Command Register"]
    pub ncmdr: crate::Reg<ncmdr::NCMDR_SPEC>,
    #[doc = "0x14 - Receive Holding Register"]
    pub rhr: crate::Reg<rhr::RHR_SPEC>,
    #[doc = "0x18 - Transmit Holding Register"]
    pub thr: crate::Reg<thr::THR_SPEC>,
    #[doc = "0x1c - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x20 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x24 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x28 - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x2c - Status Clear Register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    #[doc = "0x30 - Parameter Register"]
    pub pr: crate::Reg<pr::PR_SPEC>,
    #[doc = "0x34 - Version Register"]
    pub vr: crate::Reg<vr::VR_SPEC>,
    #[doc = "0x38 - HS-mode Clock Waveform Generator"]
    pub hscwgr: crate::Reg<hscwgr::HSCWGR_SPEC>,
    #[doc = "0x3c - Slew Rate Register"]
    pub srr: crate::Reg<srr::SRR_SPEC>,
    #[doc = "0x40 - HS-mode Slew Rate Register"]
    pub hssrr: crate::Reg<hssrr::HSSRR_SPEC>,
}
#[doc = "CMDR register accessor: an alias for `Reg<CMDR_SPEC>`"]
pub type CMDR = crate::Reg<cmdr::CMDR_SPEC>;
#[doc = "Command Register"]
pub mod cmdr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "CWGR register accessor: an alias for `Reg<CWGR_SPEC>`"]
pub type CWGR = crate::Reg<cwgr::CWGR_SPEC>;
#[doc = "Clock Waveform Generator Register"]
pub mod cwgr;
#[doc = "HSCWGR register accessor: an alias for `Reg<HSCWGR_SPEC>`"]
pub type HSCWGR = crate::Reg<hscwgr::HSCWGR_SPEC>;
#[doc = "HS-mode Clock Waveform Generator"]
pub mod hscwgr;
#[doc = "HSSRR register accessor: an alias for `Reg<HSSRR_SPEC>`"]
pub type HSSRR = crate::Reg<hssrr::HSSRR_SPEC>;
#[doc = "HS-mode Slew Rate Register"]
pub mod hssrr;
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
#[doc = "NCMDR register accessor: an alias for `Reg<NCMDR_SPEC>`"]
pub type NCMDR = crate::Reg<ncmdr::NCMDR_SPEC>;
#[doc = "Next Command Register"]
pub mod ncmdr;
#[doc = "PR register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Parameter Register"]
pub mod pr;
#[doc = "RHR register accessor: an alias for `Reg<RHR_SPEC>`"]
pub type RHR = crate::Reg<rhr::RHR_SPEC>;
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Status Clear Register"]
pub mod scr;
#[doc = "SMBTR register accessor: an alias for `Reg<SMBTR_SPEC>`"]
pub type SMBTR = crate::Reg<smbtr::SMBTR_SPEC>;
#[doc = "SMBus Timing Register"]
pub mod smbtr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "SRR register accessor: an alias for `Reg<SRR_SPEC>`"]
pub type SRR = crate::Reg<srr::SRR_SPEC>;
#[doc = "Slew Rate Register"]
pub mod srr;
#[doc = "THR register accessor: an alias for `Reg<THR_SPEC>`"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "VR register accessor: an alias for `Reg<VR_SPEC>`"]
pub type VR = crate::Reg<vr::VR_SPEC>;
#[doc = "Version Register"]
pub mod vr;
