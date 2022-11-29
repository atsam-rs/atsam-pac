#[doc = "Register `PLLIFAVERSION` reader"]
pub struct R(crate::R<PLLIFAVERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLIFAVERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLIFAVERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLIFAVERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VERSION` reader - Version number"]
pub type VERSION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VARIANT` reader - Variant nubmer"]
pub type VARIANT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - Version number"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Variant nubmer"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "PLL Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllifaversion](index.html) module"]
pub struct PLLIFAVERSION_SPEC;
impl crate::RegisterSpec for PLLIFAVERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllifaversion::R](R) reader structure"]
impl crate::Readable for PLLIFAVERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PLLIFAVERSION to value 0"]
impl crate::Resettable for PLLIFAVERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
