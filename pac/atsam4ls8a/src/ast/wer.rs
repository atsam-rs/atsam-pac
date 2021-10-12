#[doc = "Register `WER` reader"]
pub struct R(crate::R<WER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WER` writer"]
pub struct W(crate::W<WER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WER_SPEC>;
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
impl From<crate::W<WER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVF_A {
    #[doc = "0: The corresponing event will not wake up the CPU from sleep mode"]
    _0 = 0,
    #[doc = "1: The corresponding event will wake up the CPU from sleep mode"]
    _1 = 1,
}
impl From<OVF_A> for bool {
    #[inline(always)]
    fn from(variant: OVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVF` reader - Overflow"]
pub struct OVF_R(crate::FieldReader<bool, OVF_A>);
impl OVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVF_A {
        match self.bits {
            false => OVF_A::_0,
            true => OVF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OVF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OVF_A::_1
    }
}
impl core::ops::Deref for OVF_R {
    type Target = crate::FieldReader<bool, OVF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF` writer - Overflow"]
pub struct OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVF_A::_0)
    }
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVF_A::_1)
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
#[doc = "Alarm 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARM0_A {
    #[doc = "0: The corresponing event will not wake up the CPU from sleep mode"]
    _0 = 0,
    #[doc = "1: The corresponding event will wake up the CPU from sleep mode"]
    _1 = 1,
}
impl From<ALARM0_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALARM0` reader - Alarm 0"]
pub struct ALARM0_R(crate::FieldReader<bool, ALARM0_A>);
impl ALARM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALARM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM0_A {
        match self.bits {
            false => ALARM0_A::_0,
            true => ALARM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALARM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALARM0_A::_1
    }
}
impl core::ops::Deref for ALARM0_R {
    type Target = crate::FieldReader<bool, ALARM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALARM0` writer - Alarm 0"]
pub struct ALARM0_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALARM0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALARM0_A::_0)
    }
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALARM0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Alarm 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARM1_A {
    #[doc = "0: The corresponing event will not wake up the CPU from sleep mode"]
    _0 = 0,
    #[doc = "1: The corresponding event will wake up the CPU from sleep mode"]
    _1 = 1,
}
impl From<ALARM1_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALARM1` reader - Alarm 1"]
pub struct ALARM1_R(crate::FieldReader<bool, ALARM1_A>);
impl ALARM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALARM1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM1_A {
        match self.bits {
            false => ALARM1_A::_0,
            true => ALARM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALARM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALARM1_A::_1
    }
}
impl core::ops::Deref for ALARM1_R {
    type Target = crate::FieldReader<bool, ALARM1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALARM1` writer - Alarm 1"]
pub struct ALARM1_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALARM1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALARM1_A::_0)
    }
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALARM1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Periodic 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER0_A {
    #[doc = "0: The corresponing event will not wake up the CPU from sleep mode"]
    _0 = 0,
    #[doc = "1: The corresponding event will wake up the CPU from sleep mode"]
    _1 = 1,
}
impl From<PER0_A> for bool {
    #[inline(always)]
    fn from(variant: PER0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER0` reader - Periodic 0"]
pub struct PER0_R(crate::FieldReader<bool, PER0_A>);
impl PER0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PER0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER0_A {
        match self.bits {
            false => PER0_A::_0,
            true => PER0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PER0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PER0_A::_1
    }
}
impl core::ops::Deref for PER0_R {
    type Target = crate::FieldReader<bool, PER0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER0` writer - Periodic 0"]
pub struct PER0_W<'a> {
    w: &'a mut W,
}
impl<'a> PER0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PER0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PER0_A::_0)
    }
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PER0_A::_1)
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
#[doc = "Periodic 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER1_A {
    #[doc = "0: The corresponing event will not wake up the CPU from sleep mode"]
    _0 = 0,
    #[doc = "1: The corresponding event will wake up the CPU from sleep mode"]
    _1 = 1,
}
impl From<PER1_A> for bool {
    #[inline(always)]
    fn from(variant: PER1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER1` reader - Periodic 1"]
pub struct PER1_R(crate::FieldReader<bool, PER1_A>);
impl PER1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PER1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER1_A {
        match self.bits {
            false => PER1_A::_0,
            true => PER1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PER1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PER1_A::_1
    }
}
impl core::ops::Deref for PER1_R {
    type Target = crate::FieldReader<bool, PER1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER1` writer - Periodic 1"]
pub struct PER1_W<'a> {
    w: &'a mut W,
}
impl<'a> PER1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PER1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PER1_A::_0)
    }
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PER1_A::_1)
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
impl R {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alarm 0"]
    #[inline(always)]
    pub fn alarm0(&self) -> ALARM0_R {
        ALARM0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Alarm 1"]
    #[inline(always)]
    pub fn alarm1(&self) -> ALARM1_R {
        ALARM1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Periodic 0"]
    #[inline(always)]
    pub fn per0(&self) -> PER0_R {
        PER0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Periodic 1"]
    #[inline(always)]
    pub fn per1(&self) -> PER1_R {
        PER1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W {
        OVF_W { w: self }
    }
    #[doc = "Bit 8 - Alarm 0"]
    #[inline(always)]
    pub fn alarm0(&mut self) -> ALARM0_W {
        ALARM0_W { w: self }
    }
    #[doc = "Bit 9 - Alarm 1"]
    #[inline(always)]
    pub fn alarm1(&mut self) -> ALARM1_W {
        ALARM1_W { w: self }
    }
    #[doc = "Bit 16 - Periodic 0"]
    #[inline(always)]
    pub fn per0(&mut self) -> PER0_W {
        PER0_W { w: self }
    }
    #[doc = "Bit 17 - Periodic 1"]
    #[inline(always)]
    pub fn per1(&mut self) -> PER1_W {
        PER1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wer](index.html) module"]
pub struct WER_SPEC;
impl crate::RegisterSpec for WER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wer::R](R) reader structure"]
impl crate::Readable for WER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wer::W](W) writer structure"]
impl crate::Writable for WER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WER to value 0"]
impl crate::Resettable for WER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
