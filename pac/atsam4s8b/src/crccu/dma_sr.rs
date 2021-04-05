#[doc = "Reader of register DMA_SR"]
pub type R = crate::R<u32, super::DMA_SR>;
#[doc = "Reader of field `DMASR`"]
pub type DMASR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - DMA Status"]
    #[inline(always)]
    pub fn dmasr(&self) -> DMASR_R {
        DMASR_R::new((self.bits & 0x01) != 0)
    }
}
