#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Clock Mode Register"]
    pub cmr: crate::Reg<cmr::CMR_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Receive Clock Mode Register"]
    pub rcmr: crate::Reg<rcmr::RCMR_SPEC>,
    #[doc = "0x14 - Receive Frame Mode Register"]
    pub rfmr: crate::Reg<rfmr::RFMR_SPEC>,
    #[doc = "0x18 - Transmit Clock Mode Register"]
    pub tcmr: crate::Reg<tcmr::TCMR_SPEC>,
    #[doc = "0x1c - Transmit Frame Mode Register"]
    pub tfmr: crate::Reg<tfmr::TFMR_SPEC>,
    #[doc = "0x20 - Receive Holding Register"]
    pub rhr: crate::Reg<rhr::RHR_SPEC>,
    #[doc = "0x24 - Transmit Holding Register"]
    pub thr: crate::Reg<thr::THR_SPEC>,
    _reserved8: [u8; 0x08],
    #[doc = "0x30 - Receive Sync. Holding Register"]
    pub rshr: crate::Reg<rshr::RSHR_SPEC>,
    #[doc = "0x34 - Transmit Sync. Holding Register"]
    pub tshr: crate::Reg<tshr::TSHR_SPEC>,
    #[doc = "0x38 - Receive Compare 0 Register"]
    pub rc0r: crate::Reg<rc0r::RC0R_SPEC>,
    #[doc = "0x3c - Receive Compare 1 Register"]
    pub rc1r: crate::Reg<rc1r::RC1R_SPEC>,
    #[doc = "0x40 - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x48 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x4c - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    _reserved16: [u8; 0x94],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
    _reserved18: [u8; 0x14],
    #[doc = "0x100 - Receive Pointer Register"]
    pub rpr: crate::Reg<rpr::RPR_SPEC>,
    #[doc = "0x104 - Receive Counter Register"]
    pub rcr: crate::Reg<rcr::RCR_SPEC>,
    #[doc = "0x108 - Transmit Pointer Register"]
    pub tpr: crate::Reg<tpr::TPR_SPEC>,
    #[doc = "0x10c - Transmit Counter Register"]
    pub tcr: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x110 - Receive Next Pointer Register"]
    pub rnpr: crate::Reg<rnpr::RNPR_SPEC>,
    #[doc = "0x114 - Receive Next Counter Register"]
    pub rncr: crate::Reg<rncr::RNCR_SPEC>,
    #[doc = "0x118 - Transmit Next Pointer Register"]
    pub tnpr: crate::Reg<tnpr::TNPR_SPEC>,
    #[doc = "0x11c - Transmit Next Counter Register"]
    pub tncr: crate::Reg<tncr::TNCR_SPEC>,
    #[doc = "0x120 - Transfer Control Register"]
    pub ptcr: crate::Reg<ptcr::PTCR_SPEC>,
    #[doc = "0x124 - Transfer Status Register"]
    pub ptsr: crate::Reg<ptsr::PTSR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "CMR register accessor: an alias for `Reg<CMR_SPEC>`"]
pub type CMR = crate::Reg<cmr::CMR_SPEC>;
#[doc = "Clock Mode Register"]
pub mod cmr;
#[doc = "RCMR register accessor: an alias for `Reg<RCMR_SPEC>`"]
pub type RCMR = crate::Reg<rcmr::RCMR_SPEC>;
#[doc = "Receive Clock Mode Register"]
pub mod rcmr;
#[doc = "RFMR register accessor: an alias for `Reg<RFMR_SPEC>`"]
pub type RFMR = crate::Reg<rfmr::RFMR_SPEC>;
#[doc = "Receive Frame Mode Register"]
pub mod rfmr;
#[doc = "TCMR register accessor: an alias for `Reg<TCMR_SPEC>`"]
pub type TCMR = crate::Reg<tcmr::TCMR_SPEC>;
#[doc = "Transmit Clock Mode Register"]
pub mod tcmr;
#[doc = "TFMR register accessor: an alias for `Reg<TFMR_SPEC>`"]
pub type TFMR = crate::Reg<tfmr::TFMR_SPEC>;
#[doc = "Transmit Frame Mode Register"]
pub mod tfmr;
#[doc = "RHR register accessor: an alias for `Reg<RHR_SPEC>`"]
pub type RHR = crate::Reg<rhr::RHR_SPEC>;
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "THR register accessor: an alias for `Reg<THR_SPEC>`"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "RSHR register accessor: an alias for `Reg<RSHR_SPEC>`"]
pub type RSHR = crate::Reg<rshr::RSHR_SPEC>;
#[doc = "Receive Sync. Holding Register"]
pub mod rshr;
#[doc = "TSHR register accessor: an alias for `Reg<TSHR_SPEC>`"]
pub type TSHR = crate::Reg<tshr::TSHR_SPEC>;
#[doc = "Transmit Sync. Holding Register"]
pub mod tshr;
#[doc = "RC0R register accessor: an alias for `Reg<RC0R_SPEC>`"]
pub type RC0R = crate::Reg<rc0r::RC0R_SPEC>;
#[doc = "Receive Compare 0 Register"]
pub mod rc0r;
#[doc = "RC1R register accessor: an alias for `Reg<RC1R_SPEC>`"]
pub type RC1R = crate::Reg<rc1r::RC1R_SPEC>;
#[doc = "Receive Compare 1 Register"]
pub mod rc1r;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
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
#[doc = "TPR register accessor: an alias for `Reg<TPR_SPEC>`"]
pub type TPR = crate::Reg<tpr::TPR_SPEC>;
#[doc = "Transmit Pointer Register"]
pub mod tpr;
#[doc = "TCR register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Transmit Counter Register"]
pub mod tcr;
#[doc = "RNPR register accessor: an alias for `Reg<RNPR_SPEC>`"]
pub type RNPR = crate::Reg<rnpr::RNPR_SPEC>;
#[doc = "Receive Next Pointer Register"]
pub mod rnpr;
#[doc = "RNCR register accessor: an alias for `Reg<RNCR_SPEC>`"]
pub type RNCR = crate::Reg<rncr::RNCR_SPEC>;
#[doc = "Receive Next Counter Register"]
pub mod rncr;
#[doc = "TNPR register accessor: an alias for `Reg<TNPR_SPEC>`"]
pub type TNPR = crate::Reg<tnpr::TNPR_SPEC>;
#[doc = "Transmit Next Pointer Register"]
pub mod tnpr;
#[doc = "TNCR register accessor: an alias for `Reg<TNCR_SPEC>`"]
pub type TNCR = crate::Reg<tncr::TNCR_SPEC>;
#[doc = "Transmit Next Counter Register"]
pub mod tncr;
#[doc = "PTCR register accessor: an alias for `Reg<PTCR_SPEC>`"]
pub type PTCR = crate::Reg<ptcr::PTCR_SPEC>;
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "PTSR register accessor: an alias for `Reg<PTSR_SPEC>`"]
pub type PTSR = crate::Reg<ptsr::PTSR_SPEC>;
#[doc = "Transfer Status Register"]
pub mod ptsr;
