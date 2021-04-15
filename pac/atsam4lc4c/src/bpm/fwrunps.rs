#[doc = "Reader of register FWRUNPS"]
pub type R = crate::R<u32, super::FWRUNPS>;
#[doc = "Reader of field `REGLEVEL`"]
pub type REGLEVEL_R = crate::R<u8, u8>;
#[doc = "Regulator Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REGTYPE_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    LP = 1,
    #[doc = "2: `10`"]
    XULP = 2,
}
impl From<REGTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: REGTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REGTYPE`"]
pub type REGTYPE_R = crate::R<u8, REGTYPE_A>;
impl REGTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REGTYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REGTYPE_A::NORMAL),
            1 => Val(REGTYPE_A::LP),
            2 => Val(REGTYPE_A::XULP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == REGTYPE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == REGTYPE_A::LP
    }
    #[doc = "Checks if the value of the field is `XULP`"]
    #[inline(always)]
    pub fn is_xulp(&self) -> bool {
        *self == REGTYPE_A::XULP
    }
}
#[doc = "Reference Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFTYPE_A {
    #[doc = "0: `0`"]
    BOTH = 0,
    #[doc = "1: `1`"]
    BG = 1,
    #[doc = "2: `10`"]
    LPBG = 2,
    #[doc = "3: `11`"]
    INTERNAL = 3,
}
impl From<REFTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: REFTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REFTYPE`"]
pub type REFTYPE_R = crate::R<u8, REFTYPE_A>;
impl REFTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFTYPE_A {
        match self.bits {
            0 => REFTYPE_A::BOTH,
            1 => REFTYPE_A::BG,
            2 => REFTYPE_A::LPBG,
            3 => REFTYPE_A::INTERNAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == REFTYPE_A::BOTH
    }
    #[doc = "Checks if the value of the field is `BG`"]
    #[inline(always)]
    pub fn is_bg(&self) -> bool {
        *self == REFTYPE_A::BG
    }
    #[doc = "Checks if the value of the field is `LPBG`"]
    #[inline(always)]
    pub fn is_lpbg(&self) -> bool {
        *self == REFTYPE_A::LPBG
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == REFTYPE_A::INTERNAL
    }
}
#[doc = "Reader of field `FLASHLATDEL`"]
pub type FLASHLATDEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `FLASHBIAS`"]
pub type FLASHBIAS_R = crate::R<u8, u8>;
#[doc = "Reader of field `FPPW`"]
pub type FPPW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RC115`"]
pub type RC115_R = crate::R<u8, u8>;
#[doc = "Reader of field `RCFAST`"]
pub type RCFAST_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Regulator Voltage Level"]
    #[inline(always)]
    pub fn reglevel(&self) -> REGLEVEL_R {
        REGLEVEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Regulator Type"]
    #[inline(always)]
    pub fn regtype(&self) -> REGTYPE_R {
        REGTYPE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Reference Type"]
    #[inline(always)]
    pub fn reftype(&self) -> REFTYPE_R {
        REFTYPE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:12 - Flash Latch Delay Value"]
    #[inline(always)]
    pub fn flashlatdel(&self) -> FLASHLATDEL_R {
        FLASHLATDEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:16 - Flash Bias Value"]
    #[inline(always)]
    pub fn flashbias(&self) -> FLASHBIAS_R {
        FLASHBIAS_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - Flash Pico Power Mode"]
    #[inline(always)]
    pub fn fppw(&self) -> FPPW_R {
        FPPW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:24 - RC 115KHZ Calibration Value"]
    #[inline(always)]
    pub fn rc115(&self) -> RC115_R {
        RC115_R::new(((self.bits >> 18) & 0x7f) as u8)
    }
    #[doc = "Bits 25:31 - RCFAST Calibration Value"]
    #[inline(always)]
    pub fn rcfast(&self) -> RCFAST_R {
        RCFAST_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
