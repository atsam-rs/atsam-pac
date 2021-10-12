#[doc = "Register `BMR` reader"]
pub struct R(crate::R<BMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMR` writer"]
pub struct W(crate::W<BMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMR_SPEC>;
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
impl From<crate::W<BMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External Clock Signal 0 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TC0XC0S_A {
    #[doc = "0: Signal connected to XC0: TCLK0"]
    TCLK0 = 0,
    #[doc = "2: Signal connected to XC0: TIOA1"]
    TIOA1 = 2,
    #[doc = "3: Signal connected to XC0: TIOA2"]
    TIOA2 = 3,
}
impl From<TC0XC0S_A> for u8 {
    #[inline(always)]
    fn from(variant: TC0XC0S_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TC0XC0S` reader - External Clock Signal 0 Selection"]
pub struct TC0XC0S_R(crate::FieldReader<u8, TC0XC0S_A>);
impl TC0XC0S_R {
    pub(crate) fn new(bits: u8) -> Self {
        TC0XC0S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TC0XC0S_A> {
        match self.bits {
            0 => Some(TC0XC0S_A::TCLK0),
            2 => Some(TC0XC0S_A::TIOA1),
            3 => Some(TC0XC0S_A::TIOA2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCLK0`"]
    #[inline(always)]
    pub fn is_tclk0(&self) -> bool {
        **self == TC0XC0S_A::TCLK0
    }
    #[doc = "Checks if the value of the field is `TIOA1`"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        **self == TC0XC0S_A::TIOA1
    }
    #[doc = "Checks if the value of the field is `TIOA2`"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        **self == TC0XC0S_A::TIOA2
    }
}
impl core::ops::Deref for TC0XC0S_R {
    type Target = crate::FieldReader<u8, TC0XC0S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC0XC0S` writer - External Clock Signal 0 Selection"]
pub struct TC0XC0S_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0XC0S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC0XC0S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Signal connected to XC0: TCLK0"]
    #[inline(always)]
    pub fn tclk0(self) -> &'a mut W {
        self.variant(TC0XC0S_A::TCLK0)
    }
    #[doc = "Signal connected to XC0: TIOA1"]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC0XC0S_A::TIOA1)
    }
    #[doc = "Signal connected to XC0: TIOA2"]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC0XC0S_A::TIOA2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "External Clock Signal 1 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TC1XC1S_A {
    #[doc = "0: Signal connected to XC1: TCLK1"]
    TCLK1 = 0,
    #[doc = "2: Signal connected to XC1: TIOA0"]
    TIOA0 = 2,
    #[doc = "3: Signal connected to XC1: TIOA2"]
    TIOA2 = 3,
}
impl From<TC1XC1S_A> for u8 {
    #[inline(always)]
    fn from(variant: TC1XC1S_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TC1XC1S` reader - External Clock Signal 1 Selection"]
pub struct TC1XC1S_R(crate::FieldReader<u8, TC1XC1S_A>);
impl TC1XC1S_R {
    pub(crate) fn new(bits: u8) -> Self {
        TC1XC1S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TC1XC1S_A> {
        match self.bits {
            0 => Some(TC1XC1S_A::TCLK1),
            2 => Some(TC1XC1S_A::TIOA0),
            3 => Some(TC1XC1S_A::TIOA2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCLK1`"]
    #[inline(always)]
    pub fn is_tclk1(&self) -> bool {
        **self == TC1XC1S_A::TCLK1
    }
    #[doc = "Checks if the value of the field is `TIOA0`"]
    #[inline(always)]
    pub fn is_tioa0(&self) -> bool {
        **self == TC1XC1S_A::TIOA0
    }
    #[doc = "Checks if the value of the field is `TIOA2`"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        **self == TC1XC1S_A::TIOA2
    }
}
impl core::ops::Deref for TC1XC1S_R {
    type Target = crate::FieldReader<u8, TC1XC1S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC1XC1S` writer - External Clock Signal 1 Selection"]
pub struct TC1XC1S_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1XC1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC1XC1S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Signal connected to XC1: TCLK1"]
    #[inline(always)]
    pub fn tclk1(self) -> &'a mut W {
        self.variant(TC1XC1S_A::TCLK1)
    }
    #[doc = "Signal connected to XC1: TIOA0"]
    #[inline(always)]
    pub fn tioa0(self) -> &'a mut W {
        self.variant(TC1XC1S_A::TIOA0)
    }
    #[doc = "Signal connected to XC1: TIOA2"]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC1XC1S_A::TIOA2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "External Clock Signal 2 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TC2XC2S_A {
    #[doc = "0: Signal connected to XC2: TCLK2"]
    TCLK2 = 0,
    #[doc = "2: Signal connected to XC2: TIOA1"]
    TIOA1 = 2,
    #[doc = "3: Signal connected to XC2: TIOA2"]
    TIOA2 = 3,
}
impl From<TC2XC2S_A> for u8 {
    #[inline(always)]
    fn from(variant: TC2XC2S_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TC2XC2S` reader - External Clock Signal 2 Selection"]
pub struct TC2XC2S_R(crate::FieldReader<u8, TC2XC2S_A>);
impl TC2XC2S_R {
    pub(crate) fn new(bits: u8) -> Self {
        TC2XC2S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TC2XC2S_A> {
        match self.bits {
            0 => Some(TC2XC2S_A::TCLK2),
            2 => Some(TC2XC2S_A::TIOA1),
            3 => Some(TC2XC2S_A::TIOA2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCLK2`"]
    #[inline(always)]
    pub fn is_tclk2(&self) -> bool {
        **self == TC2XC2S_A::TCLK2
    }
    #[doc = "Checks if the value of the field is `TIOA1`"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        **self == TC2XC2S_A::TIOA1
    }
    #[doc = "Checks if the value of the field is `TIOA2`"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        **self == TC2XC2S_A::TIOA2
    }
}
impl core::ops::Deref for TC2XC2S_R {
    type Target = crate::FieldReader<u8, TC2XC2S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC2XC2S` writer - External Clock Signal 2 Selection"]
pub struct TC2XC2S_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2XC2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC2XC2S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Signal connected to XC2: TCLK2"]
    #[inline(always)]
    pub fn tclk2(self) -> &'a mut W {
        self.variant(TC2XC2S_A::TCLK2)
    }
    #[doc = "Signal connected to XC2: TIOA1"]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC2XC2S_A::TIOA1)
    }
    #[doc = "Signal connected to XC2: TIOA2"]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC2XC2S_A::TIOA2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `QDEN` reader - Quadrature Decoder ENabled"]
pub struct QDEN_R(crate::FieldReader<bool, bool>);
impl QDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        QDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QDEN` writer - Quadrature Decoder ENabled"]
pub struct QDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QDEN_W<'a> {
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
#[doc = "Field `POSEN` reader - POSition ENabled"]
pub struct POSEN_R(crate::FieldReader<bool, bool>);
impl POSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        POSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POSEN` writer - POSition ENabled"]
pub struct POSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> POSEN_W<'a> {
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
#[doc = "Field `SPEEDEN` reader - SPEED ENabled"]
pub struct SPEEDEN_R(crate::FieldReader<bool, bool>);
impl SPEEDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPEEDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPEEDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPEEDEN` writer - SPEED ENabled"]
pub struct SPEEDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEEDEN_W<'a> {
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
#[doc = "Field `QDTRANS` reader - Quadrature Decoding TRANSparent"]
pub struct QDTRANS_R(crate::FieldReader<bool, bool>);
impl QDTRANS_R {
    pub(crate) fn new(bits: bool) -> Self {
        QDTRANS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QDTRANS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QDTRANS` writer - Quadrature Decoding TRANSparent"]
pub struct QDTRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> QDTRANS_W<'a> {
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
#[doc = "Field `EDGPHA` reader - EDGe on PHA count mode"]
pub struct EDGPHA_R(crate::FieldReader<bool, bool>);
impl EDGPHA_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDGPHA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGPHA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGPHA` writer - EDGe on PHA count mode"]
pub struct EDGPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGPHA_W<'a> {
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
#[doc = "Field `INVA` reader - INVerted phA"]
pub struct INVA_R(crate::FieldReader<bool, bool>);
impl INVA_R {
    pub(crate) fn new(bits: bool) -> Self {
        INVA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVA` writer - INVerted phA"]
pub struct INVA_W<'a> {
    w: &'a mut W,
}
impl<'a> INVA_W<'a> {
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
#[doc = "Field `INVB` reader - INVerted phB"]
pub struct INVB_R(crate::FieldReader<bool, bool>);
impl INVB_R {
    pub(crate) fn new(bits: bool) -> Self {
        INVB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVB` writer - INVerted phB"]
pub struct INVB_W<'a> {
    w: &'a mut W,
}
impl<'a> INVB_W<'a> {
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
#[doc = "Field `INVIDX` reader - INVerted InDeX"]
pub struct INVIDX_R(crate::FieldReader<bool, bool>);
impl INVIDX_R {
    pub(crate) fn new(bits: bool) -> Self {
        INVIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVIDX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVIDX` writer - INVerted InDeX"]
pub struct INVIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> INVIDX_W<'a> {
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
#[doc = "Field `SWAP` reader - SWAP PHA and PHB"]
pub struct SWAP_R(crate::FieldReader<bool, bool>);
impl SWAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWAP` writer - SWAP PHA and PHB"]
pub struct SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP_W<'a> {
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
#[doc = "Field `IDXPHB` reader - InDeX pin is PHB pin"]
pub struct IDXPHB_R(crate::FieldReader<bool, bool>);
impl IDXPHB_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDXPHB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDXPHB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDXPHB` writer - InDeX pin is PHB pin"]
pub struct IDXPHB_W<'a> {
    w: &'a mut W,
}
impl<'a> IDXPHB_W<'a> {
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
#[doc = "Field `FILTER` reader - "]
pub struct FILTER_R(crate::FieldReader<bool, bool>);
impl FILTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        FILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER` writer - "]
pub struct FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_W<'a> {
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
#[doc = "Field `MAXFILT` reader - MAXimum FILTer"]
pub struct MAXFILT_R(crate::FieldReader<u8, u8>);
impl MAXFILT_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAXFILT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXFILT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXFILT` writer - MAXimum FILTer"]
pub struct MAXFILT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXFILT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | ((value as u32 & 0x3f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    pub fn tc0xc0s(&self) -> TC0XC0S_R {
        TC0XC0S_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    pub fn tc1xc1s(&self) -> TC1XC1S_R {
        TC1XC1S_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    pub fn tc2xc2s(&self) -> TC2XC2S_R {
        TC2XC2S_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Quadrature Decoder ENabled"]
    #[inline(always)]
    pub fn qden(&self) -> QDEN_R {
        QDEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - POSition ENabled"]
    #[inline(always)]
    pub fn posen(&self) -> POSEN_R {
        POSEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SPEED ENabled"]
    #[inline(always)]
    pub fn speeden(&self) -> SPEEDEN_R {
        SPEEDEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Quadrature Decoding TRANSparent"]
    #[inline(always)]
    pub fn qdtrans(&self) -> QDTRANS_R {
        QDTRANS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EDGe on PHA count mode"]
    #[inline(always)]
    pub fn edgpha(&self) -> EDGPHA_R {
        EDGPHA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - INVerted phA"]
    #[inline(always)]
    pub fn inva(&self) -> INVA_R {
        INVA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - INVerted phB"]
    #[inline(always)]
    pub fn invb(&self) -> INVB_R {
        INVB_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - INVerted InDeX"]
    #[inline(always)]
    pub fn invidx(&self) -> INVIDX_R {
        INVIDX_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SWAP PHA and PHB"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - InDeX pin is PHB pin"]
    #[inline(always)]
    pub fn idxphb(&self) -> IDXPHB_R {
        IDXPHB_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:25 - MAXimum FILTer"]
    #[inline(always)]
    pub fn maxfilt(&self) -> MAXFILT_R {
        MAXFILT_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    pub fn tc0xc0s(&mut self) -> TC0XC0S_W {
        TC0XC0S_W { w: self }
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    pub fn tc1xc1s(&mut self) -> TC1XC1S_W {
        TC1XC1S_W { w: self }
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    pub fn tc2xc2s(&mut self) -> TC2XC2S_W {
        TC2XC2S_W { w: self }
    }
    #[doc = "Bit 8 - Quadrature Decoder ENabled"]
    #[inline(always)]
    pub fn qden(&mut self) -> QDEN_W {
        QDEN_W { w: self }
    }
    #[doc = "Bit 9 - POSition ENabled"]
    #[inline(always)]
    pub fn posen(&mut self) -> POSEN_W {
        POSEN_W { w: self }
    }
    #[doc = "Bit 10 - SPEED ENabled"]
    #[inline(always)]
    pub fn speeden(&mut self) -> SPEEDEN_W {
        SPEEDEN_W { w: self }
    }
    #[doc = "Bit 11 - Quadrature Decoding TRANSparent"]
    #[inline(always)]
    pub fn qdtrans(&mut self) -> QDTRANS_W {
        QDTRANS_W { w: self }
    }
    #[doc = "Bit 12 - EDGe on PHA count mode"]
    #[inline(always)]
    pub fn edgpha(&mut self) -> EDGPHA_W {
        EDGPHA_W { w: self }
    }
    #[doc = "Bit 13 - INVerted phA"]
    #[inline(always)]
    pub fn inva(&mut self) -> INVA_W {
        INVA_W { w: self }
    }
    #[doc = "Bit 14 - INVerted phB"]
    #[inline(always)]
    pub fn invb(&mut self) -> INVB_W {
        INVB_W { w: self }
    }
    #[doc = "Bit 15 - INVerted InDeX"]
    #[inline(always)]
    pub fn invidx(&mut self) -> INVIDX_W {
        INVIDX_W { w: self }
    }
    #[doc = "Bit 16 - SWAP PHA and PHB"]
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W {
        SWAP_W { w: self }
    }
    #[doc = "Bit 17 - InDeX pin is PHB pin"]
    #[inline(always)]
    pub fn idxphb(&mut self) -> IDXPHB_W {
        IDXPHB_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn filter(&mut self) -> FILTER_W {
        FILTER_W { w: self }
    }
    #[doc = "Bits 20:25 - MAXimum FILTer"]
    #[inline(always)]
    pub fn maxfilt(&mut self) -> MAXFILT_W {
        MAXFILT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmr](index.html) module"]
pub struct BMR_SPEC;
impl crate::RegisterSpec for BMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmr::R](R) reader structure"]
impl crate::Readable for BMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmr::W](W) writer structure"]
impl crate::Writable for BMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMR to value 0"]
impl crate::Resettable for BMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
