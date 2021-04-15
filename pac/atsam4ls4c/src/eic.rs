#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x04 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x08 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x0c - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x10 - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x14 - Mode Register"]
    pub mode: MODE,
    #[doc = "0x18 - Edge Register"]
    pub edge: EDGE,
    #[doc = "0x1c - Level Register"]
    pub level: LEVEL,
    #[doc = "0x20 - Filter Register"]
    pub filter: FILTER,
    _reserved9: [u8; 4usize],
    #[doc = "0x28 - Asynchronous Register"]
    pub async_: ASYNC,
    _reserved10: [u8; 4usize],
    #[doc = "0x30 - Enable Register"]
    pub en: EN,
    #[doc = "0x34 - Disable Register"]
    pub dis: DIS,
    #[doc = "0x38 - Control Register"]
    pub ctrl: CTRL,
    _reserved13: [u8; 960usize],
    #[doc = "0x3fc - Version Register"]
    pub version: VERSION,
}
#[doc = "Asynchronous Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [async_](async_) module"]
pub type ASYNC = crate::Reg<u32, _ASYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASYNC;
#[doc = "`read()` method returns [async_::R](async_::R) reader structure"]
impl crate::Readable for ASYNC {}
#[doc = "`write(|w| ..)` method takes [async_::W](async_::W) writer structure"]
impl crate::Writable for ASYNC {}
#[doc = "Asynchronous Register"]
pub mod async_;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dis](dis) module"]
pub type DIS = crate::Reg<u32, _DIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIS;
#[doc = "`write(|w| ..)` method takes [dis::W](dis::W) writer structure"]
impl crate::Writable for DIS {}
#[doc = "Disable Register"]
pub mod dis;
#[doc = "Edge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edge](edge) module"]
pub type EDGE = crate::Reg<u32, _EDGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EDGE;
#[doc = "`read()` method returns [edge::R](edge::R) reader structure"]
impl crate::Readable for EDGE {}
#[doc = "`write(|w| ..)` method takes [edge::W](edge::W) writer structure"]
impl crate::Writable for EDGE {}
#[doc = "Edge Register"]
pub mod edge;
#[doc = "Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en](en) module"]
pub type EN = crate::Reg<u32, _EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN;
#[doc = "`write(|w| ..)` method takes [en::W](en::W) writer structure"]
impl crate::Writable for EN {}
#[doc = "Enable Register"]
pub mod en;
#[doc = "Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter](filter) module"]
pub type FILTER = crate::Reg<u32, _FILTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILTER;
#[doc = "`read()` method returns [filter::R](filter::R) reader structure"]
impl crate::Readable for FILTER {}
#[doc = "`write(|w| ..)` method takes [filter::W](filter::W) writer structure"]
impl crate::Writable for FILTER {}
#[doc = "Filter Register"]
pub mod filter;
#[doc = "Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "Interrupt Clear Register"]
pub mod icr;
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
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [level](level) module"]
pub type LEVEL = crate::Reg<u32, _LEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEVEL;
#[doc = "`read()` method returns [level::R](level::R) reader structure"]
impl crate::Readable for LEVEL {}
#[doc = "`write(|w| ..)` method takes [level::W](level::W) writer structure"]
impl crate::Writable for LEVEL {}
#[doc = "Level Register"]
pub mod level;
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
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "Version Register"]
pub mod version;
