#[doc = "Register `NBYTES` reader"]
pub struct R(crate::R<NBYTES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NBYTES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NBYTES_SPEC>> for R {
    #[inline(always)]
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
impl From<crate::W<NBYTES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NBYTES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NBYTES` reader - Number of Bytes to Transfer"]
pub type NBYTES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBYTES` writer - Number of Bytes to Transfer"]
pub type NBYTES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NBYTES_SPEC, u8, u8, 8, O>;
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
    #[must_use]
    pub fn nbytes(&mut self) -> NBYTES_W<0> {
        NBYTES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NBYTES to value 0"]
impl crate::Resettable for NBYTES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
