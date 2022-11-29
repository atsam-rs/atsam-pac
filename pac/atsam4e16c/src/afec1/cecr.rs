#[doc = "Register `CECR` reader"]
pub struct R(crate::R<CECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CECR` writer"]
pub struct W(crate::W<CECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CECR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Channel Error Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cecr](index.html) module"]
pub struct CECR_SPEC;
impl crate::RegisterSpec for CECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cecr::R](R) reader structure"]
impl crate::Readable for CECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cecr::W](W) writer structure"]
impl crate::Writable for CECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CECR to value 0"]
impl crate::Resettable for CECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
