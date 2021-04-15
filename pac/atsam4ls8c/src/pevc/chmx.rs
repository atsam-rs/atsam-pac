#[doc = "Reader of register CHMX%s"]
pub type R = crate::R<u32, super::CHMX>;
#[doc = "Writer for register CHMX%s"]
pub type W = crate::W<u32, super::CHMX>;
#[doc = "Register CHMX%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CHMX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Event Multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EVMX_A {
    #[doc = "0: Event 0"]
    _0X00 = 0,
    #[doc = "1: Event 1"]
    _0X01 = 1,
}
impl From<EVMX_A> for u8 {
    #[inline(always)]
    fn from(variant: EVMX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EVMX`"]
pub type EVMX_R = crate::R<u8, EVMX_A>;
impl EVMX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EVMX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EVMX_A::_0X00),
            1 => Val(EVMX_A::_0X01),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == EVMX_A::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == EVMX_A::_0X01
    }
}
#[doc = "Write proxy for field `EVMX`"]
pub struct EVMX_W<'a> {
    w: &'a mut W,
}
impl<'a> EVMX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVMX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Event 0"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(EVMX_A::_0X00)
    }
    #[doc = "Event 1"]
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut W {
        self.variant(EVMX_A::_0X01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Software Event Multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMX_A {
    #[doc = "0: Hardware events"]
    _0 = 0,
    #[doc = "1: Software event"]
    _1 = 1,
}
impl From<SMX_A> for bool {
    #[inline(always)]
    fn from(variant: SMX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMX`"]
pub type SMX_R = crate::R<bool, SMX_A>;
impl SMX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMX_A {
        match self.bits {
            false => SMX_A::_0,
            true => SMX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SMX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SMX_A::_1
    }
}
#[doc = "Write proxy for field `SMX`"]
pub struct SMX_W<'a> {
    w: &'a mut W,
}
impl<'a> SMX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware events"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMX_A::_0)
    }
    #[doc = "Software event"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMX_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Event Multiplexer"]
    #[inline(always)]
    pub fn evmx(&self) -> EVMX_R {
        EVMX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Software Event Multiplexer"]
    #[inline(always)]
    pub fn smx(&self) -> SMX_R {
        SMX_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Event Multiplexer"]
    #[inline(always)]
    pub fn evmx(&mut self) -> EVMX_W {
        EVMX_W { w: self }
    }
    #[doc = "Bit 8 - Software Event Multiplexer"]
    #[inline(always)]
    pub fn smx(&mut self) -> SMX_W {
        SMX_W { w: self }
    }
}
