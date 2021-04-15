#[doc = "Register `CMCFG` reader"]
pub struct R(crate::R<CMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CMCFG_SPEC>> for R {
    fn from(reader: crate::R<CMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMCFG` writer"]
pub struct W(crate::W<CMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMCFG_SPEC>;
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
impl core::convert::From<crate::W<CMCFG_SPEC>> for W {
    fn from(writer: crate::W<CMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DREV` reader - Digit Reverse Mode"]
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
#[doc = "Field `DREV` writer - Digit Reverse Mode"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
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
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
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
impl R {
    #[doc = "Bit 0 - Digit Reverse Mode"]
    #[inline(always)]
    pub fn drev(&self) -> DREV_R {
        DREV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Type of Digit"]
    #[inline(always)]
    pub fn tdg(&self) -> TDG_R {
        TDG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 8:13 - Start Segment"]
    #[inline(always)]
    pub fn stseg(&self) -> STSEG_R {
        STSEG_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Digit Reverse Mode"]
    #[inline(always)]
    pub fn drev(&mut self) -> DREV_W {
        DREV_W { w: self }
    }
    #[doc = "Bits 1:2 - Type of Digit"]
    #[inline(always)]
    pub fn tdg(&mut self) -> TDG_W {
        TDG_W { w: self }
    }
    #[doc = "Bits 8:13 - Start Segment"]
    #[inline(always)]
    pub fn stseg(&mut self) -> STSEG_W {
        STSEG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Character Mapping Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmcfg](index.html) module"]
pub struct CMCFG_SPEC;
impl crate::RegisterSpec for CMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmcfg::R](R) reader structure"]
impl crate::Readable for CMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmcfg::W](W) writer structure"]
impl crate::Writable for CMCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMCFG to value 0"]
impl crate::Resettable for CMCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
