#[doc = "Register `BKUPWEN` reader"]
pub struct R(crate::R<BKUPWEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKUPWEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKUPWEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKUPWEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BKUPWEN` writer"]
pub struct W(crate::W<BKUPWEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKUPWEN_SPEC>;
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
impl From<crate::W<BKUPWEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKUPWEN_SPEC>) -> Self {
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
#[doc = "Backup Wake up Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkupwen](index.html) module"]
pub struct BKUPWEN_SPEC;
impl crate::RegisterSpec for BKUPWEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkupwen::R](R) reader structure"]
impl crate::Readable for BKUPWEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bkupwen::W](W) writer structure"]
impl crate::Writable for BKUPWEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BKUPWEN to value 0"]
impl crate::Resettable for BKUPWEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
