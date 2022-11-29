#[doc = "Register `CLOCK` reader"]
pub struct R(crate::R<CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_SPEC>> for R {
    #[inline(always)]
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
impl From<crate::W<CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEN` reader - Clock Enable"]
pub type CEN_R = crate::BitReader<CENSELECT_A>;
#[doc = "Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CENSELECT_A {
    #[doc = "0: The clock is disabled"]
    _0 = 0,
    #[doc = "1: The clock is enabled"]
    _1 = 1,
}
impl From<CENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CENSELECT_A {
        match self.bits {
            false => CENSELECT_A::_0,
            true => CENSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CENSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CENSELECT_A::_1
    }
}
#[doc = "Field `CEN` writer - Clock Enable"]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCK_SPEC, CENSELECT_A, O>;
impl<'a, const O: u8> CEN_W<'a, O> {
    #[doc = "The clock is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CENSELECT_A::_0)
    }
    #[doc = "The clock is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CENSELECT_A::_1)
    }
}
#[doc = "Field `CSSEL` reader - Clock Source Selection"]
pub type CSSEL_R = crate::FieldReader<u8, CSSELSELECT_A>;
#[doc = "Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSSELSELECT_A {
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
impl From<CSSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CSSELSELECT_A) -> Self {
        variant as _
    }
}
impl CSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSSELSELECT_A> {
        match self.bits {
            0 => Some(CSSELSELECT_A::SLOWCLOCK),
            1 => Some(CSSELSELECT_A::_32KHZCLK),
            2 => Some(CSSELSELECT_A::PBCLOCK),
            3 => Some(CSSELSELECT_A::GCLK),
            4 => Some(CSSELSELECT_A::_1KHZCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SLOWCLOCK`"]
    #[inline(always)]
    pub fn is_slowclock(&self) -> bool {
        *self == CSSELSELECT_A::SLOWCLOCK
    }
    #[doc = "Checks if the value of the field is `_32KHZCLK`"]
    #[inline(always)]
    pub fn is_32khzclk(&self) -> bool {
        *self == CSSELSELECT_A::_32KHZCLK
    }
    #[doc = "Checks if the value of the field is `PBCLOCK`"]
    #[inline(always)]
    pub fn is_pbclock(&self) -> bool {
        *self == CSSELSELECT_A::PBCLOCK
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == CSSELSELECT_A::GCLK
    }
    #[doc = "Checks if the value of the field is `_1KHZCLK`"]
    #[inline(always)]
    pub fn is_1khzclk(&self) -> bool {
        *self == CSSELSELECT_A::_1KHZCLK
    }
}
#[doc = "Field `CSSEL` writer - Clock Source Selection"]
pub type CSSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLOCK_SPEC, u8, CSSELSELECT_A, 3, O>;
impl<'a, const O: u8> CSSEL_W<'a, O> {
    #[doc = "Slow clock"]
    #[inline(always)]
    pub fn slowclock(self) -> &'a mut W {
        self.variant(CSSELSELECT_A::SLOWCLOCK)
    }
    #[doc = "32 kHz clock"]
    #[inline(always)]
    pub fn _32khzclk(self) -> &'a mut W {
        self.variant(CSSELSELECT_A::_32KHZCLK)
    }
    #[doc = "PB clock"]
    #[inline(always)]
    pub fn pbclock(self) -> &'a mut W {
        self.variant(CSSELSELECT_A::PBCLOCK)
    }
    #[doc = "Generic clock"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut W {
        self.variant(CSSELSELECT_A::GCLK)
    }
    #[doc = "1kHz clock from 32 kHz oscillator"]
    #[inline(always)]
    pub fn _1khzclk(self) -> &'a mut W {
        self.variant(CSSELSELECT_A::_1KHZCLK)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - Clock Source Selection"]
    #[inline(always)]
    pub fn cssel(&self) -> CSSEL_R {
        CSSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<0> {
        CEN_W::new(self)
    }
    #[doc = "Bits 8:10 - Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cssel(&mut self) -> CSSEL_W<8> {
        CSSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLOCK to value 0"]
impl crate::Resettable for CLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
