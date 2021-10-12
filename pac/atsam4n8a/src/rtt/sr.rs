#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ALMS` reader - Real-time Alarm Status"]
pub struct ALMS_R(crate::FieldReader<bool, bool>);
impl ALMS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALMS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTTINC` reader - Real-time Timer Increment"]
pub struct RTTINC_R(crate::FieldReader<bool, bool>);
impl RTTINC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTTINC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTTINC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Real-time Alarm Status"]
    #[inline(always)]
    pub fn alms(&self) -> ALMS_R {
        ALMS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Real-time Timer Increment"]
    #[inline(always)]
    pub fn rttinc(&self) -> RTTINC_R {
        RTTINC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
