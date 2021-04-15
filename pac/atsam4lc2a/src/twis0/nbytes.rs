#[doc = "Register `NBYTES` reader"]
pub struct R(crate::R<NBYTES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NBYTES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<NBYTES_SPEC>> for R {
    fn from(reader: crate::R<NBYTES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NBYTES` writer"]
pub struct W(crate::W<NBYTES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NBYTES_SPEC>;
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
impl core::convert::From<crate::W<NBYTES_SPEC>> for W {
    fn from(writer: crate::W<NBYTES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NBYTES` reader - Number of Bytes to Transfer"]
pub struct NBYTES_R(crate::FieldReader<u8, u8>);
impl NBYTES_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBYTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBYTES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBYTES` writer - Number of Bytes to Transfer"]
pub struct NBYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NBYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of Bytes to Transfer"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of Bytes to Transfer"]
    #[inline(always)]
    pub fn nbytes(&mut self) -> NBYTES_W {
        NBYTES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NBYTES Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nbytes](index.html) module"]
pub struct NBYTES_SPEC;
impl crate::RegisterSpec for NBYTES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nbytes::R](R) reader structure"]
impl crate::Readable for NBYTES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nbytes::W](W) writer structure"]
impl crate::Writable for NBYTES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NBYTES to value 0"]
impl crate::Resettable for NBYTES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
