#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `FC0R`"]
pub type FC0R_R = crate::R<bool, bool>;
#[doc = "Reader of field `FC0S`"]
pub type FC0S_R = crate::R<bool, bool>;
#[doc = "Reader of field `FC1S`"]
pub type FC1S_R = crate::R<bool, bool>;
#[doc = "Reader of field `FC2S`"]
pub type FC2S_R = crate::R<bool, bool>;
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `WEN`"]
pub type WEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `BLKS`"]
pub type BLKS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSRS`"]
pub type CSRS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CPS`"]
pub type CPS_R = crate::R<bool, bool>;
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
