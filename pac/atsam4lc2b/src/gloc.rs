#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr0: CR,
    #[doc = "0x04 - Truth Register"]
    pub truth0: TRUTH,
    #[doc = "0x08 - Control Register"]
    pub cr1: CR,
    #[doc = "0x0c - Truth Register"]
    pub truth1: TRUTH,
    _reserved4: [u8; 0x28],
    #[doc = "0x38 - Parameter Register"]
    pub parameter: PARAMETER,
    #[doc = "0x3c - Version Register"]
    pub version: VERSION,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "PARAMETER (r) register accessor: an alias for `Reg<PARAMETER_SPEC>`"]
pub type PARAMETER = crate::Reg<parameter::PARAMETER_SPEC>;
#[doc = "Parameter Register"]
pub mod parameter;
#[doc = "TRUTH (rw) register accessor: an alias for `Reg<TRUTH_SPEC>`"]
pub type TRUTH = crate::Reg<truth::TRUTH_SPEC>;
#[doc = "Truth Register"]
pub mod truth;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
