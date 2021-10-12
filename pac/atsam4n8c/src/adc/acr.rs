#[doc = "Register `ACR` reader"]
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR` writer"]
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Internal Reference Voltage Change Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRVCE_A {
    #[doc = "0: The internal reference voltage is stuck at the default value (see the product electrical charac-teristics for further details)."]
    STUCK_AT_DEFAULT = 0,
    #[doc = "1: The internal reference voltage is defined by field IRVS."]
    SELECTION = 1,
}
impl From<IRVCE_A> for bool {
    #[inline(always)]
    fn from(variant: IRVCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRVCE` reader - Internal Reference Voltage Change Enable"]
pub struct IRVCE_R(crate::FieldReader<bool, IRVCE_A>);
impl IRVCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRVCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRVCE_A {
        match self.bits {
            false => IRVCE_A::STUCK_AT_DEFAULT,
            true => IRVCE_A::SELECTION,
        }
    }
    #[doc = "Checks if the value of the field is `STUCK_AT_DEFAULT`"]
    #[inline(always)]
    pub fn is_stuck_at_default(&self) -> bool {
        **self == IRVCE_A::STUCK_AT_DEFAULT
    }
    #[doc = "Checks if the value of the field is `SELECTION`"]
    #[inline(always)]
    pub fn is_selection(&self) -> bool {
        **self == IRVCE_A::SELECTION
    }
}
impl core::ops::Deref for IRVCE_R {
    type Target = crate::FieldReader<bool, IRVCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRVCE` writer - Internal Reference Voltage Change Enable"]
pub struct IRVCE_W<'a> {
    w: &'a mut W,
}
impl<'a> IRVCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRVCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The internal reference voltage is stuck at the default value (see the product electrical charac-teristics for further details)."]
    #[inline(always)]
    pub fn stuck_at_default(self) -> &'a mut W {
        self.variant(IRVCE_A::STUCK_AT_DEFAULT)
    }
    #[doc = "The internal reference voltage is defined by field IRVS."]
    #[inline(always)]
    pub fn selection(self) -> &'a mut W {
        self.variant(IRVCE_A::SELECTION)
    }
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
#[doc = "Field `IRVS` reader - Internal Reference Voltage Selection"]
pub struct IRVS_R(crate::FieldReader<u8, u8>);
impl IRVS_R {
    pub(crate) fn new(bits: u8) -> Self {
        IRVS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRVS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRVS` writer - Internal Reference Voltage Selection"]
pub struct IRVS_W<'a> {
    w: &'a mut W,
}
impl<'a> IRVS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | ((value as u32 & 0x0f) << 3);
        self.w
    }
}
#[doc = "Field `FORCEREF` reader - Force Internal Reference Voltage"]
pub struct FORCEREF_R(crate::FieldReader<bool, bool>);
impl FORCEREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCEREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCEREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCEREF` writer - Force Internal Reference Voltage"]
pub struct FORCEREF_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEREF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `ONREF` reader - Internal Voltage Reference ON"]
pub struct ONREF_R(crate::FieldReader<bool, bool>);
impl ONREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONREF` writer - Internal Voltage Reference ON"]
pub struct ONREF_W<'a> {
    w: &'a mut W,
}
impl<'a> ONREF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Internal Reference Voltage Change Enable"]
    #[inline(always)]
    pub fn irvce(&self) -> IRVCE_R {
        IRVCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - Internal Reference Voltage Selection"]
    #[inline(always)]
    pub fn irvs(&self) -> IRVS_R {
        IRVS_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - Force Internal Reference Voltage"]
    #[inline(always)]
    pub fn forceref(&self) -> FORCEREF_R {
        FORCEREF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Internal Voltage Reference ON"]
    #[inline(always)]
    pub fn onref(&self) -> ONREF_R {
        ONREF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Internal Reference Voltage Change Enable"]
    #[inline(always)]
    pub fn irvce(&mut self) -> IRVCE_W {
        IRVCE_W { w: self }
    }
    #[doc = "Bits 3:6 - Internal Reference Voltage Selection"]
    #[inline(always)]
    pub fn irvs(&mut self) -> IRVS_W {
        IRVS_W { w: self }
    }
    #[doc = "Bit 19 - Force Internal Reference Voltage"]
    #[inline(always)]
    pub fn forceref(&mut self) -> FORCEREF_W {
        FORCEREF_W { w: self }
    }
    #[doc = "Bit 20 - Internal Voltage Reference ON"]
    #[inline(always)]
    pub fn onref(&mut self) -> ONREF_W {
        ONREF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr::R](R) reader structure"]
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr::W](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACR to value 0x0008_0000"]
impl crate::Resettable for ACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0008_0000
    }
}
