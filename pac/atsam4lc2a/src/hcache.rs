#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x0c - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    _reserved2: [u8; 16usize],
    #[doc = "0x20 - Maintenance Register 0"]
    pub maint0: crate::Reg<maint0::MAINT0_SPEC>,
    #[doc = "0x24 - Maintenance Register 1"]
    pub maint1: crate::Reg<maint1::MAINT1_SPEC>,
    #[doc = "0x28 - Monitor Configuration Register"]
    pub mcfg: crate::Reg<mcfg::MCFG_SPEC>,
    #[doc = "0x2c - Monitor Enable Register"]
    pub men: crate::Reg<men::MEN_SPEC>,
    #[doc = "0x30 - Monitor Control Register"]
    pub mctrl: crate::Reg<mctrl::MCTRL_SPEC>,
    #[doc = "0x34 - Monitor Status Register"]
    pub msr: crate::Reg<msr::MSR_SPEC>,
    _reserved8: [u8; 196usize],
    #[doc = "0xfc - Version Register"]
    pub version: crate::Reg<version::VERSION_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "MAINT0 register accessor: an alias for `Reg<MAINT0_SPEC>`"]
pub type MAINT0 = crate::Reg<maint0::MAINT0_SPEC>;
#[doc = "Maintenance Register 0"]
pub mod maint0;
#[doc = "MAINT1 register accessor: an alias for `Reg<MAINT1_SPEC>`"]
pub type MAINT1 = crate::Reg<maint1::MAINT1_SPEC>;
#[doc = "Maintenance Register 1"]
pub mod maint1;
#[doc = "MCFG register accessor: an alias for `Reg<MCFG_SPEC>`"]
pub type MCFG = crate::Reg<mcfg::MCFG_SPEC>;
#[doc = "Monitor Configuration Register"]
pub mod mcfg;
#[doc = "MCTRL register accessor: an alias for `Reg<MCTRL_SPEC>`"]
pub type MCTRL = crate::Reg<mctrl::MCTRL_SPEC>;
#[doc = "Monitor Control Register"]
pub mod mctrl;
#[doc = "MEN register accessor: an alias for `Reg<MEN_SPEC>`"]
pub type MEN = crate::Reg<men::MEN_SPEC>;
#[doc = "Monitor Enable Register"]
pub mod men;
#[doc = "MSR register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Monitor Status Register"]
pub mod msr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
