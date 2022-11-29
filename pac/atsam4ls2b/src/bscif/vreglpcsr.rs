#[doc = "Register `VREGLPCSR` reader"]
pub struct R(crate::R<VREGLPCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREGLPCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREGLPCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREGLPCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREGLPCSR` writer"]
pub struct W(crate::W<VREGLPCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREGLPCSR_SPEC>;
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
impl From<crate::W<VREGLPCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREGLPCSR_SPEC>) -> Self {
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
#[doc = "LP Mode Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vreglpcsr](index.html) module"]
pub struct VREGLPCSR_SPEC;
impl crate::RegisterSpec for VREGLPCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vreglpcsr::R](R) reader structure"]
impl crate::Readable for VREGLPCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vreglpcsr::W](W) writer structure"]
impl crate::Writable for VREGLPCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREGLPCSR to value 0"]
impl crate::Resettable for VREGLPCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
