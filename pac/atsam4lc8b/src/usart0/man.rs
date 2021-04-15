#[doc = "Reader of register MAN"]
pub type R = crate::R<u32, super::MAN>;
#[doc = "Writer for register MAN"]
pub type W = crate::W<u32, super::MAN>;
#[doc = "Register MAN `reset()`'s with value 0x3001_1004"]
impl crate::ResetValue for super::MAN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3001_1004
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
#[doc = "Reader of field `TX_PL`"]
pub type TX_PL_R = crate::R<u8, TX_PL_A>;
impl TX_PL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_PL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_PL_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TX_PL_A::_0
    }
}
#[doc = "Write proxy for field `TX_PL`"]
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
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
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
#[doc = "Reader of field `TX_PP`"]
pub type TX_PP_R = crate::R<u8, TX_PP_A>;
impl TX_PP_R {
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
        *self == TX_PP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TX_PP_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == TX_PP_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == TX_PP_A::_3
    }
}
#[doc = "Write proxy for field `TX_PP`"]
pub struct TX_PP_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
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
#[doc = "Reader of field `TX_MPOL`"]
pub type TX_MPOL_R = crate::R<bool, TX_MPOL_A>;
impl TX_MPOL_R {
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
        *self == TX_MPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TX_MPOL_A::_1
    }
}
#[doc = "Write proxy for field `TX_MPOL`"]
pub struct TX_MPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_MPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
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
#[doc = "Reader of field `RX_PL`"]
pub type RX_PL_R = crate::R<u8, RX_PL_A>;
impl RX_PL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RX_PL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX_PL_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX_PL_A::_0
    }
}
#[doc = "Write proxy for field `RX_PL`"]
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
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
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
#[doc = "Reader of field `RX_PP`"]
pub type RX_PP_R = crate::R<u8, RX_PP_A>;
impl RX_PP_R {
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
        *self == RX_PP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX_PP_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == RX_PP_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == RX_PP_A::_3
    }
}
#[doc = "Write proxy for field `RX_PP`"]
pub struct RX_PP_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_PP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
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
#[doc = "Reader of field `RX_MPOL`"]
pub type RX_MPOL_R = crate::R<bool, RX_MPOL_A>;
impl RX_MPOL_R {
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
        *self == RX_MPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX_MPOL_A::_1
    }
}
#[doc = "Write proxy for field `RX_MPOL`"]
pub struct RX_MPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_MPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
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
#[doc = "Reader of field `DRIFT`"]
pub type DRIFT_R = crate::R<bool, DRIFT_A>;
impl DRIFT_R {
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
        *self == DRIFT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRIFT_A::_1
    }
}
#[doc = "Write proxy for field `DRIFT`"]
pub struct DRIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIFT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRIFT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
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
}
