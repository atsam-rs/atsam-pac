#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Clear Register"]
    pub clr: CLR,
    #[doc = "0x08 - Status Register"]
    pub sr: SR,
    #[doc = "0x0c - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x10 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x14 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x18 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x1c - Interrupt Clear Register"]
    pub icr: ICR,
    _reserved8: [u8; 0x03dc],
    #[doc = "0x3fc - Version Register"]
    pub version: VERSION,
}
#[doc = "CLR (w) register accessor: an alias for `Reg<CLR_SPEC>`"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "Clear Register"]
pub mod clr;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
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
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
