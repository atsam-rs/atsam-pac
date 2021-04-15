#[doc = "Reader of register DMAISR"]
pub type R = crate::R<u32, super::DMAISR>;
#[doc = "Reader of field `DMAISR`"]
pub type DMAISR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - DMA Interrupt Status"]
    #[inline(always)]
    pub fn dmaisr(&self) -> DMAISR_R {
        DMAISR_R::new((self.bits & 0x01) != 0)
    }
}
