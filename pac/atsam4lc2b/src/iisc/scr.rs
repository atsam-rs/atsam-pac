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
#[doc = "Receive Overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOR_AW {
    #[doc = "0: No effect"]
    NO = 0,
    #[doc = "1: Clears the corresponding SR bit"]
    CLEAR = 1,
}
impl From<RXOR_AW> for bool {
    #[inline(always)]
    fn from(variant: RXOR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RXOR`"]
pub struct RXOR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(RXOR_AW::NO)
    }
    #[doc = "Clears the corresponding SR bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXOR_AW::CLEAR)
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
#[doc = "Transmit Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUR_AW {
    #[doc = "0: No effect"]
    NO = 0,
    #[doc = "1: Clears the corresponding SR bit"]
    CLEAR = 1,
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
    pub fn no(self) -> &'a mut W {
        self.variant(TXUR_AW::NO)
    }
    #[doc = "Clears the corresponding SR bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXUR_AW::CLEAR)
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
#[doc = "Write proxy for field `RXORCH`"]
pub struct RXORCH_W<'a> {
    w: &'a mut W,
}
impl<'a> RXORCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `TXURCH`"]
pub struct TXURCH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXURCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl W {
    #[doc = "Bit 2 - Receive Overrun"]
    #[inline(always)]
    pub fn rxor(&mut self) -> RXOR_W {
        RXOR_W { w: self }
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    pub fn txur(&mut self) -> TXUR_W {
        TXUR_W { w: self }
    }
    #[doc = "Bits 8:9 - Receive Overrun Channels"]
    #[inline(always)]
    pub fn rxorch(&mut self) -> RXORCH_W {
        RXORCH_W { w: self }
    }
    #[doc = "Bits 20:21 - Transmit Underrun Channels"]
    #[inline(always)]
    pub fn txurch(&mut self) -> TXURCH_W {
        TXURCH_W { w: self }
    }
}
