#[doc = "Register `OSCCTRL32` reader"]
pub struct R(crate::R<OSCCTRL32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCCTRL32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OSCCTRL32_SPEC>> for R {
    fn from(reader: crate::R<OSCCTRL32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCCTRL32` writer"]
pub struct W(crate::W<OSCCTRL32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCCTRL32_SPEC>;
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
impl core::convert::From<crate::W<OSCCTRL32_SPEC>> for W {
    fn from(writer: crate::W<OSCCTRL32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSC32EN` reader - 32 KHz Oscillator Enable"]
pub struct OSC32EN_R(crate::FieldReader<bool, bool>);
impl OSC32EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSC32EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC32EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC32EN` writer - 32 KHz Oscillator Enable"]
pub struct OSC32EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC32EN_W<'a> {
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
#[doc = "Field `PINSEL` reader - Pins Select"]
pub struct PINSEL_R(crate::FieldReader<bool, bool>);
impl PINSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PINSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINSEL` writer - Pins Select"]
pub struct PINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSEL_W<'a> {
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
#[doc = "Field `EN32K` reader - 32 KHz output Enable"]
pub struct EN32K_R(crate::FieldReader<bool, bool>);
impl EN32K_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN32K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN32K` writer - 32 KHz output Enable"]
pub struct EN32K_W<'a> {
    w: &'a mut W,
}
impl<'a> EN32K_W<'a> {
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
#[doc = "Field `EN1K` reader - 1 KHz output Enable"]
pub struct EN1K_R(crate::FieldReader<bool, bool>);
impl EN1K_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN1K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN1K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN1K` writer - 1 KHz output Enable"]
pub struct EN1K_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1K_W<'a> {
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
#[doc = "Field `MODE` reader - Oscillator Mode"]
pub struct MODE_R(crate::FieldReader<u8, u8>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Oscillator Mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `SELCURR` reader - Current selection"]
pub struct SELCURR_R(crate::FieldReader<u8, u8>);
impl SELCURR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SELCURR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELCURR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELCURR` writer - Current selection"]
pub struct SELCURR_W<'a> {
    w: &'a mut W,
}
impl<'a> SELCURR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `STARTUP` reader - Oscillator Start-up Time"]
pub struct STARTUP_R(crate::FieldReader<u8, u8>);
impl STARTUP_R {
    pub(crate) fn new(bits: u8) -> Self {
        STARTUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STARTUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTUP` writer - Oscillator Start-up Time"]
pub struct STARTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 32 KHz Oscillator Enable"]
    #[inline(always)]
    pub fn osc32en(&self) -> OSC32EN_R {
        OSC32EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pins Select"]
    #[inline(always)]
    pub fn pinsel(&self) -> PINSEL_R {
        PINSEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 32 KHz output Enable"]
    #[inline(always)]
    pub fn en32k(&self) -> EN32K_R {
        EN32K_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 KHz output Enable"]
    #[inline(always)]
    pub fn en1k(&self) -> EN1K_R {
        EN1K_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:15 - Current selection"]
    #[inline(always)]
    pub fn selcurr(&self) -> SELCURR_R {
        SELCURR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Oscillator Start-up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 32 KHz Oscillator Enable"]
    #[inline(always)]
    pub fn osc32en(&mut self) -> OSC32EN_W {
        OSC32EN_W { w: self }
    }
    #[doc = "Bit 1 - Pins Select"]
    #[inline(always)]
    pub fn pinsel(&mut self) -> PINSEL_W {
        PINSEL_W { w: self }
    }
    #[doc = "Bit 2 - 32 KHz output Enable"]
    #[inline(always)]
    pub fn en32k(&mut self) -> EN32K_W {
        EN32K_W { w: self }
    }
    #[doc = "Bit 3 - 1 KHz output Enable"]
    #[inline(always)]
    pub fn en1k(&mut self) -> EN1K_W {
        EN1K_W { w: self }
    }
    #[doc = "Bits 8:10 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 12:15 - Current selection"]
    #[inline(always)]
    pub fn selcurr(&mut self) -> SELCURR_W {
        SELCURR_W { w: self }
    }
    #[doc = "Bits 16:18 - Oscillator Start-up Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W {
        STARTUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator 32 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscctrl32](index.html) module"]
pub struct OSCCTRL32_SPEC;
impl crate::RegisterSpec for OSCCTRL32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oscctrl32::R](R) reader structure"]
impl crate::Readable for OSCCTRL32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oscctrl32::W](W) writer structure"]
impl crate::Writable for OSCCTRL32_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSCCTRL32 to value 0x04"]
impl crate::Resettable for OSCCTRL32_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
