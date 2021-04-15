#[doc = "Reader of register PARAMETER"]
pub type R = crate::R<u32, super::PARAMETER>;
#[doc = "Reader of field `N`"]
pub type N_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of channels"]
    #[inline(always)]
    pub fn n(&self) -> N_R {
        N_R::new((self.bits & 0xff) as u8)
    }
}
