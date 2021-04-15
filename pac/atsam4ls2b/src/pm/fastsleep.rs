#[doc = "Reader of register FASTSLEEP"]
pub type R = crate::R<u32, super::FASTSLEEP>;
#[doc = "Writer for register FASTSLEEP"]
pub type W = crate::W<u32, super::FASTSLEEP>;
#[doc = "Register FASTSLEEP `reset()`'s with value 0"]
impl crate::ResetValue for super::FASTSLEEP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OSC`"]
pub type OSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSC`"]
pub struct OSC_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_W<'a> {
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
#[doc = "Reader of field `PLL`"]
pub type PLL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL`"]
pub struct PLL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_W<'a> {
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
#[doc = "Reader of field `FASTRCOSC`"]
pub type FASTRCOSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FASTRCOSC`"]
pub struct FASTRCOSC_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTRCOSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DFLL`"]
pub type DFLL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFLL`"]
pub struct DFLL_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Oscillator"]
    #[inline(always)]
    pub fn osc(&self) -> OSC_R {
        OSC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLL"]
    #[inline(always)]
    pub fn pll(&self) -> PLL_R {
        PLL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - RC80 or FLO"]
    #[inline(always)]
    pub fn fastrcosc(&self) -> FASTRCOSC_R {
        FASTRCOSC_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - DFLL"]
    #[inline(always)]
    pub fn dfll(&self) -> DFLL_R {
        DFLL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Oscillator"]
    #[inline(always)]
    pub fn osc(&mut self) -> OSC_W {
        OSC_W { w: self }
    }
    #[doc = "Bit 8 - PLL"]
    #[inline(always)]
    pub fn pll(&mut self) -> PLL_W {
        PLL_W { w: self }
    }
    #[doc = "Bits 16:20 - RC80 or FLO"]
    #[inline(always)]
    pub fn fastrcosc(&mut self) -> FASTRCOSC_W {
        FASTRCOSC_W { w: self }
    }
    #[doc = "Bit 24 - DFLL"]
    #[inline(always)]
    pub fn dfll(&mut self) -> DFLL_W {
        DFLL_W { w: self }
    }
}
