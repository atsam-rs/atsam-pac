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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTRX_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Resets the receiver"]
    _1 = 1,
}
impl From<RSTRX_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTRX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTRX` writer - Reset Receiver"]
pub struct RSTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTRX_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTRX_AW::_0)
    }
    #[doc = "Resets the receiver"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTRX_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Reset Transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTTX_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Resets the transmitter"]
    _1 = 1,
}
impl From<RSTTX_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTTX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTTX` writer - Reset Transmitter"]
pub struct RSTTX_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTTX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTTX_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTTX_AW::_0)
    }
    #[doc = "Resets the transmitter"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTTX_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEN_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enables the receiver, if RXDIS is 0"]
    _1 = 1,
}
impl From<RXEN_AW> for bool {
    #[inline(always)]
    fn from(variant: RXEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub struct RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXEN_AW::_0)
    }
    #[doc = "Enables the receiver, if RXDIS is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXEN_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Receiver Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDIS_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Disables the receiver"]
    _1 = 1,
}
impl From<RXDIS_AW> for bool {
    #[inline(always)]
    fn from(variant: RXDIS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub struct RXDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDIS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXDIS_AW::_0)
    }
    #[doc = "Disables the receiver"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXDIS_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Transmitter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEN_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enables the transmitter if TXDIS is 0"]
    _1 = 1,
}
impl From<TXEN_AW> for bool {
    #[inline(always)]
    fn from(variant: TXEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub struct TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXEN_AW::_0)
    }
    #[doc = "Enables the transmitter if TXDIS is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXEN_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Transmitter Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDIS_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Disables the transmitter"]
    _1 = 1,
}
impl From<TXDIS_AW> for bool {
    #[inline(always)]
    fn from(variant: TXDIS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub struct TXDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDIS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXDIS_AW::_0)
    }
    #[doc = "Disables the transmitter"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXDIS_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Reset Status Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTSTA_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Resets the status bits PARE, FRAME, OVRE and RXBRK in the CSR"]
    _1 = 1,
}
impl From<RSTSTA_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTSTA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTSTA` writer - Reset Status Bits"]
pub struct RSTSTA_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTSTA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTSTA_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTSTA_AW::_0)
    }
    #[doc = "Resets the status bits PARE, FRAME, OVRE and RXBRK in the CSR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTSTA_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Start Break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STTBRK_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Starts transmission of a break after the characters present in THR and the Transmit Shift Register have been transmitted. No effect if a break is already being transmitted"]
    _1 = 1,
}
impl From<STTBRK_AW> for bool {
    #[inline(always)]
    fn from(variant: STTBRK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STTBRK` writer - Start Break"]
pub struct STTBRK_W<'a> {
    w: &'a mut W,
}
impl<'a> STTBRK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STTBRK_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STTBRK_AW::_0)
    }
    #[doc = "Starts transmission of a break after the characters present in THR and the Transmit Shift Register have been transmitted. No effect if a break is already being transmitted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STTBRK_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Stop Break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPBRK_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Stops transmission of the break after a minimum of one character length and transmits a high level during 12-bit periods.No effect if no break is being transmitted"]
    _1 = 1,
}
impl From<STPBRK_AW> for bool {
    #[inline(always)]
    fn from(variant: STPBRK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPBRK` writer - Stop Break"]
pub struct STPBRK_W<'a> {
    w: &'a mut W,
}
impl<'a> STPBRK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPBRK_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STPBRK_AW::_0)
    }
    #[doc = "Stops transmission of the break after a minimum of one character length and transmits a high level during 12-bit periods.No effect if no break is being transmitted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STPBRK_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Start Time-out\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STTTO_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Starts waiting for a character before clocking the time-out counter"]
    _1 = 1,
}
impl From<STTTO_AW> for bool {
    #[inline(always)]
    fn from(variant: STTTO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STTTO` writer - Start Time-out"]
pub struct STTTO_W<'a> {
    w: &'a mut W,
}
impl<'a> STTTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STTTO_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STTTO_AW::_0)
    }
    #[doc = "Starts waiting for a character before clocking the time-out counter"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STTTO_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Send Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SENDA_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: In Multi-drop Mode only, the next character written to the THR is sent with the address bit set"]
    _1 = 1,
}
impl From<SENDA_AW> for bool {
    #[inline(always)]
    fn from(variant: SENDA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SENDA` writer - Send Address"]
pub struct SENDA_W<'a> {
    w: &'a mut W,
}
impl<'a> SENDA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENDA_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SENDA_AW::_0)
    }
    #[doc = "In Multi-drop Mode only, the next character written to the THR is sent with the address bit set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SENDA_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Reset Iterations\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTIT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Resets ITERATION in CSR. No effect if the ISO7816 is not enabled"]
    _1 = 1,
}
impl From<RSTIT_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTIT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTIT` writer - Reset Iterations"]
pub struct RSTIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTIT_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTIT_AW::_0)
    }
    #[doc = "Resets ITERATION in CSR. No effect if the ISO7816 is not enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTIT_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Reset Non Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTNACK_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Resets NACK in CSR"]
    _1 = 1,
}
impl From<RSTNACK_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTNACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTNACK` writer - Reset Non Acknowledge"]
pub struct RSTNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTNACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTNACK_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTNACK_AW::_0)
    }
    #[doc = "Resets NACK in CSR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTNACK_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Rearm Time-out\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETTO_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Restart Time-out"]
    _1 = 1,
}
impl From<RETTO_AW> for bool {
    #[inline(always)]
    fn from(variant: RETTO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RETTO` writer - Rearm Time-out"]
pub struct RETTO_W<'a> {
    w: &'a mut W,
}
impl<'a> RETTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RETTO_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RETTO_AW::_0)
    }
    #[doc = "Restart Time-out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RETTO_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Data Terminal Ready Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTREN_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Drives the pin DTR at 0"]
    _1 = 1,
}
impl From<DTREN_AW> for bool {
    #[inline(always)]
    fn from(variant: DTREN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTREN` writer - Data Terminal Ready Enable"]
pub struct DTREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTREN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTREN_AW::_0)
    }
    #[doc = "Drives the pin DTR at 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTREN_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Data Terminal Ready Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTRDIS_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Drives the pin DTR to 1"]
    _1 = 1,
}
impl From<DTRDIS_AW> for bool {
    #[inline(always)]
    fn from(variant: DTRDIS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTRDIS` writer - Data Terminal Ready Disable"]
pub struct DTRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTRDIS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTRDIS_AW::_0)
    }
    #[doc = "Drives the pin DTR to 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTRDIS_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Request to Send Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSEN_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Drives the pin RTS to 0"]
    _1 = 1,
}
impl From<RTSEN_AW> for bool {
    #[inline(always)]
    fn from(variant: RTSEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSEN` writer - Request to Send Enable"]
pub struct RTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTSEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTSEN_AW::_0)
    }
    #[doc = "Drives the pin RTS to 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTSEN_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Request to Send Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSDIS_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Drives the pin RTS to 1"]
    _1 = 1,
}
impl From<RTSDIS_AW> for bool {
    #[inline(always)]
    fn from(variant: RTSDIS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSDIS` writer - Request to Send Disable"]
pub struct RTSDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTSDIS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTSDIS_AW::_0)
    }
    #[doc = "Drives the pin RTS to 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTSDIS_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl W {
    #[doc = "Bit 2 - Reset Receiver"]
    #[inline(always)]
    pub fn rstrx(&mut self) -> RSTRX_W {
        RSTRX_W { w: self }
    }
    #[doc = "Bit 3 - Reset Transmitter"]
    #[inline(always)]
    pub fn rsttx(&mut self) -> RSTTX_W {
        RSTTX_W { w: self }
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W { w: self }
    }
    #[doc = "Bit 5 - Receiver Disable"]
    #[inline(always)]
    pub fn rxdis(&mut self) -> RXDIS_W {
        RXDIS_W { w: self }
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W { w: self }
    }
    #[doc = "Bit 7 - Transmitter Disable"]
    #[inline(always)]
    pub fn txdis(&mut self) -> TXDIS_W {
        TXDIS_W { w: self }
    }
    #[doc = "Bit 8 - Reset Status Bits"]
    #[inline(always)]
    pub fn rststa(&mut self) -> RSTSTA_W {
        RSTSTA_W { w: self }
    }
    #[doc = "Bit 9 - Start Break"]
    #[inline(always)]
    pub fn sttbrk(&mut self) -> STTBRK_W {
        STTBRK_W { w: self }
    }
    #[doc = "Bit 10 - Stop Break"]
    #[inline(always)]
    pub fn stpbrk(&mut self) -> STPBRK_W {
        STPBRK_W { w: self }
    }
    #[doc = "Bit 11 - Start Time-out"]
    #[inline(always)]
    pub fn sttto(&mut self) -> STTTO_W {
        STTTO_W { w: self }
    }
    #[doc = "Bit 12 - Send Address"]
    #[inline(always)]
    pub fn senda(&mut self) -> SENDA_W {
        SENDA_W { w: self }
    }
    #[doc = "Bit 13 - Reset Iterations"]
    #[inline(always)]
    pub fn rstit(&mut self) -> RSTIT_W {
        RSTIT_W { w: self }
    }
    #[doc = "Bit 14 - Reset Non Acknowledge"]
    #[inline(always)]
    pub fn rstnack(&mut self) -> RSTNACK_W {
        RSTNACK_W { w: self }
    }
    #[doc = "Bit 15 - Rearm Time-out"]
    #[inline(always)]
    pub fn retto(&mut self) -> RETTO_W {
        RETTO_W { w: self }
    }
    #[doc = "Bit 16 - Data Terminal Ready Enable"]
    #[inline(always)]
    pub fn dtren(&mut self) -> DTREN_W {
        DTREN_W { w: self }
    }
    #[doc = "Bit 17 - Data Terminal Ready Disable"]
    #[inline(always)]
    pub fn dtrdis(&mut self) -> DTRDIS_W {
        DTRDIS_W { w: self }
    }
    #[doc = "Bit 18 - Request to Send Enable"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RTSEN_W {
        RTSEN_W { w: self }
    }
    #[doc = "Bit 19 - Request to Send Disable"]
    #[inline(always)]
    pub fn rtsdis(&mut self) -> RTSDIS_W {
        RTSDIS_W { w: self }
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
}
#[doc = "`reset()` method sets CR_USART to value 0"]
impl crate::Resettable for USART_MODE_CR_USART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
