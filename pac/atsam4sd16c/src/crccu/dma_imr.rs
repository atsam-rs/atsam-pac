#[doc = "Reader of register DMA_IMR"]
pub type R = crate::R<u32, super::DMA_IMR>;
#[doc = "Reader of field `DMAIMR`"]
pub type DMAIMR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Interrupt Mask"]
    #[inline(always)]
    pub fn dmaimr(&self) -> DMAIMR_R {
        DMAIMR_R::new((self.bits & 0x01) != 0)
    }
}
