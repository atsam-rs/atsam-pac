#[doc = "Register `BODIFCVERSION` reader"]
pub struct R(crate::R<BODIFCVERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BODIFCVERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BODIFCVERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BODIFCVERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VERSION` reader - Version Number"]
pub type VERSION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VARIANT` reader - Variant Number"]
pub type VARIANT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - Version Number"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Variant Number"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "BODIFC Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodifcversion](index.html) module"]
pub struct BODIFCVERSION_SPEC;
impl crate::RegisterSpec for BODIFCVERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bodifcversion::R](R) reader structure"]
impl crate::Readable for BODIFCVERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BODIFCVERSION to value 0x0110"]
impl crate::Resettable for BODIFCVERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0110;
}
