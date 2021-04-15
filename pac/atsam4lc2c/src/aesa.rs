#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Mode Register"]
    pub mode: MODE,
    #[doc = "0x08 - Data Buffer Pointer Register"]
    pub databufptr: DATABUFPTR,
    #[doc = "0x0c - Status Register"]
    pub sr: SR,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub imr: IMR,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Key Register"]
    pub key: [KEY; 8],
    #[doc = "0x40 - Initialization Vector Register"]
    pub initvect: [INITVECT; 4],
    #[doc = "0x50 - Input Data Register"]
    pub idata: IDATA,
    _reserved10: [u8; 12usize],
    #[doc = "0x60 - Output Data Register"]
    pub odata: ODATA,
    _reserved11: [u8; 12usize],
    #[doc = "0x70 - DRNG Seed Register"]
    pub drngseed: DRNGSEED,
    _reserved12: [u8; 132usize],
    #[doc = "0xf8 - Parameter Register"]
    pub parameter: PARAMETER,
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Data Buffer Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [databufptr](databufptr) module"]
pub type DATABUFPTR = crate::Reg<u32, _DATABUFPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATABUFPTR;
#[doc = "`read()` method returns [databufptr::R](databufptr::R) reader structure"]
impl crate::Readable for DATABUFPTR {}
#[doc = "`write(|w| ..)` method takes [databufptr::W](databufptr::W) writer structure"]
impl crate::Writable for DATABUFPTR {}
#[doc = "Data Buffer Pointer Register"]
pub mod databufptr;
#[doc = "DRNG Seed Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drngseed](drngseed) module"]
pub type DRNGSEED = crate::Reg<u32, _DRNGSEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DRNGSEED;
#[doc = "`write(|w| ..)` method takes [drngseed::W](drngseed::W) writer structure"]
impl crate::Writable for DRNGSEED {}
#[doc = "DRNG Seed Register"]
pub mod drngseed;
#[doc = "Input Data Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idata](idata) module"]
pub type IDATA = crate::Reg<u32, _IDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDATA;
#[doc = "`write(|w| ..)` method takes [idata::W](idata::W) writer structure"]
impl crate::Writable for IDATA {}
#[doc = "Input Data Register"]
pub mod idata;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Initialization Vector Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [initvect](initvect) module"]
pub type INITVECT = crate::Reg<u32, _INITVECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INITVECT;
#[doc = "`write(|w| ..)` method takes [initvect::W](initvect::W) writer structure"]
impl crate::Writable for INITVECT {}
#[doc = "Initialization Vector Register"]
pub mod initvect;
#[doc = "Key Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key](key) module"]
pub type KEY = crate::Reg<u32, _KEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY;
#[doc = "`write(|w| ..)` method takes [key::W](key::W) writer structure"]
impl crate::Writable for KEY {}
#[doc = "Key Register"]
pub mod key;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "Mode Register"]
pub mod mode;
#[doc = "Output Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odata](odata) module"]
pub type ODATA = crate::Reg<u32, _ODATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODATA;
#[doc = "`read()` method returns [odata::R](odata::R) reader structure"]
impl crate::Readable for ODATA {}
#[doc = "Output Data Register"]
pub mod odata;
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [parameter](parameter) module"]
pub type PARAMETER = crate::Reg<u32, _PARAMETER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PARAMETER;
#[doc = "`read()` method returns [parameter::R](parameter::R) reader structure"]
impl crate::Readable for PARAMETER {}
#[doc = "Parameter Register"]
pub mod parameter;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "Version Register"]
pub mod version;
