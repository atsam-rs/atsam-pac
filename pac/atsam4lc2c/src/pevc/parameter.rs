#[doc = "Register `PARAMETER` reader"]
pub struct R(crate::R<PARAMETER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAMETER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARAMETER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARAMETER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IGF_COUNT` reader - Number of Input Glitch Filters"]
pub struct IGF_COUNT_R(crate::FieldReader<u8, u8>);
impl IGF_COUNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        IGF_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IGF_COUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVS_COUNT` reader - Number of Event Shapers"]
pub struct EVS_COUNT_R(crate::FieldReader<u8, u8>);
impl EVS_COUNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        EVS_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVS_COUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVIN` reader - Number of Event Inputs / Generators"]
pub struct EVIN_R(crate::FieldReader<u8, u8>);
impl EVIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        EVIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGOUT` reader - Number of Trigger Outputs / Channels / Users"]
pub struct TRIGOUT_R(crate::FieldReader<u8, u8>);
impl TRIGOUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIGOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIGOUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of Input Glitch Filters"]
    #[inline(always)]
    pub fn igf_count(&self) -> IGF_COUNT_R {
        IGF_COUNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of Event Shapers"]
    #[inline(always)]
    pub fn evs_count(&self) -> EVS_COUNT_R {
        EVS_COUNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of Event Inputs / Generators"]
    #[inline(always)]
    pub fn evin(&self) -> EVIN_R {
        EVIN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Number of Trigger Outputs / Channels / Users"]
    #[inline(always)]
    pub fn trigout(&self) -> TRIGOUT_R {
        TRIGOUT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Parameter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [parameter](index.html) module"]
pub struct PARAMETER_SPEC;
impl crate::RegisterSpec for PARAMETER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [parameter::R](R) reader structure"]
impl crate::Readable for PARAMETER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PARAMETER to value 0x1406_1824"]
impl crate::Resettable for PARAMETER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1406_1824
    }
}
