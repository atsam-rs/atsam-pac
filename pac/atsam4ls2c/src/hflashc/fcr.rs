#[doc = "Reader of register FCR"]
pub type R = crate::R<u32, super::FCR>;
#[doc = "Writer for register FCR"]
pub type W = crate::W<u32, super::FCR>;
#[doc = "Register FCR `reset()`'s with value 0"]
impl crate::ResetValue for super::FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Flash Ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRDY_A {
    #[doc = "0: Flash Ready does not generate an interrupt"]
    _0 = 0,
    #[doc = "1: Flash Ready generates an interrupt"]
    _1 = 1,
}
impl From<FRDY_A> for bool {
    #[inline(always)]
    fn from(variant: FRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRDY`"]
pub type FRDY_R = crate::R<bool, FRDY_A>;
impl FRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRDY_A {
        match self.bits {
            false => FRDY_A::_0,
            true => FRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRDY_A::_1
    }
}
#[doc = "Write proxy for field `FRDY`"]
pub struct FRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> FRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash Ready does not generate an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRDY_A::_0)
    }
    #[doc = "Flash Ready generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRDY_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Lock Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKE_A {
    #[doc = "0: Lock Error does not generate an interrupt"]
    _0 = 0,
    #[doc = "1: Lock Error generates an interrupt"]
    _1 = 1,
}
impl From<LOCKE_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCKE`"]
pub type LOCKE_R = crate::R<bool, LOCKE_A>;
impl LOCKE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKE_A {
        match self.bits {
            false => LOCKE_A::_0,
            true => LOCKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOCKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOCKE_A::_1
    }
}
#[doc = "Write proxy for field `LOCKE`"]
pub struct LOCKE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lock Error does not generate an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCKE_A::_0)
    }
    #[doc = "Lock Error generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCKE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Programming Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROGE_A {
    #[doc = "0: Programming Error does not generate an interrupt"]
    _0 = 0,
    #[doc = "1: Programming Error generates an interrupt"]
    _1 = 1,
}
impl From<PROGE_A> for bool {
    #[inline(always)]
    fn from(variant: PROGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PROGE`"]
pub type PROGE_R = crate::R<bool, PROGE_A>;
impl PROGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROGE_A {
        match self.bits {
            false => PROGE_A::_0,
            true => PROGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PROGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PROGE_A::_1
    }
}
#[doc = "Write proxy for field `PROGE`"]
pub struct PROGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PROGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Programming Error does not generate an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROGE_A::_0)
    }
    #[doc = "Programming Error generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROGE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Flash Wait State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWS_A {
    #[doc = "0: The flash is read with 0 wait states"]
    _0 = 0,
    #[doc = "1: The flash is read with 1 wait states"]
    _1 = 1,
}
impl From<FWS_A> for bool {
    #[inline(always)]
    fn from(variant: FWS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FWS`"]
pub type FWS_R = crate::R<bool, FWS_A>;
impl FWS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWS_A {
        match self.bits {
            false => FWS_A::_0,
            true => FWS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FWS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FWS_A::_1
    }
}
#[doc = "Write proxy for field `FWS`"]
pub struct FWS_W<'a> {
    w: &'a mut W,
}
impl<'a> FWS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The flash is read with 0 wait states"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FWS_A::_0)
    }
    #[doc = "The flash is read with 1 wait states"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FWS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `WS1OPT`"]
pub type WS1OPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS1OPT`"]
pub struct WS1OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> WS1OPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Lock Error Interrupt Enable"]
    #[inline(always)]
    pub fn locke(&self) -> LOCKE_R {
        LOCKE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Programming Error Interrupt Enable"]
    #[inline(always)]
    pub fn proge(&self) -> PROGE_R {
        PROGE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Flash Wait State"]
    #[inline(always)]
    pub fn fws(&self) -> FWS_R {
        FWS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wait State 1 Optimization"]
    #[inline(always)]
    pub fn ws1opt(&self) -> WS1OPT_R {
        WS1OPT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdy(&mut self) -> FRDY_W {
        FRDY_W { w: self }
    }
    #[doc = "Bit 2 - Lock Error Interrupt Enable"]
    #[inline(always)]
    pub fn locke(&mut self) -> LOCKE_W {
        LOCKE_W { w: self }
    }
    #[doc = "Bit 3 - Programming Error Interrupt Enable"]
    #[inline(always)]
    pub fn proge(&mut self) -> PROGE_W {
        PROGE_W { w: self }
    }
    #[doc = "Bit 6 - Flash Wait State"]
    #[inline(always)]
    pub fn fws(&mut self) -> FWS_W {
        FWS_W { w: self }
    }
    #[doc = "Bit 7 - Wait State 1 Optimization"]
    #[inline(always)]
    pub fn ws1opt(&mut self) -> WS1OPT_W {
        WS1OPT_W { w: self }
    }
}
