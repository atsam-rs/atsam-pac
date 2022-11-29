#[doc = "Register `CIDR` reader"]
pub struct R(crate::R<CIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Chip ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr](index.html) module"]
pub struct CIDR_SPEC;
impl crate::RegisterSpec for CIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cidr::R](R) reader structure"]
impl crate::Readable for CIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIDR to value 0xab0a_09e0"]
impl crate::Resettable for CIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xab0a_09e0;
}
