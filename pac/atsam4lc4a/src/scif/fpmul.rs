#[doc = "Register `FPMUL` reader"]
pub struct R(crate::R<FPMUL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPMUL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPMUL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPMUL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPMUL` writer"]
pub struct W(crate::W<FPMUL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPMUL_SPEC>;
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
impl From<crate::W<FPMUL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPMUL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPMUL` reader - Fractional Prescaler Multiplication Factor"]
pub type FPMUL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FPMUL` writer - Fractional Prescaler Multiplication Factor"]
pub type FPMUL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPMUL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Fractional Prescaler Multiplication Factor"]
    #[inline(always)]
    pub fn fpmul(&self) -> FPMUL_R {
        FPMUL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Fractional Prescaler Multiplication Factor"]
    #[inline(always)]
    #[must_use]
    pub fn fpmul(&mut self) -> FPMUL_W<0> {
        FPMUL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fractional Prescaler Multiplier Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpmul](index.html) module"]
pub struct FPMUL_SPEC;
impl crate::RegisterSpec for FPMUL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpmul::R](R) reader structure"]
impl crate::Readable for FPMUL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpmul::W](W) writer structure"]
impl crate::Writable for FPMUL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPMUL to value 0"]
impl crate::Resettable for FPMUL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
