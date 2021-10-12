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
#[doc = "Field `SEOC` reader - Sequencer end of conversion"]
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
#[doc = "Field `LOVR` reader - Sequencer last converted value overrun"]
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
#[doc = "Field `WM` reader - Window monitor"]
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
#[doc = "Field `SMTRG` reader - Sequencer missed trigger event"]
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
#[doc = "Field `SUTD` reader - Start-up time done"]
pub struct SUTD_R(crate::FieldReader<bool, bool>);
impl SUTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUTD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUTD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTO` reader - Timer time-out"]
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
#[doc = "Field `EN` reader - Enable Status"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBUSY` reader - Timer busy"]
pub struct TBUSY_R(crate::FieldReader<bool, bool>);
impl TBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBUSY` reader - Sequencer busy"]
pub struct SBUSY_R(crate::FieldReader<bool, bool>);
impl SBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBUSY` reader - Conversion busy"]
pub struct CBUSY_R(crate::FieldReader<bool, bool>);
impl CBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFBUF` reader - Reference buffer status"]
pub struct REFBUF_R(crate::FieldReader<bool, bool>);
impl REFBUF_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFBUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Sequencer end of conversion"]
    #[inline(always)]
    pub fn seoc(&self) -> SEOC_R {
        SEOC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sequencer last converted value overrun"]
    #[inline(always)]
    pub fn lovr(&self) -> LOVR_R {
        LOVR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Window monitor"]
    #[inline(always)]
    pub fn wm(&self) -> WM_R {
        WM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sequencer missed trigger event"]
    #[inline(always)]
    pub fn smtrg(&self) -> SMTRG_R {
        SMTRG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Start-up time done"]
    #[inline(always)]
    pub fn sutd(&self) -> SUTD_R {
        SUTD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer time-out"]
    #[inline(always)]
    pub fn tto(&self) -> TTO_R {
        TTO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable Status"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Timer busy"]
    #[inline(always)]
    pub fn tbusy(&self) -> TBUSY_R {
        TBUSY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Sequencer busy"]
    #[inline(always)]
    pub fn sbusy(&self) -> SBUSY_R {
        SBUSY_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Conversion busy"]
    #[inline(always)]
    pub fn cbusy(&self) -> CBUSY_R {
        CBUSY_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Reference buffer status"]
    #[inline(always)]
    pub fn refbuf(&self) -> REFBUF_R {
        REFBUF_R::new(((self.bits >> 28) & 0x01) != 0)
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
