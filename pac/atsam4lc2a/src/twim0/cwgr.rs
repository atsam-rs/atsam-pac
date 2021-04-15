#[doc = "Register `CWGR` reader"]
pub struct R(crate::R<CWGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CWGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CWGR_SPEC>> for R {
    fn from(reader: crate::R<CWGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CWGR` writer"]
pub struct W(crate::W<CWGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CWGR_SPEC>;
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
impl core::convert::From<crate::W<CWGR_SPEC>> for W {
    fn from(writer: crate::W<CWGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOW` reader - Clock Low Cycles"]
pub struct LOW_R(crate::FieldReader<u8, u8>);
impl LOW_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOW` writer - Clock Low Cycles"]
pub struct LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `HIGH` reader - Clock High Cycles"]
pub struct HIGH_R(crate::FieldReader<u8, u8>);
impl HIGH_R {
    pub(crate) fn new(bits: u8) -> Self {
        HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIGH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIGH` writer - Clock High Cycles"]
pub struct HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `STASTO` reader - START and STOP Cycles"]
pub struct STASTO_R(crate::FieldReader<u8, u8>);
impl STASTO_R {
    pub(crate) fn new(bits: u8) -> Self {
        STASTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STASTO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STASTO` writer - START and STOP Cycles"]
pub struct STASTO_W<'a> {
    w: &'a mut W,
}
impl<'a> STASTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `DATA` reader - Data Setup and Hold Cycles"]
pub struct DATA_R(crate::FieldReader<u8, u8>);
impl DATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - Data Setup and Hold Cycles"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
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
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock Low Cycles"]
    #[inline(always)]
    pub fn low(&self) -> LOW_R {
        LOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock High Cycles"]
    #[inline(always)]
    pub fn high(&self) -> HIGH_R {
        HIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - START and STOP Cycles"]
    #[inline(always)]
    pub fn stasto(&self) -> STASTO_R {
        STASTO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Data Setup and Hold Cycles"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Clock Prescaler"]
    #[inline(always)]
    pub fn exp(&self) -> EXP_R {
        EXP_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Low Cycles"]
    #[inline(always)]
    pub fn low(&mut self) -> LOW_W {
        LOW_W { w: self }
    }
    #[doc = "Bits 8:15 - Clock High Cycles"]
    #[inline(always)]
    pub fn high(&mut self) -> HIGH_W {
        HIGH_W { w: self }
    }
    #[doc = "Bits 16:23 - START and STOP Cycles"]
    #[inline(always)]
    pub fn stasto(&mut self) -> STASTO_W {
        STASTO_W { w: self }
    }
    #[doc = "Bits 24:27 - Data Setup and Hold Cycles"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Bits 28:30 - Clock Prescaler"]
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
#[doc = "Clock Waveform Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwgr](index.html) module"]
pub struct CWGR_SPEC;
impl crate::RegisterSpec for CWGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cwgr::R](R) reader structure"]
impl crate::Readable for CWGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cwgr::W](W) writer structure"]
impl crate::Writable for CWGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CWGR to value 0"]
impl crate::Resettable for CWGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
