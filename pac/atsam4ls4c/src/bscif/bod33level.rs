#[doc = "Register `BOD33LEVEL` reader"]
pub struct R(crate::R<BOD33LEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD33LEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD33LEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD33LEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD33LEVEL` writer"]
pub struct W(crate::W<BOD33LEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD33LEVEL_SPEC>;
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
impl From<crate::W<BOD33LEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOD33LEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - BOD Value"]
pub type VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VAL` writer - BOD Value"]
pub type VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BOD33LEVEL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - BOD Value"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - BOD Value"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<0> {
        VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOD33 Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod33level](index.html) module"]
pub struct BOD33LEVEL_SPEC;
impl crate::RegisterSpec for BOD33LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod33level::R](R) reader structure"]
impl crate::Readable for BOD33LEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod33level::W](W) writer structure"]
impl crate::Writable for BOD33LEVEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOD33LEVEL to value 0"]
impl crate::Resettable for BOD33LEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
