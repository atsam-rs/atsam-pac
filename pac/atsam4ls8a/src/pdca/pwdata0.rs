#[doc = "Register `PWDATA0` reader"]
pub struct R(crate::R<PWDATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWDATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWDATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWDATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Data Cycles Counted since last Reset"]
pub type DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Cycles Counted since last Reset"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Channel 0 Write Data Cycles\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwdata0](index.html) module"]
pub struct PWDATA0_SPEC;
impl crate::RegisterSpec for PWDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwdata0::R](R) reader structure"]
impl crate::Readable for PWDATA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWDATA0 to value 0"]
impl crate::Resettable for PWDATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
