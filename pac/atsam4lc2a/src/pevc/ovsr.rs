#[doc = "Register `OVSR` reader"]
pub struct R(crate::R<OVSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OVSR_SPEC>> for R {
    fn from(reader: crate::R<OVSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVS` reader - Overrun Interrupt Status"]
pub struct OVS_R(crate::FieldReader<u32, u32>);
impl OVS_R {
    pub(crate) fn new(bits: u32) -> Self {
        OVS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Overrun Interrupt Status"]
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new((self.bits & 0xffff_ffff) as u32)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
