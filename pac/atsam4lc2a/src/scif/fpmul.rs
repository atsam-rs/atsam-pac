#[doc = "Register `FPMUL` reader"]
pub struct R(crate::R<FPMUL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPMUL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FPMUL_SPEC>> for R {
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
impl core::convert::From<crate::W<FPMUL_SPEC>> for W {
    fn from(writer: crate::W<FPMUL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPMUL` reader - Fractional Prescaler Multiplication Factor"]
pub struct FPMUL_R(crate::FieldReader<u16, u16>);
impl FPMUL_R {
    pub(crate) fn new(bits: u16) -> Self {
        FPMUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPMUL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPMUL` writer - Fractional Prescaler Multiplication Factor"]
pub struct FPMUL_W<'a> {
    w: &'a mut W,
}
impl<'a> FPMUL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
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
    pub fn fpmul(&mut self) -> FPMUL_W {
        FPMUL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
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
}
#[doc = "`reset()` method sets FPMUL to value 0"]
impl crate::Resettable for FPMUL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
