#[doc = "Register `DMA_SR` reader"]
pub struct R(crate::R<DMA_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMASR` reader - DMA Status Register"]
pub type DMASR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DMA Status Register"]
    #[inline(always)]
    pub fn dmasr(&self) -> DMASR_R {
        DMASR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "CRCCU DMA Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_sr](index.html) module"]
pub struct DMA_SR_SPEC;
impl crate::RegisterSpec for DMA_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_sr::R](R) reader structure"]
impl crate::Readable for DMA_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_SR to value 0"]
impl crate::Resettable for DMA_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
