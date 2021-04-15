#[doc = "Reader of register RC1MCR"]
pub type R = crate::R<u32, super::RC1MCR>;
#[doc = "Writer for register RC1MCR"]
pub type W = crate::W<u32, super::RC1MCR>;
#[doc = "Register RC1MCR `reset()`'s with value 0x0f00"]
impl crate::ResetValue for super::RC1MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f00
    }
}
#[doc = "Reader of field `CLKOE`"]
pub type CLKOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKOE`"]
pub struct CLKOE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOE_W<'a> {
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
#[doc = "Reader of field `FCD`"]
pub type FCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCD`"]
pub struct FCD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCD_W<'a> {
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
#[doc = "Reader of field `CLKCAL`"]
pub type CLKCAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKCAL`"]
pub struct CLKCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1MHz RC Osc Clock Output Enable"]
    #[inline(always)]
    pub fn clkoe(&self) -> CLKOE_R {
        CLKOE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 7 - Flash Calibration Done"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - 1MHz RC Osc Calibration"]
    #[inline(always)]
    pub fn clkcal(&self) -> CLKCAL_R {
        CLKCAL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1MHz RC Osc Clock Output Enable"]
    #[inline(always)]
    pub fn clkoe(&mut self) -> CLKOE_W {
        CLKOE_W { w: self }
    }
    #[doc = "Bit 7 - Flash Calibration Done"]
    #[inline(always)]
    pub fn fcd(&mut self) -> FCD_W {
        FCD_W { w: self }
    }
    #[doc = "Bits 8:12 - 1MHz RC Osc Calibration"]
    #[inline(always)]
    pub fn clkcal(&mut self) -> CLKCAL_W {
        CLKCAL_W { w: self }
    }
}
