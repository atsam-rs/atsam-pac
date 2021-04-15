#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `FC0R`"]
pub type FC0R_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Frame Counter 0 Rollover"]
    #[inline(always)]
    pub fn fc0r(&self) -> FC0R_R {
        FC0R_R::new((self.bits & 0x01) != 0)
    }
}
