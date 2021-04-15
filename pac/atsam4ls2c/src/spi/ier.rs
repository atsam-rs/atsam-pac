#[doc = "Writer for register IER"]
pub type W = crate::W<u32, super::IER>;
#[doc = "Register IER `reset()`'s with value 0"]
impl crate::ResetValue for super::IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receive Data Register Full Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDRF_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
    _1 = 1,
}
impl From<RDRF_AW> for bool {
    #[inline(always)]
    fn from(variant: RDRF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RDRF`"]
pub struct RDRF_W<'a> {
    w: &'a mut W,
}
impl<'a> RDRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDRF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDRF_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDRF_AW::_1)
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
#[doc = "Transmit Data Register Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRE_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
    _1 = 1,
}
impl From<TDRE_AW> for bool {
    #[inline(always)]
    fn from(variant: TDRE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TDRE`"]
pub struct TDRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDRE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDRE_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDRE_AW::_1)
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
#[doc = "Mode Fault Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODF_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
    _1 = 1,
}
impl From<MODF_AW> for bool {
    #[inline(always)]
    fn from(variant: MODF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `MODF`"]
pub struct MODF_W<'a> {
    w: &'a mut W,
}
impl<'a> MODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODF_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODF_AW::_1)
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
#[doc = "Overrun Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRES_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
    _1 = 1,
}
impl From<OVRES_AW> for bool {
    #[inline(always)]
    fn from(variant: OVRES_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `OVRES`"]
pub struct OVRES_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVRES_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVRES_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVRES_AW::_1)
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
#[doc = "End of Receive Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRX_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
    _1 = 1,
}
impl From<ENDRX_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ENDRX`"]
pub struct ENDRX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDRX_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENDRX_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENDRX_AW::_1)
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
#[doc = "End of Transmit Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDTX_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
    _1 = 1,
}
impl From<ENDTX_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDTX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ENDTX`"]
pub struct ENDTX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDTX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDTX_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENDTX_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENDTX_AW::_1)
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
#[doc = "Receive Buffer Full Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBUFF_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
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
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXBUFF_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Transmit Buffer Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBUFE_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
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
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXBUFE_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "NSS Rising Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSSR_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
    _1 = 1,
}
impl From<NSSR_AW> for bool {
    #[inline(always)]
    fn from(variant: NSSR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `NSSR`"]
pub struct NSSR_W<'a> {
    w: &'a mut W,
}
impl<'a> NSSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSSR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NSSR_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NSSR_AW::_1)
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
#[doc = "Transmission Registers Empty Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPTY_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the corresponding interrupt."]
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
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXEMPTY_AW::_0)
    }
    #[doc = "Enables the corresponding interrupt."]
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
#[doc = "Write proxy for field `UNDES`"]
pub struct UNDES_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDES_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Enable"]
    #[inline(always)]
    pub fn rdrf(&mut self) -> RDRF_W {
        RDRF_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Data Register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tdre(&mut self) -> TDRE_W {
        TDRE_W { w: self }
    }
    #[doc = "Bit 2 - Mode Fault Error Interrupt Enable"]
    #[inline(always)]
    pub fn modf(&mut self) -> MODF_W {
        MODF_W { w: self }
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovres(&mut self) -> OVRES_W {
        OVRES_W { w: self }
    }
    #[doc = "Bit 4 - End of Receive Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn endrx(&mut self) -> ENDRX_W {
        ENDRX_W { w: self }
    }
    #[doc = "Bit 5 - End of Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn endtx(&mut self) -> ENDTX_W {
        ENDTX_W { w: self }
    }
    #[doc = "Bit 6 - Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    pub fn rxbuff(&mut self) -> RXBUFF_W {
        RXBUFF_W { w: self }
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn txbufe(&mut self) -> TXBUFE_W {
        TXBUFE_W { w: self }
    }
    #[doc = "Bit 8 - NSS Rising Interrupt Enable"]
    #[inline(always)]
    pub fn nssr(&mut self) -> NSSR_W {
        NSSR_W { w: self }
    }
    #[doc = "Bit 9 - Transmission Registers Empty Enable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TXEMPTY_W {
        TXEMPTY_W { w: self }
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn undes(&mut self) -> UNDES_W {
        UNDES_W { w: self }
    }
}
