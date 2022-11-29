#[doc = "Register `DMASR` reader"]
pub struct R(crate::R<DMASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMASR` reader - DMA Channel Status"]
pub type DMASR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DMA Channel Status"]
    #[inline(always)]
    pub fn dmasr(&self) -> DMASR_R {
        DMASR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "DMA Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasr](index.html) module"]
pub struct DMASR_SPEC;
impl crate::RegisterSpec for DMASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmasr::R](R) reader structure"]
impl crate::Readable for DMASR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMASR to value 0"]
impl crate::Resettable for DMASR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
