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
    #[doc = "0x14 - Mode Register"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x18 - Edge Register"]
    pub edge: crate::Reg<edge::EDGE_SPEC>,
    #[doc = "0x1c - Level Register"]
    pub level: crate::Reg<level::LEVEL_SPEC>,
    #[doc = "0x20 - Filter Register"]
    pub filter: crate::Reg<filter::FILTER_SPEC>,
    _reserved9: [u8; 4usize],
    #[doc = "0x28 - Asynchronous Register"]
    pub async_: crate::Reg<async_::ASYNC_SPEC>,
    _reserved10: [u8; 4usize],
    #[doc = "0x30 - Enable Register"]
    pub en: crate::Reg<en::EN_SPEC>,
    #[doc = "0x34 - Disable Register"]
    pub dis: crate::Reg<dis::DIS_SPEC>,
    #[doc = "0x38 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    _reserved13: [u8; 960usize],
    #[doc = "0x3fc - Version Register"]
    pub version: crate::Reg<version::VERSION_SPEC>,
}
#[doc = "ASYNC register accessor: an alias for `Reg<ASYNC_SPEC>`"]
pub type ASYNC = crate::Reg<async_::ASYNC_SPEC>;
#[doc = "Asynchronous Register"]
pub mod async_;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "DIS register accessor: an alias for `Reg<DIS_SPEC>`"]
pub type DIS = crate::Reg<dis::DIS_SPEC>;
#[doc = "Disable Register"]
pub mod dis;
#[doc = "EDGE register accessor: an alias for `Reg<EDGE_SPEC>`"]
pub type EDGE = crate::Reg<edge::EDGE_SPEC>;
#[doc = "Edge Register"]
pub mod edge;
#[doc = "EN register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "Enable Register"]
pub mod en;
#[doc = "FILTER register accessor: an alias for `Reg<FILTER_SPEC>`"]
pub type FILTER = crate::Reg<filter::FILTER_SPEC>;
#[doc = "Filter Register"]
pub mod filter;
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
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "LEVEL register accessor: an alias for `Reg<LEVEL_SPEC>`"]
pub type LEVEL = crate::Reg<level::LEVEL_SPEC>;
#[doc = "Level Register"]
pub mod level;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Mode Register"]
pub mod mode;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
