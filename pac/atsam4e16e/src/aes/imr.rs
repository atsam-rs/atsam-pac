#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `DATRDY`"]
pub type DATRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `URAD`"]
pub type URAD_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Data Ready Interrupt Mask"]
    #[inline(always)]
    pub fn datrdy(&self) -> DATRDY_R {
        DATRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Unspecified Register Access Detection Interrupt Mask"]
    #[inline(always)]
    pub fn urad(&self) -> URAD_R {
        URAD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
