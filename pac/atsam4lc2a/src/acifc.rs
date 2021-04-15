#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x1c - Interrupt Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x20 - Interrupt Status Clear Register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x24 - Test Register"]
    pub tr: crate::Reg<tr::TR_SPEC>,
    _reserved8: [u8; 8usize],
    #[doc = "0x30 - Parameter Register"]
    pub parameter: crate::Reg<parameter::PARAMETER_SPEC>,
    #[doc = "0x34 - Version Register"]
    pub version: crate::Reg<version::VERSION_SPEC>,
    _reserved10: [u8; 72usize],
    #[doc = "0x80 - Window configuration Register"]
    pub confw: [crate::Reg<confw::CONFW_SPEC>; 4],
    _reserved11: [u8; 64usize],
    #[doc = "0xd0 - AC Configuration Register"]
    pub conf: [crate::Reg<conf::CONF_SPEC>; 8],
}
#[doc = "CONF register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "AC Configuration Register"]
pub mod conf;
#[doc = "CONFW register accessor: an alias for `Reg<CONFW_SPEC>`"]
pub type CONFW = crate::Reg<confw::CONFW_SPEC>;
#[doc = "Window configuration Register"]
pub mod confw;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Status Clear Register"]
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
#[doc = "PARAMETER register accessor: an alias for `Reg<PARAMETER_SPEC>`"]
pub type PARAMETER = crate::Reg<parameter::PARAMETER_SPEC>;
#[doc = "Parameter Register"]
pub mod parameter;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "TR register accessor: an alias for `Reg<TR_SPEC>`"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "Test Register"]
pub mod tr;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
