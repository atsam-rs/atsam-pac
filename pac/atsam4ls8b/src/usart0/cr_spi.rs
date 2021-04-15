#[doc = "Writer for register CR_SPI"]
pub type W = crate::W<u32, super::CR_SPI>;
#[doc = "Register CR_SPI `reset()`'s with value 0"]
impl crate::ResetValue for super::CR_SPI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Write proxy for field `RSTRX`"]
pub struct RSTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTRX_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
#[doc = "Write proxy for field `RSTTX`"]
pub struct RSTTX_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTTX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTTX_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
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
#[doc = "Write proxy for field `RXEN`"]
pub struct RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
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
#[doc = "Write proxy for field `RXDIS`"]
pub struct RXDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDIS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
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
#[doc = "Write proxy for field `TXEN`"]
pub struct TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
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
#[doc = "Write proxy for field `TXDIS`"]
pub struct TXDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDIS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
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
#[doc = "Write proxy for field `RSTSTA`"]
pub struct RSTSTA_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTSTA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTSTA_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
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
#[doc = "Write proxy for field `STTBRK`"]
pub struct STTBRK_W<'a> {
    w: &'a mut W,
}
impl<'a> STTBRK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STTBRK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
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
#[doc = "Write proxy for field `STPBRK`"]
pub struct STPBRK_W<'a> {
    w: &'a mut W,
}
impl<'a> STPBRK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPBRK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
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
#[doc = "Write proxy for field `STTTO`"]
pub struct STTTO_W<'a> {
    w: &'a mut W,
}
impl<'a> STTTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STTTO_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
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
#[doc = "Write proxy for field `SENDA`"]
pub struct SENDA_W<'a> {
    w: &'a mut W,
}
impl<'a> SENDA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENDA_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
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
#[doc = "Write proxy for field `RSTIT`"]
pub struct RSTIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTIT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
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
#[doc = "Write proxy for field `RSTNACK`"]
pub struct RSTNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTNACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTNACK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
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
#[doc = "Write proxy for field `RETTO`"]
pub struct RETTO_W<'a> {
    w: &'a mut W,
}
impl<'a> RETTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RETTO_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
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
#[doc = "Write proxy for field `DTREN`"]
pub struct DTREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTREN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
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
#[doc = "Write proxy for field `DTRDIS`"]
pub struct DTRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTRDIS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Force SPI Chip Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCS_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Forces the Slave Select Line NSS (RTS pin) to 0, even if USART is no transmitting, in order to address SPI slave devices supporting the CSAAT Mode (Chip Select Active After Transfer)"]
    _1 = 1,
}
impl From<FCS_AW> for bool {
    #[inline(always)]
    fn from(variant: FCS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FCS`"]
pub struct FCS_W<'a> {
    w: &'a mut W,
}
impl<'a> FCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCS_AW::_0)
    }
    #[doc = "Forces the Slave Select Line NSS (RTS pin) to 0, even if USART is no transmitting, in order to address SPI slave devices supporting the CSAAT Mode (Chip Select Active After Transfer)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCS_AW::_1)
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
#[doc = "Release SPI Chip Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCS_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Releases the Slave Select Line NSS (RTS pin)"]
    _1 = 1,
}
impl From<RCS_AW> for bool {
    #[inline(always)]
    fn from(variant: RCS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RCS`"]
pub struct RCS_W<'a> {
    w: &'a mut W,
}
impl<'a> RCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCS_AW::_0)
    }
    #[doc = "Releases the Slave Select Line NSS (RTS pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCS_AW::_1)
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
    #[doc = "Bit 18 - Force SPI Chip Select"]
    #[inline(always)]
    pub fn fcs(&mut self) -> FCS_W {
        FCS_W { w: self }
    }
    #[doc = "Bit 19 - Release SPI Chip Select"]
    #[inline(always)]
    pub fn rcs(&mut self) -> RCS_W {
        RCS_W { w: self }
    }
}
