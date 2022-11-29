#[doc = "Register `AR1` reader"]
pub struct R(crate::R<AR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AR1` writer"]
pub struct W(crate::W<AR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AR1_SPEC>;
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
impl From<crate::W<AR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - Alarm Value"]
pub type VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `VALUE` writer - Alarm Value"]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AR1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Alarm Value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alarm Value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ar1](index.html) module"]
pub struct AR1_SPEC;
impl crate::RegisterSpec for AR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ar1::R](R) reader structure"]
impl crate::Readable for AR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ar1::W](W) writer structure"]
impl crate::Writable for AR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AR1 to value 0"]
impl crate::Resettable for AR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
