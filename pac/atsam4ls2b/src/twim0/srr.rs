#[doc = "Register `SRR` reader"]
pub struct R(crate::R<SRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRR` writer"]
pub struct W(crate::W<SRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRR_SPEC>;
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
impl From<crate::W<SRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DADRIVEL` reader - Data Drive Strength LOW"]
pub struct DADRIVEL_R(crate::FieldReader<u8, u8>);
impl DADRIVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DADRIVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DADRIVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DADRIVEL` writer - Data Drive Strength LOW"]
pub struct DADRIVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DADRIVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `DASLEW` reader - Data Slew Limit"]
pub struct DASLEW_R(crate::FieldReader<u8, u8>);
impl DASLEW_R {
    pub(crate) fn new(bits: u8) -> Self {
        DASLEW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DASLEW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DASLEW` writer - Data Slew Limit"]
pub struct DASLEW_W<'a> {
    w: &'a mut W,
}
impl<'a> DASLEW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `CLDRIVEL` reader - Clock Drive Strength LOW"]
pub struct CLDRIVEL_R(crate::FieldReader<u8, u8>);
impl CLDRIVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLDRIVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLDRIVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLDRIVEL` writer - Clock Drive Strength LOW"]
pub struct CLDRIVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLDRIVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `CLSLEW` reader - Clock Slew Limit"]
pub struct CLSLEW_R(crate::FieldReader<u8, u8>);
impl CLSLEW_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLSLEW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLSLEW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLSLEW` writer - Clock Slew Limit"]
pub struct CLSLEW_W<'a> {
    w: &'a mut W,
}
impl<'a> CLSLEW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `FILTER` reader - Input Spike Filter Control"]
pub struct FILTER_R(crate::FieldReader<u8, u8>);
impl FILTER_R {
    pub(crate) fn new(bits: u8) -> Self {
        FILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER` writer - Input Spike Filter Control"]
pub struct FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Data Drive Strength LOW"]
    #[inline(always)]
    pub fn dadrivel(&self) -> DADRIVEL_R {
        DADRIVEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Data Slew Limit"]
    #[inline(always)]
    pub fn daslew(&self) -> DASLEW_R {
        DASLEW_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - Clock Drive Strength LOW"]
    #[inline(always)]
    pub fn cldrivel(&self) -> CLDRIVEL_R {
        CLDRIVEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - Clock Slew Limit"]
    #[inline(always)]
    pub fn clslew(&self) -> CLSLEW_R {
        CLSLEW_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Input Spike Filter Control"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Data Drive Strength LOW"]
    #[inline(always)]
    pub fn dadrivel(&mut self) -> DADRIVEL_W {
        DADRIVEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Data Slew Limit"]
    #[inline(always)]
    pub fn daslew(&mut self) -> DASLEW_W {
        DASLEW_W { w: self }
    }
    #[doc = "Bits 16:18 - Clock Drive Strength LOW"]
    #[inline(always)]
    pub fn cldrivel(&mut self) -> CLDRIVEL_W {
        CLDRIVEL_W { w: self }
    }
    #[doc = "Bits 24:25 - Clock Slew Limit"]
    #[inline(always)]
    pub fn clslew(&mut self) -> CLSLEW_W {
        CLSLEW_W { w: self }
    }
    #[doc = "Bits 28:29 - Input Spike Filter Control"]
    #[inline(always)]
    pub fn filter(&mut self) -> FILTER_W {
        FILTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slew Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srr](index.html) module"]
pub struct SRR_SPEC;
impl crate::RegisterSpec for SRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srr::R](R) reader structure"]
impl crate::Readable for SRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srr::W](W) writer structure"]
impl crate::Writable for SRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRR to value 0"]
impl crate::Resettable for SRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
