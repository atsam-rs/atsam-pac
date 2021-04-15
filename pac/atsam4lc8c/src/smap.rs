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
    _reserved6: [u8; 16usize],
    #[doc = "0x28 - VERSION register"]
    pub version: VERSION,
    _reserved7: [u8; 196usize],
    #[doc = "0xf0 - Chip ID Register"]
    pub cidr: CIDR,
    #[doc = "0xf4 - Chip ID Extension Register"]
    pub exid: EXID,
    _reserved9: [u8; 4usize],
    #[doc = "0xfc - AP Identification register"]
    pub idr: IDR,
}
#[doc = "Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "Address Register"]
pub mod addr;
#[doc = "Chip ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr](cidr) module"]
pub type CIDR = crate::Reg<u32, _CIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR;
#[doc = "`read()` method returns [cidr::R](cidr::R) reader structure"]
impl crate::Readable for CIDR {}
#[doc = "Chip ID Register"]
pub mod cidr;
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "Data Register"]
pub mod data;
#[doc = "Chip ID Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exid](exid) module"]
pub type EXID = crate::Reg<u32, _EXID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXID;
#[doc = "`read()` method returns [exid::R](exid::R) reader structure"]
impl crate::Readable for EXID {}
#[doc = "Chip ID Extension Register"]
pub mod exid;
#[doc = "AP Identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`read()` method returns [idr::R](idr::R) reader structure"]
impl crate::Readable for IDR {}
#[doc = "AP Identification register"]
pub mod idr;
#[doc = "Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [length](length) module"]
pub type LENGTH = crate::Reg<u32, _LENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LENGTH;
#[doc = "`read()` method returns [length::R](length::R) reader structure"]
impl crate::Readable for LENGTH {}
#[doc = "`write(|w| ..)` method takes [length::W](length::W) writer structure"]
impl crate::Writable for LENGTH {}
#[doc = "Length Register"]
pub mod length;
#[doc = "Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "Status Clear Register"]
pub mod scr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register"]
pub mod sr;
#[doc = "VERSION register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "VERSION register"]
pub mod version;
