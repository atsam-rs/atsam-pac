#[doc = "Channel Control Register (channel = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "Channel Control Register (channel = 0)"]
pub mod ccr;
#[doc = "Channel Mode Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr](cmr) module"]
pub type CMR = crate::Reg<u32, _CMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR;
#[doc = "`read()` method returns [cmr::R](cmr::R) reader structure"]
impl crate::Readable for CMR {}
#[doc = "`write(|w| ..)` method takes [cmr::W](cmr::W) writer structure"]
impl crate::Writable for CMR {}
#[doc = "Channel Mode Register (channel = 0)"]
pub mod cmr;
#[doc = "Stepper Motor Mode Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smmr](smmr) module"]
pub type SMMR = crate::Reg<u32, _SMMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMMR;
#[doc = "`read()` method returns [smmr::R](smmr::R) reader structure"]
impl crate::Readable for SMMR {}
#[doc = "`write(|w| ..)` method takes [smmr::W](smmr::W) writer structure"]
impl crate::Writable for SMMR {}
#[doc = "Stepper Motor Mode Register (channel = 0)"]
pub mod smmr;
#[doc = "Register AB (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rab](rab) module"]
pub type RAB = crate::Reg<u32, _RAB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAB;
#[doc = "`read()` method returns [rab::R](rab::R) reader structure"]
impl crate::Readable for RAB {}
#[doc = "Register AB (channel = 0)"]
pub mod rab;
#[doc = "Counter Value (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cv](cv) module"]
pub type CV = crate::Reg<u32, _CV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CV;
#[doc = "`read()` method returns [cv::R](cv::R) reader structure"]
impl crate::Readable for CV {}
#[doc = "Counter Value (channel = 0)"]
pub mod cv;
#[doc = "Register A (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ra](ra) module"]
pub type RA = crate::Reg<u32, _RA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RA;
#[doc = "`read()` method returns [ra::R](ra::R) reader structure"]
impl crate::Readable for RA {}
#[doc = "`write(|w| ..)` method takes [ra::W](ra::W) writer structure"]
impl crate::Writable for RA {}
#[doc = "Register A (channel = 0)"]
pub mod ra;
#[doc = "Register B (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rb](rb) module"]
pub type RB = crate::Reg<u32, _RB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RB;
#[doc = "`read()` method returns [rb::R](rb::R) reader structure"]
impl crate::Readable for RB {}
#[doc = "`write(|w| ..)` method takes [rb::W](rb::W) writer structure"]
impl crate::Writable for RB {}
#[doc = "Register B (channel = 0)"]
pub mod rb;
#[doc = "Register C (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc](rc) module"]
pub type RC = crate::Reg<u32, _RC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RC;
#[doc = "`read()` method returns [rc::R](rc::R) reader structure"]
impl crate::Readable for RC {}
#[doc = "`write(|w| ..)` method takes [rc::W](rc::W) writer structure"]
impl crate::Writable for RC {}
#[doc = "Register C (channel = 0)"]
pub mod rc;
#[doc = "Status Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register (channel = 0)"]
pub mod sr;
#[doc = "Interrupt Enable Register (channel = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register (channel = 0)"]
pub mod ier;
#[doc = "Interrupt Disable Register (channel = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "Interrupt Disable Register (channel = 0)"]
pub mod idr;
#[doc = "Interrupt Mask Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "Interrupt Mask Register (channel = 0)"]
pub mod imr;
#[doc = "Extended Mode Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr](emr) module"]
pub type EMR = crate::Reg<u32, _EMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMR;
#[doc = "`read()` method returns [emr::R](emr::R) reader structure"]
impl crate::Readable for EMR {}
#[doc = "`write(|w| ..)` method takes [emr::W](emr::W) writer structure"]
impl crate::Writable for EMR {}
#[doc = "Extended Mode Register (channel = 0)"]
pub mod emr;
