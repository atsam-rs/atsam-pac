#[doc = "Register `BRGR` reader"]
pub struct R(crate::R<BRGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BRGR_SPEC>> for R {
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
impl core::convert::From<crate::W<BRGR_SPEC>> for W {
    fn from(writer: crate::W<BRGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Divisor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CD_A {
    #[doc = "0: Disables the clock"]
    DISABLE = 0,
    #[doc = "1: Clock Divisor Bypass"]
    BYPASS = 1,
    #[doc = "2: Baud Rate (Asynchronous Mode) = Selected Clock/(16 x CD) or (8 x CD); Baud Rate (Synchronous Mode) = Selected Clock/CD;"]
    _2 = 2,
}
impl From<CD_A> for u16 {
    #[inline(always)]
    fn from(variant: CD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CD` reader - Clock Divisor"]
pub struct CD_R(crate::FieldReader<u16, CD_A>);
impl CD_R {
    pub(crate) fn new(bits: u16) -> Self {
        CD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CD_A> {
        match self.bits {
            0 => Some(CD_A::DISABLE),
            1 => Some(CD_A::BYPASS),
            2 => Some(CD_A::_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        **self == CD_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CD_A::_2
    }
}
impl core::ops::Deref for CD_R {
    type Target = crate::FieldReader<u16, CD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CD` writer - Clock Divisor"]
pub struct CD_W<'a> {
    w: &'a mut W,
}
impl<'a> CD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disables the clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CD_A::DISABLE)
    }
    #[doc = "Clock Divisor Bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(CD_A::BYPASS)
    }
    #[doc = "Baud Rate (Asynchronous Mode) = Selected Clock/(16 x CD) or (8 x CD); Baud Rate (Synchronous Mode) = Selected Clock/CD;"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CD_A::_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Fractional Part\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FP_A {
    #[doc = "0: Fractional divider is disabled"]
    _0 = 0,
}
impl From<FP_A> for u8 {
    #[inline(always)]
    fn from(variant: FP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FP` reader - Fractional Part"]
pub struct FP_R(crate::FieldReader<u8, FP_A>);
impl FP_R {
    pub(crate) fn new(bits: u8) -> Self {
        FP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FP_A> {
        match self.bits {
            0 => Some(FP_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FP_A::_0
    }
}
impl core::ops::Deref for FP_R {
    type Target = crate::FieldReader<u8, FP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FP` writer - Fractional Part"]
pub struct FP_W<'a> {
    w: &'a mut W,
}
impl<'a> FP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Fractional divider is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FP_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
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
        FP_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    pub fn cd(&mut self) -> CD_W {
        CD_W { w: self }
    }
    #[doc = "Bits 16:18 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&mut self) -> FP_W {
        FP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
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
}
#[doc = "`reset()` method sets BRGR to value 0"]
impl crate::Resettable for BRGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
