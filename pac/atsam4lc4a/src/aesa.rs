#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x08 - Data Buffer Pointer Register"]
    pub databufptr: crate::Reg<databufptr::DATABUFPTR_SPEC>,
    #[doc = "0x0c - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20..0x40 - Key Register"]
    pub key: [crate::Reg<key::KEY_SPEC>; 8],
    #[doc = "0x40..0x50 - Initialization Vector Register"]
    pub initvect: [crate::Reg<initvect::INITVECT_SPEC>; 4],
    #[doc = "0x50 - Input Data Register"]
    pub idata: crate::Reg<idata::IDATA_SPEC>,
    _reserved10: [u8; 0x0c],
    #[doc = "0x60 - Output Data Register"]
    pub odata: crate::Reg<odata::ODATA_SPEC>,
    _reserved11: [u8; 0x0c],
    #[doc = "0x70 - DRNG Seed Register"]
    pub drngseed: crate::Reg<drngseed::DRNGSEED_SPEC>,
    _reserved12: [u8; 0x84],
    #[doc = "0xf8 - Parameter Register"]
    pub parameter: crate::Reg<parameter::PARAMETER_SPEC>,
    #[doc = "0xfc - Version Register"]
    pub version: crate::Reg<version::VERSION_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "DATABUFPTR register accessor: an alias for `Reg<DATABUFPTR_SPEC>`"]
pub type DATABUFPTR = crate::Reg<databufptr::DATABUFPTR_SPEC>;
#[doc = "Data Buffer Pointer Register"]
pub mod databufptr;
#[doc = "DRNGSEED register accessor: an alias for `Reg<DRNGSEED_SPEC>`"]
pub type DRNGSEED = crate::Reg<drngseed::DRNGSEED_SPEC>;
#[doc = "DRNG Seed Register"]
pub mod drngseed;
#[doc = "IDATA register accessor: an alias for `Reg<IDATA_SPEC>`"]
pub type IDATA = crate::Reg<idata::IDATA_SPEC>;
#[doc = "Input Data Register"]
pub mod idata;
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
#[doc = "INITVECT register accessor: an alias for `Reg<INITVECT_SPEC>`"]
pub type INITVECT = crate::Reg<initvect::INITVECT_SPEC>;
#[doc = "Initialization Vector Register"]
pub mod initvect;
#[doc = "KEY register accessor: an alias for `Reg<KEY_SPEC>`"]
pub type KEY = crate::Reg<key::KEY_SPEC>;
#[doc = "Key Register"]
pub mod key;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Mode Register"]
pub mod mode;
#[doc = "ODATA register accessor: an alias for `Reg<ODATA_SPEC>`"]
pub type ODATA = crate::Reg<odata::ODATA_SPEC>;
#[doc = "Output Data Register"]
pub mod odata;
#[doc = "PARAMETER register accessor: an alias for `Reg<PARAMETER_SPEC>`"]
pub type PARAMETER = crate::Reg<parameter::PARAMETER_SPEC>;
#[doc = "Parameter Register"]
pub mod parameter;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
