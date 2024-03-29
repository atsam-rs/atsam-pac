#[doc = "Register `VALUE` reader"]
pub struct R(crate::R<VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALUE` reader - Measured frequency"]
pub type VALUE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Measured frequency"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Value register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [value](index.html) module"]
pub struct VALUE_SPEC;
impl crate::RegisterSpec for VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [value::R](R) reader structure"]
impl crate::Readable for VALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VALUE to value 0"]
impl crate::Resettable for VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
