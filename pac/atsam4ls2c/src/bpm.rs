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
    #[doc = "0x14 - Status Register"]
    pub sr: SR,
    #[doc = "0x18 - Unlock Register"]
    pub unlock: UNLOCK,
    #[doc = "0x1c - Power Mode Control Register"]
    pub pmcon: PMCON,
    _reserved8: [u8; 8usize],
    #[doc = "0x28 - Backup Wake up Cause Register"]
    pub bkupwcause: BKUPWCAUSE,
    #[doc = "0x2c - Backup Wake up Enable Register"]
    pub bkupwen: BKUPWEN,
    #[doc = "0x30 - Backup Pin Muxing Register"]
    pub bkuppmux: BKUPPMUX,
    #[doc = "0x34 - Input Output Retention Register"]
    pub ioret: IORET,
    _reserved12: [u8; 8usize],
    #[doc = "0x40 - Bypass Register"]
    pub bpr: BPR,
    #[doc = "0x44 - Factory Word Run PS Register"]
    pub fwrunps: FWRUNPS,
    #[doc = "0x48 - Factory Word Power Save PS Register"]
    pub fwpsaveps: FWPSAVEPS,
    _reserved15: [u8; 176usize],
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
#[doc = "Backup Pin Muxing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkuppmux](bkuppmux) module"]
pub type BKUPPMUX = crate::Reg<u32, _BKUPPMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKUPPMUX;
#[doc = "`read()` method returns [bkuppmux::R](bkuppmux::R) reader structure"]
impl crate::Readable for BKUPPMUX {}
#[doc = "`write(|w| ..)` method takes [bkuppmux::W](bkuppmux::W) writer structure"]
impl crate::Writable for BKUPPMUX {}
#[doc = "Backup Pin Muxing Register"]
pub mod bkuppmux;
#[doc = "Backup Wake up Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkupwcause](bkupwcause) module"]
pub type BKUPWCAUSE = crate::Reg<u32, _BKUPWCAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKUPWCAUSE;
#[doc = "`read()` method returns [bkupwcause::R](bkupwcause::R) reader structure"]
impl crate::Readable for BKUPWCAUSE {}
#[doc = "Backup Wake up Cause Register"]
pub mod bkupwcause;
#[doc = "Backup Wake up Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkupwen](bkupwen) module"]
pub type BKUPWEN = crate::Reg<u32, _BKUPWEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKUPWEN;
#[doc = "`read()` method returns [bkupwen::R](bkupwen::R) reader structure"]
impl crate::Readable for BKUPWEN {}
#[doc = "`write(|w| ..)` method takes [bkupwen::W](bkupwen::W) writer structure"]
impl crate::Writable for BKUPWEN {}
#[doc = "Backup Wake up Enable Register"]
pub mod bkupwen;
#[doc = "Bypass Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpr](bpr) module"]
pub type BPR = crate::Reg<u32, _BPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BPR;
#[doc = "`read()` method returns [bpr::R](bpr::R) reader structure"]
impl crate::Readable for BPR {}
#[doc = "`write(|w| ..)` method takes [bpr::W](bpr::W) writer structure"]
impl crate::Writable for BPR {}
#[doc = "Bypass Register"]
pub mod bpr;
#[doc = "Factory Word Power Save PS Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwpsaveps](fwpsaveps) module"]
pub type FWPSAVEPS = crate::Reg<u32, _FWPSAVEPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWPSAVEPS;
#[doc = "`read()` method returns [fwpsaveps::R](fwpsaveps::R) reader structure"]
impl crate::Readable for FWPSAVEPS {}
#[doc = "Factory Word Power Save PS Register"]
pub mod fwpsaveps;
#[doc = "Factory Word Run PS Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwrunps](fwrunps) module"]
pub type FWRUNPS = crate::Reg<u32, _FWRUNPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWRUNPS;
#[doc = "`read()` method returns [fwrunps::R](fwrunps::R) reader structure"]
impl crate::Readable for FWRUNPS {}
#[doc = "Factory Word Run PS Register"]
pub mod fwrunps;
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
#[doc = "Input Output Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioret](ioret) module"]
pub type IORET = crate::Reg<u32, _IORET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IORET;
#[doc = "`read()` method returns [ioret::R](ioret::R) reader structure"]
impl crate::Readable for IORET {}
#[doc = "`write(|w| ..)` method takes [ioret::W](ioret::W) writer structure"]
impl crate::Writable for IORET {}
#[doc = "Input Output Retention Register"]
pub mod ioret;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Power Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmcon](pmcon) module"]
pub type PMCON = crate::Reg<u32, _PMCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMCON;
#[doc = "`read()` method returns [pmcon::R](pmcon::R) reader structure"]
impl crate::Readable for PMCON {}
#[doc = "`write(|w| ..)` method takes [pmcon::W](pmcon::W) writer structure"]
impl crate::Writable for PMCON {}
#[doc = "Power Mode Control Register"]
pub mod pmcon;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Unlock Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unlock](unlock) module"]
pub type UNLOCK = crate::Reg<u32, _UNLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNLOCK;
#[doc = "`write(|w| ..)` method takes [unlock::W](unlock::W) writer structure"]
impl crate::Writable for UNLOCK {}
#[doc = "Unlock Register"]
pub mod unlock;
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "Version Register"]
pub mod version;
