#[doc = "Register `PWLAT0` reader"]
pub struct R(crate::R<PWLAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWLAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWLAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWLAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LAT` reader - Maximum transfer initiation cycles counted since last reset"]
pub type LAT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Maximum transfer initiation cycles counted since last reset"]
    #[inline(always)]
    pub fn lat(&self) -> LAT_R {
        LAT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel0 Write Max Latency\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwlat0](index.html) module"]
pub struct PWLAT0_SPEC;
impl crate::RegisterSpec for PWLAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwlat0::R](R) reader structure"]
impl crate::Readable for PWLAT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWLAT0 to value 0"]
impl crate::Resettable for PWLAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
