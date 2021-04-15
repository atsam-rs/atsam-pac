#[doc = "Reader of register TRIMR"]
pub type R = crate::R<u32, super::TRIMR>;
#[doc = "Reader of field `TRIM`"]
pub type TRIM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Trigger Interrupt Mask"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
