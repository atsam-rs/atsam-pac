#[doc = "Reader of register DMA_ISR"]
pub type R = crate::R<u32, super::DMA_ISR>;
#[doc = "Reader of field `DMAISR`"]
pub type DMAISR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Interrupt Status register"]
    #[inline(always)]
    pub fn dmaisr(&self) -> DMAISR_R {
        DMAISR_R::new((self.bits & 0x01) != 0)
    }
}
