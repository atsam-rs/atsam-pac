#[doc = "Reader of register PMC_PCSR1"]
pub type R = crate::R<u32, super::PMC_PCSR1>;
#[doc = "Reader of field `PID32`"]
pub type PID32_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID33`"]
pub type PID33_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID34`"]
pub type PID34_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Peripheral Clock 32 Status"]
    #[inline(always)]
    pub fn pid32(&self) -> PID32_R {
        PID32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Status"]
    #[inline(always)]
    pub fn pid33(&self) -> PID33_R {
        PID33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Status"]
    #[inline(always)]
    pub fn pid34(&self) -> PID34_R {
        PID34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
