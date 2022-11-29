#[doc = "Register `CHSR` reader"]
pub struct R(crate::R<CHSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHS` reader - Channel Status"]
pub type CHS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Status"]
    #[inline(always)]
    pub fn chs(&self) -> CHS_R {
        CHS_R::new(self.bits)
    }
}
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chsr](index.html) module"]
pub struct CHSR_SPEC;
impl crate::RegisterSpec for CHSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chsr::R](R) reader structure"]
impl crate::Readable for CHSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHSR to value 0"]
impl crate::Resettable for CHSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
