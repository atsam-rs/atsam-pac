#[doc = "Register `FASTSLEEP` reader"]
pub struct R(crate::R<FASTSLEEP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FASTSLEEP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FASTSLEEP_SPEC>> for R {
    fn from(reader: crate::R<FASTSLEEP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FASTSLEEP` writer"]
pub struct W(crate::W<FASTSLEEP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FASTSLEEP_SPEC>;
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
impl core::convert::From<crate::W<FASTSLEEP_SPEC>> for W {
    fn from(writer: crate::W<FASTSLEEP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSC` reader - Oscillator"]
pub struct OSC_R(crate::FieldReader<bool, bool>);
impl OSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC` writer - Oscillator"]
pub struct OSC_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_W<'a> {
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
#[doc = "Field `PLL` reader - PLL"]
pub struct PLL_R(crate::FieldReader<bool, bool>);
impl PLL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL` writer - PLL"]
pub struct PLL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `FASTRCOSC` reader - RC80 or FLO"]
pub struct FASTRCOSC_R(crate::FieldReader<u8, u8>);
impl FASTRCOSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        FASTRCOSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FASTRCOSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FASTRCOSC` writer - RC80 or FLO"]
pub struct FASTRCOSC_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTRCOSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `DFLL` reader - DFLL"]
pub struct DFLL_R(crate::FieldReader<bool, bool>);
impl DFLL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLL` writer - DFLL"]
pub struct DFLL_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Oscillator"]
    #[inline(always)]
    pub fn osc(&self) -> OSC_R {
        OSC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLL"]
    #[inline(always)]
    pub fn pll(&self) -> PLL_R {
        PLL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - RC80 or FLO"]
    #[inline(always)]
    pub fn fastrcosc(&self) -> FASTRCOSC_R {
        FASTRCOSC_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - DFLL"]
    #[inline(always)]
    pub fn dfll(&self) -> DFLL_R {
        DFLL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Oscillator"]
    #[inline(always)]
    pub fn osc(&mut self) -> OSC_W {
        OSC_W { w: self }
    }
    #[doc = "Bit 8 - PLL"]
    #[inline(always)]
    pub fn pll(&mut self) -> PLL_W {
        PLL_W { w: self }
    }
    #[doc = "Bits 16:20 - RC80 or FLO"]
    #[inline(always)]
    pub fn fastrcosc(&mut self) -> FASTRCOSC_W {
        FASTRCOSC_W { w: self }
    }
    #[doc = "Bit 24 - DFLL"]
    #[inline(always)]
    pub fn dfll(&mut self) -> DFLL_W {
        DFLL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast Sleep Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fastsleep](index.html) module"]
pub struct FASTSLEEP_SPEC;
impl crate::RegisterSpec for FASTSLEEP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fastsleep::R](R) reader structure"]
impl crate::Readable for FASTSLEEP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fastsleep::W](W) writer structure"]
impl crate::Writable for FASTSLEEP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FASTSLEEP to value 0"]
impl crate::Resettable for FASTSLEEP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
