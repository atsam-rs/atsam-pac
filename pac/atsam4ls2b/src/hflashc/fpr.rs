#[doc = "Register `FPR` reader"]
pub struct R(crate::R<FPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FSZ` reader - Flash Size"]
pub type FSZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSZ` reader - Page Size"]
pub type PSZ_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Flash Size"]
    #[inline(always)]
    pub fn fsz(&self) -> FSZ_R {
        FSZ_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Page Size"]
    #[inline(always)]
    pub fn psz(&self) -> PSZ_R {
        PSZ_R::new(((self.bits >> 8) & 7) as u8)
    }
}
#[doc = "Flash Controller Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpr](index.html) module"]
pub struct FPR_SPEC;
impl crate::RegisterSpec for FPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpr::R](R) reader structure"]
impl crate::Readable for FPR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FPR to value 0"]
impl crate::Resettable for FPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
