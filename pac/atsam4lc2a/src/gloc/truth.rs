#[doc = "Register `TRUTH%s` reader"]
pub struct R(crate::R<TRUTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRUTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TRUTH_SPEC>> for R {
    fn from(reader: crate::R<TRUTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRUTH%s` writer"]
pub struct W(crate::W<TRUTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRUTH_SPEC>;
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
impl core::convert::From<crate::W<TRUTH_SPEC>> for W {
    fn from(writer: crate::W<TRUTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRUTH` reader - Truth"]
pub struct TRUTH_R(crate::FieldReader<u16, u16>);
impl TRUTH_R {
    pub(crate) fn new(bits: u16) -> Self {
        TRUTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRUTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRUTH` writer - Truth"]
pub struct TRUTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TRUTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Truth"]
    #[inline(always)]
    pub fn truth(&self) -> TRUTH_R {
        TRUTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Truth"]
    #[inline(always)]
    pub fn truth(&mut self) -> TRUTH_W {
        TRUTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Truth Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [truth](index.html) module"]
pub struct TRUTH_SPEC;
impl crate::RegisterSpec for TRUTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [truth::R](R) reader structure"]
impl crate::Readable for TRUTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [truth::W](W) writer structure"]
impl crate::Writable for TRUTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRUTH%s to value 0"]
impl crate::Resettable for TRUTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
