#[doc = "Register `PINSEL` reader"]
pub struct R(crate::R<PINSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINSEL` writer"]
pub struct W(crate::W<PINSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINSEL_SPEC>;
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
impl From<crate::W<PINSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PINSEL` reader - Pin Select"]
pub type PINSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PINSEL` writer - Pin Select"]
pub type PINSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PINSEL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Pin Select"]
    #[inline(always)]
    pub fn pinsel(&self) -> PINSEL_R {
        PINSEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn pinsel(&mut self) -> PINSEL_W<0> {
        PINSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinsel](index.html) module"]
pub struct PINSEL_SPEC;
impl crate::RegisterSpec for PINSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinsel::R](R) reader structure"]
impl crate::Readable for PINSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinsel::W](W) writer structure"]
impl crate::Writable for PINSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINSEL to value 0"]
impl crate::Resettable for PINSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
