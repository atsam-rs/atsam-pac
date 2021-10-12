#[doc = "Register `CPUSEL` reader"]
pub struct R(crate::R<CPUSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUSEL` writer"]
pub struct W(crate::W<CPUSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUSEL_SPEC>;
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
impl From<crate::W<CPUSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CPU Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPUSEL_A {
    #[doc = "0: fCPU:fmain. CPUDIV:"]
    _0 = 0,
    #[doc = "1: fCPU:fmain / 2^(CPUSEL+1)"]
    _1 = 1,
}
impl From<CPUSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CPUSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CPUSEL` reader - CPU Clock Select"]
pub struct CPUSEL_R(crate::FieldReader<u8, CPUSEL_A>);
impl CPUSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPUSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPUSEL_A> {
        match self.bits {
            0 => Some(CPUSEL_A::_0),
            1 => Some(CPUSEL_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CPUSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPUSEL_A::_1
    }
}
impl core::ops::Deref for CPUSEL_R {
    type Target = crate::FieldReader<u8, CPUSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPUSEL` writer - CPU Clock Select"]
pub struct CPUSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPUSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "fCPU:fmain. CPUDIV:"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPUSEL_A::_0)
    }
    #[doc = "fCPU:fmain / 2^(CPUSEL+1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPUSEL_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `CPUDIV` reader - CPU Division"]
pub struct CPUDIV_R(crate::FieldReader<bool, bool>);
impl CPUDIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPUDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPUDIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPUDIV` writer - CPU Division"]
pub struct CPUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUDIV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - CPU Clock Select"]
    #[inline(always)]
    pub fn cpusel(&self) -> CPUSEL_R {
        CPUSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 7 - CPU Division"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CPUDIV_R {
        CPUDIV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CPU Clock Select"]
    #[inline(always)]
    pub fn cpusel(&mut self) -> CPUSEL_W {
        CPUSEL_W { w: self }
    }
    #[doc = "Bit 7 - CPU Division"]
    #[inline(always)]
    pub fn cpudiv(&mut self) -> CPUDIV_W {
        CPUDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpusel](index.html) module"]
pub struct CPUSEL_SPEC;
impl crate::RegisterSpec for CPUSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpusel::R](R) reader structure"]
impl crate::Readable for CPUSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpusel::W](W) writer structure"]
impl crate::Writable for CPUSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPUSEL to value 0"]
impl crate::Resettable for CPUSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
