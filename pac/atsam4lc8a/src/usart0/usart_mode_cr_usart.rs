#[doc = "Register `CR_USART` writer"]
pub struct W(crate::W<USART_MODE_CR_USART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_MODE_CR_USART_SPEC>;
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
impl From<crate::W<USART_MODE_CR_USART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_MODE_CR_USART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reset Receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTRXSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Resets the receiver"]
    _1 = 1,
}
impl From<RSTRXSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTRXSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTRX` writer - Reset Receiver"]
pub type RSTRX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, RSTRXSELECT_AW, O>;
impl<'a, const O: u8> RSTRX_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTRXSELECT_AW::_0)
    }
    #[doc = "Resets the receiver"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTRXSELECT_AW::_1)
    }
}
#[doc = "Reset Transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTTXSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Resets the transmitter"]
    _1 = 1,
}
impl From<RSTTXSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTTXSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTTX` writer - Reset Transmitter"]
pub type RSTTX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, RSTTXSELECT_AW, O>;
impl<'a, const O: u8> RSTTX_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTTXSELECT_AW::_0)
    }
    #[doc = "Resets the transmitter"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTTXSELECT_AW::_1)
    }
}
#[doc = "Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXENSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enables the receiver, if RXDIS is 0"]
    _1 = 1,
}
impl From<RXENSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RXENSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, RXENSELECT_AW, O>;
impl<'a, const O: u8> RXEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXENSELECT_AW::_0)
    }
    #[doc = "Enables the receiver, if RXDIS is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXENSELECT_AW::_1)
    }
}
#[doc = "Receiver Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDISSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Disables the receiver"]
    _1 = 1,
}
impl From<RXDISSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RXDISSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RXDIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, RXDISSELECT_AW, O>;
impl<'a, const O: u8> RXDIS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXDISSELECT_AW::_0)
    }
    #[doc = "Disables the receiver"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXDISSELECT_AW::_1)
    }
}
#[doc = "Transmitter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXENSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enables the transmitter if TXDIS is 0"]
    _1 = 1,
}
impl From<TXENSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXENSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TXEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, TXENSELECT_AW, O>;
impl<'a, const O: u8> TXEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXENSELECT_AW::_0)
    }
    #[doc = "Enables the transmitter if TXDIS is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXENSELECT_AW::_1)
    }
}
#[doc = "Transmitter Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDISSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Disables the transmitter"]
    _1 = 1,
}
impl From<TXDISSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXDISSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TXDIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, TXDISSELECT_AW, O>;
impl<'a, const O: u8> TXDIS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXDISSELECT_AW::_0)
    }
    #[doc = "Disables the transmitter"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXDISSELECT_AW::_1)
    }
}
#[doc = "Reset Status Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTSTASELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Resets the status bits PARE, FRAME, OVRE and RXBRK in the CSR"]
    _1 = 1,
}
impl From<RSTSTASELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTSTASELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTSTA` writer - Reset Status Bits"]
pub type RSTSTA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, RSTSTASELECT_AW, O>;
impl<'a, const O: u8> RSTSTA_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTSTASELECT_AW::_0)
    }
    #[doc = "Resets the status bits PARE, FRAME, OVRE and RXBRK in the CSR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTSTASELECT_AW::_1)
    }
}
#[doc = "Start Break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STTBRKSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Starts transmission of a break after the characters present in THR and the Transmit Shift Register have been transmitted. No effect if a break is already being transmitted"]
    _1 = 1,
}
impl From<STTBRKSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: STTBRKSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STTBRK` writer - Start Break"]
pub type STTBRK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, STTBRKSELECT_AW, O>;
impl<'a, const O: u8> STTBRK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STTBRKSELECT_AW::_0)
    }
    #[doc = "Starts transmission of a break after the characters present in THR and the Transmit Shift Register have been transmitted. No effect if a break is already being transmitted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STTBRKSELECT_AW::_1)
    }
}
#[doc = "Stop Break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STPBRKSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Stops transmission of the break after a minimum of one character length and transmits a high level during 12-bit periods.No effect if no break is being transmitted"]
    _1 = 1,
}
impl From<STPBRKSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: STPBRKSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPBRK` writer - Stop Break"]
pub type STPBRK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, STPBRKSELECT_AW, O>;
impl<'a, const O: u8> STPBRK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STPBRKSELECT_AW::_0)
    }
    #[doc = "Stops transmission of the break after a minimum of one character length and transmits a high level during 12-bit periods.No effect if no break is being transmitted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STPBRKSELECT_AW::_1)
    }
}
#[doc = "Start Time-out\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STTTOSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Starts waiting for a character before clocking the time-out counter"]
    _1 = 1,
}
impl From<STTTOSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: STTTOSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STTTO` writer - Start Time-out"]
pub type STTTO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, STTTOSELECT_AW, O>;
impl<'a, const O: u8> STTTO_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STTTOSELECT_AW::_0)
    }
    #[doc = "Starts waiting for a character before clocking the time-out counter"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STTTOSELECT_AW::_1)
    }
}
#[doc = "Send Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SENDASELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: In Multi-drop Mode only, the next character written to the THR is sent with the address bit set"]
    _1 = 1,
}
impl From<SENDASELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: SENDASELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SENDA` writer - Send Address"]
pub type SENDA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, SENDASELECT_AW, O>;
impl<'a, const O: u8> SENDA_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SENDASELECT_AW::_0)
    }
    #[doc = "In Multi-drop Mode only, the next character written to the THR is sent with the address bit set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SENDASELECT_AW::_1)
    }
}
#[doc = "Reset Iterations\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTITSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Resets ITERATION in CSR. No effect if the ISO7816 is not enabled"]
    _1 = 1,
}
impl From<RSTITSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTITSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTIT` writer - Reset Iterations"]
pub type RSTIT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, RSTITSELECT_AW, O>;
impl<'a, const O: u8> RSTIT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTITSELECT_AW::_0)
    }
    #[doc = "Resets ITERATION in CSR. No effect if the ISO7816 is not enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTITSELECT_AW::_1)
    }
}
#[doc = "Reset Non Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTNACKSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Resets NACK in CSR"]
    _1 = 1,
}
impl From<RSTNACKSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTNACKSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTNACK` writer - Reset Non Acknowledge"]
pub type RSTNACK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, RSTNACKSELECT_AW, O>;
impl<'a, const O: u8> RSTNACK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTNACKSELECT_AW::_0)
    }
    #[doc = "Resets NACK in CSR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTNACKSELECT_AW::_1)
    }
}
#[doc = "Rearm Time-out\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RETTOSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Restart Time-out"]
    _1 = 1,
}
impl From<RETTOSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RETTOSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RETTO` writer - Rearm Time-out"]
pub type RETTO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, RETTOSELECT_AW, O>;
impl<'a, const O: u8> RETTO_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RETTOSELECT_AW::_0)
    }
    #[doc = "Restart Time-out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RETTOSELECT_AW::_1)
    }
}
#[doc = "Data Terminal Ready Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTRENSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Drives the pin DTR at 0"]
    _1 = 1,
}
impl From<DTRENSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: DTRENSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTREN` writer - Data Terminal Ready Enable"]
pub type DTREN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, DTRENSELECT_AW, O>;
impl<'a, const O: u8> DTREN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTRENSELECT_AW::_0)
    }
    #[doc = "Drives the pin DTR at 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTRENSELECT_AW::_1)
    }
}
#[doc = "Data Terminal Ready Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTRDISSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Drives the pin DTR to 1"]
    _1 = 1,
}
impl From<DTRDISSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: DTRDISSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTRDIS` writer - Data Terminal Ready Disable"]
pub type DTRDIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, DTRDISSELECT_AW, O>;
impl<'a, const O: u8> DTRDIS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTRDISSELECT_AW::_0)
    }
    #[doc = "Drives the pin DTR to 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTRDISSELECT_AW::_1)
    }
}
#[doc = "Request to Send Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSENSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Drives the pin RTS to 0"]
    _1 = 1,
}
impl From<RTSENSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RTSENSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSEN` writer - Request to Send Enable"]
pub type RTSEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, RTSENSELECT_AW, O>;
impl<'a, const O: u8> RTSEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTSENSELECT_AW::_0)
    }
    #[doc = "Drives the pin RTS to 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTSENSELECT_AW::_1)
    }
}
#[doc = "Request to Send Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSDISSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Drives the pin RTS to 1"]
    _1 = 1,
}
impl From<RTSDISSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: RTSDISSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSDIS` writer - Request to Send Disable"]
pub type RTSDIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USART_MODE_CR_USART_SPEC, RTSDISSELECT_AW, O>;
impl<'a, const O: u8> RTSDIS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTSDISSELECT_AW::_0)
    }
    #[doc = "Drives the pin RTS to 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTSDISSELECT_AW::_1)
    }
}
impl W {
    #[doc = "Bit 2 - Reset Receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rstrx(&mut self) -> RSTRX_W<2> {
        RSTRX_W::new(self)
    }
    #[doc = "Bit 3 - Reset Transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn rsttx(&mut self) -> RSTTX_W<3> {
        RSTTX_W::new(self)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<4> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 5 - Receiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdis(&mut self) -> RXDIS_W<5> {
        RXDIS_W::new(self)
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<6> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 7 - Transmitter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TXDIS_W<7> {
        TXDIS_W::new(self)
    }
    #[doc = "Bit 8 - Reset Status Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rststa(&mut self) -> RSTSTA_W<8> {
        RSTSTA_W::new(self)
    }
    #[doc = "Bit 9 - Start Break"]
    #[inline(always)]
    #[must_use]
    pub fn sttbrk(&mut self) -> STTBRK_W<9> {
        STTBRK_W::new(self)
    }
    #[doc = "Bit 10 - Stop Break"]
    #[inline(always)]
    #[must_use]
    pub fn stpbrk(&mut self) -> STPBRK_W<10> {
        STPBRK_W::new(self)
    }
    #[doc = "Bit 11 - Start Time-out"]
    #[inline(always)]
    #[must_use]
    pub fn sttto(&mut self) -> STTTO_W<11> {
        STTTO_W::new(self)
    }
    #[doc = "Bit 12 - Send Address"]
    #[inline(always)]
    #[must_use]
    pub fn senda(&mut self) -> SENDA_W<12> {
        SENDA_W::new(self)
    }
    #[doc = "Bit 13 - Reset Iterations"]
    #[inline(always)]
    #[must_use]
    pub fn rstit(&mut self) -> RSTIT_W<13> {
        RSTIT_W::new(self)
    }
    #[doc = "Bit 14 - Reset Non Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn rstnack(&mut self) -> RSTNACK_W<14> {
        RSTNACK_W::new(self)
    }
    #[doc = "Bit 15 - Rearm Time-out"]
    #[inline(always)]
    #[must_use]
    pub fn retto(&mut self) -> RETTO_W<15> {
        RETTO_W::new(self)
    }
    #[doc = "Bit 16 - Data Terminal Ready Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtren(&mut self) -> DTREN_W<16> {
        DTREN_W::new(self)
    }
    #[doc = "Bit 17 - Data Terminal Ready Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dtrdis(&mut self) -> DTRDIS_W<17> {
        DTRDIS_W::new(self)
    }
    #[doc = "Bit 18 - Request to Send Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RTSEN_W<18> {
        RTSEN_W::new(self)
    }
    #[doc = "Bit 19 - Request to Send Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rtsdis(&mut self) -> RTSDIS_W<19> {
        RTSDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_mode_cr_usart](index.html) module"]
pub struct USART_MODE_CR_USART_SPEC;
impl crate::RegisterSpec for USART_MODE_CR_USART_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usart_mode_cr_usart::W](W) writer structure"]
impl crate::Writable for USART_MODE_CR_USART_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR_USART to value 0"]
impl crate::Resettable for USART_MODE_CR_USART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
