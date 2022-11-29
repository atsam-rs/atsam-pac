#[doc = "Register `AWEN` reader"]
pub struct R(crate::R<AWEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWEN` writer"]
pub struct W(crate::W<AWEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWEN_SPEC>;
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
impl From<crate::W<AWEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AWEN` reader - Asynchronous Wake Up"]
pub type AWEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AWEN` writer - Asynchronous Wake Up"]
pub type AWEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AWEN_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Asynchronous Wake Up"]
    #[inline(always)]
    pub fn awen(&self) -> AWEN_R {
        AWEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Asynchronous Wake Up"]
    #[inline(always)]
    #[must_use]
    pub fn awen(&mut self) -> AWEN_W<0> {
        AWEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Asynchronous Wake Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awen](index.html) module"]
pub struct AWEN_SPEC;
impl crate::RegisterSpec for AWEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awen::R](R) reader structure"]
impl crate::Readable for AWEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awen::W](W) writer structure"]
impl crate::Writable for AWEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AWEN to value 0"]
impl crate::Resettable for AWEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
