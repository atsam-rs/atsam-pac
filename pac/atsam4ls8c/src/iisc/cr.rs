#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXENSELECT_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Enables Data Receive if RXDIS is not set"]
    ON = 1,
}
impl From<RXENSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RXENSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEN` writer - Receive Enable"]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, RXENSELECT_AW, O>;
impl<'a, const O: u8> RXEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(RXENSELECT_AW::OFF)
    }
    #[doc = "Enables Data Receive if RXDIS is not set"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(RXENSELECT_AW::ON)
    }
}
#[doc = "Receive Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDISSELECT_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Disables Data Receive"]
    ON = 1,
}
impl From<RXDISSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RXDISSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDIS` writer - Receive Disable"]
pub type RXDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, RXDISSELECT_AW, O>;
impl<'a, const O: u8> RXDIS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(RXDISSELECT_AW::OFF)
    }
    #[doc = "Disables Data Receive"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(RXDISSELECT_AW::ON)
    }
}
#[doc = "Clocks Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKENSELECT_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Enables clocks if CKDIS is not set"]
    ON = 1,
}
impl From<CKENSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: CKENSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKEN` writer - Clocks Enable"]
pub type CKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CKENSELECT_AW, O>;
impl<'a, const O: u8> CKEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CKENSELECT_AW::OFF)
    }
    #[doc = "Enables clocks if CKDIS is not set"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CKENSELECT_AW::ON)
    }
}
#[doc = "Clocks Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKDISSELECT_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Disables clocks"]
    ON = 1,
}
impl From<CKDISSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: CKDISSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKDIS` writer - Clocks Disable"]
pub type CKDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CKDISSELECT_AW, O>;
impl<'a, const O: u8> CKDIS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CKDISSELECT_AW::OFF)
    }
    #[doc = "Disables clocks"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CKDISSELECT_AW::ON)
    }
}
#[doc = "Transmit Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXENSELECT_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Enables Data Transmit if TXDIS is not set"]
    ON = 1,
}
impl From<TXENSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXENSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEN` writer - Transmit Enable"]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TXENSELECT_AW, O>;
impl<'a, const O: u8> TXEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(TXENSELECT_AW::OFF)
    }
    #[doc = "Enables Data Transmit if TXDIS is not set"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(TXENSELECT_AW::ON)
    }
}
#[doc = "Transmit Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDISSELECT_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Disables Data Transmit"]
    ON = 1,
}
impl From<TXDISSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXDISSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDIS` writer - Transmit Disable"]
pub type TXDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TXDISSELECT_AW, O>;
impl<'a, const O: u8> TXDIS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(TXDISSELECT_AW::OFF)
    }
    #[doc = "Disables Data Transmit"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(TXDISSELECT_AW::ON)
    }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRSTSELECT_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Performs a software reset. Has priority on any other bit in CR"]
    ON = 1,
}
impl From<SWRSTSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: SWRSTSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, SWRSTSELECT_AW, O>;
impl<'a, const O: u8> SWRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SWRSTSELECT_AW::OFF)
    }
    #[doc = "Performs a software reset. Has priority on any other bit in CR"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(SWRSTSELECT_AW::ON)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<0> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 1 - Receive Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdis(&mut self) -> RXDIS_W<1> {
        RXDIS_W::new(self)
    }
    #[doc = "Bit 2 - Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cken(&mut self) -> CKEN_W<2> {
        CKEN_W::new(self)
    }
    #[doc = "Bit 3 - Clocks Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ckdis(&mut self) -> CKDIS_W<3> {
        CKDIS_W::new(self)
    }
    #[doc = "Bit 4 - Transmit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<4> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TXDIS_W<5> {
        TXDIS_W::new(self)
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<7> {
        SWRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
