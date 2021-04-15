#[doc = "Register `FWRUNPS` reader"]
pub struct R(crate::R<FWRUNPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWRUNPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FWRUNPS_SPEC>> for R {
    fn from(reader: crate::R<FWRUNPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REGLEVEL` reader - Regulator Voltage Level"]
pub struct REGLEVEL_R(crate::FieldReader<u8, u8>);
impl REGLEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REGLEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGLEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Field `REGTYPE` reader - Regulator Type"]
pub struct REGTYPE_R(crate::FieldReader<u8, REGTYPE_A>);
impl REGTYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REGTYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REGTYPE_A> {
        match self.bits {
            0 => Some(REGTYPE_A::NORMAL),
            1 => Some(REGTYPE_A::LP),
            2 => Some(REGTYPE_A::XULP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == REGTYPE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        **self == REGTYPE_A::LP
    }
    #[doc = "Checks if the value of the field is `XULP`"]
    #[inline(always)]
    pub fn is_xulp(&self) -> bool {
        **self == REGTYPE_A::XULP
    }
}
impl core::ops::Deref for REGTYPE_R {
    type Target = crate::FieldReader<u8, REGTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `REFTYPE` reader - Reference Type"]
pub struct REFTYPE_R(crate::FieldReader<u8, REFTYPE_A>);
impl REFTYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REFTYPE_R(crate::FieldReader::new(bits))
    }
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
        **self == REFTYPE_A::BOTH
    }
    #[doc = "Checks if the value of the field is `BG`"]
    #[inline(always)]
    pub fn is_bg(&self) -> bool {
        **self == REFTYPE_A::BG
    }
    #[doc = "Checks if the value of the field is `LPBG`"]
    #[inline(always)]
    pub fn is_lpbg(&self) -> bool {
        **self == REFTYPE_A::LPBG
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        **self == REFTYPE_A::INTERNAL
    }
}
impl core::ops::Deref for REFTYPE_R {
    type Target = crate::FieldReader<u8, REFTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHLATDEL` reader - Flash Latch Delay Value"]
pub struct FLASHLATDEL_R(crate::FieldReader<u8, u8>);
impl FLASHLATDEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLASHLATDEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASHLATDEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHBIAS` reader - Flash Bias Value"]
pub struct FLASHBIAS_R(crate::FieldReader<u8, u8>);
impl FLASHBIAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLASHBIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASHBIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPPW` reader - Flash Pico Power Mode"]
pub struct FPPW_R(crate::FieldReader<bool, bool>);
impl FPPW_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPPW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPPW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RC115` reader - RC 115KHZ Calibration Value"]
pub struct RC115_R(crate::FieldReader<u8, u8>);
impl RC115_R {
    pub(crate) fn new(bits: u8) -> Self {
        RC115_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC115_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCFAST` reader - RCFAST Calibration Value"]
pub struct RCFAST_R(crate::FieldReader<u8, u8>);
impl RCFAST_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCFAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCFAST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Factory Word Run PS Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwrunps](index.html) module"]
pub struct FWRUNPS_SPEC;
impl crate::RegisterSpec for FWRUNPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fwrunps::R](R) reader structure"]
impl crate::Readable for FWRUNPS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FWRUNPS to value 0"]
impl crate::Resettable for FWRUNPS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
