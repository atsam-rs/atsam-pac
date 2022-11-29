#[doc = "Register `BR%s` reader"]
pub struct R(crate::R<BR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BR%s` writer"]
pub struct W(crate::W<BR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BR_SPEC>;
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
impl From<crate::W<BR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BR_SPEC>) -> Self {
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
#[doc = "Backup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [br](index.html) module"]
pub struct BR_SPEC;
impl crate::RegisterSpec for BR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [br::R](R) reader structure"]
impl crate::Readable for BR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [br::W](W) writer structure"]
impl crate::Writable for BR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BR%s to value 0"]
impl crate::Resettable for BR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
