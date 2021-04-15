#[doc = "Reader of register RCAUSE"]
pub type R = crate::R<u32, super::RCAUSE>;
#[doc = "Reader of field `POR`"]
pub type POR_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOD`"]
pub type BOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXT`"]
pub type EXT_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDT`"]
pub type WDT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OCDRST`"]
pub type OCDRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `POR33`"]
pub type POR33_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOD33`"]
pub type BOD33_R = crate::R<bool, bool>;
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
