#[doc = "Register `IER_SPI` writer"]
pub struct W(crate::W<SPI_SLAVE_MODE_IER_SPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_SLAVE_MODE_IER_SPI_SPEC>;
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
impl From<crate::W<SPI_SLAVE_MODE_IER_SPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_SLAVE_MODE_IER_SPI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receiver Ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXRDYSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the interrupt"]
    _1 = 1,
}
impl From<RXRDYSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RXRDYSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXRDY` writer - Receiver Ready Interrupt Enable"]
pub type RXRDY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_SLAVE_MODE_IER_SPI_SPEC, RXRDYSELECT_AW, O>;
impl<'a, const O: u8> RXRDY_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXRDYSELECT_AW::_0)
    }
    #[doc = "Enables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXRDYSELECT_AW::_1)
    }
}
#[doc = "Transmitter Ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXRDYSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the interrupt"]
    _1 = 1,
}
impl From<TXRDYSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXRDYSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRDY` writer - Transmitter Ready Interrupt Enable"]
pub type TXRDY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_SLAVE_MODE_IER_SPI_SPEC, TXRDYSELECT_AW, O>;
impl<'a, const O: u8> TXRDY_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRDYSELECT_AW::_0)
    }
    #[doc = "Enables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRDYSELECT_AW::_1)
    }
}
#[doc = "Receiver Break Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXBRKSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the interrupt"]
    _1 = 1,
}
impl From<RXBRKSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RXBRKSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXBRK` writer - Receiver Break Interrupt Enable"]
pub type RXBRK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_SLAVE_MODE_IER_SPI_SPEC, RXBRKSELECT_AW, O>;
impl<'a, const O: u8> RXBRK_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXBRKSELECT_AW::_0)
    }
    #[doc = "Enables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXBRKSELECT_AW::_1)
    }
}
#[doc = "Overrun Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRESELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the interrupt"]
    _1 = 1,
}
impl From<OVRESELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: OVRESELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OVRE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_SLAVE_MODE_IER_SPI_SPEC, OVRESELECT_AW, O>;
impl<'a, const O: u8> OVRE_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVRESELECT_AW::_0)
    }
    #[doc = "Enables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVRESELECT_AW::_1)
    }
}
#[doc = "Framing Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAMESELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the interrupt"]
    _1 = 1,
}
impl From<FRAMESELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: FRAMESELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAME` writer - Framing Error Interrupt Enable"]
pub type FRAME_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_SLAVE_MODE_IER_SPI_SPEC, FRAMESELECT_AW, O>;
impl<'a, const O: u8> FRAME_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRAMESELECT_AW::_0)
    }
    #[doc = "Enables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRAMESELECT_AW::_1)
    }
}
#[doc = "Parity Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARESELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the interrupt"]
    _1 = 1,
}
impl From<PARESELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: PARESELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARE` writer - Parity Error Interrupt Enable"]
pub type PARE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_SLAVE_MODE_IER_SPI_SPEC, PARESELECT_AW, O>;
impl<'a, const O: u8> PARE_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PARESELECT_AW::_0)
    }
    #[doc = "Enables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PARESELECT_AW::_1)
    }
}
#[doc = "Time-out Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEOUTSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the interrupt"]
    _1 = 1,
}
impl From<TIMEOUTSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUTSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT` writer - Time-out Interrupt Enable"]
pub type TIMEOUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_SLAVE_MODE_IER_SPI_SPEC, TIMEOUTSELECT_AW, O>;
impl<'a, const O: u8> TIMEOUT_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMEOUTSELECT_AW::_0)
    }
    #[doc = "Enables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMEOUTSELECT_AW::_1)
    }
}
#[doc = "Transmitter Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEMPTYSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the interrupt"]
    _1 = 1,
}
impl From<TXEMPTYSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXEMPTYSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPTY` writer - Transmitter Empty Interrupt Enable"]
pub type TXEMPTY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_SLAVE_MODE_IER_SPI_SPEC, TXEMPTYSELECT_AW, O>;
impl<'a, const O: u8> TXEMPTY_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXEMPTYSELECT_AW::_0)
    }
    #[doc = "Enables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXEMPTYSELECT_AW::_1)
    }
}
#[doc = "SPI Underrun Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNRESELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the interrupt"]
    _1 = 1,
}
impl From<UNRESELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: UNRESELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNRE` writer - SPI Underrun Error Interrupt Enable"]
pub type UNRE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_SLAVE_MODE_IER_SPI_SPEC, UNRESELECT_AW, O>;
impl<'a, const O: u8> UNRE_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UNRESELECT_AW::_0)
    }
    #[doc = "Enables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UNRESELECT_AW::_1)
    }
}
#[doc = "Buffer Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXBUFESELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the interrupt"]
    _1 = 1,
}
impl From<TXBUFESELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXBUFESELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXBUFE` writer - Buffer Empty Interrupt Enable"]
pub type TXBUFE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_SLAVE_MODE_IER_SPI_SPEC, TXBUFESELECT_AW, O>;
impl<'a, const O: u8> TXBUFE_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXBUFESELECT_AW::_0)
    }
    #[doc = "Enables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXBUFESELECT_AW::_1)
    }
}
#[doc = "Buffer Full Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXBUFFSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the interrupt"]
    _1 = 1,
}
impl From<RXBUFFSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RXBUFFSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXBUFF` writer - Buffer Full Interrupt Enable"]
pub type RXBUFF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_SLAVE_MODE_IER_SPI_SPEC, RXBUFFSELECT_AW, O>;
impl<'a, const O: u8> RXBUFF_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXBUFFSELECT_AW::_0)
    }
    #[doc = "Enables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXBUFFSELECT_AW::_1)
    }
}
#[doc = "Non Acknowledge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the interrupt"]
    _1 = 1,
}
impl From<NACKSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: NACKSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK` writer - Non Acknowledge Interrupt Enable"]
pub type NACK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_SLAVE_MODE_IER_SPI_SPEC, NACKSELECT_AW, O>;
impl<'a, const O: u8> NACK_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NACKSELECT_AW::_0)
    }
    #[doc = "Enables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NACKSELECT_AW::_1)
    }
}
#[doc = "Ring Indicator Input Change Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIICSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the interrupt"]
    _1 = 1,
}
impl From<RIICSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RIICSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIIC` writer - Ring Indicator Input Change Enable"]
pub type RIIC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_SLAVE_MODE_IER_SPI_SPEC, RIICSELECT_AW, O>;
impl<'a, const O: u8> RIIC_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIICSELECT_AW::_0)
    }
    #[doc = "Enables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIICSELECT_AW::_1)
    }
}
#[doc = "Data Set Ready Input Change Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSRICSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the interrupt"]
    _1 = 1,
}
impl From<DSRICSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: DSRICSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSRIC` writer - Data Set Ready Input Change Enable"]
pub type DSRIC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_SLAVE_MODE_IER_SPI_SPEC, DSRICSELECT_AW, O>;
impl<'a, const O: u8> DSRIC_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSRICSELECT_AW::_0)
    }
    #[doc = "Enables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSRICSELECT_AW::_1)
    }
}
#[doc = "Data Carrier Detect Input Change Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDICSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the interrupt"]
    _1 = 1,
}
impl From<DCDICSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: DCDICSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDIC` writer - Data Carrier Detect Input Change Interrupt Enable"]
pub type DCDIC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_SLAVE_MODE_IER_SPI_SPEC, DCDICSELECT_AW, O>;
impl<'a, const O: u8> DCDIC_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCDICSELECT_AW::_0)
    }
    #[doc = "Enables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCDICSELECT_AW::_1)
    }
}
#[doc = "Clear to Send Input Change Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSICSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the interrupt"]
    _1 = 1,
}
impl From<CTSICSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSICSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSIC` writer - Clear to Send Input Change Interrupt Enable"]
pub type CTSIC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_SLAVE_MODE_IER_SPI_SPEC, CTSICSELECT_AW, O>;
impl<'a, const O: u8> CTSIC_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSICSELECT_AW::_0)
    }
    #[doc = "Enables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSICSELECT_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<0> {
        RXRDY_W::new(self)
    }
    #[doc = "Bit 1 - Transmitter Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<1> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 2 - Receiver Break Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbrk(&mut self) -> RXBRK_W<2> {
        RXBRK_W::new(self)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<5> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FRAME_W<6> {
        FRAME_W::new(self)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pare(&mut self) -> PARE_W<7> {
        PARE_W::new(self)
    }
    #[doc = "Bit 8 - Time-out Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<8> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 9 - Transmitter Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<9> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Bit 10 - SPI Underrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn unre(&mut self) -> UNRE_W<10> {
        UNRE_W::new(self)
    }
    #[doc = "Bit 11 - Buffer Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TXBUFE_W<11> {
        TXBUFE_W::new(self)
    }
    #[doc = "Bit 12 - Buffer Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RXBUFF_W<12> {
        RXBUFF_W::new(self)
    }
    #[doc = "Bit 13 - Non Acknowledge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<13> {
        NACK_W::new(self)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn riic(&mut self) -> RIIC_W<16> {
        RIIC_W::new(self)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsric(&mut self) -> DSRIC_W<17> {
        DSRIC_W::new(self)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdic(&mut self) -> DCDIC_W<18> {
        DCDIC_W::new(self)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsic(&mut self) -> CTSIC_W<19> {
        CTSIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_slave_mode_ier_spi](index.html) module"]
pub struct SPI_SLAVE_MODE_IER_SPI_SPEC;
impl crate::RegisterSpec for SPI_SLAVE_MODE_IER_SPI_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [spi_slave_mode_ier_spi::W](W) writer structure"]
impl crate::Writable for SPI_SLAVE_MODE_IER_SPI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER_SPI to value 0"]
impl crate::Resettable for SPI_SLAVE_MODE_IER_SPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
