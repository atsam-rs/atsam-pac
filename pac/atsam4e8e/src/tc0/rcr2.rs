#[doc = "Register `RCR2` reader"]
pub struct R(crate::R<RCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCR2` writer"]
pub struct W(crate::W<RCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR2_SPEC>;
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
impl From<crate::W<RCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXCTR` reader - Receive Counter Register"]
pub struct RXCTR_R(crate::FieldReader<u16, u16>);
impl RXCTR_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXCTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCTR` writer - Receive Counter Register"]
pub struct RXCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Receive Counter Register"]
    #[inline(always)]
    pub fn rxctr(&self) -> RXCTR_R {
        RXCTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Counter Register"]
    #[inline(always)]
    pub fn rxctr(&mut self) -> RXCTR_W {
        RXCTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Counter Register (pdc = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr2](index.html) module"]
pub struct RCR2_SPEC;
impl crate::RegisterSpec for RCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcr2::R](R) reader structure"]
impl crate::Readable for RCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcr2::W](W) writer structure"]
impl crate::Writable for RCR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCR2 to value 0"]
impl crate::Resettable for RCR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
