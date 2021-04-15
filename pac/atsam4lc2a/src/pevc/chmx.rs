#[doc = "Register `CHMX%s` reader"]
pub struct R(crate::R<CHMX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHMX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CHMX_SPEC>> for R {
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
impl core::convert::From<crate::W<CHMX_SPEC>> for W {
    fn from(writer: crate::W<CHMX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Event Multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EVMX_A {
    #[doc = "0: Event 0"]
    _0X00 = 0,
    #[doc = "1: Event 1"]
    _0X01 = 1,
}
impl From<EVMX_A> for u8 {
    #[inline(always)]
    fn from(variant: EVMX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EVMX` reader - Event Multiplexer"]
pub struct EVMX_R(crate::FieldReader<u8, EVMX_A>);
impl EVMX_R {
    pub(crate) fn new(bits: u8) -> Self {
        EVMX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EVMX_A> {
        match self.bits {
            0 => Some(EVMX_A::_0X00),
            1 => Some(EVMX_A::_0X01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        **self == EVMX_A::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        **self == EVMX_A::_0X01
    }
}
impl core::ops::Deref for EVMX_R {
    type Target = crate::FieldReader<u8, EVMX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVMX` writer - Event Multiplexer"]
pub struct EVMX_W<'a> {
    w: &'a mut W,
}
impl<'a> EVMX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVMX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Event 0"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(EVMX_A::_0X00)
    }
    #[doc = "Event 1"]
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut W {
        self.variant(EVMX_A::_0X01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Software Event Multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMX_A {
    #[doc = "0: Hardware events"]
    _0 = 0,
    #[doc = "1: Software event"]
    _1 = 1,
}
impl From<SMX_A> for bool {
    #[inline(always)]
    fn from(variant: SMX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMX` reader - Software Event Multiplexer"]
pub struct SMX_R(crate::FieldReader<bool, SMX_A>);
impl SMX_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMX_A {
        match self.bits {
            false => SMX_A::_0,
            true => SMX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMX_A::_1
    }
}
impl core::ops::Deref for SMX_R {
    type Target = crate::FieldReader<bool, SMX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMX` writer - Software Event Multiplexer"]
pub struct SMX_W<'a> {
    w: &'a mut W,
}
impl<'a> SMX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hardware events"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMX_A::_0)
    }
    #[doc = "Software event"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMX_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
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
        SMX_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Event Multiplexer"]
    #[inline(always)]
    pub fn evmx(&mut self) -> EVMX_W {
        EVMX_W { w: self }
    }
    #[doc = "Bit 8 - Software Event Multiplexer"]
    #[inline(always)]
    pub fn smx(&mut self) -> SMX_W {
        SMX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
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
}
#[doc = "`reset()` method sets CHMX%s to value 0"]
impl crate::Resettable for CHMX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
