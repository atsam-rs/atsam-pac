#[doc = "Register `BKUPWCAUSE` reader"]
pub struct R(crate::R<BKUPWCAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKUPWCAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKUPWCAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKUPWCAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Backup Wake up Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkupwcause](index.html) module"]
pub struct BKUPWCAUSE_SPEC;
impl crate::RegisterSpec for BKUPWCAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkupwcause::R](R) reader structure"]
impl crate::Readable for BKUPWCAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BKUPWCAUSE to value 0"]
impl crate::Resettable for BKUPWCAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
