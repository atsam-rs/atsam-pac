#[doc = "Reader of register PCISR"]
pub type R = crate::R<u32, super::PCISR>;
#[doc = "Reader of field `DRDY`"]
pub type DRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE`"]
pub type OVRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENDRX`"]
pub type ENDRX_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXBUFF`"]
pub type RXBUFF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Parallel Capture Mode Data Ready"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parallel Capture Mode Overrun Error"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
