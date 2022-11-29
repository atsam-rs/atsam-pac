#[doc = "Register `CCR%s` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Counter Clock Enable Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKENSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the clock if CLKDIS is not 1."]
    _1 = 1,
}
impl From<CLKENSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: CLKENSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` writer - Counter Clock Enable Command"]
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, CLKENSELECT_AW, O>;
impl<'a, const O: u8> CLKEN_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKENSELECT_AW::_0)
    }
    #[doc = "Enables the clock if CLKDIS is not 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKENSELECT_AW::_1)
    }
}
#[doc = "Counter Clock Disable Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKDISSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Disables the clock."]
    _1 = 1,
}
impl From<CLKDISSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: CLKDISSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKDIS` writer - Counter Clock Disable Command"]
pub type CLKDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, CLKDISSELECT_AW, O>;
impl<'a, const O: u8> CLKDIS_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKDISSELECT_AW::_0)
    }
    #[doc = "Disables the clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKDISSELECT_AW::_1)
    }
}
#[doc = "Software Trigger Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWTRGSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: A software trigger is performed:the counter is reset and clock is started."]
    _1 = 1,
}
impl From<SWTRGSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: SWTRGSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWTRG` writer - Software Trigger Command"]
pub type SWTRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, SWTRGSELECT_AW, O>;
impl<'a, const O: u8> SWTRG_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWTRGSELECT_AW::_0)
    }
    #[doc = "A software trigger is performed:the counter is reset and clock is started."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWTRGSELECT_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Clock Enable Command"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<0> {
        CLKEN_W::new(self)
    }
    #[doc = "Bit 1 - Counter Clock Disable Command"]
    #[inline(always)]
    #[must_use]
    pub fn clkdis(&mut self) -> CLKDIS_W<1> {
        CLKDIS_W::new(self)
    }
    #[doc = "Bit 2 - Software Trigger Command"]
    #[inline(always)]
    #[must_use]
    pub fn swtrg(&mut self) -> SWTRG_W<2> {
        SWTRG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Control Register Channel\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR%s to value 0"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
