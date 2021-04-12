#[doc = "Reader of register MR"]
pub type R = crate::R<u32, super::MR>;
#[doc = "Writer for register MR"]
pub type W = crate::W<u32, super::MR>;
#[doc = "Register MR `reset()`'s with value 0"]
impl crate::ResetValue for super::MR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CLKA, CLKB Divide Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVA_A {
    #[doc = "0: CLKA, CLKB clock is turned off"]
    CLK_OFF = 0,
    #[doc = "1: CLKA, CLKB clock is clock selected by PREA, PREB"]
    CLK_DIV1 = 1,
}
impl From<DIVA_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVA`"]
pub type DIVA_R = crate::R<u8, DIVA_A>;
impl DIVA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVA_A::CLK_OFF),
            1 => Val(DIVA_A::CLK_DIV1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_OFF`"]
    #[inline(always)]
    pub fn is_clk_off(&self) -> bool {
        *self == DIVA_A::CLK_OFF
    }
    #[doc = "Checks if the value of the field is `CLK_DIV1`"]
    #[inline(always)]
    pub fn is_clk_div1(&self) -> bool {
        *self == DIVA_A::CLK_DIV1
    }
}
#[doc = "Write proxy for field `DIVA`"]
pub struct DIVA_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CLKA, CLKB clock is turned off"]
    #[inline(always)]
    pub fn clk_off(self) -> &'a mut W {
        self.variant(DIVA_A::CLK_OFF)
    }
    #[doc = "CLKA, CLKB clock is clock selected by PREA, PREB"]
    #[inline(always)]
    pub fn clk_div1(self) -> &'a mut W {
        self.variant(DIVA_A::CLK_DIV1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PREA_A {
    #[doc = "0: Master Clock"]
    MCK = 0,
    #[doc = "1: Master Clock divided by 2"]
    MCKDIV2 = 1,
    #[doc = "2: Master Clock divided by 4"]
    MCKDIV4 = 2,
    #[doc = "3: Master Clock divided by 8"]
    MCKDIV8 = 3,
    #[doc = "4: Master Clock divided by 16"]
    MCKDIV16 = 4,
    #[doc = "5: Master Clock divided by 32"]
    MCKDIV32 = 5,
    #[doc = "6: Master Clock divided by 64"]
    MCKDIV64 = 6,
    #[doc = "7: Master Clock divided by 128"]
    MCKDIV128 = 7,
    #[doc = "8: Master Clock divided by 256"]
    MCKDIV256 = 8,
    #[doc = "9: Master Clock divided by 512"]
    MCKDIV512 = 9,
    #[doc = "10: Master Clock divided by 1024"]
    MCKDIV1024 = 10,
}
impl From<PREA_A> for u8 {
    #[inline(always)]
    fn from(variant: PREA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PREA`"]
pub type PREA_R = crate::R<u8, PREA_A>;
impl PREA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PREA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PREA_A::MCK),
            1 => Val(PREA_A::MCKDIV2),
            2 => Val(PREA_A::MCKDIV4),
            3 => Val(PREA_A::MCKDIV8),
            4 => Val(PREA_A::MCKDIV16),
            5 => Val(PREA_A::MCKDIV32),
            6 => Val(PREA_A::MCKDIV64),
            7 => Val(PREA_A::MCKDIV128),
            8 => Val(PREA_A::MCKDIV256),
            9 => Val(PREA_A::MCKDIV512),
            10 => Val(PREA_A::MCKDIV1024),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == PREA_A::MCK
    }
    #[doc = "Checks if the value of the field is `MCKDIV2`"]
    #[inline(always)]
    pub fn is_mckdiv2(&self) -> bool {
        *self == PREA_A::MCKDIV2
    }
    #[doc = "Checks if the value of the field is `MCKDIV4`"]
    #[inline(always)]
    pub fn is_mckdiv4(&self) -> bool {
        *self == PREA_A::MCKDIV4
    }
    #[doc = "Checks if the value of the field is `MCKDIV8`"]
    #[inline(always)]
    pub fn is_mckdiv8(&self) -> bool {
        *self == PREA_A::MCKDIV8
    }
    #[doc = "Checks if the value of the field is `MCKDIV16`"]
    #[inline(always)]
    pub fn is_mckdiv16(&self) -> bool {
        *self == PREA_A::MCKDIV16
    }
    #[doc = "Checks if the value of the field is `MCKDIV32`"]
    #[inline(always)]
    pub fn is_mckdiv32(&self) -> bool {
        *self == PREA_A::MCKDIV32
    }
    #[doc = "Checks if the value of the field is `MCKDIV64`"]
    #[inline(always)]
    pub fn is_mckdiv64(&self) -> bool {
        *self == PREA_A::MCKDIV64
    }
    #[doc = "Checks if the value of the field is `MCKDIV128`"]
    #[inline(always)]
    pub fn is_mckdiv128(&self) -> bool {
        *self == PREA_A::MCKDIV128
    }
    #[doc = "Checks if the value of the field is `MCKDIV256`"]
    #[inline(always)]
    pub fn is_mckdiv256(&self) -> bool {
        *self == PREA_A::MCKDIV256
    }
    #[doc = "Checks if the value of the field is `MCKDIV512`"]
    #[inline(always)]
    pub fn is_mckdiv512(&self) -> bool {
        *self == PREA_A::MCKDIV512
    }
    #[doc = "Checks if the value of the field is `MCKDIV1024`"]
    #[inline(always)]
    pub fn is_mckdiv1024(&self) -> bool {
        *self == PREA_A::MCKDIV1024
    }
}
#[doc = "Write proxy for field `PREA`"]
pub struct PREA_W<'a> {
    w: &'a mut W,
}
impl<'a> PREA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Master Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(PREA_A::MCK)
    }
    #[doc = "Master Clock divided by 2"]
    #[inline(always)]
    pub fn mckdiv2(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV2)
    }
    #[doc = "Master Clock divided by 4"]
    #[inline(always)]
    pub fn mckdiv4(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV4)
    }
    #[doc = "Master Clock divided by 8"]
    #[inline(always)]
    pub fn mckdiv8(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV8)
    }
    #[doc = "Master Clock divided by 16"]
    #[inline(always)]
    pub fn mckdiv16(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV16)
    }
    #[doc = "Master Clock divided by 32"]
    #[inline(always)]
    pub fn mckdiv32(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV32)
    }
    #[doc = "Master Clock divided by 64"]
    #[inline(always)]
    pub fn mckdiv64(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV64)
    }
    #[doc = "Master Clock divided by 128"]
    #[inline(always)]
    pub fn mckdiv128(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV128)
    }
    #[doc = "Master Clock divided by 256"]
    #[inline(always)]
    pub fn mckdiv256(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV256)
    }
    #[doc = "Master Clock divided by 512"]
    #[inline(always)]
    pub fn mckdiv512(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV512)
    }
    #[doc = "Master Clock divided by 1024"]
    #[inline(always)]
    pub fn mckdiv1024(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "CLKA, CLKB Divide Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVB_A {
    #[doc = "0: CLKA, CLKB clock is turned off"]
    CLK_OFF = 0,
    #[doc = "1: CLKA, CLKB clock is clock selected by PREA, PREB"]
    CLK_DIV1 = 1,
}
impl From<DIVB_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVB`"]
pub type DIVB_R = crate::R<u8, DIVB_A>;
impl DIVB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVB_A::CLK_OFF),
            1 => Val(DIVB_A::CLK_DIV1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_OFF`"]
    #[inline(always)]
    pub fn is_clk_off(&self) -> bool {
        *self == DIVB_A::CLK_OFF
    }
    #[doc = "Checks if the value of the field is `CLK_DIV1`"]
    #[inline(always)]
    pub fn is_clk_div1(&self) -> bool {
        *self == DIVB_A::CLK_DIV1
    }
}
#[doc = "Write proxy for field `DIVB`"]
pub struct DIVB_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CLKA, CLKB clock is turned off"]
    #[inline(always)]
    pub fn clk_off(self) -> &'a mut W {
        self.variant(DIVB_A::CLK_OFF)
    }
    #[doc = "CLKA, CLKB clock is clock selected by PREA, PREB"]
    #[inline(always)]
    pub fn clk_div1(self) -> &'a mut W {
        self.variant(DIVB_A::CLK_DIV1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PREB_A {
    #[doc = "0: Master Clock"]
    MCK = 0,
    #[doc = "1: Master Clock divided by 2"]
    MCKDIV2 = 1,
    #[doc = "2: Master Clock divided by 4"]
    MCKDIV4 = 2,
    #[doc = "3: Master Clock divided by 8"]
    MCKDIV8 = 3,
    #[doc = "4: Master Clock divided by 16"]
    MCKDIV16 = 4,
    #[doc = "5: Master Clock divided by 32"]
    MCKDIV32 = 5,
    #[doc = "6: Master Clock divided by 64"]
    MCKDIV64 = 6,
    #[doc = "7: Master Clock divided by 128"]
    MCKDIV128 = 7,
    #[doc = "8: Master Clock divided by 256"]
    MCKDIV256 = 8,
    #[doc = "9: Master Clock divided by 512"]
    MCKDIV512 = 9,
    #[doc = "10: Master Clock divided by 1024"]
    MCKDIV1024 = 10,
}
impl From<PREB_A> for u8 {
    #[inline(always)]
    fn from(variant: PREB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PREB`"]
pub type PREB_R = crate::R<u8, PREB_A>;
impl PREB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PREB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PREB_A::MCK),
            1 => Val(PREB_A::MCKDIV2),
            2 => Val(PREB_A::MCKDIV4),
            3 => Val(PREB_A::MCKDIV8),
            4 => Val(PREB_A::MCKDIV16),
            5 => Val(PREB_A::MCKDIV32),
            6 => Val(PREB_A::MCKDIV64),
            7 => Val(PREB_A::MCKDIV128),
            8 => Val(PREB_A::MCKDIV256),
            9 => Val(PREB_A::MCKDIV512),
            10 => Val(PREB_A::MCKDIV1024),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == PREB_A::MCK
    }
    #[doc = "Checks if the value of the field is `MCKDIV2`"]
    #[inline(always)]
    pub fn is_mckdiv2(&self) -> bool {
        *self == PREB_A::MCKDIV2
    }
    #[doc = "Checks if the value of the field is `MCKDIV4`"]
    #[inline(always)]
    pub fn is_mckdiv4(&self) -> bool {
        *self == PREB_A::MCKDIV4
    }
    #[doc = "Checks if the value of the field is `MCKDIV8`"]
    #[inline(always)]
    pub fn is_mckdiv8(&self) -> bool {
        *self == PREB_A::MCKDIV8
    }
    #[doc = "Checks if the value of the field is `MCKDIV16`"]
    #[inline(always)]
    pub fn is_mckdiv16(&self) -> bool {
        *self == PREB_A::MCKDIV16
    }
    #[doc = "Checks if the value of the field is `MCKDIV32`"]
    #[inline(always)]
    pub fn is_mckdiv32(&self) -> bool {
        *self == PREB_A::MCKDIV32
    }
    #[doc = "Checks if the value of the field is `MCKDIV64`"]
    #[inline(always)]
    pub fn is_mckdiv64(&self) -> bool {
        *self == PREB_A::MCKDIV64
    }
    #[doc = "Checks if the value of the field is `MCKDIV128`"]
    #[inline(always)]
    pub fn is_mckdiv128(&self) -> bool {
        *self == PREB_A::MCKDIV128
    }
    #[doc = "Checks if the value of the field is `MCKDIV256`"]
    #[inline(always)]
    pub fn is_mckdiv256(&self) -> bool {
        *self == PREB_A::MCKDIV256
    }
    #[doc = "Checks if the value of the field is `MCKDIV512`"]
    #[inline(always)]
    pub fn is_mckdiv512(&self) -> bool {
        *self == PREB_A::MCKDIV512
    }
    #[doc = "Checks if the value of the field is `MCKDIV1024`"]
    #[inline(always)]
    pub fn is_mckdiv1024(&self) -> bool {
        *self == PREB_A::MCKDIV1024
    }
}
#[doc = "Write proxy for field `PREB`"]
pub struct PREB_W<'a> {
    w: &'a mut W,
}
impl<'a> PREB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Master Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(PREB_A::MCK)
    }
    #[doc = "Master Clock divided by 2"]
    #[inline(always)]
    pub fn mckdiv2(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV2)
    }
    #[doc = "Master Clock divided by 4"]
    #[inline(always)]
    pub fn mckdiv4(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV4)
    }
    #[doc = "Master Clock divided by 8"]
    #[inline(always)]
    pub fn mckdiv8(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV8)
    }
    #[doc = "Master Clock divided by 16"]
    #[inline(always)]
    pub fn mckdiv16(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV16)
    }
    #[doc = "Master Clock divided by 32"]
    #[inline(always)]
    pub fn mckdiv32(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV32)
    }
    #[doc = "Master Clock divided by 64"]
    #[inline(always)]
    pub fn mckdiv64(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV64)
    }
    #[doc = "Master Clock divided by 128"]
    #[inline(always)]
    pub fn mckdiv128(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV128)
    }
    #[doc = "Master Clock divided by 256"]
    #[inline(always)]
    pub fn mckdiv256(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV256)
    }
    #[doc = "Master Clock divided by 512"]
    #[inline(always)]
    pub fn mckdiv512(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV512)
    }
    #[doc = "Master Clock divided by 1024"]
    #[inline(always)]
    pub fn mckdiv1024(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn prea(&self) -> PREA_R {
        PREA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn divb(&self) -> DIVB_R {
        DIVB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn preb(&self) -> PREB_R {
        PREB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn diva(&mut self) -> DIVA_W {
        DIVA_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn prea(&mut self) -> PREA_W {
        PREA_W { w: self }
    }
    #[doc = "Bits 16:23 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn divb(&mut self) -> DIVB_W {
        DIVB_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn preb(&mut self) -> PREB_W {
        PREB_W { w: self }
    }
}
