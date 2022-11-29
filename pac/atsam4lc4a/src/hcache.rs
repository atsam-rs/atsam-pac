#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Status Register"]
    pub sr: SR,
    _reserved2: [u8; 0x10],
    #[doc = "0x20 - Maintenance Register 0"]
    pub maint0: MAINT0,
    #[doc = "0x24 - Maintenance Register 1"]
    pub maint1: MAINT1,
    #[doc = "0x28 - Monitor Configuration Register"]
    pub mcfg: MCFG,
    #[doc = "0x2c - Monitor Enable Register"]
    pub men: MEN,
    #[doc = "0x30 - Monitor Control Register"]
    pub mctrl: MCTRL,
    #[doc = "0x34 - Monitor Status Register"]
    pub msr: MSR,
    _reserved8: [u8; 0xc4],
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
#[doc = "CTRL (w) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "MAINT0 (w) register accessor: an alias for `Reg<MAINT0_SPEC>`"]
pub type MAINT0 = crate::Reg<maint0::MAINT0_SPEC>;
#[doc = "Maintenance Register 0"]
pub mod maint0;
#[doc = "MAINT1 (w) register accessor: an alias for `Reg<MAINT1_SPEC>`"]
pub type MAINT1 = crate::Reg<maint1::MAINT1_SPEC>;
#[doc = "Maintenance Register 1"]
pub mod maint1;
#[doc = "MCFG (rw) register accessor: an alias for `Reg<MCFG_SPEC>`"]
pub type MCFG = crate::Reg<mcfg::MCFG_SPEC>;
#[doc = "Monitor Configuration Register"]
pub mod mcfg;
#[doc = "MCTRL (w) register accessor: an alias for `Reg<MCTRL_SPEC>`"]
pub type MCTRL = crate::Reg<mctrl::MCTRL_SPEC>;
#[doc = "Monitor Control Register"]
pub mod mctrl;
#[doc = "MEN (rw) register accessor: an alias for `Reg<MEN_SPEC>`"]
pub type MEN = crate::Reg<men::MEN_SPEC>;
#[doc = "Monitor Enable Register"]
pub mod men;
#[doc = "MSR (r) register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Monitor Status Register"]
pub mod msr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
