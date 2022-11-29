#[doc = "Register `PR` reader"]
pub struct R(crate::R<PR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HS` reader - HS-mode"]
pub type HS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - HS-mode"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr](index.html) module"]
pub struct PR_SPEC;
impl crate::RegisterSpec for PR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr::R](R) reader structure"]
impl crate::Readable for PR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PR to value 0x01"]
impl crate::Resettable for PR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
