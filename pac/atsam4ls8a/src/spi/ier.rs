#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive Data Register Full Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDRFSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
    _1 = 1,
}
impl From<RDRFSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RDRFSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDRF` writer - Receive Data Register Full Interrupt Enable"]
pub type RDRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, RDRFSELECT_AW, O>;
impl<'a, const O: u8> RDRF_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDRFSELECT_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDRFSELECT_AW::_1)
    }
}
#[doc = "Transmit Data Register Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDRESELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
    _1 = 1,
}
impl From<TDRESELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TDRESELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDRE` writer - Transmit Data Register Empty Interrupt Enable"]
pub type TDRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, TDRESELECT_AW, O>;
impl<'a, const O: u8> TDRE_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDRESELECT_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDRESELECT_AW::_1)
    }
}
#[doc = "Mode Fault Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODFSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
    _1 = 1,
}
impl From<MODFSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: MODFSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODF` writer - Mode Fault Error Interrupt Enable"]
pub type MODF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, MODFSELECT_AW, O>;
impl<'a, const O: u8> MODF_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODFSELECT_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODFSELECT_AW::_1)
    }
}
#[doc = "Overrun Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRESSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
    _1 = 1,
}
impl From<OVRESSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: OVRESSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRES` writer - Overrun Error Interrupt Enable"]
pub type OVRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, OVRESSELECT_AW, O>;
impl<'a, const O: u8> OVRES_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVRESSELECT_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVRESSELECT_AW::_1)
    }
}
#[doc = "End of Receive Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDRXSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
    _1 = 1,
}
impl From<ENDRXSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDRXSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Enable"]
pub type ENDRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, ENDRXSELECT_AW, O>;
impl<'a, const O: u8> ENDRX_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENDRXSELECT_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENDRXSELECT_AW::_1)
    }
}
#[doc = "End of Transmit Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDTXSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
    _1 = 1,
}
impl From<ENDTXSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDTXSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDTX` writer - End of Transmit Buffer Interrupt Enable"]
pub type ENDTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, ENDTXSELECT_AW, O>;
impl<'a, const O: u8> ENDTX_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENDTXSELECT_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENDTXSELECT_AW::_1)
    }
}
#[doc = "Receive Buffer Full Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXBUFFSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
    _1 = 1,
}
impl From<RXBUFFSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RXBUFFSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Enable"]
pub type RXBUFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, RXBUFFSELECT_AW, O>;
impl<'a, const O: u8> RXBUFF_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXBUFFSELECT_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXBUFFSELECT_AW::_1)
    }
}
#[doc = "Transmit Buffer Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXBUFESELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
    _1 = 1,
}
impl From<TXBUFESELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXBUFESELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXBUFE` writer - Transmit Buffer Empty Interrupt Enable"]
pub type TXBUFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, TXBUFESELECT_AW, O>;
impl<'a, const O: u8> TXBUFE_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXBUFESELECT_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXBUFESELECT_AW::_1)
    }
}
#[doc = "NSS Rising Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSSRSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
    _1 = 1,
}
impl From<NSSRSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: NSSRSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSSR` writer - NSS Rising Interrupt Enable"]
pub type NSSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, NSSRSELECT_AW, O>;
impl<'a, const O: u8> NSSR_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NSSRSELECT_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NSSRSELECT_AW::_1)
    }
}
#[doc = "Transmission Registers Empty Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEMPTYSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
    _1 = 1,
}
impl From<TXEMPTYSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXEMPTYSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPTY` writer - Transmission Registers Empty Enable"]
pub type TXEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, TXEMPTYSELECT_AW, O>;
impl<'a, const O: u8> TXEMPTY_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXEMPTYSELECT_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXEMPTYSELECT_AW::_1)
    }
}
#[doc = "Field `UNDES` writer - Underrun Error Interrupt Enable"]
pub type UNDES_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdrf(&mut self) -> RDRF_W<0> {
        RDRF_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdre(&mut self) -> TDRE_W<1> {
        TDRE_W::new(self)
    }
    #[doc = "Bit 2 - Mode Fault Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn modf(&mut self) -> MODF_W<2> {
        MODF_W::new(self)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovres(&mut self) -> OVRES_W<3> {
        OVRES_W::new(self)
    }
    #[doc = "Bit 4 - End of Receive Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> ENDRX_W<4> {
        ENDRX_W::new(self)
    }
    #[doc = "Bit 5 - End of Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> ENDTX_W<5> {
        ENDTX_W::new(self)
    }
    #[doc = "Bit 6 - Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RXBUFF_W<6> {
        RXBUFF_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TXBUFE_W<7> {
        TXBUFE_W::new(self)
    }
    #[doc = "Bit 8 - NSS Rising Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nssr(&mut self) -> NSSR_W<8> {
        NSSR_W::new(self)
    }
    #[doc = "Bit 9 - Transmission Registers Empty Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<9> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn undes(&mut self) -> UNDES_W<10> {
        UNDES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
