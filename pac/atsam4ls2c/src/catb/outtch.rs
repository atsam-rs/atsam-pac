#[doc = "Reader of register OUTTCH%s"]
pub type R = crate::R<u32, super::OUTTCH>;
#[doc = "Reader of field `OUTTCH`"]
pub type OUTTCH_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Out-of-Touch"]
    #[inline(always)]
    pub fn outtch(&self) -> OUTTCH_R {
        OUTTCH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
