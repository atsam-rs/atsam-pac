#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receive Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEN_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Enables Data Receive if RXDIS is not set"]
    ON = 1,
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
    pub fn off(self) -> &'a mut W {
        self.variant(RXEN_AW::OFF)
    }
    #[doc = "Enables Data Receive if RXDIS is not set"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(RXEN_AW::ON)
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
#[doc = "Receive Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDIS_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Disables Data Receive"]
    ON = 1,
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
    pub fn off(self) -> &'a mut W {
        self.variant(RXDIS_AW::OFF)
    }
    #[doc = "Disables Data Receive"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(RXDIS_AW::ON)
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
#[doc = "Clocks Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKEN_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Enables clocks if CKDIS is not set"]
    ON = 1,
}
impl From<CKEN_AW> for bool {
    #[inline(always)]
    fn from(variant: CKEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CKEN`"]
pub struct CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CKEN_AW::OFF)
    }
    #[doc = "Enables clocks if CKDIS is not set"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CKEN_AW::ON)
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
#[doc = "Clocks Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKDIS_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Disables clocks"]
    ON = 1,
}
impl From<CKDIS_AW> for bool {
    #[inline(always)]
    fn from(variant: CKDIS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CKDIS`"]
pub struct CKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CKDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKDIS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CKDIS_AW::OFF)
    }
    #[doc = "Disables clocks"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CKDIS_AW::ON)
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
#[doc = "Transmit Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEN_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Enables Data Transmit if TXDIS is not set"]
    ON = 1,
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
    pub fn off(self) -> &'a mut W {
        self.variant(TXEN_AW::OFF)
    }
    #[doc = "Enables Data Transmit if TXDIS is not set"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(TXEN_AW::ON)
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
#[doc = "Transmit Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDIS_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Disables Data Transmit"]
    ON = 1,
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
    pub fn off(self) -> &'a mut W {
        self.variant(TXDIS_AW::OFF)
    }
    #[doc = "Disables Data Transmit"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(TXDIS_AW::ON)
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
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRST_AW {
    #[doc = "0: No effect"]
    OFF = 0,
    #[doc = "1: Performs a software reset. Has priority on any other bit in CR"]
    ON = 1,
}
impl From<SWRST_AW> for bool {
    #[inline(always)]
    fn from(variant: SWRST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SWRST`"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRST_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SWRST_AW::OFF)
    }
    #[doc = "Performs a software reset. Has priority on any other bit in CR"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(SWRST_AW::ON)
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
impl W {
    #[doc = "Bit 0 - Receive Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W { w: self }
    }
    #[doc = "Bit 1 - Receive Disable"]
    #[inline(always)]
    pub fn rxdis(&mut self) -> RXDIS_W {
        RXDIS_W { w: self }
    }
    #[doc = "Bit 2 - Clocks Enable"]
    #[inline(always)]
    pub fn cken(&mut self) -> CKEN_W {
        CKEN_W { w: self }
    }
    #[doc = "Bit 3 - Clocks Disable"]
    #[inline(always)]
    pub fn ckdis(&mut self) -> CKDIS_W {
        CKDIS_W { w: self }
    }
    #[doc = "Bit 4 - Transmit Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W { w: self }
    }
    #[doc = "Bit 5 - Transmit Disable"]
    #[inline(always)]
    pub fn txdis(&mut self) -> TXDIS_W {
        TXDIS_W { w: self }
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
}
