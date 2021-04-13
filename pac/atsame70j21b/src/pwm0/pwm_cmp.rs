#[doc = "PWM Comparison 0 Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpv](cmpv) module"]
pub type CMPV = crate::Reg<u32, _CMPV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPV;
#[doc = "`read()` method returns [cmpv::R](cmpv::R) reader structure"]
impl crate::Readable for CMPV {}
#[doc = "`write(|w| ..)` method takes [cmpv::W](cmpv::W) writer structure"]
impl crate::Writable for CMPV {}
#[doc = "PWM Comparison 0 Value Register"]
pub mod cmpv;
#[doc = "PWM Comparison 0 Value Update Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpvupd](cmpvupd) module"]
pub type CMPVUPD = crate::Reg<u32, _CMPVUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPVUPD;
#[doc = "`write(|w| ..)` method takes [cmpvupd::W](cmpvupd::W) writer structure"]
impl crate::Writable for CMPVUPD {}
#[doc = "PWM Comparison 0 Value Update Register"]
pub mod cmpvupd;
#[doc = "PWM Comparison 0 Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpm](cmpm) module"]
pub type CMPM = crate::Reg<u32, _CMPM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPM;
#[doc = "`read()` method returns [cmpm::R](cmpm::R) reader structure"]
impl crate::Readable for CMPM {}
#[doc = "`write(|w| ..)` method takes [cmpm::W](cmpm::W) writer structure"]
impl crate::Writable for CMPM {}
#[doc = "PWM Comparison 0 Mode Register"]
pub mod cmpm;
#[doc = "PWM Comparison 0 Mode Update Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpmupd](cmpmupd) module"]
pub type CMPMUPD = crate::Reg<u32, _CMPMUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPMUPD;
#[doc = "`write(|w| ..)` method takes [cmpmupd::W](cmpmupd::W) writer structure"]
impl crate::Writable for CMPMUPD {}
#[doc = "PWM Comparison 0 Mode Update Register"]
pub mod cmpmupd;
