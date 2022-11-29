#[doc = "Register `CNTCR` reader"]
pub struct R(crate::R<CNTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTCR` writer"]
pub struct W(crate::W<CNTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTCR_SPEC>;
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
impl From<crate::W<CNTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOP` reader - Counter Top Value"]
pub type TOP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TOP` writer - Counter Top Value"]
pub type TOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNTCR_SPEC, u32, u32, 24, O>;
#[doc = "Field `SPREAD` reader - Spread Spectrum"]
pub type SPREAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPREAD` writer - Spread Spectrum"]
pub type SPREAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNTCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `REPEAT` reader - Repeat Measurements"]
pub type REPEAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REPEAT` writer - Repeat Measurements"]
pub type REPEAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNTCR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:23 - Counter Top Value"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:27 - Spread Spectrum"]
    #[inline(always)]
    pub fn spread(&self) -> SPREAD_R {
        SPREAD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Repeat Measurements"]
    #[inline(always)]
    pub fn repeat(&self) -> REPEAT_R {
        REPEAT_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Counter Top Value"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<0> {
        TOP_W::new(self)
    }
    #[doc = "Bits 24:27 - Spread Spectrum"]
    #[inline(always)]
    #[must_use]
    pub fn spread(&mut self) -> SPREAD_W<24> {
        SPREAD_W::new(self)
    }
    #[doc = "Bits 28:30 - Repeat Measurements"]
    #[inline(always)]
    #[must_use]
    pub fn repeat(&mut self) -> REPEAT_W<28> {
        REPEAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntcr](index.html) module"]
pub struct CNTCR_SPEC;
impl crate::RegisterSpec for CNTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntcr::R](R) reader structure"]
impl crate::Readable for CNTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntcr::W](W) writer structure"]
impl crate::Writable for CNTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTCR to value 0"]
impl crate::Resettable for CNTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
