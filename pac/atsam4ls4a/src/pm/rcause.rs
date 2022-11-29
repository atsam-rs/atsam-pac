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
pub type POR_R = crate::BitReader<bool>;
#[doc = "Field `BOD` reader - Brown-out Reset"]
pub type BOD_R = crate::BitReader<bool>;
#[doc = "Field `EXT` reader - External Reset Pin"]
pub type EXT_R = crate::BitReader<bool>;
#[doc = "Field `WDT` reader - Watchdog Reset"]
pub type WDT_R = crate::BitReader<bool>;
#[doc = "Field `OCDRST` reader - OCD Reset"]
pub type OCDRST_R = crate::BitReader<bool>;
#[doc = "Field `POR33` reader - Power-on Reset"]
pub type POR33_R = crate::BitReader<bool>;
#[doc = "Field `BOD33` reader - Brown-out 3.3V Reset"]
pub type BOD33_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Power-on Reset"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Brown-out Reset"]
    #[inline(always)]
    pub fn bod(&self) -> BOD_R {
        BOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Reset Pin"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog Reset"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - OCD Reset"]
    #[inline(always)]
    pub fn ocdrst(&self) -> OCDRST_R {
        OCDRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Power-on Reset"]
    #[inline(always)]
    pub fn por33(&self) -> POR33_R {
        POR33_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Brown-out 3.3V Reset"]
    #[inline(always)]
    pub fn bod33(&self) -> BOD33_R {
        BOD33_R::new(((self.bits >> 13) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
