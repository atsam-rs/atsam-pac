#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `ODATARDY`"]
pub type ODATARDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `IBUFRDY`"]
pub type IBUFRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Output Data Ready Interrupt Mask"]
    #[inline(always)]
    pub fn odatardy(&self) -> ODATARDY_R {
        ODATARDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Input Buffer Ready Interrupt Mask"]
    #[inline(always)]
    pub fn ibufrdy(&self) -> IBUFRDY_R {
        IBUFRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
