#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CFG_SPEC>> for R {
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl core::convert::From<crate::W<CFG_SPEC>> for W {
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XBIAS` reader - External Bias Generation"]
pub struct XBIAS_R(crate::FieldReader<bool, bool>);
impl XBIAS_R {
    pub(crate) fn new(bits: bool) -> Self {
        XBIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XBIAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XBIAS` writer - External Bias Generation"]
pub struct XBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> XBIAS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `WMOD` reader - Waveform Mode"]
pub struct WMOD_R(crate::FieldReader<bool, bool>);
impl WMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        WMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WMOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WMOD` writer - Waveform Mode"]
pub struct WMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> WMOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `BLANK` reader - Blank LCD"]
pub struct BLANK_R(crate::FieldReader<bool, bool>);
impl BLANK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLANK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLANK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLANK` writer - Blank LCD"]
pub struct BLANK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLANK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `LOCK` reader - Lock"]
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` writer - Lock"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DUTY` reader - Duty Select"]
pub struct DUTY_R(crate::FieldReader<u8, u8>);
impl DUTY_R {
    pub(crate) fn new(bits: u8) -> Self {
        DUTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY` writer - Duty Select"]
pub struct DUTY_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `FCST` reader - Fine Contrast"]
pub struct FCST_R(crate::FieldReader<u8, u8>);
impl FCST_R {
    pub(crate) fn new(bits: u8) -> Self {
        FCST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCST` writer - Fine Contrast"]
pub struct FCST_W<'a> {
    w: &'a mut W,
}
impl<'a> FCST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `NSU` reader - Number of Segment Terminals in Use"]
pub struct NSU_R(crate::FieldReader<u8, u8>);
impl NSU_R {
    pub(crate) fn new(bits: u8) -> Self {
        NSU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSU` writer - Number of Segment Terminals in Use"]
pub struct NSU_W<'a> {
    w: &'a mut W,
}
impl<'a> NSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - External Bias Generation"]
    #[inline(always)]
    pub fn xbias(&self) -> XBIAS_R {
        XBIAS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Waveform Mode"]
    #[inline(always)]
    pub fn wmod(&self) -> WMOD_R {
        WMOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Blank LCD"]
    #[inline(always)]
    pub fn blank(&self) -> BLANK_R {
        BLANK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Duty Select"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:21 - Fine Contrast"]
    #[inline(always)]
    pub fn fcst(&self) -> FCST_R {
        FCST_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Number of Segment Terminals in Use"]
    #[inline(always)]
    pub fn nsu(&self) -> NSU_R {
        NSU_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Bias Generation"]
    #[inline(always)]
    pub fn xbias(&mut self) -> XBIAS_W {
        XBIAS_W { w: self }
    }
    #[doc = "Bit 1 - Waveform Mode"]
    #[inline(always)]
    pub fn wmod(&mut self) -> WMOD_W {
        WMOD_W { w: self }
    }
    #[doc = "Bit 2 - Blank LCD"]
    #[inline(always)]
    pub fn blank(&mut self) -> BLANK_W {
        BLANK_W { w: self }
    }
    #[doc = "Bit 3 - Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Bits 8:9 - Duty Select"]
    #[inline(always)]
    pub fn duty(&mut self) -> DUTY_W {
        DUTY_W { w: self }
    }
    #[doc = "Bits 16:21 - Fine Contrast"]
    #[inline(always)]
    pub fn fcst(&mut self) -> FCST_W {
        FCST_W { w: self }
    }
    #[doc = "Bits 24:29 - Number of Segment Terminals in Use"]
    #[inline(always)]
    pub fn nsu(&mut self) -> NSU_W {
        NSU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
