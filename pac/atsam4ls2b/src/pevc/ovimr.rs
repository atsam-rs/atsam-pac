#[doc = "Register `OVIMR` reader"]
pub struct R(crate::R<OVIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVIM` reader - Overrun Interrupt Mask"]
pub type OVIM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn ovim(&self) -> OVIM_R {
        OVIM_R::new(self.bits)
    }
}
#[doc = "Overrun Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovimr](index.html) module"]
pub struct OVIMR_SPEC;
impl crate::RegisterSpec for OVIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ovimr::R](R) reader structure"]
impl crate::Readable for OVIMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OVIMR to value 0"]
impl crate::Resettable for OVIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
