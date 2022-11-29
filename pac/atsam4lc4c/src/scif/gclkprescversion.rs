#[doc = "Register `GCLKPRESCVERSION` reader"]
pub struct R(crate::R<GCLKPRESCVERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCLKPRESCVERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCLKPRESCVERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCLKPRESCVERSION_SPEC>) -> Self {
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
#[doc = "Generic Clock Prescaler Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gclkprescversion](index.html) module"]
pub struct GCLKPRESCVERSION_SPEC;
impl crate::RegisterSpec for GCLKPRESCVERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gclkprescversion::R](R) reader structure"]
impl crate::Readable for GCLKPRESCVERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GCLKPRESCVERSION to value 0"]
impl crate::Resettable for GCLKPRESCVERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
