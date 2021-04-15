#[doc = "Reader of register DMASR"]
pub type R = crate::R<u32, super::DMASR>;
#[doc = "Reader of field `DMASR`"]
pub type DMASR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - DMA Channel Status"]
    #[inline(always)]
    pub fn dmasr(&self) -> DMASR_R {
        DMASR_R::new((self.bits & 0x01) != 0)
    }
}
