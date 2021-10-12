#[doc = "Register `THRESH` reader"]
pub struct R(crate::R<THRESH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRESH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRESH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRESH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THRESH` writer"]
pub struct W(crate::W<THRESH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THRESH_SPEC>;
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
impl From<crate::W<THRESH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THRESH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTHRESH` reader - Fractional part of Threshold Value"]
pub struct FTHRESH_R(crate::FieldReader<u16, u16>);
impl FTHRESH_R {
    pub(crate) fn new(bits: u16) -> Self {
        FTHRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTHRESH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTHRESH` writer - Fractional part of Threshold Value"]
pub struct FTHRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> FTHRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `RTHRESH` reader - Rational part of Threshold Value"]
pub struct RTHRESH_R(crate::FieldReader<u8, u8>);
impl RTHRESH_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTHRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTHRESH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTHRESH` writer - Rational part of Threshold Value"]
pub struct RTHRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTHRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | ((value as u32 & 0xff) << 12);
        self.w
    }
}
#[doc = "Field `DIR` reader - Threshold Direction"]
pub struct DIR_R(crate::FieldReader<bool, bool>);
impl DIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` writer - Threshold Direction"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `LENGTH` reader - Threshold Length"]
pub struct LENGTH_R(crate::FieldReader<u8, u8>);
impl LENGTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LENGTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LENGTH` writer - Threshold Length"]
pub struct LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Fractional part of Threshold Value"]
    #[inline(always)]
    pub fn fthresh(&self) -> FTHRESH_R {
        FTHRESH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - Rational part of Threshold Value"]
    #[inline(always)]
    pub fn rthresh(&self) -> RTHRESH_R {
        RTHRESH_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 23 - Threshold Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - Threshold Length"]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Fractional part of Threshold Value"]
    #[inline(always)]
    pub fn fthresh(&mut self) -> FTHRESH_W {
        FTHRESH_W { w: self }
    }
    #[doc = "Bits 12:19 - Rational part of Threshold Value"]
    #[inline(always)]
    pub fn rthresh(&mut self) -> RTHRESH_W {
        RTHRESH_W { w: self }
    }
    #[doc = "Bit 23 - Threshold Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bits 24:28 - Threshold Length"]
    #[inline(always)]
    pub fn length(&mut self) -> LENGTH_W {
        LENGTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thresh](index.html) module"]
pub struct THRESH_SPEC;
impl crate::RegisterSpec for THRESH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [thresh::R](R) reader structure"]
impl crate::Readable for THRESH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [thresh::W](W) writer structure"]
impl crate::Writable for THRESH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets THRESH to value 0"]
impl crate::Resettable for THRESH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
