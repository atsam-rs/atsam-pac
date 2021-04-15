#[doc = "Register `SMBTR` reader"]
pub struct R(crate::R<SMBTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMBTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SMBTR_SPEC>> for R {
    fn from(reader: crate::R<SMBTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMBTR` writer"]
pub struct W(crate::W<SMBTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMBTR_SPEC>;
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
impl core::convert::From<crate::W<SMBTR_SPEC>> for W {
    fn from(writer: crate::W<SMBTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLOWS` reader - Slave Clock stretch maximum cycles"]
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
#[doc = "Field `TLOWS` writer - Slave Clock stretch maximum cycles"]
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
#[doc = "Field `TLOWM` reader - Master Clock stretch maximum cycles"]
pub struct TLOWM_R(crate::FieldReader<u8, u8>);
impl TLOWM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TLOWM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLOWM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLOWM` writer - Master Clock stretch maximum cycles"]
pub struct TLOWM_W<'a> {
    w: &'a mut W,
}
impl<'a> TLOWM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `THMAX` reader - Clock High maximum cycles"]
pub struct THMAX_R(crate::FieldReader<u8, u8>);
impl THMAX_R {
    pub(crate) fn new(bits: u8) -> Self {
        THMAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THMAX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THMAX` writer - Clock High maximum cycles"]
pub struct THMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> THMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `EXP` reader - SMBus Timeout Clock prescaler"]
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
#[doc = "Field `EXP` writer - SMBus Timeout Clock prescaler"]
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
    #[doc = "Bits 0:7 - Slave Clock stretch maximum cycles"]
    #[inline(always)]
    pub fn tlows(&self) -> TLOWS_R {
        TLOWS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Master Clock stretch maximum cycles"]
    #[inline(always)]
    pub fn tlowm(&self) -> TLOWM_R {
        TLOWM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Clock High maximum cycles"]
    #[inline(always)]
    pub fn thmax(&self) -> THMAX_R {
        THMAX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - SMBus Timeout Clock prescaler"]
    #[inline(always)]
    pub fn exp(&self) -> EXP_R {
        EXP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave Clock stretch maximum cycles"]
    #[inline(always)]
    pub fn tlows(&mut self) -> TLOWS_W {
        TLOWS_W { w: self }
    }
    #[doc = "Bits 8:15 - Master Clock stretch maximum cycles"]
    #[inline(always)]
    pub fn tlowm(&mut self) -> TLOWM_W {
        TLOWM_W { w: self }
    }
    #[doc = "Bits 16:23 - Clock High maximum cycles"]
    #[inline(always)]
    pub fn thmax(&mut self) -> THMAX_W {
        THMAX_W { w: self }
    }
    #[doc = "Bits 28:31 - SMBus Timeout Clock prescaler"]
    #[inline(always)]
    pub fn exp(&mut self) -> EXP_W {
        EXP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMBus Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smbtr](index.html) module"]
pub struct SMBTR_SPEC;
impl crate::RegisterSpec for SMBTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smbtr::R](R) reader structure"]
impl crate::Readable for SMBTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smbtr::W](W) writer structure"]
impl crate::Writable for SMBTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMBTR to value 0"]
impl crate::Resettable for SMBTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
