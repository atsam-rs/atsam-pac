#[doc = "Register `SSR` writer"]
pub struct W(crate::W<SSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSR_SPEC>;
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
impl From<crate::W<SSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive Overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXORSELECT_AW {
    #[doc = "0: No effect"]
    NO = 0,
    #[doc = "1: Sets corresponding SR bit"]
    SET = 1,
}
impl From<RXORSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RXORSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOR` writer - Receive Overrun"]
pub type RXOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSR_SPEC, RXORSELECT_AW, O>;
impl<'a, const O: u8> RXOR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(RXORSELECT_AW::NO)
    }
    #[doc = "Sets corresponding SR bit"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RXORSELECT_AW::SET)
    }
}
#[doc = "Transmit Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXURSELECT_AW {
    #[doc = "0: No effect"]
    NO = 0,
    #[doc = "1: Sets corresponding SR bit"]
    SET = 1,
}
impl From<TXURSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXURSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXUR` writer - Transmit Underrun"]
pub type TXUR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSR_SPEC, TXURSELECT_AW, O>;
impl<'a, const O: u8> TXUR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(TXURSELECT_AW::NO)
    }
    #[doc = "Sets corresponding SR bit"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TXURSELECT_AW::SET)
    }
}
#[doc = "Field `RXORCH` writer - Receive Overrun Channels"]
pub type RXORCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TXURCH` writer - Transmit Underrun Channels"]
pub type TXURCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSR_SPEC, u8, u8, 2, O>;
impl W {
    #[doc = "Bit 2 - Receive Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rxor(&mut self) -> RXOR_W<2> {
        RXOR_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn txur(&mut self) -> TXUR_W<6> {
        TXUR_W::new(self)
    }
    #[doc = "Bits 8:9 - Receive Overrun Channels"]
    #[inline(always)]
    #[must_use]
    pub fn rxorch(&mut self) -> RXORCH_W<8> {
        RXORCH_W::new(self)
    }
    #[doc = "Bits 20:21 - Transmit Underrun Channels"]
    #[inline(always)]
    #[must_use]
    pub fn txurch(&mut self) -> TXURCH_W<20> {
        TXURCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssr](index.html) module"]
pub struct SSR_SPEC;
impl crate::RegisterSpec for SSR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ssr::W](W) writer structure"]
impl crate::Writable for SSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSR to value 0"]
impl crate::Resettable for SSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
