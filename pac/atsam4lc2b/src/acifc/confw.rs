#[doc = "Register `CONFW%s` reader"]
pub struct R(crate::R<CONFW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFW%s` writer"]
pub struct W(crate::W<CONFW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFW_SPEC>;
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
impl From<crate::W<CONFW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIS` reader - Window Mode Interrupt Settings"]
pub struct WIS_R(crate::FieldReader<u8, u8>);
impl WIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        WIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIS` writer - Window Mode Interrupt Settings"]
pub struct WIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `WEVSRC` reader - Peripheral Event Sourse Selection for Window Mode"]
pub struct WEVSRC_R(crate::FieldReader<u8, u8>);
impl WEVSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        WEVSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WEVSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WEVSRC` writer - Peripheral Event Sourse Selection for Window Mode"]
pub struct WEVSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> WEVSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `WEVEN` reader - Window Peripheral Event Enable"]
pub struct WEVEN_R(crate::FieldReader<bool, bool>);
impl WEVEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WEVEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WEVEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WEVEN` writer - Window Peripheral Event Enable"]
pub struct WEVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WEVEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `WFEN` reader - Window Mode Enable"]
pub struct WFEN_R(crate::FieldReader<bool, bool>);
impl WFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WFEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WFEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WFEN` writer - Window Mode Enable"]
pub struct WFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WFEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Window Mode Interrupt Settings"]
    #[inline(always)]
    pub fn wis(&self) -> WIS_R {
        WIS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Peripheral Event Sourse Selection for Window Mode"]
    #[inline(always)]
    pub fn wevsrc(&self) -> WEVSRC_R {
        WEVSRC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Window Peripheral Event Enable"]
    #[inline(always)]
    pub fn weven(&self) -> WEVEN_R {
        WEVEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Window Mode Enable"]
    #[inline(always)]
    pub fn wfen(&self) -> WFEN_R {
        WFEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Window Mode Interrupt Settings"]
    #[inline(always)]
    pub fn wis(&mut self) -> WIS_W {
        WIS_W { w: self }
    }
    #[doc = "Bits 8:10 - Peripheral Event Sourse Selection for Window Mode"]
    #[inline(always)]
    pub fn wevsrc(&mut self) -> WEVSRC_W {
        WEVSRC_W { w: self }
    }
    #[doc = "Bit 11 - Window Peripheral Event Enable"]
    #[inline(always)]
    pub fn weven(&mut self) -> WEVEN_W {
        WEVEN_W { w: self }
    }
    #[doc = "Bit 16 - Window Mode Enable"]
    #[inline(always)]
    pub fn wfen(&mut self) -> WFEN_W {
        WFEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confw](index.html) module"]
pub struct CONFW_SPEC;
impl crate::RegisterSpec for CONFW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [confw::R](R) reader structure"]
impl crate::Readable for CONFW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [confw::W](W) writer structure"]
impl crate::Writable for CONFW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFW%s to value 0"]
impl crate::Resettable for CONFW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
