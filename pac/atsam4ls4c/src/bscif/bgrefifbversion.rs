#[doc = "Register `BGREFIFBVERSION` reader"]
pub struct R(crate::R<BGREFIFBVERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGREFIFBVERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGREFIFBVERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGREFIFBVERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VERSION` reader - Version Number"]
pub struct VERSION_R(crate::FieldReader<u16, u16>);
impl VERSION_R {
    pub(crate) fn new(bits: u16) -> Self {
        VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VERSION_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VARIANT` reader - Variant Number"]
pub struct VARIANT_R(crate::FieldReader<u8, u8>);
impl VARIANT_R {
    pub(crate) fn new(bits: u8) -> Self {
        VARIANT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VARIANT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "BGREFIFB Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgrefifbversion](index.html) module"]
pub struct BGREFIFBVERSION_SPEC;
impl crate::RegisterSpec for BGREFIFBVERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgrefifbversion::R](R) reader structure"]
impl crate::Readable for BGREFIFBVERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BGREFIFBVERSION to value 0x0110"]
impl crate::Resettable for BGREFIFBVERSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0110
    }
}
