#[doc = "SMC Setup Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup](setup) module"]
pub type SETUP = crate::Reg<u32, _SETUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP;
#[doc = "`read()` method returns [setup::R](setup::R) reader structure"]
impl crate::Readable for SETUP {}
#[doc = "`write(|w| ..)` method takes [setup::W](setup::W) writer structure"]
impl crate::Writable for SETUP {}
#[doc = "SMC Setup Register (CS_number = 0)"]
pub mod setup;
#[doc = "SMC Pulse Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulse](pulse) module"]
pub type PULSE = crate::Reg<u32, _PULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PULSE;
#[doc = "`read()` method returns [pulse::R](pulse::R) reader structure"]
impl crate::Readable for PULSE {}
#[doc = "`write(|w| ..)` method takes [pulse::W](pulse::W) writer structure"]
impl crate::Writable for PULSE {}
#[doc = "SMC Pulse Register (CS_number = 0)"]
pub mod pulse;
#[doc = "SMC Cycle Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle](cycle) module"]
pub type CYCLE = crate::Reg<u32, _CYCLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CYCLE;
#[doc = "`read()` method returns [cycle::R](cycle::R) reader structure"]
impl crate::Readable for CYCLE {}
#[doc = "`write(|w| ..)` method takes [cycle::W](cycle::W) writer structure"]
impl crate::Writable for CYCLE {}
#[doc = "SMC Cycle Register (CS_number = 0)"]
pub mod cycle;
#[doc = "SMC MODE Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "SMC MODE Register (CS_number = 0)"]
pub mod mode;
