#[doc = "Reader of register DFLL0RATIO"]
pub type R = crate::R<u32, super::DFLL0RATIO>;
#[doc = "Reader of field `RATIODIFF`"]
pub type RATIODIFF_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Multiplication Ratio Difference"]
    #[inline(always)]
    pub fn ratiodiff(&self) -> RATIODIFF_R {
        RATIODIFF_R::new((self.bits & 0xffff) as u16)
    }
}
