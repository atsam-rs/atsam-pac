#[doc = "Reader of register BMR"]
pub type R = crate::R<u32, super::BMR>;
#[doc = "Writer for register BMR"]
pub type W = crate::W<u32, super::BMR>;
#[doc = "Register BMR `reset()`'s with value 0"]
impl crate::ResetValue for super::BMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External Clock Signal 0 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TC0XC0S_A {
    #[doc = "0: Select TCLK0 as clock signal 0."]
    TCLK0 = 0,
    #[doc = "1: Select no clock as clock signal 0."]
    NO_CLK = 1,
    #[doc = "2: Select TIOA1 as clock signal 0."]
    TIOA1 = 2,
    #[doc = "3: Select TIOA2 as clock signal 0."]
    TIOA2 = 3,
}
impl From<TC0XC0S_A> for u8 {
    #[inline(always)]
    fn from(variant: TC0XC0S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TC0XC0S`"]
pub type TC0XC0S_R = crate::R<u8, TC0XC0S_A>;
impl TC0XC0S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC0XC0S_A {
        match self.bits {
            0 => TC0XC0S_A::TCLK0,
            1 => TC0XC0S_A::NO_CLK,
            2 => TC0XC0S_A::TIOA1,
            3 => TC0XC0S_A::TIOA2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TCLK0`"]
    #[inline(always)]
    pub fn is_tclk0(&self) -> bool {
        *self == TC0XC0S_A::TCLK0
    }
    #[doc = "Checks if the value of the field is `NO_CLK`"]
    #[inline(always)]
    pub fn is_no_clk(&self) -> bool {
        *self == TC0XC0S_A::NO_CLK
    }
    #[doc = "Checks if the value of the field is `TIOA1`"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        *self == TC0XC0S_A::TIOA1
    }
    #[doc = "Checks if the value of the field is `TIOA2`"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        *self == TC0XC0S_A::TIOA2
    }
}
#[doc = "Write proxy for field `TC0XC0S`"]
pub struct TC0XC0S_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0XC0S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC0XC0S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Select TCLK0 as clock signal 0."]
    #[inline(always)]
    pub fn tclk0(self) -> &'a mut W {
        self.variant(TC0XC0S_A::TCLK0)
    }
    #[doc = "Select no clock as clock signal 0."]
    #[inline(always)]
    pub fn no_clk(self) -> &'a mut W {
        self.variant(TC0XC0S_A::NO_CLK)
    }
    #[doc = "Select TIOA1 as clock signal 0."]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC0XC0S_A::TIOA1)
    }
    #[doc = "Select TIOA2 as clock signal 0."]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC0XC0S_A::TIOA2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "External Clock Signal 1 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TC1XC1S_A {
    #[doc = "0: Select TCLK1 as clock signal 1."]
    TCLK1 = 0,
    #[doc = "1: Select no clock as clock signal 1."]
    NO_CLK = 1,
    #[doc = "2: Select TIOA0 as clock signal 1."]
    TIOA0 = 2,
    #[doc = "3: Select TIOA2 as clock signal 1."]
    TIOA2 = 3,
}
impl From<TC1XC1S_A> for u8 {
    #[inline(always)]
    fn from(variant: TC1XC1S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TC1XC1S`"]
pub type TC1XC1S_R = crate::R<u8, TC1XC1S_A>;
impl TC1XC1S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC1XC1S_A {
        match self.bits {
            0 => TC1XC1S_A::TCLK1,
            1 => TC1XC1S_A::NO_CLK,
            2 => TC1XC1S_A::TIOA0,
            3 => TC1XC1S_A::TIOA2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TCLK1`"]
    #[inline(always)]
    pub fn is_tclk1(&self) -> bool {
        *self == TC1XC1S_A::TCLK1
    }
    #[doc = "Checks if the value of the field is `NO_CLK`"]
    #[inline(always)]
    pub fn is_no_clk(&self) -> bool {
        *self == TC1XC1S_A::NO_CLK
    }
    #[doc = "Checks if the value of the field is `TIOA0`"]
    #[inline(always)]
    pub fn is_tioa0(&self) -> bool {
        *self == TC1XC1S_A::TIOA0
    }
    #[doc = "Checks if the value of the field is `TIOA2`"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        *self == TC1XC1S_A::TIOA2
    }
}
#[doc = "Write proxy for field `TC1XC1S`"]
pub struct TC1XC1S_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1XC1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC1XC1S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Select TCLK1 as clock signal 1."]
    #[inline(always)]
    pub fn tclk1(self) -> &'a mut W {
        self.variant(TC1XC1S_A::TCLK1)
    }
    #[doc = "Select no clock as clock signal 1."]
    #[inline(always)]
    pub fn no_clk(self) -> &'a mut W {
        self.variant(TC1XC1S_A::NO_CLK)
    }
    #[doc = "Select TIOA0 as clock signal 1."]
    #[inline(always)]
    pub fn tioa0(self) -> &'a mut W {
        self.variant(TC1XC1S_A::TIOA0)
    }
    #[doc = "Select TIOA2 as clock signal 1."]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC1XC1S_A::TIOA2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "External Clock Signal 2 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TC2XC2S_A {
    #[doc = "0: Select TCLK2 as clock signal 2."]
    TCLK2 = 0,
    #[doc = "1: Select no clock as clock signal 2."]
    NO_CLK = 1,
    #[doc = "2: Select TIOA0 as clock signal 2."]
    TIOA0 = 2,
    #[doc = "3: Select TIOA1 as clock signal 2."]
    TIOA1 = 3,
}
impl From<TC2XC2S_A> for u8 {
    #[inline(always)]
    fn from(variant: TC2XC2S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TC2XC2S`"]
pub type TC2XC2S_R = crate::R<u8, TC2XC2S_A>;
impl TC2XC2S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC2XC2S_A {
        match self.bits {
            0 => TC2XC2S_A::TCLK2,
            1 => TC2XC2S_A::NO_CLK,
            2 => TC2XC2S_A::TIOA0,
            3 => TC2XC2S_A::TIOA1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TCLK2`"]
    #[inline(always)]
    pub fn is_tclk2(&self) -> bool {
        *self == TC2XC2S_A::TCLK2
    }
    #[doc = "Checks if the value of the field is `NO_CLK`"]
    #[inline(always)]
    pub fn is_no_clk(&self) -> bool {
        *self == TC2XC2S_A::NO_CLK
    }
    #[doc = "Checks if the value of the field is `TIOA0`"]
    #[inline(always)]
    pub fn is_tioa0(&self) -> bool {
        *self == TC2XC2S_A::TIOA0
    }
    #[doc = "Checks if the value of the field is `TIOA1`"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        *self == TC2XC2S_A::TIOA1
    }
}
#[doc = "Write proxy for field `TC2XC2S`"]
pub struct TC2XC2S_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2XC2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC2XC2S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Select TCLK2 as clock signal 2."]
    #[inline(always)]
    pub fn tclk2(self) -> &'a mut W {
        self.variant(TC2XC2S_A::TCLK2)
    }
    #[doc = "Select no clock as clock signal 2."]
    #[inline(always)]
    pub fn no_clk(self) -> &'a mut W {
        self.variant(TC2XC2S_A::NO_CLK)
    }
    #[doc = "Select TIOA0 as clock signal 2."]
    #[inline(always)]
    pub fn tioa0(self) -> &'a mut W {
        self.variant(TC2XC2S_A::TIOA0)
    }
    #[doc = "Select TIOA1 as clock signal 2."]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC2XC2S_A::TIOA1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    pub fn tc0xc0s(&self) -> TC0XC0S_R {
        TC0XC0S_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    pub fn tc1xc1s(&self) -> TC1XC1S_R {
        TC1XC1S_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    pub fn tc2xc2s(&self) -> TC2XC2S_R {
        TC2XC2S_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    pub fn tc0xc0s(&mut self) -> TC0XC0S_W {
        TC0XC0S_W { w: self }
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    pub fn tc1xc1s(&mut self) -> TC1XC1S_W {
        TC1XC1S_W { w: self }
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    pub fn tc2xc2s(&mut self) -> TC2XC2S_W {
        TC2XC2S_W { w: self }
    }
}
