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
    #[doc = "0x14 - Mode Register"]
    pub mode: MODE,
    #[doc = "0x18 - Edge Register"]
    pub edge: EDGE,
    #[doc = "0x1c - Level Register"]
    pub level: LEVEL,
    #[doc = "0x20 - Filter Register"]
    pub filter: FILTER,
    _reserved9: [u8; 0x04],
    #[doc = "0x28 - Asynchronous Register"]
    pub async_: ASYNC,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Enable Register"]
    pub en: EN,
    #[doc = "0x34 - Disable Register"]
    pub dis: DIS,
    #[doc = "0x38 - Control Register"]
    pub ctrl: CTRL,
    _reserved13: [u8; 0x03c0],
    #[doc = "0x3fc - Version Register"]
    pub version: VERSION,
}
#[doc = "ASYNC (rw) register accessor: an alias for `Reg<ASYNC_SPEC>`"]
pub type ASYNC = crate::Reg<async_::ASYNC_SPEC>;
#[doc = "Asynchronous Register"]
pub mod async_;
#[doc = "CTRL (r) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "DIS (w) register accessor: an alias for `Reg<DIS_SPEC>`"]
pub type DIS = crate::Reg<dis::DIS_SPEC>;
#[doc = "Disable Register"]
pub mod dis;
#[doc = "EDGE (rw) register accessor: an alias for `Reg<EDGE_SPEC>`"]
pub type EDGE = crate::Reg<edge::EDGE_SPEC>;
#[doc = "Edge Register"]
pub mod edge;
#[doc = "EN (w) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "Enable Register"]
pub mod en;
#[doc = "FILTER (rw) register accessor: an alias for `Reg<FILTER_SPEC>`"]
pub type FILTER = crate::Reg<filter::FILTER_SPEC>;
#[doc = "Filter Register"]
pub mod filter;
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
#[doc = "LEVEL (rw) register accessor: an alias for `Reg<LEVEL_SPEC>`"]
pub type LEVEL = crate::Reg<level::LEVEL_SPEC>;
#[doc = "Level Register"]
pub mod level;
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Mode Register"]
pub mod mode;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
