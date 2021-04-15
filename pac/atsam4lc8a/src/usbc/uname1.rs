#[doc = "Reader of register UNAME1"]
pub type R = crate::R<u32, super::UNAME1>;
#[doc = "Reader of field `UNAME1`"]
pub type UNAME1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IP Name Part One"]
    #[inline(always)]
    pub fn uname1(&self) -> UNAME1_R {
        UNAME1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
