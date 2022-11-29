#[doc = "Register `TRIMR` reader"]
pub struct R(crate::R<TRIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TRIM` reader - Trigger Interrupt Mask"]
pub type TRIM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Trigger Interrupt Mask"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(self.bits)
    }
}
#[doc = "Trigger Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trimr](index.html) module"]
pub struct TRIMR_SPEC;
impl crate::RegisterSpec for TRIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trimr::R](R) reader structure"]
impl crate::Readable for TRIMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRIMR to value 0"]
impl crate::Resettable for TRIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
