#[doc = "Register `PRLAT0` reader"]
pub struct R(crate::R<PRLAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRLAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRLAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRLAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LAT` reader - Maximum Transfer Initiation cycles counted since last reset"]
pub type LAT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Maximum Transfer Initiation cycles counted since last reset"]
    #[inline(always)]
    pub fn lat(&self) -> LAT_R {
        LAT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel 0 Read Max Latency\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prlat0](index.html) module"]
pub struct PRLAT0_SPEC;
impl crate::RegisterSpec for PRLAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prlat0::R](R) reader structure"]
impl crate::Readable for PRLAT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRLAT0 to value 0"]
impl crate::Resettable for PRLAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
