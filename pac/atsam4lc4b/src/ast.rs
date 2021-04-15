#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Counter Value"]
    pub cv: CV,
    #[doc = "0x08 - Status Register"]
    pub sr: SR,
    #[doc = "0x0c - Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x1c - Wake Enable Register"]
    pub wer: WER,
    #[doc = "0x20 - Alarm Register 0"]
    pub ar0: AR0,
    #[doc = "0x24 - Alarm Register 1"]
    pub ar1: AR1,
    _reserved10: [u8; 8usize],
    #[doc = "0x30 - Periodic Interval Register 0"]
    pub pir0: PIR0,
    #[doc = "0x34 - Periodic Interval Register 1"]
    pub pir1: PIR1,
    _reserved12: [u8; 8usize],
    #[doc = "0x40 - Clock Control Register"]
    pub clock: CLOCK,
    #[doc = "0x44 - Digital Tuner Register"]
    pub dtr: DTR,
    #[doc = "0x48 - Event Enable Register"]
    pub eve: EVE,
    #[doc = "0x4c - Event Disable Register"]
    pub evd: EVD,
    #[doc = "0x50 - Event Mask Register"]
    pub evm: EVM,
    #[doc = "0x54 - Calendar Value"]
    pub calv: CALV,
    _reserved18: [u8; 152usize],
    #[doc = "0xf0 - Parameter Register"]
    pub parameter: PARAMETER,
    _reserved19: [u8; 8usize],
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
#[doc = "Alarm Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ar0](ar0) module"]
pub type AR0 = crate::Reg<u32, _AR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AR0;
#[doc = "`read()` method returns [ar0::R](ar0::R) reader structure"]
impl crate::Readable for AR0 {}
#[doc = "`write(|w| ..)` method takes [ar0::W](ar0::W) writer structure"]
impl crate::Writable for AR0 {}
#[doc = "Alarm Register 0"]
pub mod ar0;
#[doc = "Alarm Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ar1](ar1) module"]
pub type AR1 = crate::Reg<u32, _AR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AR1;
#[doc = "`read()` method returns [ar1::R](ar1::R) reader structure"]
impl crate::Readable for AR1 {}
#[doc = "`write(|w| ..)` method takes [ar1::W](ar1::W) writer structure"]
impl crate::Writable for AR1 {}
#[doc = "Alarm Register 1"]
pub mod ar1;
#[doc = "Calendar Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calv](calv) module"]
pub type CALV = crate::Reg<u32, _CALV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALV;
#[doc = "`read()` method returns [calv::R](calv::R) reader structure"]
impl crate::Readable for CALV {}
#[doc = "`write(|w| ..)` method takes [calv::W](calv::W) writer structure"]
impl crate::Writable for CALV {}
#[doc = "Calendar Value"]
pub mod calv;
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock](clock) module"]
pub type CLOCK = crate::Reg<u32, _CLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK;
#[doc = "`read()` method returns [clock::R](clock::R) reader structure"]
impl crate::Readable for CLOCK {}
#[doc = "`write(|w| ..)` method takes [clock::W](clock::W) writer structure"]
impl crate::Writable for CLOCK {}
#[doc = "Clock Control Register"]
pub mod clock;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cv](cv) module"]
pub type CV = crate::Reg<u32, _CV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CV;
#[doc = "`read()` method returns [cv::R](cv::R) reader structure"]
impl crate::Readable for CV {}
#[doc = "`write(|w| ..)` method takes [cv::W](cv::W) writer structure"]
impl crate::Writable for CV {}
#[doc = "Counter Value"]
pub mod cv;
#[doc = "Digital Tuner Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtr](dtr) module"]
pub type DTR = crate::Reg<u32, _DTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTR;
#[doc = "`read()` method returns [dtr::R](dtr::R) reader structure"]
impl crate::Readable for DTR {}
#[doc = "`write(|w| ..)` method takes [dtr::W](dtr::W) writer structure"]
impl crate::Writable for DTR {}
#[doc = "Digital Tuner Register"]
pub mod dtr;
#[doc = "Event Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evd](evd) module"]
pub type EVD = crate::Reg<u32, _EVD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVD;
#[doc = "`write(|w| ..)` method takes [evd::W](evd::W) writer structure"]
impl crate::Writable for EVD {}
#[doc = "Event Disable Register"]
pub mod evd;
#[doc = "Event Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eve](eve) module"]
pub type EVE = crate::Reg<u32, _EVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVE;
#[doc = "`write(|w| ..)` method takes [eve::W](eve::W) writer structure"]
impl crate::Writable for EVE {}
#[doc = "Event Enable Register"]
pub mod eve;
#[doc = "Event Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evm](evm) module"]
pub type EVM = crate::Reg<u32, _EVM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVM;
#[doc = "`read()` method returns [evm::R](evm::R) reader structure"]
impl crate::Readable for EVM {}
#[doc = "Event Mask Register"]
pub mod evm;
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
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [parameter](parameter) module"]
pub type PARAMETER = crate::Reg<u32, _PARAMETER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PARAMETER;
#[doc = "`read()` method returns [parameter::R](parameter::R) reader structure"]
impl crate::Readable for PARAMETER {}
#[doc = "Parameter Register"]
pub mod parameter;
#[doc = "Periodic Interval Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pir0](pir0) module"]
pub type PIR0 = crate::Reg<u32, _PIR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIR0;
#[doc = "`read()` method returns [pir0::R](pir0::R) reader structure"]
impl crate::Readable for PIR0 {}
#[doc = "`write(|w| ..)` method takes [pir0::W](pir0::W) writer structure"]
impl crate::Writable for PIR0 {}
#[doc = "Periodic Interval Register 0"]
pub mod pir0;
#[doc = "Periodic Interval Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pir1](pir1) module"]
pub type PIR1 = crate::Reg<u32, _PIR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIR1;
#[doc = "`read()` method returns [pir1::R](pir1::R) reader structure"]
impl crate::Readable for PIR1 {}
#[doc = "`write(|w| ..)` method takes [pir1::W](pir1::W) writer structure"]
impl crate::Writable for PIR1 {}
#[doc = "Periodic Interval Register 1"]
pub mod pir1;
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
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "Version Register"]
pub mod version;
#[doc = "Wake Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wer](wer) module"]
pub type WER = crate::Reg<u32, _WER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WER;
#[doc = "`read()` method returns [wer::R](wer::R) reader structure"]
impl crate::Readable for WER {}
#[doc = "`write(|w| ..)` method takes [wer::W](wer::W) writer structure"]
impl crate::Writable for WER {}
#[doc = "Wake Enable Register"]
pub mod wer;
