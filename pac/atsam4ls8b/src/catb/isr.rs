#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Reader of field `SAMPLE`"]
pub type SAMPLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTCH`"]
pub type INTCH_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTTCH`"]
pub type OUTTCH_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Sample Ready Interrupt Status"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - In-touch Interrupt Status"]
    #[inline(always)]
    pub fn intch(&self) -> INTCH_R {
        INTCH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Out-of-Touch Interrupt Status"]
    #[inline(always)]
    pub fn outtch(&self) -> OUTTCH_R {
        OUTTCH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
