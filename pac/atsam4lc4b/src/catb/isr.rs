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
#[doc = "Field `SAMPLE` reader - Sample Ready Interrupt Status"]
pub struct SAMPLE_R(crate::FieldReader<bool, bool>);
impl SAMPLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAMPLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMPLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTCH` reader - In-touch Interrupt Status"]
pub struct INTCH_R(crate::FieldReader<bool, bool>);
impl INTCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTTCH` reader - Out-of-Touch Interrupt Status"]
pub struct OUTTCH_R(crate::FieldReader<bool, bool>);
impl OUTTCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTTCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTTCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Sample Ready Interrupt Status"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - In-touch Interrupt Status"]
    #[inline(always)]
    pub fn intch(&self) -> INTCH_R {
        INTCH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Out-of-Touch Interrupt Status"]
    #[inline(always)]
    pub fn outtch(&self) -> OUTTCH_R {
        OUTTCH_R::new(((self.bits >> 2) & 0x01) != 0)
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
