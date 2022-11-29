#[doc = "Register `IGFDR` reader"]
pub struct R(crate::R<IGFDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IGFDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IGFDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IGFDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IGFDR` writer"]
pub struct W(crate::W<IGFDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IGFDR_SPEC>;
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
impl From<crate::W<IGFDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IGFDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IGFDR` reader - Input Glitch Filter Divider Register"]
pub type IGFDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IGFDR` writer - Input Glitch Filter Divider Register"]
pub type IGFDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IGFDR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Input Glitch Filter Divider Register"]
    #[inline(always)]
    pub fn igfdr(&self) -> IGFDR_R {
        IGFDR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input Glitch Filter Divider Register"]
    #[inline(always)]
    #[must_use]
    pub fn igfdr(&mut self) -> IGFDR_W<0> {
        IGFDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Glitch Filter Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [igfdr](index.html) module"]
pub struct IGFDR_SPEC;
impl crate::RegisterSpec for IGFDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [igfdr::R](R) reader structure"]
impl crate::Readable for IGFDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [igfdr::W](W) writer structure"]
impl crate::Writable for IGFDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IGFDR to value 0"]
impl crate::Resettable for IGFDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
