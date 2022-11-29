#[doc = "Register `ASR%s` reader"]
pub struct R(crate::R<ASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASR%s` writer"]
pub struct W(crate::W<ASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASR_SPEC>;
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
impl From<crate::W<ASR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AR` reader - Access Error"]
pub type AR_R = crate::BitReader<bool>;
#[doc = "Field `AR` writer - Access Error"]
pub type AR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Access Error"]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access Error"]
    #[inline(always)]
    #[must_use]
    pub fn ar(&mut self) -> AR_W<0> {
        AR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asr](index.html) module"]
pub struct ASR_SPEC;
impl crate::RegisterSpec for ASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asr::R](R) reader structure"]
impl crate::Readable for ASR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asr::W](W) writer structure"]
impl crate::Writable for ASR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASR%s to value 0"]
impl crate::Resettable for ASR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
