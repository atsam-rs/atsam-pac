#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Lock Bits Word 0"]
    pub word0: WORD0,
    #[doc = "0x04 - Lock Bits Word 1"]
    pub word1: WORD1,
}
#[doc = "Lock Bits Word 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [word0](word0) module"]
pub type WORD0 = crate::Reg<u32, _WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD0;
#[doc = "`read()` method returns [word0::R](word0::R) reader structure"]
impl crate::Readable for WORD0 {}
#[doc = "`write(|w| ..)` method takes [word0::W](word0::W) writer structure"]
impl crate::Writable for WORD0 {}
#[doc = "Lock Bits Word 0"]
pub mod word0;
#[doc = "Lock Bits Word 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [word1](word1) module"]
pub type WORD1 = crate::Reg<u32, _WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD1;
#[doc = "`read()` method returns [word1::R](word1::R) reader structure"]
impl crate::Readable for WORD1 {}
#[doc = "`write(|w| ..)` method takes [word1::W](word1::W) writer structure"]
impl crate::Writable for WORD1 {}
#[doc = "Lock Bits Word 1"]
pub mod word1;
