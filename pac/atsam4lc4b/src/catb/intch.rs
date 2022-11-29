#[doc = "Register `INTCH%s` reader"]
pub struct R(crate::R<INTCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTCH` reader - In-Touch"]
pub type INTCH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - In-Touch"]
    #[inline(always)]
    pub fn intch(&self) -> INTCH_R {
        INTCH_R::new(self.bits)
    }
}
#[doc = "In-Touch Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intch](index.html) module"]
pub struct INTCH_SPEC;
impl crate::RegisterSpec for INTCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intch::R](R) reader structure"]
impl crate::Readable for INTCH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTCH%s to value 0"]
impl crate::Resettable for INTCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
