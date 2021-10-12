#[doc = "Register `ACMCFG` reader"]
pub struct R(crate::R<ACMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACMCFG` writer"]
pub struct W(crate::W<ACMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACMCFG_SPEC>;
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
impl From<crate::W<ACMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Field `FCS` reader - Frame Counter Selection"]
pub struct FCS_R(crate::FieldReader<u8, u8>);
impl FCS_R {
    pub(crate) fn new(bits: u8) -> Self {
        FCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCS` writer - Frame Counter Selection"]
pub struct FCS_W<'a> {
    w: &'a mut W,
}
impl<'a> FCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `MODE` reader - Mode (sequential or scrolling)"]
pub struct MODE_R(crate::FieldReader<bool, bool>);
impl MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Mode (sequential or scrolling)"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
#[doc = "Field `DREV` reader - Digit Reverse"]
pub struct DREV_R(crate::FieldReader<bool, bool>);
impl DREV_R {
    pub(crate) fn new(bits: bool) -> Self {
        DREV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DREV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DREV` writer - Digit Reverse"]
pub struct DREV_W<'a> {
    w: &'a mut W,
}
impl<'a> DREV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TDG` reader - Type of Digit"]
pub struct TDG_R(crate::FieldReader<u8, u8>);
impl TDG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TDG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDG` writer - Type of Digit"]
pub struct TDG_W<'a> {
    w: &'a mut W,
}
impl<'a> TDG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `STSEG` reader - Start Segment"]
pub struct STSEG_R(crate::FieldReader<u8, u8>);
impl STSEG_R {
    pub(crate) fn new(bits: u8) -> Self {
        STSEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STSEG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STSEG` writer - Start Segment"]
pub struct STSEG_W<'a> {
    w: &'a mut W,
}
impl<'a> STSEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `STEPS` reader - Scrolling Steps"]
pub struct STEPS_R(crate::FieldReader<u8, u8>);
impl STEPS_R {
    pub(crate) fn new(bits: u8) -> Self {
        STEPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STEPS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STEPS` writer - Scrolling Steps"]
pub struct STEPS_W<'a> {
    w: &'a mut W,
}
impl<'a> STEPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `DIGN` reader - Digit Number"]
pub struct DIGN_R(crate::FieldReader<u8, u8>);
impl DIGN_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIGN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIGN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIGN` writer - Digit Number"]
pub struct DIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIGN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Mode (sequential or scrolling)"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Digit Reverse"]
    #[inline(always)]
    pub fn drev(&self) -> DREV_R {
        DREV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Type of Digit"]
    #[inline(always)]
    pub fn tdg(&self) -> TDG_R {
        TDG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 8:13 - Start Segment"]
    #[inline(always)]
    pub fn stseg(&self) -> STSEG_R {
        STSEG_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - Scrolling Steps"]
    #[inline(always)]
    pub fn steps(&self) -> STEPS_R {
        STEPS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Digit Number"]
    #[inline(always)]
    pub fn dign(&self) -> DIGN_R {
        DIGN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    pub fn fcs(&mut self) -> FCS_W {
        FCS_W { w: self }
    }
    #[doc = "Bit 3 - Mode (sequential or scrolling)"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 4 - Digit Reverse"]
    #[inline(always)]
    pub fn drev(&mut self) -> DREV_W {
        DREV_W { w: self }
    }
    #[doc = "Bits 5:6 - Type of Digit"]
    #[inline(always)]
    pub fn tdg(&mut self) -> TDG_W {
        TDG_W { w: self }
    }
    #[doc = "Bits 8:13 - Start Segment"]
    #[inline(always)]
    pub fn stseg(&mut self) -> STSEG_W {
        STSEG_W { w: self }
    }
    #[doc = "Bits 16:23 - Scrolling Steps"]
    #[inline(always)]
    pub fn steps(&mut self) -> STEPS_W {
        STEPS_W { w: self }
    }
    #[doc = "Bits 24:27 - Digit Number"]
    #[inline(always)]
    pub fn dign(&mut self) -> DIGN_W {
        DIGN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Automated Character Mapping Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmcfg](index.html) module"]
pub struct ACMCFG_SPEC;
impl crate::RegisterSpec for ACMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acmcfg::R](R) reader structure"]
impl crate::Readable for ACMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acmcfg::W](W) writer structure"]
impl crate::Writable for ACMCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACMCFG to value 0"]
impl crate::Resettable for ACMCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
