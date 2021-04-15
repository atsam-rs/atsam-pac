#[doc = "Reader of register TRSR"]
pub type R = crate::R<u32, super::TRSR>;
#[doc = "Reader of field `TRS`"]
pub type TRS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Trigger Interrupt Status"]
    #[inline(always)]
    pub fn trs(&self) -> TRS_R {
        TRS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
