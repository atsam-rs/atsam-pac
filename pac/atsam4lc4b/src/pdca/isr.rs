#[doc = "Register `ISR%s` reader"]
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
#[doc = "Field `RCZ` reader - Reload Counter Zero"]
pub struct RCZ_R(crate::FieldReader<bool, bool>);
impl RCZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRC` reader - Transfer Complete"]
pub struct TRC_R(crate::FieldReader<bool, bool>);
impl TRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERR` reader - Transfer Error"]
pub struct TERR_R(crate::FieldReader<bool, bool>);
impl TERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Reload Counter Zero"]
    #[inline(always)]
    pub fn rcz(&self) -> RCZ_R {
        RCZ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transfer Error"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 2) & 0x01) != 0)
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
#[doc = "`reset()` method sets ISR%s to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
