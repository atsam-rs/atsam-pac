#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - NBYTES Register"]
    pub nbytes: crate::Reg<nbytes::NBYTES_SPEC>,
    #[doc = "0x08 - Timing Register"]
    pub tr: crate::Reg<tr::TR_SPEC>,
    #[doc = "0x0c - Receive Holding Register"]
    pub rhr: crate::Reg<rhr::RHR_SPEC>,
    #[doc = "0x10 - Transmit Holding Register"]
    pub thr: crate::Reg<thr::THR_SPEC>,
    #[doc = "0x14 - Packet Error Check Register"]
    pub pecr: crate::Reg<pecr::PECR_SPEC>,
    #[doc = "0x18 - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x1c - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x20 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x24 - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x28 - Status Clear Register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    #[doc = "0x2c - Parameter Register"]
    pub pr: crate::Reg<pr::PR_SPEC>,
    #[doc = "0x30 - Version Register"]
    pub vr: crate::Reg<vr::VR_SPEC>,
    #[doc = "0x34 - HS-mode Timing Register"]
    pub hstr: crate::Reg<hstr::HSTR_SPEC>,
    #[doc = "0x38 - Slew Rate Register"]
    pub srr: crate::Reg<srr::SRR_SPEC>,
    #[doc = "0x3c - HS-mode Slew Rate Register"]
    pub hssrr: crate::Reg<hssrr::HSSRR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "HSSRR register accessor: an alias for `Reg<HSSRR_SPEC>`"]
pub type HSSRR = crate::Reg<hssrr::HSSRR_SPEC>;
#[doc = "HS-mode Slew Rate Register"]
pub mod hssrr;
#[doc = "HSTR register accessor: an alias for `Reg<HSTR_SPEC>`"]
pub type HSTR = crate::Reg<hstr::HSTR_SPEC>;
#[doc = "HS-mode Timing Register"]
pub mod hstr;
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
#[doc = "NBYTES register accessor: an alias for `Reg<NBYTES_SPEC>`"]
pub type NBYTES = crate::Reg<nbytes::NBYTES_SPEC>;
#[doc = "NBYTES Register"]
pub mod nbytes;
#[doc = "PECR register accessor: an alias for `Reg<PECR_SPEC>`"]
pub type PECR = crate::Reg<pecr::PECR_SPEC>;
#[doc = "Packet Error Check Register"]
pub mod pecr;
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
#[doc = "TR register accessor: an alias for `Reg<TR_SPEC>`"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "Timing Register"]
pub mod tr;
#[doc = "VR register accessor: an alias for `Reg<VR_SPEC>`"]
pub type VR = crate::Reg<vr::VR_SPEC>;
#[doc = "Version Register"]
pub mod vr;
