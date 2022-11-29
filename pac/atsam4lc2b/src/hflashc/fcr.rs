#[doc = "Register `FCR` reader"]
pub struct R(crate::R<FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRDY` reader - Flash Ready Interrupt Enable"]
pub type FRDY_R = crate::BitReader<FRDYSELECT_A>;
#[doc = "Flash Ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRDYSELECT_A {
    #[doc = "0: Flash Ready does not generate an interrupt"]
    _0 = 0,
    #[doc = "1: Flash Ready generates an interrupt"]
    _1 = 1,
}
impl From<FRDYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: FRDYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl FRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRDYSELECT_A {
        match self.bits {
            false => FRDYSELECT_A::_0,
            true => FRDYSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRDYSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRDYSELECT_A::_1
    }
}
#[doc = "Field `FRDY` writer - Flash Ready Interrupt Enable"]
pub type FRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, FRDYSELECT_A, O>;
impl<'a, const O: u8> FRDY_W<'a, O> {
    #[doc = "Flash Ready does not generate an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRDYSELECT_A::_0)
    }
    #[doc = "Flash Ready generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRDYSELECT_A::_1)
    }
}
#[doc = "Field `LOCKE` reader - Lock Error Interrupt Enable"]
pub type LOCKE_R = crate::BitReader<LOCKESELECT_A>;
#[doc = "Lock Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKESELECT_A {
    #[doc = "0: Lock Error does not generate an interrupt"]
    _0 = 0,
    #[doc = "1: Lock Error generates an interrupt"]
    _1 = 1,
}
impl From<LOCKESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKESELECT_A {
        match self.bits {
            false => LOCKESELECT_A::_0,
            true => LOCKESELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOCKESELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOCKESELECT_A::_1
    }
}
#[doc = "Field `LOCKE` writer - Lock Error Interrupt Enable"]
pub type LOCKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, LOCKESELECT_A, O>;
impl<'a, const O: u8> LOCKE_W<'a, O> {
    #[doc = "Lock Error does not generate an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCKESELECT_A::_0)
    }
    #[doc = "Lock Error generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCKESELECT_A::_1)
    }
}
#[doc = "Field `PROGE` reader - Programming Error Interrupt Enable"]
pub type PROGE_R = crate::BitReader<PROGESELECT_A>;
#[doc = "Programming Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROGESELECT_A {
    #[doc = "0: Programming Error does not generate an interrupt"]
    _0 = 0,
    #[doc = "1: Programming Error generates an interrupt"]
    _1 = 1,
}
impl From<PROGESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PROGESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PROGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROGESELECT_A {
        match self.bits {
            false => PROGESELECT_A::_0,
            true => PROGESELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PROGESELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PROGESELECT_A::_1
    }
}
#[doc = "Field `PROGE` writer - Programming Error Interrupt Enable"]
pub type PROGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, PROGESELECT_A, O>;
impl<'a, const O: u8> PROGE_W<'a, O> {
    #[doc = "Programming Error does not generate an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROGESELECT_A::_0)
    }
    #[doc = "Programming Error generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROGESELECT_A::_1)
    }
}
#[doc = "Field `FWS` reader - Flash Wait State"]
pub type FWS_R = crate::BitReader<FWSSELECT_A>;
#[doc = "Flash Wait State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWSSELECT_A {
    #[doc = "0: The flash is read with 0 wait states"]
    _0 = 0,
    #[doc = "1: The flash is read with 1 wait states"]
    _1 = 1,
}
impl From<FWSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: FWSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl FWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWSSELECT_A {
        match self.bits {
            false => FWSSELECT_A::_0,
            true => FWSSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FWSSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FWSSELECT_A::_1
    }
}
#[doc = "Field `FWS` writer - Flash Wait State"]
pub type FWS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, FWSSELECT_A, O>;
impl<'a, const O: u8> FWS_W<'a, O> {
    #[doc = "The flash is read with 0 wait states"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FWSSELECT_A::_0)
    }
    #[doc = "The flash is read with 1 wait states"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FWSSELECT_A::_1)
    }
}
#[doc = "Field `WS1OPT` reader - Wait State 1 Optimization"]
pub type WS1OPT_R = crate::BitReader<bool>;
#[doc = "Field `WS1OPT` writer - Wait State 1 Optimization"]
pub type WS1OPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Lock Error Interrupt Enable"]
    #[inline(always)]
    pub fn locke(&self) -> LOCKE_R {
        LOCKE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Programming Error Interrupt Enable"]
    #[inline(always)]
    pub fn proge(&self) -> PROGE_R {
        PROGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Flash Wait State"]
    #[inline(always)]
    pub fn fws(&self) -> FWS_R {
        FWS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wait State 1 Optimization"]
    #[inline(always)]
    pub fn ws1opt(&self) -> WS1OPT_R {
        WS1OPT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frdy(&mut self) -> FRDY_W<0> {
        FRDY_W::new(self)
    }
    #[doc = "Bit 2 - Lock Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn locke(&mut self) -> LOCKE_W<2> {
        LOCKE_W::new(self)
    }
    #[doc = "Bit 3 - Programming Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn proge(&mut self) -> PROGE_W<3> {
        PROGE_W::new(self)
    }
    #[doc = "Bit 6 - Flash Wait State"]
    #[inline(always)]
    #[must_use]
    pub fn fws(&mut self) -> FWS_W<6> {
        FWS_W::new(self)
    }
    #[doc = "Bit 7 - Wait State 1 Optimization"]
    #[inline(always)]
    #[must_use]
    pub fn ws1opt(&mut self) -> WS1OPT_W<7> {
        WS1OPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Controller Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcr::R](R) reader structure"]
impl crate::Readable for FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
