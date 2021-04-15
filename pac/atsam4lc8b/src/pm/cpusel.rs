#[doc = "Reader of register CPUSEL"]
pub type R = crate::R<u32, super::CPUSEL>;
#[doc = "Writer for register CPUSEL"]
pub type W = crate::W<u32, super::CPUSEL>;
#[doc = "Register CPUSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::CPUSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CPU Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPUSEL_A {
    #[doc = "0: fCPU:fmain. CPUDIV:"]
    _0 = 0,
    #[doc = "1: fCPU:fmain / 2^(CPUSEL+1)"]
    _1 = 1,
}
impl From<CPUSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CPUSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CPUSEL`"]
pub type CPUSEL_R = crate::R<u8, CPUSEL_A>;
impl CPUSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CPUSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CPUSEL_A::_0),
            1 => Val(CPUSEL_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPUSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPUSEL_A::_1
    }
}
#[doc = "Write proxy for field `CPUSEL`"]
pub struct CPUSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPUSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "fCPU:fmain. CPUDIV:"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPUSEL_A::_0)
    }
    #[doc = "fCPU:fmain / 2^(CPUSEL+1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPUSEL_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `CPUDIV`"]
pub type CPUDIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPUDIV`"]
pub struct CPUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUDIV_W<'a> {
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
    #[doc = "Bits 0:2 - CPU Clock Select"]
    #[inline(always)]
    pub fn cpusel(&self) -> CPUSEL_R {
        CPUSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 7 - CPU Division"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CPUDIV_R {
        CPUDIV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CPU Clock Select"]
    #[inline(always)]
    pub fn cpusel(&mut self) -> CPUSEL_W {
        CPUSEL_W { w: self }
    }
    #[doc = "Bit 7 - CPU Division"]
    #[inline(always)]
    pub fn cpudiv(&mut self) -> CPUDIV_W {
        CPUDIV_W { w: self }
    }
}
