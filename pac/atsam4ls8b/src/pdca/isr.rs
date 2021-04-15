#[doc = "Reader of register ISR%s"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Reader of field `RCZ`"]
pub type RCZ_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRC`"]
pub type TRC_R = crate::R<bool, bool>;
#[doc = "Reader of field `TERR`"]
pub type TERR_R = crate::R<bool, bool>;
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
