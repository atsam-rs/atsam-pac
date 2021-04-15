#[doc = "Writer for register SCR"]
pub type W = crate::W<u32, super::SCR>;
#[doc = "Register SCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmit Ready Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRDY_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the Audio DAC TX Ready interrupt"]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRDY_AW::_0)
    }
    #[doc = "Clear the Audio DAC TX Ready interrupt"]
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
#[doc = "Transmit Underrun Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUR_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear the Audio DAC Underrun interrupt"]
    _1 = 1,
}
impl From<TXUR_AW> for bool {
    #[inline(always)]
    fn from(variant: TXUR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TXUR`"]
pub struct TXUR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXUR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXUR_AW::_0)
    }
    #[doc = "Clear the Audio DAC Underrun interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXUR_AW::_1)
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
impl W {
    #[doc = "Bit 1 - Transmit Ready Interrupt Clear"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TXRDY_W {
        TXRDY_W { w: self }
    }
    #[doc = "Bit 2 - Transmit Underrun Interrupt Clear"]
    #[inline(always)]
    pub fn txur(&mut self) -> TXUR_W {
        TXUR_W { w: self }
    }
}
