#[doc = "Register `PWSTALL0` reader"]
pub struct R(crate::R<PWSTALL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWSTALL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWSTALL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWSTALL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STALL` reader - Stall cycles counted since last reset"]
pub struct STALL_R(crate::FieldReader<u32, u32>);
impl STALL_R {
    pub(crate) fn new(bits: u32) -> Self {
        STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Stall cycles counted since last reset"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Channel 0 Write Stall Cycles\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwstall0](index.html) module"]
pub struct PWSTALL0_SPEC;
impl crate::RegisterSpec for PWSTALL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwstall0::R](R) reader structure"]
impl crate::Readable for PWSTALL0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWSTALL0 to value 0"]
impl crate::Resettable for PWSTALL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
