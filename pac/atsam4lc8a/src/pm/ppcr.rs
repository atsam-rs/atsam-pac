#[doc = "Register `PPCR` reader"]
pub struct R(crate::R<PPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPCR` writer"]
pub struct W(crate::W<PPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPCR_SPEC>;
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
impl From<crate::W<PPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTPUN` reader - Reset Pullup"]
pub struct RSTPUN_R(crate::FieldReader<bool, bool>);
impl RSTPUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTPUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTPUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTPUN` writer - Reset Pullup"]
pub struct RSTPUN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTPUN_W<'a> {
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
#[doc = "Field `CATBRCMASK` reader - CAT Request Clock Mask"]
pub struct CATBRCMASK_R(crate::FieldReader<bool, bool>);
impl CATBRCMASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CATBRCMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CATBRCMASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CATBRCMASK` writer - CAT Request Clock Mask"]
pub struct CATBRCMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CATBRCMASK_W<'a> {
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
#[doc = "Field `ACIFCRCMASK` reader - ACIFC Request Clock Mask"]
pub struct ACIFCRCMASK_R(crate::FieldReader<bool, bool>);
impl ACIFCRCMASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACIFCRCMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACIFCRCMASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACIFCRCMASK` writer - ACIFC Request Clock Mask"]
pub struct ACIFCRCMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACIFCRCMASK_W<'a> {
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
#[doc = "Field `ASTRCMASK` reader - AST Request Clock Mask"]
pub struct ASTRCMASK_R(crate::FieldReader<bool, bool>);
impl ASTRCMASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASTRCMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASTRCMASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASTRCMASK` writer - AST Request Clock Mask"]
pub struct ASTRCMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ASTRCMASK_W<'a> {
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
#[doc = "Field `TWIS0RCMASK` reader - TWIS0 Request Clock Mask"]
pub struct TWIS0RCMASK_R(crate::FieldReader<bool, bool>);
impl TWIS0RCMASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TWIS0RCMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWIS0RCMASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWIS0RCMASK` writer - TWIS0 Request Clock Mask"]
pub struct TWIS0RCMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TWIS0RCMASK_W<'a> {
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
#[doc = "Field `TWIS1RCMASK` reader - TWIS1 Request Clock Mask"]
pub struct TWIS1RCMASK_R(crate::FieldReader<bool, bool>);
impl TWIS1RCMASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TWIS1RCMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWIS1RCMASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWIS1RCMASK` writer - TWIS1 Request Clock Mask"]
pub struct TWIS1RCMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TWIS1RCMASK_W<'a> {
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
#[doc = "Field `PEVCRCMASK` reader - PEVC Request Clock Mask"]
pub struct PEVCRCMASK_R(crate::FieldReader<bool, bool>);
impl PEVCRCMASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEVCRCMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEVCRCMASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEVCRCMASK` writer - PEVC Request Clock Mask"]
pub struct PEVCRCMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PEVCRCMASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `ADCIFERCMASK` reader - ADCIFE Request Clock Mask"]
pub struct ADCIFERCMASK_R(crate::FieldReader<bool, bool>);
impl ADCIFERCMASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCIFERCMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCIFERCMASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCIFERCMASK` writer - ADCIFE Request Clock Mask"]
pub struct ADCIFERCMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCIFERCMASK_W<'a> {
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
#[doc = "Field `VREGRCMASK` reader - VREG Request Clock Mask"]
pub struct VREGRCMASK_R(crate::FieldReader<bool, bool>);
impl VREGRCMASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREGRCMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREGRCMASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREGRCMASK` writer - VREG Request Clock Mask"]
pub struct VREGRCMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> VREGRCMASK_W<'a> {
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
#[doc = "Field `FWBGREF` reader - Flash Wait BGREF"]
pub struct FWBGREF_R(crate::FieldReader<bool, bool>);
impl FWBGREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FWBGREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWBGREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWBGREF` writer - Flash Wait BGREF"]
pub struct FWBGREF_W<'a> {
    w: &'a mut W,
}
impl<'a> FWBGREF_W<'a> {
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
#[doc = "Field `FWBOD18` reader - Flash Wait BOD18"]
pub struct FWBOD18_R(crate::FieldReader<bool, bool>);
impl FWBOD18_R {
    pub(crate) fn new(bits: bool) -> Self {
        FWBOD18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWBOD18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWBOD18` writer - Flash Wait BOD18"]
pub struct FWBOD18_W<'a> {
    w: &'a mut W,
}
impl<'a> FWBOD18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Reset Pullup"]
    #[inline(always)]
    pub fn rstpun(&self) -> RSTPUN_R {
        RSTPUN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CAT Request Clock Mask"]
    #[inline(always)]
    pub fn catbrcmask(&self) -> CATBRCMASK_R {
        CATBRCMASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ACIFC Request Clock Mask"]
    #[inline(always)]
    pub fn acifcrcmask(&self) -> ACIFCRCMASK_R {
        ACIFCRCMASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AST Request Clock Mask"]
    #[inline(always)]
    pub fn astrcmask(&self) -> ASTRCMASK_R {
        ASTRCMASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TWIS0 Request Clock Mask"]
    #[inline(always)]
    pub fn twis0rcmask(&self) -> TWIS0RCMASK_R {
        TWIS0RCMASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TWIS1 Request Clock Mask"]
    #[inline(always)]
    pub fn twis1rcmask(&self) -> TWIS1RCMASK_R {
        TWIS1RCMASK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PEVC Request Clock Mask"]
    #[inline(always)]
    pub fn pevcrcmask(&self) -> PEVCRCMASK_R {
        PEVCRCMASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADCIFE Request Clock Mask"]
    #[inline(always)]
    pub fn adcifercmask(&self) -> ADCIFERCMASK_R {
        ADCIFERCMASK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - VREG Request Clock Mask"]
    #[inline(always)]
    pub fn vregrcmask(&self) -> VREGRCMASK_R {
        VREGRCMASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Flash Wait BGREF"]
    #[inline(always)]
    pub fn fwbgref(&self) -> FWBGREF_R {
        FWBGREF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Flash Wait BOD18"]
    #[inline(always)]
    pub fn fwbod18(&self) -> FWBOD18_R {
        FWBOD18_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset Pullup"]
    #[inline(always)]
    pub fn rstpun(&mut self) -> RSTPUN_W {
        RSTPUN_W { w: self }
    }
    #[doc = "Bit 1 - CAT Request Clock Mask"]
    #[inline(always)]
    pub fn catbrcmask(&mut self) -> CATBRCMASK_W {
        CATBRCMASK_W { w: self }
    }
    #[doc = "Bit 2 - ACIFC Request Clock Mask"]
    #[inline(always)]
    pub fn acifcrcmask(&mut self) -> ACIFCRCMASK_W {
        ACIFCRCMASK_W { w: self }
    }
    #[doc = "Bit 3 - AST Request Clock Mask"]
    #[inline(always)]
    pub fn astrcmask(&mut self) -> ASTRCMASK_W {
        ASTRCMASK_W { w: self }
    }
    #[doc = "Bit 4 - TWIS0 Request Clock Mask"]
    #[inline(always)]
    pub fn twis0rcmask(&mut self) -> TWIS0RCMASK_W {
        TWIS0RCMASK_W { w: self }
    }
    #[doc = "Bit 5 - TWIS1 Request Clock Mask"]
    #[inline(always)]
    pub fn twis1rcmask(&mut self) -> TWIS1RCMASK_W {
        TWIS1RCMASK_W { w: self }
    }
    #[doc = "Bit 6 - PEVC Request Clock Mask"]
    #[inline(always)]
    pub fn pevcrcmask(&mut self) -> PEVCRCMASK_W {
        PEVCRCMASK_W { w: self }
    }
    #[doc = "Bit 7 - ADCIFE Request Clock Mask"]
    #[inline(always)]
    pub fn adcifercmask(&mut self) -> ADCIFERCMASK_W {
        ADCIFERCMASK_W { w: self }
    }
    #[doc = "Bit 8 - VREG Request Clock Mask"]
    #[inline(always)]
    pub fn vregrcmask(&mut self) -> VREGRCMASK_W {
        VREGRCMASK_W { w: self }
    }
    #[doc = "Bit 9 - Flash Wait BGREF"]
    #[inline(always)]
    pub fn fwbgref(&mut self) -> FWBGREF_W {
        FWBGREF_W { w: self }
    }
    #[doc = "Bit 10 - Flash Wait BOD18"]
    #[inline(always)]
    pub fn fwbod18(&mut self) -> FWBOD18_W {
        FWBOD18_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppcr](index.html) module"]
pub struct PPCR_SPEC;
impl crate::RegisterSpec for PPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppcr::R](R) reader structure"]
impl crate::Readable for PPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppcr::W](W) writer structure"]
impl crate::Writable for PPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPCR to value 0x01fe"]
impl crate::Resettable for PPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01fe
    }
}
