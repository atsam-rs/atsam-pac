#[doc = "Reader of register OVIMR"]
pub type R = crate::R<u32, super::OVIMR>;
#[doc = "Reader of field `OVIM`"]
pub type OVIM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn ovim(&self) -> OVIM_R {
        OVIM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
