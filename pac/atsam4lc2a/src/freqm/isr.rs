#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DONE` reader - Frequency measurment done"]
pub struct DONE_R(crate::FieldReader<bool, bool>);
impl DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCLKRDY` reader - Reference Clock ready"]
pub struct RCLKRDY_R(crate::FieldReader<bool, bool>);
impl RCLKRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCLKRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCLKRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Frequency measurment done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reference Clock ready"]
    #[inline(always)]
    pub fn rclkrdy(&self) -> RCLKRDY_R {
        RCLKRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
