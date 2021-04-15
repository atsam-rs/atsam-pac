#[doc = "Register `RCFASTCFG` reader"]
pub struct R(crate::R<RCFASTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCFASTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RCFASTCFG_SPEC>> for R {
    fn from(reader: crate::R<RCFASTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCFASTCFG` writer"]
pub struct W(crate::W<RCFASTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCFASTCFG_SPEC>;
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
impl core::convert::From<crate::W<RCFASTCFG_SPEC>> for W {
    fn from(writer: crate::W<RCFASTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Oscillator Enable"]
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
#[doc = "Field `EN` writer - Oscillator Enable"]
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
#[doc = "Field `TUNEEN` reader - Tuner Enable"]
pub struct TUNEEN_R(crate::FieldReader<bool, bool>);
impl TUNEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TUNEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TUNEEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TUNEEN` writer - Tuner Enable"]
pub struct TUNEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNEEN_W<'a> {
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
#[doc = "Field `JITMODE` reader - Jitter Mode"]
pub struct JITMODE_R(crate::FieldReader<bool, bool>);
impl JITMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        JITMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JITMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JITMODE` writer - Jitter Mode"]
pub struct JITMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> JITMODE_W<'a> {
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
#[doc = "Field `NBPERIODS` reader - Number of 32kHz Periods"]
pub struct NBPERIODS_R(crate::FieldReader<u8, u8>);
impl NBPERIODS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBPERIODS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBPERIODS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBPERIODS` writer - Number of 32kHz Periods"]
pub struct NBPERIODS_W<'a> {
    w: &'a mut W,
}
impl<'a> NBPERIODS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `FCD` reader - RCFAST Fuse Calibration Done"]
pub struct FCD_R(crate::FieldReader<bool, bool>);
impl FCD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCD` writer - RCFAST Fuse Calibration Done"]
pub struct FCD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `FRANGE` reader - Frequency Range"]
pub struct FRANGE_R(crate::FieldReader<u8, u8>);
impl FRANGE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRANGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRANGE` writer - Frequency Range"]
pub struct FRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRANGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `LOCKMARGIN` reader - Accepted Count Error for Lock"]
pub struct LOCKMARGIN_R(crate::FieldReader<u8, u8>);
impl LOCKMARGIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOCKMARGIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKMARGIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKMARGIN` writer - Accepted Count Error for Lock"]
pub struct LOCKMARGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKMARGIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `CALIB` reader - Oscillator Calibration Value"]
pub struct CALIB_R(crate::FieldReader<u8, u8>);
impl CALIB_R {
    pub(crate) fn new(bits: u8) -> Self {
        CALIB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALIB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALIB` writer - Oscillator Calibration Value"]
pub struct CALIB_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Oscillator Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tuner Enable"]
    #[inline(always)]
    pub fn tuneen(&self) -> TUNEEN_R {
        TUNEEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Jitter Mode"]
    #[inline(always)]
    pub fn jitmode(&self) -> JITMODE_R {
        JITMODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Number of 32kHz Periods"]
    #[inline(always)]
    pub fn nbperiods(&self) -> NBPERIODS_R {
        NBPERIODS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - RCFAST Fuse Calibration Done"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Frequency Range"]
    #[inline(always)]
    pub fn frange(&self) -> FRANGE_R {
        FRANGE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:15 - Accepted Count Error for Lock"]
    #[inline(always)]
    pub fn lockmargin(&self) -> LOCKMARGIN_R {
        LOCKMARGIN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:22 - Oscillator Calibration Value"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Oscillator Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Tuner Enable"]
    #[inline(always)]
    pub fn tuneen(&mut self) -> TUNEEN_W {
        TUNEEN_W { w: self }
    }
    #[doc = "Bit 2 - Jitter Mode"]
    #[inline(always)]
    pub fn jitmode(&mut self) -> JITMODE_W {
        JITMODE_W { w: self }
    }
    #[doc = "Bits 4:6 - Number of 32kHz Periods"]
    #[inline(always)]
    pub fn nbperiods(&mut self) -> NBPERIODS_W {
        NBPERIODS_W { w: self }
    }
    #[doc = "Bit 7 - RCFAST Fuse Calibration Done"]
    #[inline(always)]
    pub fn fcd(&mut self) -> FCD_W {
        FCD_W { w: self }
    }
    #[doc = "Bits 8:9 - Frequency Range"]
    #[inline(always)]
    pub fn frange(&mut self) -> FRANGE_W {
        FRANGE_W { w: self }
    }
    #[doc = "Bits 12:15 - Accepted Count Error for Lock"]
    #[inline(always)]
    pub fn lockmargin(&mut self) -> LOCKMARGIN_W {
        LOCKMARGIN_W { w: self }
    }
    #[doc = "Bits 16:22 - Oscillator Calibration Value"]
    #[inline(always)]
    pub fn calib(&mut self) -> CALIB_W {
        CALIB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "4/8/12 MHz RC Oscillator Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcfastcfg](index.html) module"]
pub struct RCFASTCFG_SPEC;
impl crate::RegisterSpec for RCFASTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcfastcfg::R](R) reader structure"]
impl crate::Readable for RCFASTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcfastcfg::W](W) writer structure"]
impl crate::Writable for RCFASTCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCFASTCFG to value 0"]
impl crate::Resettable for RCFASTCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
