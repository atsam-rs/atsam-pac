#[doc = "Reader of register MCFG"]
pub type R = crate::R<u32, super::MCFG>;
#[doc = "Writer for register MCFG"]
pub type W = crate::W<u32, super::MCFG>;
#[doc = "Register MCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::MCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Cache Controller Monitor Counter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Cycle Counter"]
    CYCLE = 0,
    #[doc = "1: Instruction Hit Counter"]
    IHIT = 1,
    #[doc = "2: Data Hit Counter"]
    DHIT = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::CYCLE),
            1 => Val(MODE_A::IHIT),
            2 => Val(MODE_A::DHIT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLE`"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == MODE_A::CYCLE
    }
    #[doc = "Checks if the value of the field is `IHIT`"]
    #[inline(always)]
    pub fn is_ihit(&self) -> bool {
        *self == MODE_A::IHIT
    }
    #[doc = "Checks if the value of the field is `DHIT`"]
    #[inline(always)]
    pub fn is_dhit(&self) -> bool {
        *self == MODE_A::DHIT
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Cycle Counter"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut W {
        self.variant(MODE_A::CYCLE)
    }
    #[doc = "Instruction Hit Counter"]
    #[inline(always)]
    pub fn ihit(self) -> &'a mut W {
        self.variant(MODE_A::IHIT)
    }
    #[doc = "Data Hit Counter"]
    #[inline(always)]
    pub fn dhit(self) -> &'a mut W {
        self.variant(MODE_A::DHIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Cache Controller Monitor Counter Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Cache Controller Monitor Counter Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
