#[doc = "Register `MAN` reader"]
pub struct R(crate::R<MAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MAN_SPEC>> for R {
    fn from(reader: crate::R<MAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAN` writer"]
pub struct W(crate::W<MAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAN_SPEC>;
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
impl core::convert::From<crate::W<MAN_SPEC>> for W {
    fn from(writer: crate::W<MAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmitter Preamble Length\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_PL_A {
    #[doc = "0: The Transmitter Preamble pattern generation is disabled"]
    _0 = 0,
}
impl From<TX_PL_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TX_PL` reader - Transmitter Preamble Length"]
pub struct TX_PL_R(crate::FieldReader<u8, TX_PL_A>);
impl TX_PL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_PL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_PL_A> {
        match self.bits {
            0 => Some(TX_PL_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TX_PL_A::_0
    }
}
impl core::ops::Deref for TX_PL_R {
    type Target = crate::FieldReader<u8, TX_PL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PL` writer - Transmitter Preamble Length"]
pub struct TX_PL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The Transmitter Preamble pattern generation is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_PL_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Transmitter Preamble Pattern\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_PP_A {
    #[doc = "0: ALL_ONE"]
    _0 = 0,
    #[doc = "1: ALL_ZERO"]
    _1 = 1,
    #[doc = "2: ZERO_ONE"]
    _2 = 2,
    #[doc = "3: ONE_ZERO"]
    _3 = 3,
}
impl From<TX_PP_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TX_PP` reader - Transmitter Preamble Pattern"]
pub struct TX_PP_R(crate::FieldReader<u8, TX_PP_A>);
impl TX_PP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_PP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_PP_A {
        match self.bits {
            0 => TX_PP_A::_0,
            1 => TX_PP_A::_1,
            2 => TX_PP_A::_2,
            3 => TX_PP_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TX_PP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TX_PP_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TX_PP_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TX_PP_A::_3
    }
}
impl core::ops::Deref for TX_PP_R {
    type Target = crate::FieldReader<u8, TX_PP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PP` writer - Transmitter Preamble Pattern"]
pub struct TX_PP_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "ALL_ONE"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_PP_A::_0)
    }
    #[doc = "ALL_ZERO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_PP_A::_1)
    }
    #[doc = "ZERO_ONE"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TX_PP_A::_2)
    }
    #[doc = "ONE_ZERO"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TX_PP_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Transmitter Manchester Polarity\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_MPOL_A {
    #[doc = "0: Logic Zero is coded as a zero-to-one transition, Logic One is coded as a one-to-zero transition"]
    _0 = 0,
    #[doc = "1: Logic Zero is coded as a one-to-zero transition, Logic One is coded as a zero-to-one transition"]
    _1 = 1,
}
impl From<TX_MPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TX_MPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_MPOL` reader - Transmitter Manchester Polarity"]
pub struct TX_MPOL_R(crate::FieldReader<bool, TX_MPOL_A>);
impl TX_MPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_MPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_MPOL_A {
        match self.bits {
            false => TX_MPOL_A::_0,
            true => TX_MPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TX_MPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TX_MPOL_A::_1
    }
}
impl core::ops::Deref for TX_MPOL_R {
    type Target = crate::FieldReader<bool, TX_MPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_MPOL` writer - Transmitter Manchester Polarity"]
pub struct TX_MPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_MPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Logic Zero is coded as a zero-to-one transition, Logic One is coded as a one-to-zero transition"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_MPOL_A::_0)
    }
    #[doc = "Logic Zero is coded as a one-to-zero transition, Logic One is coded as a zero-to-one transition"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_MPOL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Receiver Preamble Length\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_PL_A {
    #[doc = "0: The receiver preamble pattern detection is disabled"]
    _0 = 0,
}
impl From<RX_PL_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_PL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RX_PL` reader - Receiver Preamble Length"]
pub struct RX_PL_R(crate::FieldReader<u8, RX_PL_A>);
impl RX_PL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_PL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RX_PL_A> {
        match self.bits {
            0 => Some(RX_PL_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RX_PL_A::_0
    }
}
impl core::ops::Deref for RX_PL_R {
    type Target = crate::FieldReader<u8, RX_PL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PL` writer - Receiver Preamble Length"]
pub struct RX_PL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_PL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The receiver preamble pattern detection is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_PL_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Receiver Preamble Pattern detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_PP_A {
    #[doc = "0: ALL_ONE"]
    _0 = 0,
    #[doc = "1: ALL_ZERO"]
    _1 = 1,
    #[doc = "2: ZERO_ONE"]
    _2 = 2,
    #[doc = "3: ONE_ZERO"]
    _3 = 3,
}
impl From<RX_PP_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_PP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RX_PP` reader - Receiver Preamble Pattern detected"]
pub struct RX_PP_R(crate::FieldReader<u8, RX_PP_A>);
impl RX_PP_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_PP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_PP_A {
        match self.bits {
            0 => RX_PP_A::_0,
            1 => RX_PP_A::_1,
            2 => RX_PP_A::_2,
            3 => RX_PP_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RX_PP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RX_PP_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == RX_PP_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == RX_PP_A::_3
    }
}
impl core::ops::Deref for RX_PP_R {
    type Target = crate::FieldReader<u8, RX_PP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PP` writer - Receiver Preamble Pattern detected"]
pub struct RX_PP_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_PP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "ALL_ONE"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_PP_A::_0)
    }
    #[doc = "ALL_ZERO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_PP_A::_1)
    }
    #[doc = "ZERO_ONE"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(RX_PP_A::_2)
    }
    #[doc = "ONE_ZERO"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(RX_PP_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Receiver Manchester Polarity\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_MPOL_A {
    #[doc = "0: Logic Zero is coded as a zero-to-one transition, Logic One is coded as a one-to-zero transition"]
    _0 = 0,
    #[doc = "1: Logic Zero is coded as a one-to-zero transition, Logic One is coded as a zero-to-one transition"]
    _1 = 1,
}
impl From<RX_MPOL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_MPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_MPOL` reader - Receiver Manchester Polarity"]
pub struct RX_MPOL_R(crate::FieldReader<bool, RX_MPOL_A>);
impl RX_MPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_MPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_MPOL_A {
        match self.bits {
            false => RX_MPOL_A::_0,
            true => RX_MPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RX_MPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RX_MPOL_A::_1
    }
}
impl core::ops::Deref for RX_MPOL_R {
    type Target = crate::FieldReader<bool, RX_MPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_MPOL` writer - Receiver Manchester Polarity"]
pub struct RX_MPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_MPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Logic Zero is coded as a zero-to-one transition, Logic One is coded as a one-to-zero transition"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_MPOL_A::_0)
    }
    #[doc = "Logic Zero is coded as a one-to-zero transition, Logic One is coded as a zero-to-one transition"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_MPOL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Drift compensation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRIFT_A {
    #[doc = "0: The USART can not recover from an important clock drift"]
    _0 = 0,
    #[doc = "1: The USART can recover from clock drift. The 16X clock mode must be enabled"]
    _1 = 1,
}
impl From<DRIFT_A> for bool {
    #[inline(always)]
    fn from(variant: DRIFT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRIFT` reader - Drift compensation"]
pub struct DRIFT_R(crate::FieldReader<bool, DRIFT_A>);
impl DRIFT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRIFT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIFT_A {
        match self.bits {
            false => DRIFT_A::_0,
            true => DRIFT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DRIFT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DRIFT_A::_1
    }
}
impl core::ops::Deref for DRIFT_R {
    type Target = crate::FieldReader<bool, DRIFT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRIFT` writer - Drift compensation"]
pub struct DRIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIFT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRIFT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The USART can not recover from an important clock drift"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRIFT_A::_0)
    }
    #[doc = "The USART can recover from clock drift. The 16X clock mode must be enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRIFT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline(always)]
    pub fn tx_pl(&self) -> TX_PL_R {
        TX_PL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline(always)]
    pub fn tx_pp(&self) -> TX_PP_R {
        TX_PP_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline(always)]
    pub fn tx_mpol(&self) -> TX_MPOL_R {
        TX_MPOL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline(always)]
    pub fn rx_pl(&self) -> RX_PL_R {
        RX_PL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline(always)]
    pub fn rx_pp(&self) -> RX_PP_R {
        RX_PP_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline(always)]
    pub fn rx_mpol(&self) -> RX_MPOL_R {
        RX_MPOL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Drift compensation"]
    #[inline(always)]
    pub fn drift(&self) -> DRIFT_R {
        DRIFT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline(always)]
    pub fn tx_pl(&mut self) -> TX_PL_W {
        TX_PL_W { w: self }
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline(always)]
    pub fn tx_pp(&mut self) -> TX_PP_W {
        TX_PP_W { w: self }
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline(always)]
    pub fn tx_mpol(&mut self) -> TX_MPOL_W {
        TX_MPOL_W { w: self }
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline(always)]
    pub fn rx_pl(&mut self) -> RX_PL_W {
        RX_PL_W { w: self }
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline(always)]
    pub fn rx_pp(&mut self) -> RX_PP_W {
        RX_PP_W { w: self }
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline(always)]
    pub fn rx_mpol(&mut self) -> RX_MPOL_W {
        RX_MPOL_W { w: self }
    }
    #[doc = "Bit 30 - Drift compensation"]
    #[inline(always)]
    pub fn drift(&mut self) -> DRIFT_W {
        DRIFT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Manchester Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [man](index.html) module"]
pub struct MAN_SPEC;
impl crate::RegisterSpec for MAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [man::R](R) reader structure"]
impl crate::Readable for MAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [man::W](W) writer structure"]
impl crate::Writable for MAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAN to value 0x3001_1004"]
impl crate::Resettable for MAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3001_1004
    }
}
