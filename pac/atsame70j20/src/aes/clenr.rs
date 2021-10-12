#[doc = "Register `CLENR` reader"]
pub struct R(crate::R<CLENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLENR` writer"]
pub struct W(crate::W<CLENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLENR_SPEC>;
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
impl From<crate::W<CLENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLEN` reader - Plaintext/Ciphertext Length"]
pub struct CLEN_R(crate::FieldReader<u32, u32>);
impl CLEN_R {
    pub(crate) fn new(bits: u32) -> Self {
        CLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLEN` writer - Plaintext/Ciphertext Length"]
pub struct CLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Plaintext/Ciphertext Length"]
    #[inline(always)]
    pub fn clen(&self) -> CLEN_R {
        CLEN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Plaintext/Ciphertext Length"]
    #[inline(always)]
    pub fn clen(&mut self) -> CLEN_W {
        CLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Plaintext/Ciphertext Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clenr](index.html) module"]
pub struct CLENR_SPEC;
impl crate::RegisterSpec for CLENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clenr::R](R) reader structure"]
impl crate::Readable for CLENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clenr::W](W) writer structure"]
impl crate::Writable for CLENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLENR to value 0"]
impl crate::Resettable for CLENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
