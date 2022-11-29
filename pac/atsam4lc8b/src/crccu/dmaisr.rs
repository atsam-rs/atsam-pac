#[doc = "Register `DMAISR` reader"]
pub struct R(crate::R<DMAISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMAISR` reader - DMA Interrupt Status"]
pub type DMAISR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DMA Interrupt Status"]
    #[inline(always)]
    pub fn dmaisr(&self) -> DMAISR_R {
        DMAISR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "DMA Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaisr](index.html) module"]
pub struct DMAISR_SPEC;
impl crate::RegisterSpec for DMAISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaisr::R](R) reader structure"]
impl crate::Readable for DMAISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMAISR to value 0"]
impl crate::Resettable for DMAISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
