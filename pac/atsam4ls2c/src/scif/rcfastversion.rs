#[doc = "Register `RCFASTVERSION` reader"]
pub struct R(crate::R<RCFASTVERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCFASTVERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCFASTVERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCFASTVERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VERSION` reader - Version number"]
pub type VERSION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VARIANT` reader - Variant number"]
pub type VARIANT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - Version number"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Variant number"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "4/8/12 MHz RC Oscillator Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcfastversion](index.html) module"]
pub struct RCFASTVERSION_SPEC;
impl crate::RegisterSpec for RCFASTVERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcfastversion::R](R) reader structure"]
impl crate::Readable for RCFASTVERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RCFASTVERSION to value 0"]
impl crate::Resettable for RCFASTVERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
