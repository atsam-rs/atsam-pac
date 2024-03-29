#[doc = "Register `PWDATA1` reader"]
pub struct R(crate::R<PWDATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWDATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWDATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWDATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Data cycles Counted Since last reset"]
pub type DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data cycles Counted Since last reset"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Channel 1 Write Data Cycles\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwdata1](index.html) module"]
pub struct PWDATA1_SPEC;
impl crate::RegisterSpec for PWDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwdata1::R](R) reader structure"]
impl crate::Readable for PWDATA1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWDATA1 to value 0"]
impl crate::Resettable for PWDATA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
