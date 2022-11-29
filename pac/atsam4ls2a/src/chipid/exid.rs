#[doc = "Register `EXID` reader"]
pub struct R(crate::R<EXID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Chip ID Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exid](index.html) module"]
pub struct EXID_SPEC;
impl crate::RegisterSpec for EXID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exid::R](R) reader structure"]
impl crate::Readable for EXID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXID to value 0x0400_000f"]
impl crate::Resettable for EXID_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_000f;
}
