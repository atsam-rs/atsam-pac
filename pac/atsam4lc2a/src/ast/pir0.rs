#[doc = "Register `PIR0` reader"]
pub struct R(crate::R<PIR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PIR0_SPEC>> for R {
    fn from(reader: crate::R<PIR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIR0` writer"]
pub struct W(crate::W<PIR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIR0_SPEC>;
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
impl core::convert::From<crate::W<PIR0_SPEC>> for W {
    fn from(writer: crate::W<PIR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSEL` reader - Interval Select"]
pub struct INSEL_R(crate::FieldReader<u8, u8>);
impl INSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        INSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSEL` writer - Interval Select"]
pub struct INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Interval Select"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Interval Select"]
    #[inline(always)]
    pub fn insel(&mut self) -> INSEL_W {
        INSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Periodic Interval Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pir0](index.html) module"]
pub struct PIR0_SPEC;
impl crate::RegisterSpec for PIR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pir0::R](R) reader structure"]
impl crate::Readable for PIR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pir0::W](W) writer structure"]
impl crate::Writable for PIR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIR0 to value 0"]
impl crate::Resettable for PIR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
