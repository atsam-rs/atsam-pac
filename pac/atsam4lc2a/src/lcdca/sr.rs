#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SR_SPEC>> for R {
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FC0R` reader - Frame Counter 0 Rollover"]
pub struct FC0R_R(crate::FieldReader<bool, bool>);
impl FC0R_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC0R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC0R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC0S` reader - Frame Counter 0 Status"]
pub struct FC0S_R(crate::FieldReader<bool, bool>);
impl FC0S_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC0S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC0S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC1S` reader - Frame Counter 1 Status"]
pub struct FC1S_R(crate::FieldReader<bool, bool>);
impl FC1S_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC1S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC1S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC2S` reader - Frame Counter 2 Status"]
pub struct FC2S_R(crate::FieldReader<bool, bool>);
impl FC2S_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC2S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC2S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` reader - LCDCA Status"]
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
#[doc = "Field `WEN` reader - Wake up Status"]
pub struct WEN_R(crate::FieldReader<bool, bool>);
impl WEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLKS` reader - Blink Status"]
pub struct BLKS_R(crate::FieldReader<bool, bool>);
impl BLKS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLKS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSRS` reader - Circular Shift Register Status"]
pub struct CSRS_R(crate::FieldReader<bool, bool>);
impl CSRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPS` reader - Charge Pump Status"]
pub struct CPS_R(crate::FieldReader<bool, bool>);
impl CPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Frame Counter 0 Rollover"]
    #[inline(always)]
    pub fn fc0r(&self) -> FC0R_R {
        FC0R_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Frame Counter 0 Status"]
    #[inline(always)]
    pub fn fc0s(&self) -> FC0S_R {
        FC0S_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Frame Counter 1 Status"]
    #[inline(always)]
    pub fn fc1s(&self) -> FC1S_R {
        FC1S_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Frame Counter 2 Status"]
    #[inline(always)]
    pub fn fc2s(&self) -> FC2S_R {
        FC2S_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LCDCA Status"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wake up Status"]
    #[inline(always)]
    pub fn wen(&self) -> WEN_R {
        WEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Blink Status"]
    #[inline(always)]
    pub fn blks(&self) -> BLKS_R {
        BLKS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Circular Shift Register Status"]
    #[inline(always)]
    pub fn csrs(&self) -> CSRS_R {
        CSRS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Charge Pump Status"]
    #[inline(always)]
    pub fn cps(&self) -> CPS_R {
        CPS_R::new(((self.bits >> 8) & 0x01) != 0)
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
