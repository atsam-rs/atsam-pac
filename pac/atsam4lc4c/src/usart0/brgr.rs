#[doc = "Register `BRGR` reader"]
pub struct R(crate::R<BRGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRGR` writer"]
pub struct W(crate::W<BRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRGR_SPEC>;
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
impl From<crate::W<BRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CD` reader - Clock Divisor"]
pub type CD_R = crate::FieldReader<u16, CDSELECT_A>;
#[doc = "Clock Divisor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum CDSELECT_A {
    #[doc = "0: Disables the clock"]
    DISABLE = 0,
    #[doc = "1: Clock Divisor Bypass"]
    BYPASS = 1,
    #[doc = "2: Baud Rate (Asynchronous Mode) = Selected Clock/(16 x CD) or (8 x CD); Baud Rate (Synchronous Mode) = Selected Clock/CD;"]
    _2 = 2,
}
impl From<CDSELECT_A> for u16 {
    #[inline(always)]
    fn from(variant: CDSELECT_A) -> Self {
        variant as _
    }
}
impl CD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CDSELECT_A> {
        match self.bits {
            0 => Some(CDSELECT_A::DISABLE),
            1 => Some(CDSELECT_A::BYPASS),
            2 => Some(CDSELECT_A::_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CDSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == CDSELECT_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == CDSELECT_A::_2
    }
}
#[doc = "Field `CD` writer - Clock Divisor"]
pub type CD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BRGR_SPEC, u16, CDSELECT_A, 16, O>;
impl<'a, const O: u8> CD_W<'a, O> {
    #[doc = "Disables the clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CDSELECT_A::DISABLE)
    }
    #[doc = "Clock Divisor Bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(CDSELECT_A::BYPASS)
    }
    #[doc = "Baud Rate (Asynchronous Mode) = Selected Clock/(16 x CD) or (8 x CD); Baud Rate (Synchronous Mode) = Selected Clock/CD;"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CDSELECT_A::_2)
    }
}
#[doc = "Field `FP` reader - Fractional Part"]
pub type FP_R = crate::FieldReader<u8, FPSELECT_A>;
#[doc = "Fractional Part\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FPSELECT_A {
    #[doc = "0: Fractional divider is disabled"]
    _0 = 0,
}
impl From<FPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FPSELECT_A) -> Self {
        variant as _
    }
}
impl FP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FPSELECT_A> {
        match self.bits {
            0 => Some(FPSELECT_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FPSELECT_A::_0
    }
}
#[doc = "Field `FP` writer - Fractional Part"]
pub type FP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BRGR_SPEC, u8, FPSELECT_A, 3, O>;
impl<'a, const O: u8> FP_W<'a, O> {
    #[doc = "Fractional divider is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPSELECT_A::_0)
    }
}
impl R {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&self) -> FP_R {
        FP_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    #[must_use]
    pub fn cd(&mut self) -> CD_W<0> {
        CD_W::new(self)
    }
    #[doc = "Bits 16:18 - Fractional Part"]
    #[inline(always)]
    #[must_use]
    pub fn fp(&mut self) -> FP_W<16> {
        FP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud Rate Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brgr](index.html) module"]
pub struct BRGR_SPEC;
impl crate::RegisterSpec for BRGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brgr::R](R) reader structure"]
impl crate::Readable for BRGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brgr::W](W) writer structure"]
impl crate::Writable for BRGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRGR to value 0"]
impl crate::Resettable for BRGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
