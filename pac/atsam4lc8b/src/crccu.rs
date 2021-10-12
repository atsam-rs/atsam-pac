#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Descriptor Base Register"]
    pub dscr: crate::Reg<dscr::DSCR_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - DMA Enable Register"]
    pub dmaen: crate::Reg<dmaen::DMAEN_SPEC>,
    #[doc = "0x0c - DMA Disable Register"]
    pub dmadis: crate::Reg<dmadis::DMADIS_SPEC>,
    #[doc = "0x10 - DMA Status Register"]
    pub dmasr: crate::Reg<dmasr::DMASR_SPEC>,
    #[doc = "0x14 - DMA Interrupt Enable Register"]
    pub dmaier: crate::Reg<dmaier::DMAIER_SPEC>,
    #[doc = "0x18 - DMA Interrupt Disable Register"]
    pub dmaidr: crate::Reg<dmaidr::DMAIDR_SPEC>,
    #[doc = "0x1c - DMA Interrupt Mask Register"]
    pub dmaimr: crate::Reg<dmaimr::DMAIMR_SPEC>,
    #[doc = "0x20 - DMA Interrupt Status Register"]
    pub dmaisr: crate::Reg<dmaisr::DMAISR_SPEC>,
    _reserved8: [u8; 0x10],
    #[doc = "0x34 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x38 - Mode Register"]
    pub mr: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x3c - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x40 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x44 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x48 - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x4c - Interrupt Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    _reserved15: [u8; 0xac],
    #[doc = "0xfc - Version Register"]
    pub version: crate::Reg<version::VERSION_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "DMADIS register accessor: an alias for `Reg<DMADIS_SPEC>`"]
pub type DMADIS = crate::Reg<dmadis::DMADIS_SPEC>;
#[doc = "DMA Disable Register"]
pub mod dmadis;
#[doc = "DMAEN register accessor: an alias for `Reg<DMAEN_SPEC>`"]
pub type DMAEN = crate::Reg<dmaen::DMAEN_SPEC>;
#[doc = "DMA Enable Register"]
pub mod dmaen;
#[doc = "DMAIDR register accessor: an alias for `Reg<DMAIDR_SPEC>`"]
pub type DMAIDR = crate::Reg<dmaidr::DMAIDR_SPEC>;
#[doc = "DMA Interrupt Disable Register"]
pub mod dmaidr;
#[doc = "DMAIER register accessor: an alias for `Reg<DMAIER_SPEC>`"]
pub type DMAIER = crate::Reg<dmaier::DMAIER_SPEC>;
#[doc = "DMA Interrupt Enable Register"]
pub mod dmaier;
#[doc = "DMAIMR register accessor: an alias for `Reg<DMAIMR_SPEC>`"]
pub type DMAIMR = crate::Reg<dmaimr::DMAIMR_SPEC>;
#[doc = "DMA Interrupt Mask Register"]
pub mod dmaimr;
#[doc = "DMAISR register accessor: an alias for `Reg<DMAISR_SPEC>`"]
pub type DMAISR = crate::Reg<dmaisr::DMAISR_SPEC>;
#[doc = "DMA Interrupt Status Register"]
pub mod dmaisr;
#[doc = "DMASR register accessor: an alias for `Reg<DMASR_SPEC>`"]
pub type DMASR = crate::Reg<dmasr::DMASR_SPEC>;
#[doc = "DMA Status Register"]
pub mod dmasr;
#[doc = "DSCR register accessor: an alias for `Reg<DSCR_SPEC>`"]
pub type DSCR = crate::Reg<dscr::DSCR_SPEC>;
#[doc = "Descriptor Base Register"]
pub mod dscr;
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
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
