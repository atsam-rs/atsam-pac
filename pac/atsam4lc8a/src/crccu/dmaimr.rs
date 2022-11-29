#[doc = "Register `DMAIMR` reader"]
pub struct R(crate::R<DMAIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "DMA Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaimr](index.html) module"]
pub struct DMAIMR_SPEC;
impl crate::RegisterSpec for DMAIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaimr::R](R) reader structure"]
impl crate::Readable for DMAIMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMAIMR to value 0"]
impl crate::Resettable for DMAIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
