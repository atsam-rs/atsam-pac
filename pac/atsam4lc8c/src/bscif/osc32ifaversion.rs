#[doc = "Register `OSC32IFAVERSION` reader"]
pub struct R(crate::R<OSC32IFAVERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC32IFAVERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC32IFAVERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC32IFAVERSION_SPEC>) -> Self {
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
#[doc = "32 KHz Oscillator Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc32ifaversion](index.html) module"]
pub struct OSC32IFAVERSION_SPEC;
impl crate::RegisterSpec for OSC32IFAVERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc32ifaversion::R](R) reader structure"]
impl crate::Readable for OSC32IFAVERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OSC32IFAVERSION to value 0x0200"]
impl crate::Resettable for OSC32IFAVERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
