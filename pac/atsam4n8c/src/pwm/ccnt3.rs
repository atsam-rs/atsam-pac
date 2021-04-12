#[doc = "Reader of register CCNT3"]
pub type R = crate::R<u32, super::CCNT3>;
#[doc = "Reader of field `CNT`"]
pub type CNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Counter Register"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}