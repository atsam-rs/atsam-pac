#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `HCR`"]
pub type HCR_R = crate::R<bool, bool>;
#[doc = "Reader of field `BERR`"]
pub type BERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `FAIL`"]
pub type FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `LCK`"]
pub type LCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `PROT`"]
pub type PROT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DBGP`"]
pub type DBGP_R = crate::R<bool, bool>;
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Operation done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Hold Core reset"]
    #[inline(always)]
    pub fn hcr(&self) -> HCR_R {
        HCR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Failure"]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Lock"]
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enabled"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Protected"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Debugger Present"]
    #[inline(always)]
    pub fn dbgp(&self) -> DBGP_R {
        DBGP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - State"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
