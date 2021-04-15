#[doc = "Register `CLOCK` reader"]
pub struct R(crate::R<CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLOCK_SPEC>> for R {
    fn from(reader: crate::R<CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK` writer"]
pub struct W(crate::W<CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_SPEC>;
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
impl core::convert::From<crate::W<CLOCK_SPEC>> for W {
    fn from(writer: crate::W<CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEN_A {
    #[doc = "0: The clock is disabled"]
    _0 = 0,
    #[doc = "1: The clock is enabled"]
    _1 = 1,
}
impl From<CEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN` reader - Clock Enable"]
pub struct CEN_R(crate::FieldReader<bool, CEN_A>);
impl CEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEN_A {
        match self.bits {
            false => CEN_A::_0,
            true => CEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CEN_A::_1
    }
}
impl core::ops::Deref for CEN_R {
    type Target = crate::FieldReader<bool, CEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEN` writer - Clock Enable"]
pub struct CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CEN_A::_0)
    }
    #[doc = "The clock is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CEN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSSEL_A {
    #[doc = "0: Slow clock"]
    SLOWCLOCK = 0,
    #[doc = "1: 32 kHz clock"]
    _32KHZCLK = 1,
    #[doc = "2: PB clock"]
    PBCLOCK = 2,
    #[doc = "3: Generic clock"]
    GCLK = 3,
    #[doc = "4: 1kHz clock from 32 kHz oscillator"]
    _1KHZCLK = 4,
}
impl From<CSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CSSEL` reader - Clock Source Selection"]
pub struct CSSEL_R(crate::FieldReader<u8, CSSEL_A>);
impl CSSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSSEL_A> {
        match self.bits {
            0 => Some(CSSEL_A::SLOWCLOCK),
            1 => Some(CSSEL_A::_32KHZCLK),
            2 => Some(CSSEL_A::PBCLOCK),
            3 => Some(CSSEL_A::GCLK),
            4 => Some(CSSEL_A::_1KHZCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SLOWCLOCK`"]
    #[inline(always)]
    pub fn is_slowclock(&self) -> bool {
        **self == CSSEL_A::SLOWCLOCK
    }
    #[doc = "Checks if the value of the field is `_32KHZCLK`"]
    #[inline(always)]
    pub fn is_32khzclk(&self) -> bool {
        **self == CSSEL_A::_32KHZCLK
    }
    #[doc = "Checks if the value of the field is `PBCLOCK`"]
    #[inline(always)]
    pub fn is_pbclock(&self) -> bool {
        **self == CSSEL_A::PBCLOCK
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        **self == CSSEL_A::GCLK
    }
    #[doc = "Checks if the value of the field is `_1KHZCLK`"]
    #[inline(always)]
    pub fn is_1khzclk(&self) -> bool {
        **self == CSSEL_A::_1KHZCLK
    }
}
impl core::ops::Deref for CSSEL_R {
    type Target = crate::FieldReader<u8, CSSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSEL` writer - Clock Source Selection"]
pub struct CSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Slow clock"]
    #[inline(always)]
    pub fn slowclock(self) -> &'a mut W {
        self.variant(CSSEL_A::SLOWCLOCK)
    }
    #[doc = "32 kHz clock"]
    #[inline(always)]
    pub fn _32khzclk(self) -> &'a mut W {
        self.variant(CSSEL_A::_32KHZCLK)
    }
    #[doc = "PB clock"]
    #[inline(always)]
    pub fn pbclock(self) -> &'a mut W {
        self.variant(CSSEL_A::PBCLOCK)
    }
    #[doc = "Generic clock"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut W {
        self.variant(CSSEL_A::GCLK)
    }
    #[doc = "1kHz clock from 32 kHz oscillator"]
    #[inline(always)]
    pub fn _1khzclk(self) -> &'a mut W {
        self.variant(CSSEL_A::_1KHZCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Clock Source Selection"]
    #[inline(always)]
    pub fn cssel(&self) -> CSSEL_R {
        CSSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W { w: self }
    }
    #[doc = "Bits 8:10 - Clock Source Selection"]
    #[inline(always)]
    pub fn cssel(&mut self) -> CSSEL_W {
        CSSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock](index.html) module"]
pub struct CLOCK_SPEC;
impl crate::RegisterSpec for CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock::R](R) reader structure"]
impl crate::Readable for CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock::W](W) writer structure"]
impl crate::Writable for CLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCK to value 0"]
impl crate::Resettable for CLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
