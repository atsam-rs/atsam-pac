#[doc = "Register `OVSR` reader"]
pub struct R(crate::R<OVSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVS` reader - Overrun Interrupt Status"]
pub type OVS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Overrun Interrupt Status"]
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new(self.bits)
    }
}
#[doc = "Overrun Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovsr](index.html) module"]
pub struct OVSR_SPEC;
impl crate::RegisterSpec for OVSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ovsr::R](R) reader structure"]
impl crate::Readable for OVSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OVSR to value 0"]
impl crate::Resettable for OVSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
