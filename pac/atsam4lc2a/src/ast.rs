#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Counter Value"]
    pub cv: crate::Reg<cv::CV_SPEC>,
    #[doc = "0x08 - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x0c - Status Clear Register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x1c - Wake Enable Register"]
    pub wer: crate::Reg<wer::WER_SPEC>,
    #[doc = "0x20 - Alarm Register 0"]
    pub ar0: crate::Reg<ar0::AR0_SPEC>,
    #[doc = "0x24 - Alarm Register 1"]
    pub ar1: crate::Reg<ar1::AR1_SPEC>,
    _reserved10: [u8; 8usize],
    #[doc = "0x30 - Periodic Interval Register 0"]
    pub pir0: crate::Reg<pir0::PIR0_SPEC>,
    #[doc = "0x34 - Periodic Interval Register 1"]
    pub pir1: crate::Reg<pir1::PIR1_SPEC>,
    _reserved12: [u8; 8usize],
    #[doc = "0x40 - Clock Control Register"]
    pub clock: crate::Reg<clock::CLOCK_SPEC>,
    #[doc = "0x44 - Digital Tuner Register"]
    pub dtr: crate::Reg<dtr::DTR_SPEC>,
    #[doc = "0x48 - Event Enable Register"]
    pub eve: crate::Reg<eve::EVE_SPEC>,
    #[doc = "0x4c - Event Disable Register"]
    pub evd: crate::Reg<evd::EVD_SPEC>,
    #[doc = "0x50 - Event Mask Register"]
    pub evm: crate::Reg<evm::EVM_SPEC>,
    #[doc = "0x54 - Calendar Value"]
    pub calv: crate::Reg<calv::CALV_SPEC>,
    _reserved18: [u8; 152usize],
    #[doc = "0xf0 - Parameter Register"]
    pub parameter: crate::Reg<parameter::PARAMETER_SPEC>,
    _reserved19: [u8; 8usize],
    #[doc = "0xfc - Version Register"]
    pub version: crate::Reg<version::VERSION_SPEC>,
}
#[doc = "AR0 register accessor: an alias for `Reg<AR0_SPEC>`"]
pub type AR0 = crate::Reg<ar0::AR0_SPEC>;
#[doc = "Alarm Register 0"]
pub mod ar0;
#[doc = "AR1 register accessor: an alias for `Reg<AR1_SPEC>`"]
pub type AR1 = crate::Reg<ar1::AR1_SPEC>;
#[doc = "Alarm Register 1"]
pub mod ar1;
#[doc = "CALV register accessor: an alias for `Reg<CALV_SPEC>`"]
pub type CALV = crate::Reg<calv::CALV_SPEC>;
#[doc = "Calendar Value"]
pub mod calv;
#[doc = "CLOCK register accessor: an alias for `Reg<CLOCK_SPEC>`"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "Clock Control Register"]
pub mod clock;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "CV register accessor: an alias for `Reg<CV_SPEC>`"]
pub type CV = crate::Reg<cv::CV_SPEC>;
#[doc = "Counter Value"]
pub mod cv;
#[doc = "DTR register accessor: an alias for `Reg<DTR_SPEC>`"]
pub type DTR = crate::Reg<dtr::DTR_SPEC>;
#[doc = "Digital Tuner Register"]
pub mod dtr;
#[doc = "EVD register accessor: an alias for `Reg<EVD_SPEC>`"]
pub type EVD = crate::Reg<evd::EVD_SPEC>;
#[doc = "Event Disable Register"]
pub mod evd;
#[doc = "EVE register accessor: an alias for `Reg<EVE_SPEC>`"]
pub type EVE = crate::Reg<eve::EVE_SPEC>;
#[doc = "Event Enable Register"]
pub mod eve;
#[doc = "EVM register accessor: an alias for `Reg<EVM_SPEC>`"]
pub type EVM = crate::Reg<evm::EVM_SPEC>;
#[doc = "Event Mask Register"]
pub mod evm;
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
#[doc = "PARAMETER register accessor: an alias for `Reg<PARAMETER_SPEC>`"]
pub type PARAMETER = crate::Reg<parameter::PARAMETER_SPEC>;
#[doc = "Parameter Register"]
pub mod parameter;
#[doc = "PIR0 register accessor: an alias for `Reg<PIR0_SPEC>`"]
pub type PIR0 = crate::Reg<pir0::PIR0_SPEC>;
#[doc = "Periodic Interval Register 0"]
pub mod pir0;
#[doc = "PIR1 register accessor: an alias for `Reg<PIR1_SPEC>`"]
pub type PIR1 = crate::Reg<pir1::PIR1_SPEC>;
#[doc = "Periodic Interval Register 1"]
pub mod pir1;
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Status Clear Register"]
pub mod scr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
#[doc = "WER register accessor: an alias for `Reg<WER_SPEC>`"]
pub type WER = crate::Reg<wer::WER_SPEC>;
#[doc = "Wake Enable Register"]
pub mod wer;
