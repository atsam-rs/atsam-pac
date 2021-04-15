#[doc = "Reader of register CLOCK"]
pub type R = crate::R<u32, super::CLOCK>;
#[doc = "Writer for register CLOCK"]
pub type W = crate::W<u32, super::CLOCK>;
#[doc = "Register CLOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::CLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEN_A {
    #[doc = "0: The clock is disabled"]
    _0 = 0,
    #[doc = "1: The clock is enabled"]
    _1 = 1,
}
impl From<CEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEN`"]
pub type CEN_R = crate::R<bool, CEN_A>;
impl CEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEN_A {
        match self.bits {
            false => CEN_A::_0,
            true => CEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CEN_A::_1
    }
}
#[doc = "Write proxy for field `CEN`"]
pub struct CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The clock is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CEN_A::_0)
    }
    #[doc = "The clock is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CEN_A::_1)
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
#[doc = "Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSSEL_A {
    #[doc = "0: Slow clock"]
    SLOWCLOCK = 0,
    #[doc = "1: 32 kHz clock"]
    _32KHZCLK = 1,
    #[doc = "2: PB clock"]
    PBCLOCK = 2,
    #[doc = "3: Generic clock"]
    GCLK = 3,
    #[doc = "4: 1kHz clock from 32 kHz oscillator"]
    _1KHZCLK = 4,
}
impl From<CSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CSSEL`"]
pub type CSSEL_R = crate::R<u8, CSSEL_A>;
impl CSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CSSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CSSEL_A::SLOWCLOCK),
            1 => Val(CSSEL_A::_32KHZCLK),
            2 => Val(CSSEL_A::PBCLOCK),
            3 => Val(CSSEL_A::GCLK),
            4 => Val(CSSEL_A::_1KHZCLK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLOWCLOCK`"]
    #[inline(always)]
    pub fn is_slowclock(&self) -> bool {
        *self == CSSEL_A::SLOWCLOCK
    }
    #[doc = "Checks if the value of the field is `_32KHZCLK`"]
    #[inline(always)]
    pub fn is_32khzclk(&self) -> bool {
        *self == CSSEL_A::_32KHZCLK
    }
    #[doc = "Checks if the value of the field is `PBCLOCK`"]
    #[inline(always)]
    pub fn is_pbclock(&self) -> bool {
        *self == CSSEL_A::PBCLOCK
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == CSSEL_A::GCLK
    }
    #[doc = "Checks if the value of the field is `_1KHZCLK`"]
    #[inline(always)]
    pub fn is_1khzclk(&self) -> bool {
        *self == CSSEL_A::_1KHZCLK
    }
}
#[doc = "Write proxy for field `CSSEL`"]
pub struct CSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Slow clock"]
    #[inline(always)]
    pub fn slowclock(self) -> &'a mut W {
        self.variant(CSSEL_A::SLOWCLOCK)
    }
    #[doc = "32 kHz clock"]
    #[inline(always)]
    pub fn _32khzclk(self) -> &'a mut W {
        self.variant(CSSEL_A::_32KHZCLK)
    }
    #[doc = "PB clock"]
    #[inline(always)]
    pub fn pbclock(self) -> &'a mut W {
        self.variant(CSSEL_A::PBCLOCK)
    }
    #[doc = "Generic clock"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut W {
        self.variant(CSSEL_A::GCLK)
    }
    #[doc = "1kHz clock from 32 kHz oscillator"]
    #[inline(always)]
    pub fn _1khzclk(self) -> &'a mut W {
        self.variant(CSSEL_A::_1KHZCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Clock Source Selection"]
    #[inline(always)]
    pub fn cssel(&self) -> CSSEL_R {
        CSSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W { w: self }
    }
    #[doc = "Bits 8:10 - Clock Source Selection"]
    #[inline(always)]
    pub fn cssel(&mut self) -> CSSEL_W {
        CSSEL_W { w: self }
    }
}
