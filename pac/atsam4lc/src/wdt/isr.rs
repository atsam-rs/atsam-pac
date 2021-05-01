#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Reader of field `WINT`"]
pub type WINT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn wint(&self) -> WINT_R {
        WINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
