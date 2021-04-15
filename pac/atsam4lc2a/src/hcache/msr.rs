#[doc = "Register `MSR` reader"]
pub struct R(crate::R<MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MSR_SPEC>> for R {
    fn from(reader: crate::R<MSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EVENTCNT` reader - Monitor Event Counter"]
pub struct EVENTCNT_R(crate::FieldReader<u32, u32>);
impl EVENTCNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        EVENTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENTCNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Monitor Event Counter"]
    #[inline(always)]
    pub fn eventcnt(&self) -> EVENTCNT_R {
        EVENTCNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Monitor Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](index.html) module"]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msr::R](R) reader structure"]
impl crate::Readable for MSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
