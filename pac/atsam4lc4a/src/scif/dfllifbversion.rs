#[doc = "Register `DFLLIFBVERSION` reader"]
pub struct R(crate::R<DFLLIFBVERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLLIFBVERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLLIFBVERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLLIFBVERSION_SPEC>) -> Self {
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
#[doc = "DFLL Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllifbversion](index.html) module"]
pub struct DFLLIFBVERSION_SPEC;
impl crate::RegisterSpec for DFLLIFBVERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfllifbversion::R](R) reader structure"]
impl crate::Readable for DFLLIFBVERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFLLIFBVERSION to value 0"]
impl crate::Resettable for DFLLIFBVERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
