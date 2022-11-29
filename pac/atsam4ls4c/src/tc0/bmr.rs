#[doc = "Register `BMR` reader"]
pub struct R(crate::R<BMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMR` writer"]
pub struct W(crate::W<BMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMR_SPEC>;
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
impl From<crate::W<BMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TC0XC0S` reader - External Clock Signal 0 Selection"]
pub type TC0XC0S_R = crate::FieldReader<u8, TC0XC0SSELECT_A>;
#[doc = "External Clock Signal 0 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TC0XC0SSELECT_A {
    #[doc = "0: Select TCLK0 as clock signal 0."]
    TCLK0 = 0,
    #[doc = "1: Select no clock as clock signal 0."]
    NO_CLK = 1,
    #[doc = "2: Select TIOA1 as clock signal 0."]
    TIOA1 = 2,
    #[doc = "3: Select TIOA2 as clock signal 0."]
    TIOA2 = 3,
}
impl From<TC0XC0SSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TC0XC0SSELECT_A) -> Self {
        variant as _
    }
}
impl TC0XC0S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC0XC0SSELECT_A {
        match self.bits {
            0 => TC0XC0SSELECT_A::TCLK0,
            1 => TC0XC0SSELECT_A::NO_CLK,
            2 => TC0XC0SSELECT_A::TIOA1,
            3 => TC0XC0SSELECT_A::TIOA2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TCLK0`"]
    #[inline(always)]
    pub fn is_tclk0(&self) -> bool {
        *self == TC0XC0SSELECT_A::TCLK0
    }
    #[doc = "Checks if the value of the field is `NO_CLK`"]
    #[inline(always)]
    pub fn is_no_clk(&self) -> bool {
        *self == TC0XC0SSELECT_A::NO_CLK
    }
    #[doc = "Checks if the value of the field is `TIOA1`"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        *self == TC0XC0SSELECT_A::TIOA1
    }
    #[doc = "Checks if the value of the field is `TIOA2`"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        *self == TC0XC0SSELECT_A::TIOA2
    }
}
#[doc = "Field `TC0XC0S` writer - External Clock Signal 0 Selection"]
pub type TC0XC0S_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BMR_SPEC, u8, TC0XC0SSELECT_A, 2, O>;
impl<'a, const O: u8> TC0XC0S_W<'a, O> {
    #[doc = "Select TCLK0 as clock signal 0."]
    #[inline(always)]
    pub fn tclk0(self) -> &'a mut W {
        self.variant(TC0XC0SSELECT_A::TCLK0)
    }
    #[doc = "Select no clock as clock signal 0."]
    #[inline(always)]
    pub fn no_clk(self) -> &'a mut W {
        self.variant(TC0XC0SSELECT_A::NO_CLK)
    }
    #[doc = "Select TIOA1 as clock signal 0."]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC0XC0SSELECT_A::TIOA1)
    }
    #[doc = "Select TIOA2 as clock signal 0."]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC0XC0SSELECT_A::TIOA2)
    }
}
#[doc = "Field `TC1XC1S` reader - External Clock Signal 1 Selection"]
pub type TC1XC1S_R = crate::FieldReader<u8, TC1XC1SSELECT_A>;
#[doc = "External Clock Signal 1 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TC1XC1SSELECT_A {
    #[doc = "0: Select TCLK1 as clock signal 1."]
    TCLK1 = 0,
    #[doc = "1: Select no clock as clock signal 1."]
    NO_CLK = 1,
    #[doc = "2: Select TIOA0 as clock signal 1."]
    TIOA0 = 2,
    #[doc = "3: Select TIOA2 as clock signal 1."]
    TIOA2 = 3,
}
impl From<TC1XC1SSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TC1XC1SSELECT_A) -> Self {
        variant as _
    }
}
impl TC1XC1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC1XC1SSELECT_A {
        match self.bits {
            0 => TC1XC1SSELECT_A::TCLK1,
            1 => TC1XC1SSELECT_A::NO_CLK,
            2 => TC1XC1SSELECT_A::TIOA0,
            3 => TC1XC1SSELECT_A::TIOA2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TCLK1`"]
    #[inline(always)]
    pub fn is_tclk1(&self) -> bool {
        *self == TC1XC1SSELECT_A::TCLK1
    }
    #[doc = "Checks if the value of the field is `NO_CLK`"]
    #[inline(always)]
    pub fn is_no_clk(&self) -> bool {
        *self == TC1XC1SSELECT_A::NO_CLK
    }
    #[doc = "Checks if the value of the field is `TIOA0`"]
    #[inline(always)]
    pub fn is_tioa0(&self) -> bool {
        *self == TC1XC1SSELECT_A::TIOA0
    }
    #[doc = "Checks if the value of the field is `TIOA2`"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        *self == TC1XC1SSELECT_A::TIOA2
    }
}
#[doc = "Field `TC1XC1S` writer - External Clock Signal 1 Selection"]
pub type TC1XC1S_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BMR_SPEC, u8, TC1XC1SSELECT_A, 2, O>;
impl<'a, const O: u8> TC1XC1S_W<'a, O> {
    #[doc = "Select TCLK1 as clock signal 1."]
    #[inline(always)]
    pub fn tclk1(self) -> &'a mut W {
        self.variant(TC1XC1SSELECT_A::TCLK1)
    }
    #[doc = "Select no clock as clock signal 1."]
    #[inline(always)]
    pub fn no_clk(self) -> &'a mut W {
        self.variant(TC1XC1SSELECT_A::NO_CLK)
    }
    #[doc = "Select TIOA0 as clock signal 1."]
    #[inline(always)]
    pub fn tioa0(self) -> &'a mut W {
        self.variant(TC1XC1SSELECT_A::TIOA0)
    }
    #[doc = "Select TIOA2 as clock signal 1."]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC1XC1SSELECT_A::TIOA2)
    }
}
#[doc = "Field `TC2XC2S` reader - External Clock Signal 2 Selection"]
pub type TC2XC2S_R = crate::FieldReader<u8, TC2XC2SSELECT_A>;
#[doc = "External Clock Signal 2 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TC2XC2SSELECT_A {
    #[doc = "0: Select TCLK2 as clock signal 2."]
    TCLK2 = 0,
    #[doc = "1: Select no clock as clock signal 2."]
    NO_CLK = 1,
    #[doc = "2: Select TIOA0 as clock signal 2."]
    TIOA0 = 2,
    #[doc = "3: Select TIOA1 as clock signal 2."]
    TIOA1 = 3,
}
impl From<TC2XC2SSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TC2XC2SSELECT_A) -> Self {
        variant as _
    }
}
impl TC2XC2S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC2XC2SSELECT_A {
        match self.bits {
            0 => TC2XC2SSELECT_A::TCLK2,
            1 => TC2XC2SSELECT_A::NO_CLK,
            2 => TC2XC2SSELECT_A::TIOA0,
            3 => TC2XC2SSELECT_A::TIOA1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TCLK2`"]
    #[inline(always)]
    pub fn is_tclk2(&self) -> bool {
        *self == TC2XC2SSELECT_A::TCLK2
    }
    #[doc = "Checks if the value of the field is `NO_CLK`"]
    #[inline(always)]
    pub fn is_no_clk(&self) -> bool {
        *self == TC2XC2SSELECT_A::NO_CLK
    }
    #[doc = "Checks if the value of the field is `TIOA0`"]
    #[inline(always)]
    pub fn is_tioa0(&self) -> bool {
        *self == TC2XC2SSELECT_A::TIOA0
    }
    #[doc = "Checks if the value of the field is `TIOA1`"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        *self == TC2XC2SSELECT_A::TIOA1
    }
}
#[doc = "Field `TC2XC2S` writer - External Clock Signal 2 Selection"]
pub type TC2XC2S_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BMR_SPEC, u8, TC2XC2SSELECT_A, 2, O>;
impl<'a, const O: u8> TC2XC2S_W<'a, O> {
    #[doc = "Select TCLK2 as clock signal 2."]
    #[inline(always)]
    pub fn tclk2(self) -> &'a mut W {
        self.variant(TC2XC2SSELECT_A::TCLK2)
    }
    #[doc = "Select no clock as clock signal 2."]
    #[inline(always)]
    pub fn no_clk(self) -> &'a mut W {
        self.variant(TC2XC2SSELECT_A::NO_CLK)
    }
    #[doc = "Select TIOA0 as clock signal 2."]
    #[inline(always)]
    pub fn tioa0(self) -> &'a mut W {
        self.variant(TC2XC2SSELECT_A::TIOA0)
    }
    #[doc = "Select TIOA1 as clock signal 2."]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC2XC2SSELECT_A::TIOA1)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    pub fn tc0xc0s(&self) -> TC0XC0S_R {
        TC0XC0S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    pub fn tc1xc1s(&self) -> TC1XC1S_R {
        TC1XC1S_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    pub fn tc2xc2s(&self) -> TC2XC2S_R {
        TC2XC2S_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tc0xc0s(&mut self) -> TC0XC0S_W<0> {
        TC0XC0S_W::new(self)
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tc1xc1s(&mut self) -> TC1XC1S_W<2> {
        TC1XC1S_W::new(self)
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tc2xc2s(&mut self) -> TC2XC2S_W<4> {
        TC2XC2S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TC Block Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmr](index.html) module"]
pub struct BMR_SPEC;
impl crate::RegisterSpec for BMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmr::R](R) reader structure"]
impl crate::Readable for BMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmr::W](W) writer structure"]
impl crate::Writable for BMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BMR to value 0"]
impl crate::Resettable for BMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
