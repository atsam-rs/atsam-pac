#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVA` reader - CLKA, CLKB Divide Factor"]
pub type DIVA_R = crate::FieldReader<u8, DIVA_A>;
#[doc = "CLKA, CLKB Divide Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DIVA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVA_A> {
        match self.bits {
            0 => Some(DIVA_A::CLK_OFF),
            1 => Some(DIVA_A::CLK_DIV1),
            _ => None,
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
#[doc = "Field `DIVA` writer - CLKA, CLKB Divide Factor"]
pub type DIVA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, DIVA_A, 8, O>;
impl<'a, const O: u8> DIVA_W<'a, O> {
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
}
#[doc = "Field `PREA` reader - "]
pub type PREA_R = crate::FieldReader<u8, PREA_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PREA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PREA_A> {
        match self.bits {
            0 => Some(PREA_A::MCK),
            1 => Some(PREA_A::MCKDIV2),
            2 => Some(PREA_A::MCKDIV4),
            3 => Some(PREA_A::MCKDIV8),
            4 => Some(PREA_A::MCKDIV16),
            5 => Some(PREA_A::MCKDIV32),
            6 => Some(PREA_A::MCKDIV64),
            7 => Some(PREA_A::MCKDIV128),
            8 => Some(PREA_A::MCKDIV256),
            9 => Some(PREA_A::MCKDIV512),
            10 => Some(PREA_A::MCKDIV1024),
            _ => None,
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
#[doc = "Field `PREA` writer - "]
pub type PREA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, PREA_A, 4, O>;
impl<'a, const O: u8> PREA_W<'a, O> {
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
}
#[doc = "Field `DIVB` reader - CLKA, CLKB Divide Factor"]
pub type DIVB_R = crate::FieldReader<u8, DIVB_A>;
#[doc = "CLKA, CLKB Divide Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DIVB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVB_A> {
        match self.bits {
            0 => Some(DIVB_A::CLK_OFF),
            1 => Some(DIVB_A::CLK_DIV1),
            _ => None,
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
#[doc = "Field `DIVB` writer - CLKA, CLKB Divide Factor"]
pub type DIVB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, DIVB_A, 8, O>;
impl<'a, const O: u8> DIVB_W<'a, O> {
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
}
#[doc = "Field `PREB` reader - "]
pub type PREB_R = crate::FieldReader<u8, PREB_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PREB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PREB_A> {
        match self.bits {
            0 => Some(PREB_A::MCK),
            1 => Some(PREB_A::MCKDIV2),
            2 => Some(PREB_A::MCKDIV4),
            3 => Some(PREB_A::MCKDIV8),
            4 => Some(PREB_A::MCKDIV16),
            5 => Some(PREB_A::MCKDIV32),
            6 => Some(PREB_A::MCKDIV64),
            7 => Some(PREB_A::MCKDIV128),
            8 => Some(PREB_A::MCKDIV256),
            9 => Some(PREB_A::MCKDIV512),
            10 => Some(PREB_A::MCKDIV1024),
            _ => None,
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
#[doc = "Field `PREB` writer - "]
pub type PREB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, PREB_A, 4, O>;
impl<'a, const O: u8> PREB_W<'a, O> {
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
    #[must_use]
    pub fn diva(&mut self) -> DIVA_W<0> {
        DIVA_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn prea(&mut self) -> PREA_W<8> {
        PREA_W::new(self)
    }
    #[doc = "Bits 16:23 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    #[must_use]
    pub fn divb(&mut self) -> DIVB_W<16> {
        DIVB_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn preb(&mut self) -> PREB_W<24> {
        PREB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
