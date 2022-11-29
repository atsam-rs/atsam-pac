#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Status Register"]
    pub sr: SR,
    #[doc = "0x08 - Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x0c - Address Register"]
    pub addr: ADDR,
    #[doc = "0x10 - Length Register"]
    pub length: LENGTH,
    #[doc = "0x14 - Data Register"]
    pub data: DATA,
    _reserved6: [u8; 0x10],
    #[doc = "0x28 - VERSION register"]
    pub version: VERSION,
    _reserved7: [u8; 0xc4],
    #[doc = "0xf0 - Chip ID Register"]
    pub cidr: CIDR,
    #[doc = "0xf4 - Chip ID Extension Register"]
    pub exid: EXID,
    _reserved9: [u8; 0x04],
    #[doc = "0xfc - AP Identification register"]
    pub idr: IDR,
}
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address Register"]
pub mod addr;
#[doc = "CIDR (r) register accessor: an alias for `Reg<CIDR_SPEC>`"]
pub type CIDR = crate::Reg<cidr::CIDR_SPEC>;
#[doc = "Chip ID Register"]
pub mod cidr;
#[doc = "CR (w) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data Register"]
pub mod data;
#[doc = "EXID (r) register accessor: an alias for `Reg<EXID_SPEC>`"]
pub type EXID = crate::Reg<exid::EXID_SPEC>;
#[doc = "Chip ID Extension Register"]
pub mod exid;
#[doc = "IDR (r) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "AP Identification register"]
pub mod idr;
#[doc = "LENGTH (rw) register accessor: an alias for `Reg<LENGTH_SPEC>`"]
pub type LENGTH = crate::Reg<length::LENGTH_SPEC>;
#[doc = "Length Register"]
pub mod length;
#[doc = "SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Status Clear Register"]
pub mod scr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "VERSION register"]
pub mod version;
