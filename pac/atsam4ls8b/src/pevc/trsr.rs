#[doc = "Register `TRSR` reader"]
pub struct R(crate::R<TRSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TRS` reader - Trigger Interrupt Status"]
pub struct TRS_R(crate::FieldReader<u32, u32>);
impl TRS_R {
    pub(crate) fn new(bits: u32) -> Self {
        TRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Trigger Interrupt Status"]
    #[inline(always)]
    pub fn trs(&self) -> TRS_R {
        TRS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Trigger Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trsr](index.html) module"]
pub struct TRSR_SPEC;
impl crate::RegisterSpec for TRSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trsr::R](R) reader structure"]
impl crate::Readable for TRSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRSR to value 0"]
impl crate::Resettable for TRSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
