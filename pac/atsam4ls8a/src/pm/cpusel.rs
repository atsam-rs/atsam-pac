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
#[doc = "Field `CPUSEL` reader - CPU Clock Select"]
pub type CPUSEL_R = crate::FieldReader<u8, CPUSELSELECT_A>;
#[doc = "CPU Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPUSELSELECT_A {
    #[doc = "0: fCPU:fmain. CPUDIV:"]
    _0 = 0,
    #[doc = "1: fCPU:fmain / 2^(CPUSEL+1)"]
    _1 = 1,
}
impl From<CPUSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CPUSELSELECT_A) -> Self {
        variant as _
    }
}
impl CPUSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPUSELSELECT_A> {
        match self.bits {
            0 => Some(CPUSELSELECT_A::_0),
            1 => Some(CPUSELSELECT_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPUSELSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPUSELSELECT_A::_1
    }
}
#[doc = "Field `CPUSEL` writer - CPU Clock Select"]
pub type CPUSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPUSEL_SPEC, u8, CPUSELSELECT_A, 3, O>;
impl<'a, const O: u8> CPUSEL_W<'a, O> {
    #[doc = "fCPU:fmain. CPUDIV:"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPUSELSELECT_A::_0)
    }
    #[doc = "fCPU:fmain / 2^(CPUSEL+1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPUSELSELECT_A::_1)
    }
}
#[doc = "Field `CPUDIV` reader - CPU Division"]
pub type CPUDIV_R = crate::BitReader<bool>;
#[doc = "Field `CPUDIV` writer - CPU Division"]
pub type CPUDIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUSEL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - CPU Clock Select"]
    #[inline(always)]
    pub fn cpusel(&self) -> CPUSEL_R {
        CPUSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - CPU Division"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CPUDIV_R {
        CPUDIV_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CPU Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn cpusel(&mut self) -> CPUSEL_W<0> {
        CPUSEL_W::new(self)
    }
    #[doc = "Bit 7 - CPU Division"]
    #[inline(always)]
    #[must_use]
    pub fn cpudiv(&mut self) -> CPUDIV_W<7> {
        CPUDIV_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUSEL to value 0"]
impl crate::Resettable for CPUSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
