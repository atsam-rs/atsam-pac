#[doc = "Register `PLL` reader"]
pub struct R(crate::R<PLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL` writer"]
pub struct W(crate::W<PLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_SPEC>;
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
impl From<crate::W<PLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLEN` reader - PLL Enable"]
pub struct PLLEN_R(crate::FieldReader<bool, bool>);
impl PLLEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLEN` writer - PLL Enable"]
pub struct PLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLEN_W<'a> {
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
#[doc = "Field `PLLOSC` reader - PLL Oscillator Select"]
pub struct PLLOSC_R(crate::FieldReader<u8, u8>);
impl PLLOSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLOSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLOSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLOSC` writer - PLL Oscillator Select"]
pub struct PLLOSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLOSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `PLLOPT` reader - PLL Option"]
pub struct PLLOPT_R(crate::FieldReader<u8, u8>);
impl PLLOPT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLOPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLOPT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLOPT` writer - PLL Option"]
pub struct PLLOPT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLOPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `PLLDIV` reader - PLL Division Factor"]
pub struct PLLDIV_R(crate::FieldReader<u8, u8>);
impl PLLDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLDIV` writer - PLL Division Factor"]
pub struct PLLDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PLLMUL` reader - PLL Multiply Factor"]
pub struct PLLMUL_R(crate::FieldReader<u8, u8>);
impl PLLMUL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLMUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLMUL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLMUL` writer - PLL Multiply Factor"]
pub struct PLLMUL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLMUL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PLLCOUNT` reader - PLL Count"]
pub struct PLLCOUNT_R(crate::FieldReader<u8, u8>);
impl PLLCOUNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLCOUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLCOUNT` writer - PLL Count"]
pub struct PLLCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PLL Enable"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - PLL Oscillator Select"]
    #[inline(always)]
    pub fn pllosc(&self) -> PLLOSC_R {
        PLLOSC_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - PLL Option"]
    #[inline(always)]
    pub fn pllopt(&self) -> PLLOPT_R {
        PLLOPT_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - PLL Division Factor"]
    #[inline(always)]
    pub fn plldiv(&self) -> PLLDIV_R {
        PLLDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PLL Multiply Factor"]
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - PLL Count"]
    #[inline(always)]
    pub fn pllcount(&self) -> PLLCOUNT_R {
        PLLCOUNT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Enable"]
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W {
        PLLEN_W { w: self }
    }
    #[doc = "Bits 1:2 - PLL Oscillator Select"]
    #[inline(always)]
    pub fn pllosc(&mut self) -> PLLOSC_W {
        PLLOSC_W { w: self }
    }
    #[doc = "Bits 3:5 - PLL Option"]
    #[inline(always)]
    pub fn pllopt(&mut self) -> PLLOPT_W {
        PLLOPT_W { w: self }
    }
    #[doc = "Bits 8:11 - PLL Division Factor"]
    #[inline(always)]
    pub fn plldiv(&mut self) -> PLLDIV_W {
        PLLDIV_W { w: self }
    }
    #[doc = "Bits 16:19 - PLL Multiply Factor"]
    #[inline(always)]
    pub fn pllmul(&mut self) -> PLLMUL_W {
        PLLMUL_W { w: self }
    }
    #[doc = "Bits 24:29 - PLL Count"]
    #[inline(always)]
    pub fn pllcount(&mut self) -> PLLCOUNT_W {
        PLLCOUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll](index.html) module"]
pub struct PLL_SPEC;
impl crate::RegisterSpec for PLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll::R](R) reader structure"]
impl crate::Readable for PLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll::W](W) writer structure"]
impl crate::Writable for PLL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL to value 0"]
impl crate::Resettable for PLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
