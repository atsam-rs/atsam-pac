#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - NBYTES Register"]
    pub nbytes: NBYTES,
    #[doc = "0x08 - Timing Register"]
    pub tr: TR,
    #[doc = "0x0c - Receive Holding Register"]
    pub rhr: RHR,
    #[doc = "0x10 - Transmit Holding Register"]
    pub thr: THR,
    #[doc = "0x14 - Packet Error Check Register"]
    pub pecr: PECR,
    #[doc = "0x18 - Status Register"]
    pub sr: SR,
    #[doc = "0x1c - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x20 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x24 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x28 - Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x2c - Parameter Register"]
    pub pr: PR,
    #[doc = "0x30 - Version Register"]
    pub vr: VR,
    #[doc = "0x34 - HS-mode Timing Register"]
    pub hstr: HSTR,
    #[doc = "0x38 - Slew Rate Register"]
    pub srr: SRR,
    #[doc = "0x3c - HS-mode Slew Rate Register"]
    pub hssrr: HSSRR,
}
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
#[doc = "HS-mode Slew Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hssrr](hssrr) module"]
pub type HSSRR = crate::Reg<u32, _HSSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSSRR;
#[doc = "`read()` method returns [hssrr::R](hssrr::R) reader structure"]
impl crate::Readable for HSSRR {}
#[doc = "`write(|w| ..)` method takes [hssrr::W](hssrr::W) writer structure"]
impl crate::Writable for HSSRR {}
#[doc = "HS-mode Slew Rate Register"]
pub mod hssrr;
#[doc = "HS-mode Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstr](hstr) module"]
pub type HSTR = crate::Reg<u32, _HSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTR;
#[doc = "`read()` method returns [hstr::R](hstr::R) reader structure"]
impl crate::Readable for HSTR {}
#[doc = "`write(|w| ..)` method takes [hstr::W](hstr::W) writer structure"]
impl crate::Writable for HSTR {}
#[doc = "HS-mode Timing Register"]
pub mod hstr;
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
#[doc = "NBYTES Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nbytes](nbytes) module"]
pub type NBYTES = crate::Reg<u32, _NBYTES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NBYTES;
#[doc = "`read()` method returns [nbytes::R](nbytes::R) reader structure"]
impl crate::Readable for NBYTES {}
#[doc = "`write(|w| ..)` method takes [nbytes::W](nbytes::W) writer structure"]
impl crate::Writable for NBYTES {}
#[doc = "NBYTES Register"]
pub mod nbytes;
#[doc = "Packet Error Check Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pecr](pecr) module"]
pub type PECR = crate::Reg<u32, _PECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PECR;
#[doc = "`read()` method returns [pecr::R](pecr::R) reader structure"]
impl crate::Readable for PECR {}
#[doc = "Packet Error Check Register"]
pub mod pecr;
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr](pr) module"]
pub type PR = crate::Reg<u32, _PR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR;
#[doc = "`read()` method returns [pr::R](pr::R) reader structure"]
impl crate::Readable for PR {}
#[doc = "Parameter Register"]
pub mod pr;
#[doc = "Receive Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rhr](rhr) module"]
pub type RHR = crate::Reg<u32, _RHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RHR;
#[doc = "`read()` method returns [rhr::R](rhr::R) reader structure"]
impl crate::Readable for RHR {}
#[doc = "Receive Holding Register"]
pub mod rhr;
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
#[doc = "Slew Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srr](srr) module"]
pub type SRR = crate::Reg<u32, _SRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRR;
#[doc = "`read()` method returns [srr::R](srr::R) reader structure"]
impl crate::Readable for SRR {}
#[doc = "`write(|w| ..)` method takes [srr::W](srr::W) writer structure"]
impl crate::Writable for SRR {}
#[doc = "Slew Rate Register"]
pub mod srr;
#[doc = "Transmit Holding Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr](thr) module"]
pub type THR = crate::Reg<u32, _THR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THR;
#[doc = "`write(|w| ..)` method takes [thr::W](thr::W) writer structure"]
impl crate::Writable for THR {}
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr](tr) module"]
pub type TR = crate::Reg<u32, _TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR;
#[doc = "`read()` method returns [tr::R](tr::R) reader structure"]
impl crate::Readable for TR {}
#[doc = "`write(|w| ..)` method takes [tr::W](tr::W) writer structure"]
impl crate::Writable for TR {}
#[doc = "Timing Register"]
pub mod tr;
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vr](vr) module"]
pub type VR = crate::Reg<u32, _VR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VR;
#[doc = "`read()` method returns [vr::R](vr::R) reader structure"]
impl crate::Readable for VR {}
#[doc = "Version Register"]
pub mod vr;
