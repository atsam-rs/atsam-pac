#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0340],
    #[doc = "0x340 - Chip ID Register"]
    pub cidr: crate::Reg<cidr::CIDR_SPEC>,
    #[doc = "0x344 - Chip ID Extension Register"]
    pub exid: crate::Reg<exid::EXID_SPEC>,
}
#[doc = "CIDR register accessor: an alias for `Reg<CIDR_SPEC>`"]
pub type CIDR = crate::Reg<cidr::CIDR_SPEC>;
#[doc = "Chip ID Register"]
pub mod cidr;
#[doc = "EXID register accessor: an alias for `Reg<EXID_SPEC>`"]
pub type EXID = crate::Reg<exid::EXID_SPEC>;
#[doc = "Chip ID Extension Register"]
pub mod exid;
