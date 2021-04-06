#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Reader of field `ERRISR`"]
pub type ERRISR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - CRC Error Interrupt Status"]
    #[inline(always)]
    pub fn errisr(&self) -> ERRISR_R {
        ERRISR_R::new((self.bits & 0x01) != 0)
    }
}
