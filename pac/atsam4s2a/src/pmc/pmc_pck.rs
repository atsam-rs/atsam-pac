#[doc = "Register `PMC_PCK[%s]` reader"]
pub struct R(crate::R<PMC_PCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_PCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_PCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_PCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC_PCK[%s]` writer"]
pub struct W(crate::W<PMC_PCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_PCK_SPEC>;
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
impl From<crate::W<PMC_PCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_PCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Master Clock Source Selection"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSS_A {
    #[doc = "0: Slow Clock is selected"]
    SLOW_CLK = 0,
    #[doc = "1: Main Clock is selected"]
    MAIN_CLK = 1,
    #[doc = "2: PLLA Clock is selected"]
    PLLA_CLK = 2,
    #[doc = "3: PLLB Clock is selected"]
    PLLB_CLK = 3,
    #[doc = "4: Master Clock is selected"]
    MCK = 4,
}
impl From<CSS_A> for u8 {
    #[inline(always)]
    fn from(variant: CSS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CSS` reader - Master Clock Source Selection"]
pub struct CSS_R(crate::FieldReader<u8, CSS_A>);
impl CSS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSS_A> {
        match self.bits {
            0 => Some(CSS_A::SLOW_CLK),
            1 => Some(CSS_A::MAIN_CLK),
            2 => Some(CSS_A::PLLA_CLK),
            3 => Some(CSS_A::PLLB_CLK),
            4 => Some(CSS_A::MCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SLOW_CLK`"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        **self == CSS_A::SLOW_CLK
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        **self == CSS_A::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `PLLA_CLK`"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        **self == CSS_A::PLLA_CLK
    }
    #[doc = "Checks if the value of the field is `PLLB_CLK`"]
    #[inline(always)]
    pub fn is_pllb_clk(&self) -> bool {
        **self == CSS_A::PLLB_CLK
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        **self == CSS_A::MCK
    }
}
impl core::ops::Deref for CSS_R {
    type Target = crate::FieldReader<u8, CSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSS` writer - Master Clock Source Selection"]
pub struct CSS_W<'a> {
    w: &'a mut W,
}
impl<'a> CSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Slow Clock is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut W {
        self.variant(CSS_A::SLOW_CLK)
    }
    #[doc = "Main Clock is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(CSS_A::MAIN_CLK)
    }
    #[doc = "PLLA Clock is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut W {
        self.variant(CSS_A::PLLA_CLK)
    }
    #[doc = "PLLB Clock is selected"]
    #[inline(always)]
    pub fn pllb_clk(self) -> &'a mut W {
        self.variant(CSS_A::PLLB_CLK)
    }
    #[doc = "Master Clock is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(CSS_A::MCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Programmable Clock Prescaler"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRES_A {
    #[doc = "0: Selected clock"]
    CLK_1 = 0,
    #[doc = "1: Selected clock divided by 2"]
    CLK_2 = 1,
    #[doc = "2: Selected clock divided by 4"]
    CLK_4 = 2,
    #[doc = "3: Selected clock divided by 8"]
    CLK_8 = 3,
    #[doc = "4: Selected clock divided by 16"]
    CLK_16 = 4,
    #[doc = "5: Selected clock divided by 32"]
    CLK_32 = 5,
    #[doc = "6: Selected clock divided by 64"]
    CLK_64 = 6,
}
impl From<PRES_A> for u8 {
    #[inline(always)]
    fn from(variant: PRES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRES` reader - Programmable Clock Prescaler"]
pub struct PRES_R(crate::FieldReader<u8, PRES_A>);
impl PRES_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRES_A> {
        match self.bits {
            0 => Some(PRES_A::CLK_1),
            1 => Some(PRES_A::CLK_2),
            2 => Some(PRES_A::CLK_4),
            3 => Some(PRES_A::CLK_8),
            4 => Some(PRES_A::CLK_16),
            5 => Some(PRES_A::CLK_32),
            6 => Some(PRES_A::CLK_64),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_1`"]
    #[inline(always)]
    pub fn is_clk_1(&self) -> bool {
        **self == PRES_A::CLK_1
    }
    #[doc = "Checks if the value of the field is `CLK_2`"]
    #[inline(always)]
    pub fn is_clk_2(&self) -> bool {
        **self == PRES_A::CLK_2
    }
    #[doc = "Checks if the value of the field is `CLK_4`"]
    #[inline(always)]
    pub fn is_clk_4(&self) -> bool {
        **self == PRES_A::CLK_4
    }
    #[doc = "Checks if the value of the field is `CLK_8`"]
    #[inline(always)]
    pub fn is_clk_8(&self) -> bool {
        **self == PRES_A::CLK_8
    }
    #[doc = "Checks if the value of the field is `CLK_16`"]
    #[inline(always)]
    pub fn is_clk_16(&self) -> bool {
        **self == PRES_A::CLK_16
    }
    #[doc = "Checks if the value of the field is `CLK_32`"]
    #[inline(always)]
    pub fn is_clk_32(&self) -> bool {
        **self == PRES_A::CLK_32
    }
    #[doc = "Checks if the value of the field is `CLK_64`"]
    #[inline(always)]
    pub fn is_clk_64(&self) -> bool {
        **self == PRES_A::CLK_64
    }
}
impl core::ops::Deref for PRES_R {
    type Target = crate::FieldReader<u8, PRES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRES` writer - Programmable Clock Prescaler"]
pub struct PRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRES_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selected clock"]
    #[inline(always)]
    pub fn clk_1(self) -> &'a mut W {
        self.variant(PRES_A::CLK_1)
    }
    #[doc = "Selected clock divided by 2"]
    #[inline(always)]
    pub fn clk_2(self) -> &'a mut W {
        self.variant(PRES_A::CLK_2)
    }
    #[doc = "Selected clock divided by 4"]
    #[inline(always)]
    pub fn clk_4(self) -> &'a mut W {
        self.variant(PRES_A::CLK_4)
    }
    #[doc = "Selected clock divided by 8"]
    #[inline(always)]
    pub fn clk_8(self) -> &'a mut W {
        self.variant(PRES_A::CLK_8)
    }
    #[doc = "Selected clock divided by 16"]
    #[inline(always)]
    pub fn clk_16(self) -> &'a mut W {
        self.variant(PRES_A::CLK_16)
    }
    #[doc = "Selected clock divided by 32"]
    #[inline(always)]
    pub fn clk_32(self) -> &'a mut W {
        self.variant(PRES_A::CLK_32)
    }
    #[doc = "Selected clock divided by 64"]
    #[inline(always)]
    pub fn clk_64(self) -> &'a mut W {
        self.variant(PRES_A::CLK_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Programmable Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&mut self) -> CSS_W {
        CSS_W { w: self }
    }
    #[doc = "Bits 4:6 - Programmable Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&mut self) -> PRES_W {
        PRES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Programmable Clock 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pck](index.html) module"]
pub struct PMC_PCK_SPEC;
impl crate::RegisterSpec for PMC_PCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_pck::R](R) reader structure"]
impl crate::Readable for PMC_PCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_pck::W](W) writer structure"]
impl crate::Writable for PMC_PCK_SPEC {
    type Writer = W;
}
