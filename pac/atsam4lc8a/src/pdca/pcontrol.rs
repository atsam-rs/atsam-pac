#[doc = "Register `PCONTROL` reader"]
pub struct R(crate::R<PCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCONTROL` writer"]
pub struct W(crate::W<PCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCONTROL_SPEC>;
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
impl From<crate::W<PCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0EN` reader - Channel 0 Enabled"]
pub struct CH0EN_R(crate::FieldReader<bool, bool>);
impl CH0EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0EN` writer - Channel 0 Enabled"]
pub struct CH0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0EN_W<'a> {
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
#[doc = "Field `CH1EN` reader - Channel 1 Enabled."]
pub struct CH1EN_R(crate::FieldReader<bool, bool>);
impl CH1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1EN` writer - Channel 1 Enabled."]
pub struct CH1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1EN_W<'a> {
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
#[doc = "Field `CH0OF` reader - Channel 0 Overflow Freeze"]
pub struct CH0OF_R(crate::FieldReader<bool, bool>);
impl CH0OF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0OF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0OF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0OF` writer - Channel 0 Overflow Freeze"]
pub struct CH0OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0OF_W<'a> {
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
#[doc = "Field `CH1OF` reader - Channel 1 overflow freeze"]
pub struct CH1OF_R(crate::FieldReader<bool, bool>);
impl CH1OF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1OF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1OF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1OF` writer - Channel 1 overflow freeze"]
pub struct CH1OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CH0RES` reader - Channel 0 counter reset"]
pub struct CH0RES_R(crate::FieldReader<bool, bool>);
impl CH0RES_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0RES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0RES` writer - Channel 0 counter reset"]
pub struct CH0RES_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0RES_W<'a> {
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
#[doc = "Field `CH1RES` reader - Channel 1 counter reset"]
pub struct CH1RES_R(crate::FieldReader<bool, bool>);
impl CH1RES_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1RES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1RES` writer - Channel 1 counter reset"]
pub struct CH1RES_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1RES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `MON0CH` reader - PDCA Channel to monitor with counter 0"]
pub struct MON0CH_R(crate::FieldReader<u8, u8>);
impl MON0CH_R {
    pub(crate) fn new(bits: u8) -> Self {
        MON0CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MON0CH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MON0CH` writer - PDCA Channel to monitor with counter 0"]
pub struct MON0CH_W<'a> {
    w: &'a mut W,
}
impl<'a> MON0CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `MON1CH` reader - PDCA Channel to monitor with counter 1"]
pub struct MON1CH_R(crate::FieldReader<u8, u8>);
impl MON1CH_R {
    pub(crate) fn new(bits: u8) -> Self {
        MON1CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MON1CH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MON1CH` writer - PDCA Channel to monitor with counter 1"]
pub struct MON1CH_W<'a> {
    w: &'a mut W,
}
impl<'a> MON1CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Enabled"]
    #[inline(always)]
    pub fn ch0en(&self) -> CH0EN_R {
        CH0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Enabled."]
    #[inline(always)]
    pub fn ch1en(&self) -> CH1EN_R {
        CH1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 0 Overflow Freeze"]
    #[inline(always)]
    pub fn ch0of(&self) -> CH0OF_R {
        CH0OF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 1 overflow freeze"]
    #[inline(always)]
    pub fn ch1of(&self) -> CH1OF_R {
        CH1OF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 0 counter reset"]
    #[inline(always)]
    pub fn ch0res(&self) -> CH0RES_R {
        CH0RES_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 1 counter reset"]
    #[inline(always)]
    pub fn ch1res(&self) -> CH1RES_R {
        CH1RES_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - PDCA Channel to monitor with counter 0"]
    #[inline(always)]
    pub fn mon0ch(&self) -> MON0CH_R {
        MON0CH_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - PDCA Channel to monitor with counter 1"]
    #[inline(always)]
    pub fn mon1ch(&self) -> MON1CH_R {
        MON1CH_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Enabled"]
    #[inline(always)]
    pub fn ch0en(&mut self) -> CH0EN_W {
        CH0EN_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Enabled."]
    #[inline(always)]
    pub fn ch1en(&mut self) -> CH1EN_W {
        CH1EN_W { w: self }
    }
    #[doc = "Bit 4 - Channel 0 Overflow Freeze"]
    #[inline(always)]
    pub fn ch0of(&mut self) -> CH0OF_W {
        CH0OF_W { w: self }
    }
    #[doc = "Bit 5 - Channel 1 overflow freeze"]
    #[inline(always)]
    pub fn ch1of(&mut self) -> CH1OF_W {
        CH1OF_W { w: self }
    }
    #[doc = "Bit 8 - Channel 0 counter reset"]
    #[inline(always)]
    pub fn ch0res(&mut self) -> CH0RES_W {
        CH0RES_W { w: self }
    }
    #[doc = "Bit 9 - Channel 1 counter reset"]
    #[inline(always)]
    pub fn ch1res(&mut self) -> CH1RES_W {
        CH1RES_W { w: self }
    }
    #[doc = "Bits 16:21 - PDCA Channel to monitor with counter 0"]
    #[inline(always)]
    pub fn mon0ch(&mut self) -> MON0CH_W {
        MON0CH_W { w: self }
    }
    #[doc = "Bits 24:29 - PDCA Channel to monitor with counter 1"]
    #[inline(always)]
    pub fn mon1ch(&mut self) -> MON1CH_W {
        MON1CH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Performance Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcontrol](index.html) module"]
pub struct PCONTROL_SPEC;
impl crate::RegisterSpec for PCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcontrol::R](R) reader structure"]
impl crate::Readable for PCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcontrol::W](W) writer structure"]
impl crate::Writable for PCONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCONTROL to value 0"]
impl crate::Resettable for PCONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
