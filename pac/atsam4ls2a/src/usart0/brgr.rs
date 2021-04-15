#[doc = "Reader of register BRGR"]
pub type R = crate::R<u32, super::BRGR>;
#[doc = "Writer for register BRGR"]
pub type W = crate::W<u32, super::BRGR>;
#[doc = "Register BRGR `reset()`'s with value 0"]
impl crate::ResetValue for super::BRGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Divisor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CD_A {
    #[doc = "0: Disables the clock"]
    DISABLE = 0,
    #[doc = "1: Clock Divisor Bypass"]
    BYPASS = 1,
    #[doc = "2: Baud Rate (Asynchronous Mode) = Selected Clock/(16 x CD) or (8 x CD); Baud Rate (Synchronous Mode) = Selected Clock/CD;"]
    _2 = 2,
}
impl From<CD_A> for u16 {
    #[inline(always)]
    fn from(variant: CD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CD`"]
pub type CD_R = crate::R<u16, CD_A>;
impl CD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, CD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CD_A::DISABLE),
            1 => Val(CD_A::BYPASS),
            2 => Val(CD_A::_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == CD_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == CD_A::_2
    }
}
#[doc = "Write proxy for field `CD`"]
pub struct CD_W<'a> {
    w: &'a mut W,
}
impl<'a> CD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disables the clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CD_A::DISABLE)
    }
    #[doc = "Clock Divisor Bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(CD_A::BYPASS)
    }
    #[doc = "Baud Rate (Asynchronous Mode) = Selected Clock/(16 x CD) or (8 x CD); Baud Rate (Synchronous Mode) = Selected Clock/CD;"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CD_A::_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Fractional Part\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FP_A {
    #[doc = "0: Fractional divider is disabled"]
    _0 = 0,
}
impl From<FP_A> for u8 {
    #[inline(always)]
    fn from(variant: FP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FP`"]
pub type FP_R = crate::R<u8, FP_A>;
impl FP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FP_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FP_A::_0
    }
}
#[doc = "Write proxy for field `FP`"]
pub struct FP_W<'a> {
    w: &'a mut W,
}
impl<'a> FP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Fractional divider is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FP_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&self) -> FP_R {
        FP_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    pub fn cd(&mut self) -> CD_W {
        CD_W { w: self }
    }
    #[doc = "Bits 16:18 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&mut self) -> FP_W {
        FP_W { w: self }
    }
}
