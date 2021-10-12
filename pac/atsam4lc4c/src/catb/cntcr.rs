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
pub struct TOP_R(crate::FieldReader<u32, u32>);
impl TOP_R {
    pub(crate) fn new(bits: u32) -> Self {
        TOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOP` writer - Counter Top Value"]
pub struct TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Field `SPREAD` reader - Spread Spectrum"]
pub struct SPREAD_R(crate::FieldReader<u8, u8>);
impl SPREAD_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPREAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPREAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPREAD` writer - Spread Spectrum"]
pub struct SPREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPREAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `REPEAT` reader - Repeat Measurements"]
pub struct REPEAT_R(crate::FieldReader<u8, u8>);
impl REPEAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        REPEAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REPEAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REPEAT` writer - Repeat Measurements"]
pub struct REPEAT_W<'a> {
    w: &'a mut W,
}
impl<'a> REPEAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Counter Top Value"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:27 - Spread Spectrum"]
    #[inline(always)]
    pub fn spread(&self) -> SPREAD_R {
        SPREAD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Repeat Measurements"]
    #[inline(always)]
    pub fn repeat(&self) -> REPEAT_R {
        REPEAT_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Counter Top Value"]
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W {
        TOP_W { w: self }
    }
    #[doc = "Bits 24:27 - Spread Spectrum"]
    #[inline(always)]
    pub fn spread(&mut self) -> SPREAD_W {
        SPREAD_W { w: self }
    }
    #[doc = "Bits 28:30 - Repeat Measurements"]
    #[inline(always)]
    pub fn repeat(&mut self) -> REPEAT_W {
        REPEAT_W { w: self }
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
}
#[doc = "`reset()` method sets CNTCR to value 0"]
impl crate::Resettable for CNTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
