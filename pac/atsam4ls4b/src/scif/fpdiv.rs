#[doc = "Register `FPDIV` reader"]
pub struct R(crate::R<FPDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPDIV` writer"]
pub struct W(crate::W<FPDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPDIV_SPEC>;
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
impl From<crate::W<FPDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPDIV` reader - Fractional Prescaler Division Factor"]
pub struct FPDIV_R(crate::FieldReader<u16, u16>);
impl FPDIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        FPDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPDIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPDIV` writer - Fractional Prescaler Division Factor"]
pub struct FPDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FPDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Fractional Prescaler Division Factor"]
    #[inline(always)]
    pub fn fpdiv(&self) -> FPDIV_R {
        FPDIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Fractional Prescaler Division Factor"]
    #[inline(always)]
    pub fn fpdiv(&mut self) -> FPDIV_W {
        FPDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fractional Prescaler DIVIDER Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpdiv](index.html) module"]
pub struct FPDIV_SPEC;
impl crate::RegisterSpec for FPDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpdiv::R](R) reader structure"]
impl crate::Readable for FPDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpdiv::W](W) writer structure"]
impl crate::Writable for FPDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPDIV to value 0"]
impl crate::Resettable for FPDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
