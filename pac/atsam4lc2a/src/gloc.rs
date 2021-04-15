#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr0: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Truth Register"]
    pub truth0: crate::Reg<truth::TRUTH_SPEC>,
    #[doc = "0x08 - Control Register"]
    pub cr1: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x0c - Truth Register"]
    pub truth1: crate::Reg<truth::TRUTH_SPEC>,
    _reserved4: [u8; 40usize],
    #[doc = "0x38 - Parameter Register"]
    pub parameter: crate::Reg<parameter::PARAMETER_SPEC>,
    #[doc = "0x3c - Version Register"]
    pub version: crate::Reg<version::VERSION_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "PARAMETER register accessor: an alias for `Reg<PARAMETER_SPEC>`"]
pub type PARAMETER = crate::Reg<parameter::PARAMETER_SPEC>;
#[doc = "Parameter Register"]
pub mod parameter;
#[doc = "TRUTH register accessor: an alias for `Reg<TRUTH_SPEC>`"]
pub type TRUTH = crate::Reg<truth::TRUTH_SPEC>;
#[doc = "Truth Register"]
pub mod truth;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
