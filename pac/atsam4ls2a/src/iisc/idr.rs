#[doc = "Register `IDR` writer"]
pub struct W(crate::W<IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR_SPEC>;
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
impl From<crate::W<IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive Ready Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXRDYSELECT_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Disables the corresponding interrupt"]
    ON = 1,
}
impl From<RXRDYSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RXRDYSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXRDY` writer - Receive Ready Interrupt Disable"]
pub type RXRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, RXRDYSELECT_AW, O>;
impl<'a, const O: u8> RXRDY_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(RXRDYSELECT_AW::OFF)
    }
    #[doc = "Disables the corresponding interrupt"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(RXRDYSELECT_AW::ON)
    }
}
#[doc = "Receive Overrun Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXORSELECT_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Disables the corresponding interrupt"]
    ON = 1,
}
impl From<RXORSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RXORSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOR` writer - Receive Overrun Interrupt Disable"]
pub type RXOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, RXORSELECT_AW, O>;
impl<'a, const O: u8> RXOR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(RXORSELECT_AW::OFF)
    }
    #[doc = "Disables the corresponding interrupt"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(RXORSELECT_AW::ON)
    }
}
#[doc = "Transmit Ready Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXRDYSELECT_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Disables the corresponding interrupt"]
    ON = 1,
}
impl From<TXRDYSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXRDYSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Disable"]
pub type TXRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, TXRDYSELECT_AW, O>;
impl<'a, const O: u8> TXRDY_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(TXRDYSELECT_AW::OFF)
    }
    #[doc = "Disables the corresponding interrupt"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(TXRDYSELECT_AW::ON)
    }
}
#[doc = "Transmit Underrun Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXURSELECT_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Disables the corresponding interrupt"]
    ON = 1,
}
impl From<TXURSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXURSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXUR` writer - Transmit Underrun Interrupt Disable"]
pub type TXUR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, TXURSELECT_AW, O>;
impl<'a, const O: u8> TXUR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(TXURSELECT_AW::OFF)
    }
    #[doc = "Disables the corresponding interrupt"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(TXURSELECT_AW::ON)
    }
}
impl W {
    #[doc = "Bit 1 - Receive Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<1> {
        RXRDY_W::new(self)
    }
    #[doc = "Bit 2 - Receive Overrun Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxor(&mut self) -> RXOR_W<2> {
        RXOR_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<5> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Underrun Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txur(&mut self) -> TXUR_W<6> {
        TXUR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr::W](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
