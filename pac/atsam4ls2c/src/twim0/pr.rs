#[doc = "Reader of register PR"]
pub type R = crate::R<u32, super::PR>;
#[doc = "Reader of field `HS`"]
pub type HS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - HS-mode"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new((self.bits & 0x01) != 0)
    }
}
