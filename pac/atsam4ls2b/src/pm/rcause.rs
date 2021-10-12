#[doc = "Register `RCAUSE` reader"]
pub struct R(crate::R<RCAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `POR` reader - Power-on Reset"]
pub struct POR_R(crate::FieldReader<bool, bool>);
impl POR_R {
    pub(crate) fn new(bits: bool) -> Self {
        POR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD` reader - Brown-out Reset"]
pub struct BOD_R(crate::FieldReader<bool, bool>);
impl BOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT` reader - External Reset Pin"]
pub struct EXT_R(crate::FieldReader<bool, bool>);
impl EXT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT` reader - Watchdog Reset"]
pub struct WDT_R(crate::FieldReader<bool, bool>);
impl WDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCDRST` reader - OCD Reset"]
pub struct OCDRST_R(crate::FieldReader<bool, bool>);
impl OCDRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCDRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCDRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR33` reader - Power-on Reset"]
pub struct POR33_R(crate::FieldReader<bool, bool>);
impl POR33_R {
    pub(crate) fn new(bits: bool) -> Self {
        POR33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR33_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD33` reader - Brown-out 3.3V Reset"]
pub struct BOD33_R(crate::FieldReader<bool, bool>);
impl BOD33_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOD33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD33_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Power-on Reset"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Brown-out Reset"]
    #[inline(always)]
    pub fn bod(&self) -> BOD_R {
        BOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Reset Pin"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Watchdog Reset"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OCD Reset"]
    #[inline(always)]
    pub fn ocdrst(&self) -> OCDRST_R {
        OCDRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Power-on Reset"]
    #[inline(always)]
    pub fn por33(&self) -> POR33_R {
        POR33_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Brown-out 3.3V Reset"]
    #[inline(always)]
    pub fn bod33(&self) -> BOD33_R {
        BOD33_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
#[doc = "Reset Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcause](index.html) module"]
pub struct RCAUSE_SPEC;
impl crate::RegisterSpec for RCAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcause::R](R) reader structure"]
impl crate::Readable for RCAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RCAUSE to value 0"]
impl crate::Resettable for RCAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
