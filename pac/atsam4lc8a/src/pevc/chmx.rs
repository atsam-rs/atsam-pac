#[doc = "Register `CHMX%s` reader"]
pub struct R(crate::R<CHMX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHMX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHMX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHMX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHMX%s` writer"]
pub struct W(crate::W<CHMX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHMX_SPEC>;
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
impl From<crate::W<CHMX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHMX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVMX` reader - Event Multiplexer"]
pub type EVMX_R = crate::FieldReader<u8, EVMXSELECT_A>;
#[doc = "Event Multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVMXSELECT_A {
    #[doc = "0: Event 0"]
    _0X00 = 0,
    #[doc = "1: Event 1"]
    _0X01 = 1,
}
impl From<EVMXSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVMXSELECT_A) -> Self {
        variant as _
    }
}
impl EVMX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EVMXSELECT_A> {
        match self.bits {
            0 => Some(EVMXSELECT_A::_0X00),
            1 => Some(EVMXSELECT_A::_0X01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == EVMXSELECT_A::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == EVMXSELECT_A::_0X01
    }
}
#[doc = "Field `EVMX` writer - Event Multiplexer"]
pub type EVMX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMX_SPEC, u8, EVMXSELECT_A, 6, O>;
impl<'a, const O: u8> EVMX_W<'a, O> {
    #[doc = "Event 0"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(EVMXSELECT_A::_0X00)
    }
    #[doc = "Event 1"]
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut W {
        self.variant(EVMXSELECT_A::_0X01)
    }
}
#[doc = "Field `SMX` reader - Software Event Multiplexer"]
pub type SMX_R = crate::BitReader<SMXSELECT_A>;
#[doc = "Software Event Multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMXSELECT_A {
    #[doc = "0: Hardware events"]
    _0 = 0,
    #[doc = "1: Software event"]
    _1 = 1,
}
impl From<SMXSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SMXSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SMX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMXSELECT_A {
        match self.bits {
            false => SMXSELECT_A::_0,
            true => SMXSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SMXSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SMXSELECT_A::_1
    }
}
#[doc = "Field `SMX` writer - Software Event Multiplexer"]
pub type SMX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHMX_SPEC, SMXSELECT_A, O>;
impl<'a, const O: u8> SMX_W<'a, O> {
    #[doc = "Hardware events"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMXSELECT_A::_0)
    }
    #[doc = "Software event"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMXSELECT_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:5 - Event Multiplexer"]
    #[inline(always)]
    pub fn evmx(&self) -> EVMX_R {
        EVMX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Software Event Multiplexer"]
    #[inline(always)]
    pub fn smx(&self) -> SMX_R {
        SMX_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Event Multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn evmx(&mut self) -> EVMX_W<0> {
        EVMX_W::new(self)
    }
    #[doc = "Bit 8 - Software Event Multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn smx(&mut self) -> SMX_W<8> {
        SMX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Multiplexer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmx](index.html) module"]
pub struct CHMX_SPEC;
impl crate::RegisterSpec for CHMX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chmx::R](R) reader structure"]
impl crate::Readable for CHMX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chmx::W](W) writer structure"]
impl crate::Writable for CHMX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHMX%s to value 0"]
impl crate::Resettable for CHMX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
