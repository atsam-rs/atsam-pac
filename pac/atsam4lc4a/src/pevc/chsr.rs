#[doc = "Reader of register CHSR"]
pub type R = crate::R<u32, super::CHSR>;
#[doc = "Reader of field `CHS`"]
pub type CHS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Status"]
    #[inline(always)]
    pub fn chs(&self) -> CHS_R {
        CHS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
