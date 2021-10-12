#[doc = "Register `LEVEL` reader"]
pub struct R(crate::R<LEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FLEVEL` reader - Fractional Sensor Level"]
pub struct FLEVEL_R(crate::FieldReader<u16, u16>);
impl FLEVEL_R {
    pub(crate) fn new(bits: u16) -> Self {
        FLEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLEVEL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLEVEL` reader - Integer Sensor Level"]
pub struct RLEVEL_R(crate::FieldReader<u8, u8>);
impl RLEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RLEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RLEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - Fractional Sensor Level"]
    #[inline(always)]
    pub fn flevel(&self) -> FLEVEL_R {
        FLEVEL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - Integer Sensor Level"]
    #[inline(always)]
    pub fn rlevel(&self) -> RLEVEL_R {
        RLEVEL_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
#[doc = "Sensor Relative Level\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [level](index.html) module"]
pub struct LEVEL_SPEC;
impl crate::RegisterSpec for LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [level::R](R) reader structure"]
impl crate::Readable for LEVEL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LEVEL to value 0"]
impl crate::Resettable for LEVEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
