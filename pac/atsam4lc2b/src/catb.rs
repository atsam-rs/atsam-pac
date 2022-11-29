#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Counter Control Register"]
    pub cntcr: CNTCR,
    #[doc = "0x08 - Sensor Idle Level"]
    pub idle: IDLE,
    #[doc = "0x0c - Sensor Relative Level"]
    pub level: LEVEL,
    #[doc = "0x10 - Sensor Raw Value"]
    pub raw: RAW,
    #[doc = "0x14 - Filter Timing Register"]
    pub timing: TIMING,
    #[doc = "0x18 - Threshold Register"]
    pub thresh: THRESH,
    #[doc = "0x1c - Pin Selection Register"]
    pub pinsel: PINSEL,
    #[doc = "0x20 - Direct Memory Access Register"]
    pub dma: DMA,
    #[doc = "0x24 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x2c - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x30 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x34 - Status Clear Register"]
    pub scr: SCR,
    _reserved14: [u8; 0x08],
    #[doc = "0x40 - In-Touch Status Register"]
    pub intch: [INTCH; 1],
    _reserved15: [u8; 0x0c],
    #[doc = "0x50 - In-Touch Status Clear Register"]
    pub intchclr: [INTCHCLR; 1],
    _reserved16: [u8; 0x0c],
    #[doc = "0x60 - Out-of-Touch Status Register"]
    pub outtch: [OUTTCH; 1],
    _reserved17: [u8; 0x0c],
    #[doc = "0x70 - Out-of-Touch Status Clear Register"]
    pub outtchclr: [OUTTCHCLR; 1],
    _reserved18: [u8; 0x84],
    #[doc = "0xf8 - Parameter Register"]
    pub parameter: PARAMETER,
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
#[doc = "CNTCR (rw) register accessor: an alias for `Reg<CNTCR_SPEC>`"]
pub type CNTCR = crate::Reg<cntcr::CNTCR_SPEC>;
#[doc = "Counter Control Register"]
pub mod cntcr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "DMA (rw) register accessor: an alias for `Reg<DMA_SPEC>`"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "Direct Memory Access Register"]
pub mod dma;
#[doc = "IDLE (rw) register accessor: an alias for `Reg<IDLE_SPEC>`"]
pub type IDLE = crate::Reg<idle::IDLE_SPEC>;
#[doc = "Sensor Idle Level"]
pub mod idle;
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
#[doc = "INTCHCLR (w) register accessor: an alias for `Reg<INTCHCLR_SPEC>`"]
pub type INTCHCLR = crate::Reg<intchclr::INTCHCLR_SPEC>;
#[doc = "In-Touch Status Clear Register"]
pub mod intchclr;
#[doc = "INTCH (r) register accessor: an alias for `Reg<INTCH_SPEC>`"]
pub type INTCH = crate::Reg<intch::INTCH_SPEC>;
#[doc = "In-Touch Status Register"]
pub mod intch;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "LEVEL (r) register accessor: an alias for `Reg<LEVEL_SPEC>`"]
pub type LEVEL = crate::Reg<level::LEVEL_SPEC>;
#[doc = "Sensor Relative Level"]
pub mod level;
#[doc = "OUTTCHCLR (w) register accessor: an alias for `Reg<OUTTCHCLR_SPEC>`"]
pub type OUTTCHCLR = crate::Reg<outtchclr::OUTTCHCLR_SPEC>;
#[doc = "Out-of-Touch Status Clear Register"]
pub mod outtchclr;
#[doc = "OUTTCH (r) register accessor: an alias for `Reg<OUTTCH_SPEC>`"]
pub type OUTTCH = crate::Reg<outtch::OUTTCH_SPEC>;
#[doc = "Out-of-Touch Status Register"]
pub mod outtch;
#[doc = "PARAMETER (r) register accessor: an alias for `Reg<PARAMETER_SPEC>`"]
pub type PARAMETER = crate::Reg<parameter::PARAMETER_SPEC>;
#[doc = "Parameter Register"]
pub mod parameter;
#[doc = "PINSEL (rw) register accessor: an alias for `Reg<PINSEL_SPEC>`"]
pub type PINSEL = crate::Reg<pinsel::PINSEL_SPEC>;
#[doc = "Pin Selection Register"]
pub mod pinsel;
#[doc = "RAW (r) register accessor: an alias for `Reg<RAW_SPEC>`"]
pub type RAW = crate::Reg<raw::RAW_SPEC>;
#[doc = "Sensor Raw Value"]
pub mod raw;
#[doc = "SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Status Clear Register"]
pub mod scr;
#[doc = "THRESH (rw) register accessor: an alias for `Reg<THRESH_SPEC>`"]
pub type THRESH = crate::Reg<thresh::THRESH_SPEC>;
#[doc = "Threshold Register"]
pub mod thresh;
#[doc = "TIMING (rw) register accessor: an alias for `Reg<TIMING_SPEC>`"]
pub type TIMING = crate::Reg<timing::TIMING_SPEC>;
#[doc = "Filter Timing Register"]
pub mod timing;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
