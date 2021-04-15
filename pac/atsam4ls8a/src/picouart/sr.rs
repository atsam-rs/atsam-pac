#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRDY`"]
pub type DRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Enable Interrupt Status"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Ready Interrupt Status"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
