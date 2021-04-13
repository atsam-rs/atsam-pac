#[doc = "PWM Channel Mode Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr](cmr) module"]
pub type CMR = crate::Reg<u32, _CMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR;
#[doc = "`read()` method returns [cmr::R](cmr::R) reader structure"]
impl crate::Readable for CMR {}
#[doc = "`write(|w| ..)` method takes [cmr::W](cmr::W) writer structure"]
impl crate::Writable for CMR {}
#[doc = "PWM Channel Mode Register (ch_num = 0)"]
pub mod cmr;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdty](cdty) module"]
pub type CDTY = crate::Reg<u32, _CDTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDTY;
#[doc = "`read()` method returns [cdty::R](cdty::R) reader structure"]
impl crate::Readable for CDTY {}
#[doc = "`write(|w| ..)` method takes [cdty::W](cdty::W) writer structure"]
impl crate::Writable for CDTY {}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 0)"]
pub mod cdty;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdtyupd](cdtyupd) module"]
pub type CDTYUPD = crate::Reg<u32, _CDTYUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDTYUPD;
#[doc = "`write(|w| ..)` method takes [cdtyupd::W](cdtyupd::W) writer structure"]
impl crate::Writable for CDTYUPD {}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 0)"]
pub mod cdtyupd;
#[doc = "PWM Channel Period Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprd](cprd) module"]
pub type CPRD = crate::Reg<u32, _CPRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPRD;
#[doc = "`read()` method returns [cprd::R](cprd::R) reader structure"]
impl crate::Readable for CPRD {}
#[doc = "`write(|w| ..)` method takes [cprd::W](cprd::W) writer structure"]
impl crate::Writable for CPRD {}
#[doc = "PWM Channel Period Register (ch_num = 0)"]
pub mod cprd;
#[doc = "PWM Channel Period Update Register (ch_num = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprdupd](cprdupd) module"]
pub type CPRDUPD = crate::Reg<u32, _CPRDUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPRDUPD;
#[doc = "`write(|w| ..)` method takes [cprdupd::W](cprdupd::W) writer structure"]
impl crate::Writable for CPRDUPD {}
#[doc = "PWM Channel Period Update Register (ch_num = 0)"]
pub mod cprdupd;
#[doc = "PWM Channel Counter Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccnt](ccnt) module"]
pub type CCNT = crate::Reg<u32, _CCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCNT;
#[doc = "`read()` method returns [ccnt::R](ccnt::R) reader structure"]
impl crate::Readable for CCNT {}
#[doc = "PWM Channel Counter Register (ch_num = 0)"]
pub mod ccnt;
#[doc = "PWM Channel Dead Time Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt](dt) module"]
pub type DT = crate::Reg<u32, _DT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DT;
#[doc = "`read()` method returns [dt::R](dt::R) reader structure"]
impl crate::Readable for DT {}
#[doc = "`write(|w| ..)` method takes [dt::W](dt::W) writer structure"]
impl crate::Writable for DT {}
#[doc = "PWM Channel Dead Time Register (ch_num = 0)"]
pub mod dt;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtupd](dtupd) module"]
pub type DTUPD = crate::Reg<u32, _DTUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTUPD;
#[doc = "`write(|w| ..)` method takes [dtupd::W](dtupd::W) writer structure"]
impl crate::Writable for DTUPD {}
#[doc = "PWM Channel Dead Time Update Register (ch_num = 0)"]
pub mod dtupd;
