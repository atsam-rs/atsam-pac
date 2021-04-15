#[doc = "Register `RC32KCR` reader"]
pub struct R(crate::R<RC32KCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC32KCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RC32KCR_SPEC>> for R {
    fn from(reader: crate::R<RC32KCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RC32KCR` writer"]
pub struct W(crate::W<RC32KCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC32KCR_SPEC>;
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
impl core::convert::From<crate::W<RC32KCR_SPEC>> for W {
    fn from(writer: crate::W<RC32KCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable as Generic clock source"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Enable as Generic clock source"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Field `TCEN` reader - Temperature Compensation Enable"]
pub struct TCEN_R(crate::FieldReader<bool, bool>);
impl TCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCEN` writer - Temperature Compensation Enable"]
pub struct TCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCEN_W<'a> {
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
#[doc = "Field `EN32K` reader - Enable 32 KHz output"]
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
#[doc = "Field `EN32K` writer - Enable 32 KHz output"]
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
#[doc = "Field `EN1K` reader - Enable 1 kHz output"]
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
#[doc = "Field `EN1K` writer - Enable 1 kHz output"]
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
#[doc = "Field `MODE` reader - Mode Selection"]
pub struct MODE_R(crate::FieldReader<bool, bool>);
impl MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Mode Selection"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
#[doc = "Field `REF` reader - Reference select"]
pub struct REF_R(crate::FieldReader<bool, bool>);
impl REF_R {
    pub(crate) fn new(bits: bool) -> Self {
        REF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REF` writer - Reference select"]
pub struct REF_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_W<'a> {
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
#[doc = "Field `FCD` reader - Flash calibration done"]
pub struct FCD_R(crate::FieldReader<bool, bool>);
impl FCD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCD` writer - Flash calibration done"]
pub struct FCD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCD_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Enable as Generic clock source"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Temperature Compensation Enable"]
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable 32 KHz output"]
    #[inline(always)]
    pub fn en32k(&self) -> EN32K_R {
        EN32K_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable 1 kHz output"]
    #[inline(always)]
    pub fn en1k(&self) -> EN1K_R {
        EN1K_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mode Selection"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Reference select"]
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Flash calibration done"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable as Generic clock source"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Temperature Compensation Enable"]
    #[inline(always)]
    pub fn tcen(&mut self) -> TCEN_W {
        TCEN_W { w: self }
    }
    #[doc = "Bit 2 - Enable 32 KHz output"]
    #[inline(always)]
    pub fn en32k(&mut self) -> EN32K_W {
        EN32K_W { w: self }
    }
    #[doc = "Bit 3 - Enable 1 kHz output"]
    #[inline(always)]
    pub fn en1k(&mut self) -> EN1K_W {
        EN1K_W { w: self }
    }
    #[doc = "Bit 4 - Mode Selection"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 5 - Reference select"]
    #[inline(always)]
    pub fn ref_(&mut self) -> REF_W {
        REF_W { w: self }
    }
    #[doc = "Bit 7 - Flash calibration done"]
    #[inline(always)]
    pub fn fcd(&mut self) -> FCD_W {
        FCD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32 kHz RC Oscillator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc32kcr](index.html) module"]
pub struct RC32KCR_SPEC;
impl crate::RegisterSpec for RC32KCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc32kcr::R](R) reader structure"]
impl crate::Readable for RC32KCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc32kcr::W](W) writer structure"]
impl crate::Writable for RC32KCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RC32KCR to value 0"]
impl crate::Resettable for RC32KCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
