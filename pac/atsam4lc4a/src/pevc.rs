#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Status Register"]
    pub chsr: crate::Reg<chsr::CHSR_SPEC>,
    #[doc = "0x04 - Channel Enable Register"]
    pub cher: crate::Reg<cher::CHER_SPEC>,
    #[doc = "0x08 - Channel Disable Register"]
    pub chdr: crate::Reg<chdr::CHDR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Software Event"]
    pub sev: crate::Reg<sev::SEV_SPEC>,
    #[doc = "0x14 - Channel / User Busy"]
    pub busy: crate::Reg<busy::BUSY_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - Trigger Interrupt Mask Enable Register"]
    pub trier: crate::Reg<trier::TRIER_SPEC>,
    #[doc = "0x24 - Trigger Interrupt Mask Disable Register"]
    pub tridr: crate::Reg<tridr::TRIDR_SPEC>,
    #[doc = "0x28 - Trigger Interrupt Mask Register"]
    pub trimr: crate::Reg<trimr::TRIMR_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x30 - Trigger Status Register"]
    pub trsr: crate::Reg<trsr::TRSR_SPEC>,
    #[doc = "0x34 - Trigger Status Clear Register"]
    pub trscr: crate::Reg<trscr::TRSCR_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x40 - Overrun Interrupt Mask Enable Register"]
    pub ovier: crate::Reg<ovier::OVIER_SPEC>,
    #[doc = "0x44 - Overrun Interrupt Mask Disable Register"]
    pub ovidr: crate::Reg<ovidr::OVIDR_SPEC>,
    #[doc = "0x48 - Overrun Interrupt Mask Register"]
    pub ovimr: crate::Reg<ovimr::OVIMR_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x50 - Overrun Status Register"]
    pub ovsr: crate::Reg<ovsr::OVSR_SPEC>,
    #[doc = "0x54 - Overrun Status Clear Register"]
    pub ovscr: crate::Reg<ovscr::OVSCR_SPEC>,
    _reserved15: [u8; 0xa8],
    #[doc = "0x100..0x14c - Channel Multiplexer"]
    pub chmx: [crate::Reg<chmx::CHMX_SPEC>; 19],
    _reserved16: [u8; 0xb4],
    #[doc = "0x200..0x27c - Event Shaper"]
    pub evs: [crate::Reg<evs::EVS_SPEC>; 31],
    _reserved17: [u8; 0x84],
    #[doc = "0x300 - Input Glitch Filter Divider Register"]
    pub igfdr: crate::Reg<igfdr::IGFDR_SPEC>,
    _reserved18: [u8; 0xf4],
    #[doc = "0x3f8 - Parameter"]
    pub parameter: crate::Reg<parameter::PARAMETER_SPEC>,
    #[doc = "0x3fc - Version"]
    pub version: crate::Reg<version::VERSION_SPEC>,
}
#[doc = "BUSY register accessor: an alias for `Reg<BUSY_SPEC>`"]
pub type BUSY = crate::Reg<busy::BUSY_SPEC>;
#[doc = "Channel / User Busy"]
pub mod busy;
#[doc = "CHDR register accessor: an alias for `Reg<CHDR_SPEC>`"]
pub type CHDR = crate::Reg<chdr::CHDR_SPEC>;
#[doc = "Channel Disable Register"]
pub mod chdr;
#[doc = "CHER register accessor: an alias for `Reg<CHER_SPEC>`"]
pub type CHER = crate::Reg<cher::CHER_SPEC>;
#[doc = "Channel Enable Register"]
pub mod cher;
#[doc = "CHMX register accessor: an alias for `Reg<CHMX_SPEC>`"]
pub type CHMX = crate::Reg<chmx::CHMX_SPEC>;
#[doc = "Channel Multiplexer"]
pub mod chmx;
#[doc = "CHSR register accessor: an alias for `Reg<CHSR_SPEC>`"]
pub type CHSR = crate::Reg<chsr::CHSR_SPEC>;
#[doc = "Channel Status Register"]
pub mod chsr;
#[doc = "EVS register accessor: an alias for `Reg<EVS_SPEC>`"]
pub type EVS = crate::Reg<evs::EVS_SPEC>;
#[doc = "Event Shaper"]
pub mod evs;
#[doc = "IGFDR register accessor: an alias for `Reg<IGFDR_SPEC>`"]
pub type IGFDR = crate::Reg<igfdr::IGFDR_SPEC>;
#[doc = "Input Glitch Filter Divider Register"]
pub mod igfdr;
#[doc = "OVIDR register accessor: an alias for `Reg<OVIDR_SPEC>`"]
pub type OVIDR = crate::Reg<ovidr::OVIDR_SPEC>;
#[doc = "Overrun Interrupt Mask Disable Register"]
pub mod ovidr;
#[doc = "OVIER register accessor: an alias for `Reg<OVIER_SPEC>`"]
pub type OVIER = crate::Reg<ovier::OVIER_SPEC>;
#[doc = "Overrun Interrupt Mask Enable Register"]
pub mod ovier;
#[doc = "OVIMR register accessor: an alias for `Reg<OVIMR_SPEC>`"]
pub type OVIMR = crate::Reg<ovimr::OVIMR_SPEC>;
#[doc = "Overrun Interrupt Mask Register"]
pub mod ovimr;
#[doc = "OVSCR register accessor: an alias for `Reg<OVSCR_SPEC>`"]
pub type OVSCR = crate::Reg<ovscr::OVSCR_SPEC>;
#[doc = "Overrun Status Clear Register"]
pub mod ovscr;
#[doc = "OVSR register accessor: an alias for `Reg<OVSR_SPEC>`"]
pub type OVSR = crate::Reg<ovsr::OVSR_SPEC>;
#[doc = "Overrun Status Register"]
pub mod ovsr;
#[doc = "PARAMETER register accessor: an alias for `Reg<PARAMETER_SPEC>`"]
pub type PARAMETER = crate::Reg<parameter::PARAMETER_SPEC>;
#[doc = "Parameter"]
pub mod parameter;
#[doc = "SEV register accessor: an alias for `Reg<SEV_SPEC>`"]
pub type SEV = crate::Reg<sev::SEV_SPEC>;
#[doc = "Software Event"]
pub mod sev;
#[doc = "TRIDR register accessor: an alias for `Reg<TRIDR_SPEC>`"]
pub type TRIDR = crate::Reg<tridr::TRIDR_SPEC>;
#[doc = "Trigger Interrupt Mask Disable Register"]
pub mod tridr;
#[doc = "TRIER register accessor: an alias for `Reg<TRIER_SPEC>`"]
pub type TRIER = crate::Reg<trier::TRIER_SPEC>;
#[doc = "Trigger Interrupt Mask Enable Register"]
pub mod trier;
#[doc = "TRIMR register accessor: an alias for `Reg<TRIMR_SPEC>`"]
pub type TRIMR = crate::Reg<trimr::TRIMR_SPEC>;
#[doc = "Trigger Interrupt Mask Register"]
pub mod trimr;
#[doc = "TRSCR register accessor: an alias for `Reg<TRSCR_SPEC>`"]
pub type TRSCR = crate::Reg<trscr::TRSCR_SPEC>;
#[doc = "Trigger Status Clear Register"]
pub mod trscr;
#[doc = "TRSR register accessor: an alias for `Reg<TRSR_SPEC>`"]
pub type TRSR = crate::Reg<trsr::TRSR_SPEC>;
#[doc = "Trigger Status Register"]
pub mod trsr;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version"]
pub mod version;
