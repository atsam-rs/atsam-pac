#[doc = "Register `EVS%s` reader"]
pub struct R(crate::R<EVS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EVS_SPEC>> for R {
    fn from(reader: crate::R<EVS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVS%s` writer"]
pub struct W(crate::W<EVS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVS_SPEC>;
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
impl core::convert::From<crate::W<EVS_SPEC>> for W {
    fn from(writer: crate::W<EVS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Event Shaper Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: No Action"]
    _0 = 0,
    #[doc = "1: Event Shaper enable"]
    _1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Event Shaper Enable"]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::_0,
            true => EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EN_A::_1
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Event Shaper Enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN_A::_0)
    }
    #[doc = "Event Shaper enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Input Glitch Filter Rise\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IGFR_A {
    #[doc = "0: No Action"]
    _0 = 0,
    #[doc = "1: Input Glitch Filter : a rising edge on event input will raise trigger output"]
    _1 = 1,
}
impl From<IGFR_A> for bool {
    #[inline(always)]
    fn from(variant: IGFR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IGFR` reader - Input Glitch Filter Rise"]
pub struct IGFR_R(crate::FieldReader<bool, IGFR_A>);
impl IGFR_R {
    pub(crate) fn new(bits: bool) -> Self {
        IGFR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IGFR_A {
        match self.bits {
            false => IGFR_A::_0,
            true => IGFR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IGFR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IGFR_A::_1
    }
}
impl core::ops::Deref for IGFR_R {
    type Target = crate::FieldReader<bool, IGFR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IGFR` writer - Input Glitch Filter Rise"]
pub struct IGFR_W<'a> {
    w: &'a mut W,
}
impl<'a> IGFR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IGFR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IGFR_A::_0)
    }
    #[doc = "Input Glitch Filter : a rising edge on event input will raise trigger output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IGFR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Input Glitch Filter Fall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IGFF_A {
    #[doc = "0: No Action"]
    _0 = 0,
    #[doc = "1: Input Glitch Filter : a falling edge on event input will raise trigger output"]
    _1 = 1,
}
impl From<IGFF_A> for bool {
    #[inline(always)]
    fn from(variant: IGFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IGFF` reader - Input Glitch Filter Fall"]
pub struct IGFF_R(crate::FieldReader<bool, IGFF_A>);
impl IGFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IGFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IGFF_A {
        match self.bits {
            false => IGFF_A::_0,
            true => IGFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IGFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IGFF_A::_1
    }
}
impl core::ops::Deref for IGFF_R {
    type Target = crate::FieldReader<bool, IGFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IGFF` writer - Input Glitch Filter Fall"]
pub struct IGFF_W<'a> {
    w: &'a mut W,
}
impl<'a> IGFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IGFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IGFF_A::_0)
    }
    #[doc = "Input Glitch Filter : a falling edge on event input will raise trigger output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IGFF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `IGFON` reader - Input Glitch Filter Status"]
pub struct IGFON_R(crate::FieldReader<bool, bool>);
impl IGFON_R {
    pub(crate) fn new(bits: bool) -> Self {
        IGFON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IGFON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IGFON` writer - Input Glitch Filter Status"]
pub struct IGFON_W<'a> {
    w: &'a mut W,
}
impl<'a> IGFON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Event Shaper Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Input Glitch Filter Rise"]
    #[inline(always)]
    pub fn igfr(&self) -> IGFR_R {
        IGFR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Input Glitch Filter Fall"]
    #[inline(always)]
    pub fn igff(&self) -> IGFF_R {
        IGFF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Input Glitch Filter Status"]
    #[inline(always)]
    pub fn igfon(&self) -> IGFON_R {
        IGFON_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Shaper Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 16 - Input Glitch Filter Rise"]
    #[inline(always)]
    pub fn igfr(&mut self) -> IGFR_W {
        IGFR_W { w: self }
    }
    #[doc = "Bit 17 - Input Glitch Filter Fall"]
    #[inline(always)]
    pub fn igff(&mut self) -> IGFF_W {
        IGFF_W { w: self }
    }
    #[doc = "Bit 18 - Input Glitch Filter Status"]
    #[inline(always)]
    pub fn igfon(&mut self) -> IGFON_W {
        IGFON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Shaper\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evs](index.html) module"]
pub struct EVS_SPEC;
impl crate::RegisterSpec for EVS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evs::R](R) reader structure"]
impl crate::Readable for EVS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evs::W](W) writer structure"]
impl crate::Writable for EVS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVS%s to value 0"]
impl crate::Resettable for EVS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
