#[doc = "Register `IDR_LIN` writer"]
pub struct W(crate::W<LIN_MODE_IDR_LIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIN_MODE_IDR_LIN_SPEC>;
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
impl From<crate::W<LIN_MODE_IDR_LIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIN_MODE_IDR_LIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receiver Ready Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXRDYSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<RXRDYSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RXRDYSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXRDY` writer - Receiver Ready Interrupt Disable"]
pub type RXRDY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, RXRDYSELECT_AW, O>;
impl<'a, const O: u8> RXRDY_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXRDYSELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXRDYSELECT_AW::_1)
    }
}
#[doc = "Transmitter Ready Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXRDYSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<TXRDYSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXRDYSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRDY` writer - Transmitter Ready Interrupt Disable"]
pub type TXRDY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, TXRDYSELECT_AW, O>;
impl<'a, const O: u8> TXRDY_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRDYSELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRDYSELECT_AW::_1)
    }
}
#[doc = "Receiver Break Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXBRKSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<RXBRKSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RXBRKSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXBRK` writer - Receiver Break Interrupt Disable"]
pub type RXBRK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, RXBRKSELECT_AW, O>;
impl<'a, const O: u8> RXBRK_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXBRKSELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXBRKSELECT_AW::_1)
    }
}
#[doc = "Overrun Error Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRESELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<OVRESELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: OVRESELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Disable"]
pub type OVRE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, OVRESELECT_AW, O>;
impl<'a, const O: u8> OVRE_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVRESELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVRESELECT_AW::_1)
    }
}
#[doc = "Framing Error Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAMESELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<FRAMESELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: FRAMESELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAME` writer - Framing Error Interrupt Disable"]
pub type FRAME_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, FRAMESELECT_AW, O>;
impl<'a, const O: u8> FRAME_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRAMESELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRAMESELECT_AW::_1)
    }
}
#[doc = "Parity Error Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARESELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<PARESELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: PARESELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARE` writer - Parity Error Interrupt Disable"]
pub type PARE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, PARESELECT_AW, O>;
impl<'a, const O: u8> PARE_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PARESELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PARESELECT_AW::_1)
    }
}
#[doc = "Time-out Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEOUTSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<TIMEOUTSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUTSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT` writer - Time-out Interrupt Disable"]
pub type TIMEOUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, TIMEOUTSELECT_AW, O>;
impl<'a, const O: u8> TIMEOUT_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMEOUTSELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMEOUTSELECT_AW::_1)
    }
}
#[doc = "Transmitter Empty Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEMPTYSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<TXEMPTYSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXEMPTYSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPTY` writer - Transmitter Empty Interrupt Disable"]
pub type TXEMPTY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, TXEMPTYSELECT_AW, O>;
impl<'a, const O: u8> TXEMPTY_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXEMPTYSELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXEMPTYSELECT_AW::_1)
    }
}
#[doc = "Iteration Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITERSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<ITERSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: ITERSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITER` writer - Iteration Interrupt Disable"]
pub type ITER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, ITERSELECT_AW, O>;
impl<'a, const O: u8> ITER_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ITERSELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ITERSELECT_AW::_1)
    }
}
#[doc = "Buffer Empty Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXBUFESELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<TXBUFESELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXBUFESELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXBUFE` writer - Buffer Empty Interrupt Disable"]
pub type TXBUFE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, TXBUFESELECT_AW, O>;
impl<'a, const O: u8> TXBUFE_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXBUFESELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXBUFESELECT_AW::_1)
    }
}
#[doc = "Buffer Full Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXBUFFSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<RXBUFFSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RXBUFFSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXBUFF` writer - Buffer Full Interrupt Disable"]
pub type RXBUFF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, RXBUFFSELECT_AW, O>;
impl<'a, const O: u8> RXBUFF_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXBUFFSELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXBUFFSELECT_AW::_1)
    }
}
#[doc = "Non Acknowledge or LIN Break Sent or LIN Break Received Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<NACKSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: NACKSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK` writer - Non Acknowledge or LIN Break Sent or LIN Break Received Interrupt Disable"]
pub type NACK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, NACKSELECT_AW, O>;
impl<'a, const O: u8> NACK_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NACKSELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NACKSELECT_AW::_1)
    }
}
#[doc = "Field `LINID` writer - LIN Identifier Sent or LIN Identifier Received Interrupt Disable"]
pub type LINID_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, bool, O>;
#[doc = "Field `LINTC` writer - LIN Transfer Conpleted Interrupt Disable"]
pub type LINTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, bool, O>;
#[doc = "Ring Indicator Input Change Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIICSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<RIICSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RIICSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIIC` writer - Ring Indicator Input Change Disable"]
pub type RIIC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, RIICSELECT_AW, O>;
impl<'a, const O: u8> RIIC_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIICSELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIICSELECT_AW::_1)
    }
}
#[doc = "Data Set Ready Input Change Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSRICSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<DSRICSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: DSRICSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSRIC` writer - Data Set Ready Input Change Disable"]
pub type DSRIC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, DSRICSELECT_AW, O>;
impl<'a, const O: u8> DSRIC_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSRICSELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSRICSELECT_AW::_1)
    }
}
#[doc = "Data Carrier Detect Input Change Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDICSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<DCDICSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: DCDICSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDIC` writer - Data Carrier Detect Input Change Interrupt Disable"]
pub type DCDIC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, DCDICSELECT_AW, O>;
impl<'a, const O: u8> DCDIC_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCDICSELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCDICSELECT_AW::_1)
    }
}
#[doc = "Clear to Send Input Change Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSICSELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<CTSICSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSICSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSIC` writer - Clear to Send Input Change Interrupt Disable"]
pub type CTSIC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, CTSICSELECT_AW, O>;
impl<'a, const O: u8> CTSIC_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSICSELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSICSELECT_AW::_1)
    }
}
#[doc = "Field `LINBE` writer - LIN Bus Error Interrupt Disable"]
pub type LINBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, bool, O>;
#[doc = "Field `LINISFE` writer - LIN Inconsistent Synch Field Error Interrupt Disable"]
pub type LINISFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, bool, O>;
#[doc = "Field `LINIPE` writer - LIN Identifier Parity Interrupt Disable"]
pub type LINIPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, bool, O>;
#[doc = "Field `LINCE` writer - LIN Checksum Error Interrupt Disable"]
pub type LINCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, bool, O>;
#[doc = "Field `LINSNRE` writer - LIN Slave Not Responding Error Interrupt Disable"]
pub type LINSNRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, bool, O>;
#[doc = "LIN Synch Tolerance Error Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINSTESELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<LINSTESELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: LINSTESELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINSTE` writer - LIN Synch Tolerance Error Interrupt Disable"]
pub type LINSTE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, LINSTESELECT_AW, O>;
impl<'a, const O: u8> LINSTE_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINSTESELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINSTESELECT_AW::_1)
    }
}
#[doc = "LIN Header Timeout Error Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINHTESELECT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<LINHTESELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: LINHTESELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINHTE` writer - LIN Header Timeout Error Interrupt Disable"]
pub type LINHTE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_SPEC, LINHTESELECT_AW, O>;
impl<'a, const O: u8> LINHTE_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINHTESELECT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINHTESELECT_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<0> {
        RXRDY_W::new(self)
    }
    #[doc = "Bit 1 - Transmitter Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<1> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 2 - Receiver Break Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbrk(&mut self) -> RXBRK_W<2> {
        RXBRK_W::new(self)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<5> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FRAME_W<6> {
        FRAME_W::new(self)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pare(&mut self) -> PARE_W<7> {
        PARE_W::new(self)
    }
    #[doc = "Bit 8 - Time-out Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<8> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 9 - Transmitter Empty Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<9> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Bit 10 - Iteration Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn iter(&mut self) -> ITER_W<10> {
        ITER_W::new(self)
    }
    #[doc = "Bit 11 - Buffer Empty Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TXBUFE_W<11> {
        TXBUFE_W::new(self)
    }
    #[doc = "Bit 12 - Buffer Full Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RXBUFF_W<12> {
        RXBUFF_W::new(self)
    }
    #[doc = "Bit 13 - Non Acknowledge or LIN Break Sent or LIN Break Received Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<13> {
        NACK_W::new(self)
    }
    #[doc = "Bit 14 - LIN Identifier Sent or LIN Identifier Received Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn linid(&mut self) -> LINID_W<14> {
        LINID_W::new(self)
    }
    #[doc = "Bit 15 - LIN Transfer Conpleted Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lintc(&mut self) -> LINTC_W<15> {
        LINTC_W::new(self)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Disable"]
    #[inline(always)]
    #[must_use]
    pub fn riic(&mut self) -> RIIC_W<16> {
        RIIC_W::new(self)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dsric(&mut self) -> DSRIC_W<17> {
        DSRIC_W::new(self)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdic(&mut self) -> DCDIC_W<18> {
        DCDIC_W::new(self)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsic(&mut self) -> CTSIC_W<19> {
        CTSIC_W::new(self)
    }
    #[doc = "Bit 25 - LIN Bus Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn linbe(&mut self) -> LINBE_W<25> {
        LINBE_W::new(self)
    }
    #[doc = "Bit 26 - LIN Inconsistent Synch Field Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn linisfe(&mut self) -> LINISFE_W<26> {
        LINISFE_W::new(self)
    }
    #[doc = "Bit 27 - LIN Identifier Parity Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn linipe(&mut self) -> LINIPE_W<27> {
        LINIPE_W::new(self)
    }
    #[doc = "Bit 28 - LIN Checksum Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lince(&mut self) -> LINCE_W<28> {
        LINCE_W::new(self)
    }
    #[doc = "Bit 29 - LIN Slave Not Responding Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn linsnre(&mut self) -> LINSNRE_W<29> {
        LINSNRE_W::new(self)
    }
    #[doc = "Bit 30 - LIN Synch Tolerance Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn linste(&mut self) -> LINSTE_W<30> {
        LINSTE_W::new(self)
    }
    #[doc = "Bit 31 - LIN Header Timeout Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn linhte(&mut self) -> LINHTE_W<31> {
        LINHTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lin_mode_idr_lin](index.html) module"]
pub struct LIN_MODE_IDR_LIN_SPEC;
impl crate::RegisterSpec for LIN_MODE_IDR_LIN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lin_mode_idr_lin::W](W) writer structure"]
impl crate::Writable for LIN_MODE_IDR_LIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR_LIN to value 0"]
impl crate::Resettable for LIN_MODE_IDR_LIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
