#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACK` reader - Acknowledge Update Interrupt Mask"]
pub struct ACK_R(crate::FieldReader<bool, bool>);
impl ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALR` reader - Alarm Interrupt Mask"]
pub struct ALR_R(crate::FieldReader<bool, bool>);
impl ALR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC` reader - Second Event Interrupt Mask"]
pub struct SEC_R(crate::FieldReader<bool, bool>);
impl SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM` reader - Time Event Interrupt Mask"]
pub struct TIM_R(crate::FieldReader<bool, bool>);
impl TIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL` reader - Calendar Event Interrupt Mask"]
pub struct CAL_R(crate::FieldReader<bool, bool>);
impl CAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDERR` reader - Time and/or Date Error Mask"]
pub struct TDERR_R(crate::FieldReader<bool, bool>);
impl TDERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Acknowledge Update Interrupt Mask"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alarm Interrupt Mask"]
    #[inline(always)]
    pub fn alr(&self) -> ALR_R {
        ALR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Second Event Interrupt Mask"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Time Event Interrupt Mask"]
    #[inline(always)]
    pub fn tim(&self) -> TIM_R {
        TIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Calendar Event Interrupt Mask"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Time and/or Date Error Mask"]
    #[inline(always)]
    pub fn tderr(&self) -> TDERR_R {
        TDERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
