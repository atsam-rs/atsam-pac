#[doc = "Register `TR` reader"]
pub struct R(crate::R<TR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR` writer"]
pub struct W(crate::W<TR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_SPEC>;
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
impl From<crate::W<TR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLOWS` reader - SMBus Tlow:sext Cycles"]
pub struct TLOWS_R(crate::FieldReader<u8, u8>);
impl TLOWS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TLOWS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLOWS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLOWS` writer - SMBus Tlow:sext Cycles"]
pub struct TLOWS_W<'a> {
    w: &'a mut W,
}
impl<'a> TLOWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `TTOUT` reader - SMBus Ttimeout Cycles"]
pub struct TTOUT_R(crate::FieldReader<u8, u8>);
impl TTOUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TTOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTOUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTOUT` writer - SMBus Ttimeout Cycles"]
pub struct TTOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TTOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SUDAT` reader - Data Setup Cycles"]
pub struct SUDAT_R(crate::FieldReader<u8, u8>);
impl SUDAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SUDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUDAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUDAT` writer - Data Setup Cycles"]
pub struct SUDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SUDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `EXP` reader - Clock Prescaler"]
pub struct EXP_R(crate::FieldReader<u8, u8>);
impl EXP_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXP` writer - Clock Prescaler"]
pub struct EXP_W<'a> {
    w: &'a mut W,
}
impl<'a> EXP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SMBus Tlow:sext Cycles"]
    #[inline(always)]
    pub fn tlows(&self) -> TLOWS_R {
        TLOWS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SMBus Ttimeout Cycles"]
    #[inline(always)]
    pub fn ttout(&self) -> TTOUT_R {
        TTOUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data Setup Cycles"]
    #[inline(always)]
    pub fn sudat(&self) -> SUDAT_R {
        SUDAT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - Clock Prescaler"]
    #[inline(always)]
    pub fn exp(&self) -> EXP_R {
        EXP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SMBus Tlow:sext Cycles"]
    #[inline(always)]
    pub fn tlows(&mut self) -> TLOWS_W {
        TLOWS_W { w: self }
    }
    #[doc = "Bits 8:15 - SMBus Ttimeout Cycles"]
    #[inline(always)]
    pub fn ttout(&mut self) -> TTOUT_W {
        TTOUT_W { w: self }
    }
    #[doc = "Bits 16:23 - Data Setup Cycles"]
    #[inline(always)]
    pub fn sudat(&mut self) -> SUDAT_W {
        SUDAT_W { w: self }
    }
    #[doc = "Bits 28:31 - Clock Prescaler"]
    #[inline(always)]
    pub fn exp(&mut self) -> EXP_W {
        EXP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr](index.html) module"]
pub struct TR_SPEC;
impl crate::RegisterSpec for TR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr::R](R) reader structure"]
impl crate::Readable for TR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr::W](W) writer structure"]
impl crate::Writable for TR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TR to value 0"]
impl crate::Resettable for TR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
