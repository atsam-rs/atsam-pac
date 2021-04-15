#[doc = "Writer for register IDR_LIN"]
pub type W = crate::W<u32, super::IDR_LIN>;
#[doc = "Register IDR_LIN `reset()`'s with value 0"]
impl crate::ResetValue for super::IDR_LIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receiver Ready Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRDY_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<RXRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: RXRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RXRDY`"]
pub struct RXRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXRDY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXRDY_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXRDY_AW::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Transmitter Ready Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRDY_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<TXRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: TXRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TXRDY`"]
pub struct TXRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRDY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRDY_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRDY_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Receiver Break Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBRK_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<RXBRK_AW> for bool {
    #[inline(always)]
    fn from(variant: RXBRK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RXBRK`"]
pub struct RXBRK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBRK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXBRK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXBRK_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXBRK_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Overrun Error Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRE_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<OVRE_AW> for bool {
    #[inline(always)]
    fn from(variant: OVRE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `OVRE`"]
pub struct OVRE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVRE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVRE_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVRE_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Framing Error Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAME_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<FRAME_AW> for bool {
    #[inline(always)]
    fn from(variant: FRAME_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FRAME`"]
pub struct FRAME_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAME_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRAME_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRAME_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Parity Error Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARE_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<PARE_AW> for bool {
    #[inline(always)]
    fn from(variant: PARE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PARE`"]
pub struct PARE_W<'a> {
    w: &'a mut W,
}
impl<'a> PARE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PARE_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PARE_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Time-out Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<TIMEOUT_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TIMEOUT`"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMEOUT_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMEOUT_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Transmitter Empty Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPTY_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<TXEMPTY_AW> for bool {
    #[inline(always)]
    fn from(variant: TXEMPTY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TXEMPTY`"]
pub struct TXEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEMPTY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXEMPTY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXEMPTY_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXEMPTY_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Iteration Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITER_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<ITER_AW> for bool {
    #[inline(always)]
    fn from(variant: ITER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ITER`"]
pub struct ITER_W<'a> {
    w: &'a mut W,
}
impl<'a> ITER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITER_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ITER_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ITER_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Buffer Empty Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBUFE_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<TXBUFE_AW> for bool {
    #[inline(always)]
    fn from(variant: TXBUFE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TXBUFE`"]
pub struct TXBUFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBUFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXBUFE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXBUFE_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXBUFE_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Buffer Full Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBUFF_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<RXBUFF_AW> for bool {
    #[inline(always)]
    fn from(variant: RXBUFF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RXBUFF`"]
pub struct RXBUFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBUFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXBUFF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXBUFF_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXBUFF_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Non Acknowledge or LIN Break Sent or LIN Break Received Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACK_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<NACK_AW> for bool {
    #[inline(always)]
    fn from(variant: NACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `NACK`"]
pub struct NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> NACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NACK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NACK_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NACK_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `LINID`"]
pub struct LINID_W<'a> {
    w: &'a mut W,
}
impl<'a> LINID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `LINTC`"]
pub struct LINTC_W<'a> {
    w: &'a mut W,
}
impl<'a> LINTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Ring Indicator Input Change Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIIC_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<RIIC_AW> for bool {
    #[inline(always)]
    fn from(variant: RIIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RIIC`"]
pub struct RIIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RIIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RIIC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIIC_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIIC_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Data Set Ready Input Change Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSRIC_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<DSRIC_AW> for bool {
    #[inline(always)]
    fn from(variant: DSRIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DSRIC`"]
pub struct DSRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSRIC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSRIC_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSRIC_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Data Carrier Detect Input Change Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDIC_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<DCDIC_AW> for bool {
    #[inline(always)]
    fn from(variant: DCDIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DCDIC`"]
pub struct DCDIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDIC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCDIC_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCDIC_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Clear to Send Input Change Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSIC_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<CTSIC_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CTSIC`"]
pub struct CTSIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSIC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSIC_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSIC_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Write proxy for field `LINBE`"]
pub struct LINBE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Write proxy for field `LINISFE`"]
pub struct LINISFE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINISFE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Write proxy for field `LINIPE`"]
pub struct LINIPE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINIPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Write proxy for field `LINCE`"]
pub struct LINCE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Write proxy for field `LINSNRE`"]
pub struct LINSNRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINSNRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "LIN Synch Tolerance Error Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINSTE_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<LINSTE_AW> for bool {
    #[inline(always)]
    fn from(variant: LINSTE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `LINSTE`"]
pub struct LINSTE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINSTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINSTE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINSTE_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINSTE_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "LIN Header Timeout Error Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINHTE_AW {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Disables the interrupt"]
    _1 = 1,
}
impl From<LINHTE_AW> for bool {
    #[inline(always)]
    fn from(variant: LINHTE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `LINHTE`"]
pub struct LINHTE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINHTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINHTE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINHTE_AW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINHTE_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Ready Interrupt Disable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RXRDY_W {
        RXRDY_W { w: self }
    }
    #[doc = "Bit 1 - Transmitter Ready Interrupt Disable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TXRDY_W {
        TXRDY_W { w: self }
    }
    #[doc = "Bit 2 - Receiver Break Interrupt Disable"]
    #[inline(always)]
    pub fn rxbrk(&mut self) -> RXBRK_W {
        RXBRK_W { w: self }
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OVRE_W {
        OVRE_W { w: self }
    }
    #[doc = "Bit 6 - Framing Error Interrupt Disable"]
    #[inline(always)]
    pub fn frame(&mut self) -> FRAME_W {
        FRAME_W { w: self }
    }
    #[doc = "Bit 7 - Parity Error Interrupt Disable"]
    #[inline(always)]
    pub fn pare(&mut self) -> PARE_W {
        PARE_W { w: self }
    }
    #[doc = "Bit 8 - Time-out Interrupt Disable"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Bit 9 - Transmitter Empty Interrupt Disable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TXEMPTY_W {
        TXEMPTY_W { w: self }
    }
    #[doc = "Bit 10 - Iteration Interrupt Disable"]
    #[inline(always)]
    pub fn iter(&mut self) -> ITER_W {
        ITER_W { w: self }
    }
    #[doc = "Bit 11 - Buffer Empty Interrupt Disable"]
    #[inline(always)]
    pub fn txbufe(&mut self) -> TXBUFE_W {
        TXBUFE_W { w: self }
    }
    #[doc = "Bit 12 - Buffer Full Interrupt Disable"]
    #[inline(always)]
    pub fn rxbuff(&mut self) -> RXBUFF_W {
        RXBUFF_W { w: self }
    }
    #[doc = "Bit 13 - Non Acknowledge or LIN Break Sent or LIN Break Received Interrupt Disable"]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W {
        NACK_W { w: self }
    }
    #[doc = "Bit 14 - LIN Identifier Sent or LIN Identifier Received Interrupt Disable"]
    #[inline(always)]
    pub fn linid(&mut self) -> LINID_W {
        LINID_W { w: self }
    }
    #[doc = "Bit 15 - LIN Transfer Conpleted Interrupt Disable"]
    #[inline(always)]
    pub fn lintc(&mut self) -> LINTC_W {
        LINTC_W { w: self }
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Disable"]
    #[inline(always)]
    pub fn riic(&mut self) -> RIIC_W {
        RIIC_W { w: self }
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Disable"]
    #[inline(always)]
    pub fn dsric(&mut self) -> DSRIC_W {
        DSRIC_W { w: self }
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Disable"]
    #[inline(always)]
    pub fn dcdic(&mut self) -> DCDIC_W {
        DCDIC_W { w: self }
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Disable"]
    #[inline(always)]
    pub fn ctsic(&mut self) -> CTSIC_W {
        CTSIC_W { w: self }
    }
    #[doc = "Bit 25 - LIN Bus Error Interrupt Disable"]
    #[inline(always)]
    pub fn linbe(&mut self) -> LINBE_W {
        LINBE_W { w: self }
    }
    #[doc = "Bit 26 - LIN Inconsistent Synch Field Error Interrupt Disable"]
    #[inline(always)]
    pub fn linisfe(&mut self) -> LINISFE_W {
        LINISFE_W { w: self }
    }
    #[doc = "Bit 27 - LIN Identifier Parity Interrupt Disable"]
    #[inline(always)]
    pub fn linipe(&mut self) -> LINIPE_W {
        LINIPE_W { w: self }
    }
    #[doc = "Bit 28 - LIN Checksum Error Interrupt Disable"]
    #[inline(always)]
    pub fn lince(&mut self) -> LINCE_W {
        LINCE_W { w: self }
    }
    #[doc = "Bit 29 - LIN Slave Not Responding Error Interrupt Disable"]
    #[inline(always)]
    pub fn linsnre(&mut self) -> LINSNRE_W {
        LINSNRE_W { w: self }
    }
    #[doc = "Bit 30 - LIN Synch Tolerance Error Interrupt Disable"]
    #[inline(always)]
    pub fn linste(&mut self) -> LINSTE_W {
        LINSTE_W { w: self }
    }
    #[doc = "Bit 31 - LIN Header Timeout Error Interrupt Disable"]
    #[inline(always)]
    pub fn linhte(&mut self) -> LINHTE_W {
        LINHTE_W { w: self }
    }
}
