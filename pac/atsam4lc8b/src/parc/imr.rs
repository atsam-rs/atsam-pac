#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `DRDY`"]
pub type DRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVR`"]
pub type OVR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - Data Ready Interrupt Mask"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
