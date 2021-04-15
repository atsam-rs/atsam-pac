#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IMR_SPEC>> for R {
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEOC` reader - Sequencer end of conversion Interrupt Mask"]
pub struct SEOC_R(crate::FieldReader<bool, bool>);
impl SEOC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEOC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOVR` reader - Sequencer last converted value overrun Interrupt Mask"]
pub struct LOVR_R(crate::FieldReader<bool, bool>);
impl LOVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WM` reader - Window monitor Interrupt Mask"]
pub struct WM_R(crate::FieldReader<bool, bool>);
impl WM_R {
    pub(crate) fn new(bits: bool) -> Self {
        WM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTRG` reader - Sequencer missed trigger event Interrupt Mask"]
pub struct SMTRG_R(crate::FieldReader<bool, bool>);
impl SMTRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMTRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMTRG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTO` reader - Timer time-out Interrupt Mask"]
pub struct TTO_R(crate::FieldReader<bool, bool>);
impl TTO_R {
    pub(crate) fn new(bits: bool) -> Self {
        TTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Sequencer end of conversion Interrupt Mask"]
    #[inline(always)]
    pub fn seoc(&self) -> SEOC_R {
        SEOC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sequencer last converted value overrun Interrupt Mask"]
    #[inline(always)]
    pub fn lovr(&self) -> LOVR_R {
        LOVR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Window monitor Interrupt Mask"]
    #[inline(always)]
    pub fn wm(&self) -> WM_R {
        WM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sequencer missed trigger event Interrupt Mask"]
    #[inline(always)]
    pub fn smtrg(&self) -> SMTRG_R {
        SMTRG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer time-out Interrupt Mask"]
    #[inline(always)]
    pub fn tto(&self) -> TTO_R {
        TTO_R::new(((self.bits >> 5) & 0x01) != 0)
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
