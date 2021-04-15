#[doc = "Reader of register UNAME2"]
pub type R = crate::R<u32, super::UNAME2>;
#[doc = "Reader of field `UNAME2`"]
pub type UNAME2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IP Name Part Two"]
    #[inline(always)]
    pub fn uname2(&self) -> UNAME2_R {
        UNAME2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
