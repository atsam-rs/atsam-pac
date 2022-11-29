#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit Ready Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXRDYSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the Audio DAC TX Ready interrupt"]
    _1 = 1,
}
impl From<TXRDYSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXRDYSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Clear"]
pub type TXRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, TXRDYSELECT_AW, O>;
impl<'a, const O: u8> TXRDY_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRDYSELECT_AW::_0)
    }
    #[doc = "Clear the Audio DAC TX Ready interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRDYSELECT_AW::_1)
    }
}
#[doc = "Transmit Underrun Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXURSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the Audio DAC Underrun interrupt"]
    _1 = 1,
}
impl From<TXURSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXURSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXUR` writer - Transmit Underrun Interrupt Clear"]
pub type TXUR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, TXURSELECT_AW, O>;
impl<'a, const O: u8> TXUR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXURSELECT_AW::_0)
    }
    #[doc = "Clear the Audio DAC Underrun interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXURSELECT_AW::_1)
    }
}
impl W {
    #[doc = "Bit 1 - Transmit Ready Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<1> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Underrun Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txur(&mut self) -> TXUR_W<2> {
        TXUR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
