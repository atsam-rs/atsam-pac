#[doc = "Register `DMA_ISR` reader"]
pub struct R(crate::R<DMA_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMAISR` reader - Interrupt Status register"]
pub type DMAISR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Interrupt Status register"]
    #[inline(always)]
    pub fn dmaisr(&self) -> DMAISR_R {
        DMAISR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "CRCCU DMA Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_isr](index.html) module"]
pub struct DMA_ISR_SPEC;
impl crate::RegisterSpec for DMA_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_isr::R](R) reader structure"]
impl crate::Readable for DMA_ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_ISR to value 0"]
impl crate::Resettable for DMA_ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
