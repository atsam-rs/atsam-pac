#[doc = "Reader of register INTCH%s"]
pub type R = crate::R<u32, super::INTCH>;
#[doc = "Reader of field `INTCH`"]
pub type INTCH_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - In-Touch"]
    #[inline(always)]
    pub fn intch(&self) -> INTCH_R {
        INTCH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
