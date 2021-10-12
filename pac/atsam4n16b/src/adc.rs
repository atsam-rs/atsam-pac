#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub mr: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x08..0x10 - Channel Sequence Register"]
    pub seqr: [crate::Reg<seqr::SEQR_SPEC>; 2],
    #[doc = "0x10 - Channel Enable Register"]
    pub cher: crate::Reg<cher::CHER_SPEC>,
    #[doc = "0x14 - Channel Disable Register"]
    pub chdr: crate::Reg<chdr::CHDR_SPEC>,
    #[doc = "0x18 - Channel Status Register"]
    pub chsr: crate::Reg<chsr::CHSR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Last Converted Data Register"]
    pub lcdr: crate::Reg<lcdr::LCDR_SPEC>,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x30 - Interrupt Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x34 - Temperature Sensor Mode Register"]
    pub tempmr: crate::Reg<tempmr::TEMPMR_SPEC>,
    #[doc = "0x38 - Temperature Compare Window Register"]
    pub tempcwr: crate::Reg<tempcwr::TEMPCWR_SPEC>,
    #[doc = "0x3c - Overrun Status Register"]
    pub over: crate::Reg<over::OVER_SPEC>,
    #[doc = "0x40 - Extended Mode Register"]
    pub emr: crate::Reg<emr::EMR_SPEC>,
    #[doc = "0x44 - Compare Window Register"]
    pub cwr: crate::Reg<cwr::CWR_SPEC>,
    _reserved16: [u8; 0x08],
    #[doc = "0x50..0x94 - Channel Data Register"]
    pub cdr: [crate::Reg<cdr::CDR_SPEC>; 17],
    #[doc = "0x94 - Analog Control Register"]
    pub acr: crate::Reg<acr::ACR_SPEC>,
    _reserved18: [u8; 0x4c],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
    _reserved20: [u8; 0x14],
    #[doc = "0x100 - Receive Pointer Register"]
    pub rpr: crate::Reg<rpr::RPR_SPEC>,
    #[doc = "0x104 - Receive Counter Register"]
    pub rcr: crate::Reg<rcr::RCR_SPEC>,
    _reserved22: [u8; 0x08],
    #[doc = "0x110 - Receive Next Pointer Register"]
    pub rnpr: crate::Reg<rnpr::RNPR_SPEC>,
    #[doc = "0x114 - Receive Next Counter Register"]
    pub rncr: crate::Reg<rncr::RNCR_SPEC>,
    _reserved24: [u8; 0x08],
    #[doc = "0x120 - Transfer Control Register"]
    pub ptcr: crate::Reg<ptcr::PTCR_SPEC>,
    #[doc = "0x124 - Transfer Status Register"]
    pub ptsr: crate::Reg<ptsr::PTSR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "SEQR register accessor: an alias for `Reg<SEQR_SPEC>`"]
pub type SEQR = crate::Reg<seqr::SEQR_SPEC>;
#[doc = "Channel Sequence Register"]
pub mod seqr;
#[doc = "CHER register accessor: an alias for `Reg<CHER_SPEC>`"]
pub type CHER = crate::Reg<cher::CHER_SPEC>;
#[doc = "Channel Enable Register"]
pub mod cher;
#[doc = "CHDR register accessor: an alias for `Reg<CHDR_SPEC>`"]
pub type CHDR = crate::Reg<chdr::CHDR_SPEC>;
#[doc = "Channel Disable Register"]
pub mod chdr;
#[doc = "CHSR register accessor: an alias for `Reg<CHSR_SPEC>`"]
pub type CHSR = crate::Reg<chsr::CHSR_SPEC>;
#[doc = "Channel Status Register"]
pub mod chsr;
#[doc = "LCDR register accessor: an alias for `Reg<LCDR_SPEC>`"]
pub type LCDR = crate::Reg<lcdr::LCDR_SPEC>;
#[doc = "Last Converted Data Register"]
pub mod lcdr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "TEMPMR register accessor: an alias for `Reg<TEMPMR_SPEC>`"]
pub type TEMPMR = crate::Reg<tempmr::TEMPMR_SPEC>;
#[doc = "Temperature Sensor Mode Register"]
pub mod tempmr;
#[doc = "TEMPCWR register accessor: an alias for `Reg<TEMPCWR_SPEC>`"]
pub type TEMPCWR = crate::Reg<tempcwr::TEMPCWR_SPEC>;
#[doc = "Temperature Compare Window Register"]
pub mod tempcwr;
#[doc = "OVER register accessor: an alias for `Reg<OVER_SPEC>`"]
pub type OVER = crate::Reg<over::OVER_SPEC>;
#[doc = "Overrun Status Register"]
pub mod over;
#[doc = "EMR register accessor: an alias for `Reg<EMR_SPEC>`"]
pub type EMR = crate::Reg<emr::EMR_SPEC>;
#[doc = "Extended Mode Register"]
pub mod emr;
#[doc = "CWR register accessor: an alias for `Reg<CWR_SPEC>`"]
pub type CWR = crate::Reg<cwr::CWR_SPEC>;
#[doc = "Compare Window Register"]
pub mod cwr;
#[doc = "CDR register accessor: an alias for `Reg<CDR_SPEC>`"]
pub type CDR = crate::Reg<cdr::CDR_SPEC>;
#[doc = "Channel Data Register"]
pub mod cdr;
#[doc = "ACR register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Analog Control Register"]
pub mod acr;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
#[doc = "RPR register accessor: an alias for `Reg<RPR_SPEC>`"]
pub type RPR = crate::Reg<rpr::RPR_SPEC>;
#[doc = "Receive Pointer Register"]
pub mod rpr;
#[doc = "RCR register accessor: an alias for `Reg<RCR_SPEC>`"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "Receive Counter Register"]
pub mod rcr;
#[doc = "RNPR register accessor: an alias for `Reg<RNPR_SPEC>`"]
pub type RNPR = crate::Reg<rnpr::RNPR_SPEC>;
#[doc = "Receive Next Pointer Register"]
pub mod rnpr;
#[doc = "RNCR register accessor: an alias for `Reg<RNCR_SPEC>`"]
pub type RNCR = crate::Reg<rncr::RNCR_SPEC>;
#[doc = "Receive Next Counter Register"]
pub mod rncr;
#[doc = "PTCR register accessor: an alias for `Reg<PTCR_SPEC>`"]
pub type PTCR = crate::Reg<ptcr::PTCR_SPEC>;
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "PTSR register accessor: an alias for `Reg<PTSR_SPEC>`"]
pub type PTSR = crate::Reg<ptsr::PTSR_SPEC>;
#[doc = "Transfer Status Register"]
pub mod ptsr;
