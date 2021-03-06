#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NMI` reader - External Non Maskable CPU interrupt"]
pub struct NMI_R(crate::FieldReader<bool, bool>);
impl NMI_R {
    pub(crate) fn new(bits: bool) -> Self {
        NMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NMI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMI` writer - External Non Maskable CPU interrupt"]
pub struct NMI_W<'a> {
    w: &'a mut W,
}
impl<'a> NMI_W<'a> {
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
#[doc = "External Interrupt 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT1_A {
    #[doc = "0: Edge triggered interrupt"]
    _0 = 0,
    #[doc = "1: Level triggered interrupt"]
    _1 = 1,
}
impl From<INT1_A> for bool {
    #[inline(always)]
    fn from(variant: INT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT1` reader - External Interrupt 1"]
pub struct INT1_R(crate::FieldReader<bool, INT1_A>);
impl INT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT1_A {
        match self.bits {
            false => INT1_A::_0,
            true => INT1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INT1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INT1_A::_1
    }
}
impl core::ops::Deref for INT1_R {
    type Target = crate::FieldReader<bool, INT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT1` writer - External Interrupt 1"]
pub struct INT1_W<'a> {
    w: &'a mut W,
}
impl<'a> INT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge triggered interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT1_A::_0)
    }
    #[doc = "Level triggered interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "External Interrupt 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT2_A {
    #[doc = "0: Edge triggered interrupt"]
    _0 = 0,
    #[doc = "1: Level triggered interrupt"]
    _1 = 1,
}
impl From<INT2_A> for bool {
    #[inline(always)]
    fn from(variant: INT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT2` reader - External Interrupt 2"]
pub struct INT2_R(crate::FieldReader<bool, INT2_A>);
impl INT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT2_A {
        match self.bits {
            false => INT2_A::_0,
            true => INT2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INT2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INT2_A::_1
    }
}
impl core::ops::Deref for INT2_R {
    type Target = crate::FieldReader<bool, INT2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT2` writer - External Interrupt 2"]
pub struct INT2_W<'a> {
    w: &'a mut W,
}
impl<'a> INT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge triggered interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT2_A::_0)
    }
    #[doc = "Level triggered interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT2_A::_1)
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
#[doc = "External Interrupt 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT3_A {
    #[doc = "0: Edge triggered interrupt"]
    _0 = 0,
    #[doc = "1: Level triggered interrupt"]
    _1 = 1,
}
impl From<INT3_A> for bool {
    #[inline(always)]
    fn from(variant: INT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT3` reader - External Interrupt 3"]
pub struct INT3_R(crate::FieldReader<bool, INT3_A>);
impl INT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT3_A {
        match self.bits {
            false => INT3_A::_0,
            true => INT3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INT3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INT3_A::_1
    }
}
impl core::ops::Deref for INT3_R {
    type Target = crate::FieldReader<bool, INT3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT3` writer - External Interrupt 3"]
pub struct INT3_W<'a> {
    w: &'a mut W,
}
impl<'a> INT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge triggered interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT3_A::_0)
    }
    #[doc = "Level triggered interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "External Interrupt 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT4_A {
    #[doc = "0: Edge triggered interrupt"]
    _0 = 0,
    #[doc = "1: Level triggered interrupt"]
    _1 = 1,
}
impl From<INT4_A> for bool {
    #[inline(always)]
    fn from(variant: INT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT4` reader - External Interrupt 4"]
pub struct INT4_R(crate::FieldReader<bool, INT4_A>);
impl INT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT4_A {
        match self.bits {
            false => INT4_A::_0,
            true => INT4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INT4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INT4_A::_1
    }
}
impl core::ops::Deref for INT4_R {
    type Target = crate::FieldReader<bool, INT4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT4` writer - External Interrupt 4"]
pub struct INT4_W<'a> {
    w: &'a mut W,
}
impl<'a> INT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge triggered interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT4_A::_0)
    }
    #[doc = "Level triggered interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `INT5` reader - External Interrupt 5"]
pub struct INT5_R(crate::FieldReader<bool, bool>);
impl INT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT5` writer - External Interrupt 5"]
pub struct INT5_W<'a> {
    w: &'a mut W,
}
impl<'a> INT5_W<'a> {
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
#[doc = "Field `INT6` reader - External Interrupt 6"]
pub struct INT6_R(crate::FieldReader<bool, bool>);
impl INT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT6` writer - External Interrupt 6"]
pub struct INT6_W<'a> {
    w: &'a mut W,
}
impl<'a> INT6_W<'a> {
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
#[doc = "Field `INT7` reader - External Interrupt 7"]
pub struct INT7_R(crate::FieldReader<bool, bool>);
impl INT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT7` writer - External Interrupt 7"]
pub struct INT7_W<'a> {
    w: &'a mut W,
}
impl<'a> INT7_W<'a> {
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
#[doc = "Field `INT8` reader - External Interrupt 8"]
pub struct INT8_R(crate::FieldReader<bool, bool>);
impl INT8_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT8` writer - External Interrupt 8"]
pub struct INT8_W<'a> {
    w: &'a mut W,
}
impl<'a> INT8_W<'a> {
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
#[doc = "Field `INT9` reader - External Interrupt 9"]
pub struct INT9_R(crate::FieldReader<bool, bool>);
impl INT9_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT9` writer - External Interrupt 9"]
pub struct INT9_W<'a> {
    w: &'a mut W,
}
impl<'a> INT9_W<'a> {
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
#[doc = "Field `INT10` reader - External Interrupt 10"]
pub struct INT10_R(crate::FieldReader<bool, bool>);
impl INT10_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT10` writer - External Interrupt 10"]
pub struct INT10_W<'a> {
    w: &'a mut W,
}
impl<'a> INT10_W<'a> {
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
#[doc = "Field `INT11` reader - External Interrupt 11"]
pub struct INT11_R(crate::FieldReader<bool, bool>);
impl INT11_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT11` writer - External Interrupt 11"]
pub struct INT11_W<'a> {
    w: &'a mut W,
}
impl<'a> INT11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `INT12` reader - External Interrupt 12"]
pub struct INT12_R(crate::FieldReader<bool, bool>);
impl INT12_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT12` writer - External Interrupt 12"]
pub struct INT12_W<'a> {
    w: &'a mut W,
}
impl<'a> INT12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `INT13` reader - External Interrupt 13"]
pub struct INT13_R(crate::FieldReader<bool, bool>);
impl INT13_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT13` writer - External Interrupt 13"]
pub struct INT13_W<'a> {
    w: &'a mut W,
}
impl<'a> INT13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `INT14` reader - External Interrupt 14"]
pub struct INT14_R(crate::FieldReader<bool, bool>);
impl INT14_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT14` writer - External Interrupt 14"]
pub struct INT14_W<'a> {
    w: &'a mut W,
}
impl<'a> INT14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `INT15` reader - External Interrupt 15"]
pub struct INT15_R(crate::FieldReader<bool, bool>);
impl INT15_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT15` writer - External Interrupt 15"]
pub struct INT15_W<'a> {
    w: &'a mut W,
}
impl<'a> INT15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - External Non Maskable CPU interrupt"]
    #[inline(always)]
    pub fn nmi(&self) -> NMI_R {
        NMI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2"]
    #[inline(always)]
    pub fn int2(&self) -> INT2_R {
        INT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3"]
    #[inline(always)]
    pub fn int3(&self) -> INT3_R {
        INT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4"]
    #[inline(always)]
    pub fn int4(&self) -> INT4_R {
        INT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5"]
    #[inline(always)]
    pub fn int5(&self) -> INT5_R {
        INT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6"]
    #[inline(always)]
    pub fn int6(&self) -> INT6_R {
        INT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7"]
    #[inline(always)]
    pub fn int7(&self) -> INT7_R {
        INT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - External Interrupt 8"]
    #[inline(always)]
    pub fn int8(&self) -> INT8_R {
        INT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - External Interrupt 9"]
    #[inline(always)]
    pub fn int9(&self) -> INT9_R {
        INT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - External Interrupt 10"]
    #[inline(always)]
    pub fn int10(&self) -> INT10_R {
        INT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External Interrupt 11"]
    #[inline(always)]
    pub fn int11(&self) -> INT11_R {
        INT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - External Interrupt 12"]
    #[inline(always)]
    pub fn int12(&self) -> INT12_R {
        INT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - External Interrupt 13"]
    #[inline(always)]
    pub fn int13(&self) -> INT13_R {
        INT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - External Interrupt 14"]
    #[inline(always)]
    pub fn int14(&self) -> INT14_R {
        INT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - External Interrupt 15"]
    #[inline(always)]
    pub fn int15(&self) -> INT15_R {
        INT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Non Maskable CPU interrupt"]
    #[inline(always)]
    pub fn nmi(&mut self) -> NMI_W {
        NMI_W { w: self }
    }
    #[doc = "Bit 1 - External Interrupt 1"]
    #[inline(always)]
    pub fn int1(&mut self) -> INT1_W {
        INT1_W { w: self }
    }
    #[doc = "Bit 2 - External Interrupt 2"]
    #[inline(always)]
    pub fn int2(&mut self) -> INT2_W {
        INT2_W { w: self }
    }
    #[doc = "Bit 3 - External Interrupt 3"]
    #[inline(always)]
    pub fn int3(&mut self) -> INT3_W {
        INT3_W { w: self }
    }
    #[doc = "Bit 4 - External Interrupt 4"]
    #[inline(always)]
    pub fn int4(&mut self) -> INT4_W {
        INT4_W { w: self }
    }
    #[doc = "Bit 5 - External Interrupt 5"]
    #[inline(always)]
    pub fn int5(&mut self) -> INT5_W {
        INT5_W { w: self }
    }
    #[doc = "Bit 6 - External Interrupt 6"]
    #[inline(always)]
    pub fn int6(&mut self) -> INT6_W {
        INT6_W { w: self }
    }
    #[doc = "Bit 7 - External Interrupt 7"]
    #[inline(always)]
    pub fn int7(&mut self) -> INT7_W {
        INT7_W { w: self }
    }
    #[doc = "Bit 8 - External Interrupt 8"]
    #[inline(always)]
    pub fn int8(&mut self) -> INT8_W {
        INT8_W { w: self }
    }
    #[doc = "Bit 9 - External Interrupt 9"]
    #[inline(always)]
    pub fn int9(&mut self) -> INT9_W {
        INT9_W { w: self }
    }
    #[doc = "Bit 10 - External Interrupt 10"]
    #[inline(always)]
    pub fn int10(&mut self) -> INT10_W {
        INT10_W { w: self }
    }
    #[doc = "Bit 11 - External Interrupt 11"]
    #[inline(always)]
    pub fn int11(&mut self) -> INT11_W {
        INT11_W { w: self }
    }
    #[doc = "Bit 12 - External Interrupt 12"]
    #[inline(always)]
    pub fn int12(&mut self) -> INT12_W {
        INT12_W { w: self }
    }
    #[doc = "Bit 13 - External Interrupt 13"]
    #[inline(always)]
    pub fn int13(&mut self) -> INT13_W {
        INT13_W { w: self }
    }
    #[doc = "Bit 14 - External Interrupt 14"]
    #[inline(always)]
    pub fn int14(&mut self) -> INT14_W {
        INT14_W { w: self }
    }
    #[doc = "Bit 15 - External Interrupt 15"]
    #[inline(always)]
    pub fn int15(&mut self) -> INT15_W {
        INT15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
