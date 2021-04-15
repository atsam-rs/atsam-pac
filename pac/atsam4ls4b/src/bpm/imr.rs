#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `PSOK`"]
pub type PSOK_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE`"]
pub type AE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Power Scaling OK Interrupt Mask"]
    #[inline(always)]
    pub fn psok(&self) -> PSOK_R {
        PSOK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 31 - Access Error Interrupt Mask"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
