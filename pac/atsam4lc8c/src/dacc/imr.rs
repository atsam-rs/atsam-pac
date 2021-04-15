#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `TXRDY`"]
pub type TXRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Transmit Ready Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new((self.bits & 0x01) != 0)
    }
}
