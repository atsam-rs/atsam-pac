#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Sample Data Register 0"]
    pub sdr0: SDR0,
    #[doc = "0x08 - Sample Data Register 1"]
    pub sdr1: SDR1,
    #[doc = "0x0c - Volume Control Register 0"]
    pub vcr0: VCR0,
    #[doc = "0x10 - Volume Control Register 1"]
    pub vcr1: VCR1,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x18 - Interupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x20 - Status Register"]
    pub sr: SR,
    #[doc = "0x24 - Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x28 - Parameter Register"]
    pub parameter: PARAMETER,
    #[doc = "0x2c - Version Register"]
    pub version: VERSION,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interupt Disable Register"]
pub mod idr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "PARAMETER (r) register accessor: an alias for `Reg<PARAMETER_SPEC>`"]
pub type PARAMETER = crate::Reg<parameter::PARAMETER_SPEC>;
#[doc = "Parameter Register"]
pub mod parameter;
#[doc = "SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Status Clear Register"]
pub mod scr;
#[doc = "SDR0 (rw) register accessor: an alias for `Reg<SDR0_SPEC>`"]
pub type SDR0 = crate::Reg<sdr0::SDR0_SPEC>;
#[doc = "Sample Data Register 0"]
pub mod sdr0;
#[doc = "SDR1 (rw) register accessor: an alias for `Reg<SDR1_SPEC>`"]
pub type SDR1 = crate::Reg<sdr1::SDR1_SPEC>;
#[doc = "Sample Data Register 1"]
pub mod sdr1;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "VCR0 (rw) register accessor: an alias for `Reg<VCR0_SPEC>`"]
pub type VCR0 = crate::Reg<vcr0::VCR0_SPEC>;
#[doc = "Volume Control Register 0"]
pub mod vcr0;
#[doc = "VCR1 (rw) register accessor: an alias for `Reg<VCR1_SPEC>`"]
pub type VCR1 = crate::Reg<vcr1::VCR1_SPEC>;
#[doc = "Volume Control Register 1"]
pub mod vcr1;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
