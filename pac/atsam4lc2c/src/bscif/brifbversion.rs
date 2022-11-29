#[doc = "Register `BRIFBVERSION` reader"]
pub struct R(crate::R<BRIFBVERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRIFBVERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRIFBVERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRIFBVERSION_SPEC>) -> Self {
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
#[doc = "Backup Register Interface Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brifbversion](index.html) module"]
pub struct BRIFBVERSION_SPEC;
impl crate::RegisterSpec for BRIFBVERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brifbversion::R](R) reader structure"]
impl crate::Readable for BRIFBVERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BRIFBVERSION to value 0x0100"]
impl crate::Resettable for BRIFBVERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
