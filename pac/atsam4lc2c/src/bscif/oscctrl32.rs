#[doc = "Reader of register OSCCTRL32"]
pub type R = crate::R<u32, super::OSCCTRL32>;
#[doc = "Writer for register OSCCTRL32"]
pub type W = crate::W<u32, super::OSCCTRL32>;
#[doc = "Register OSCCTRL32 `reset()`'s with value 0x04"]
impl crate::ResetValue for super::OSCCTRL32 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `OSC32EN`"]
pub type OSC32EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSC32EN`"]
pub struct OSC32EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC32EN_W<'a> {
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
#[doc = "Reader of field `PINSEL`"]
pub type PINSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSEL`"]
pub struct PINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSEL_W<'a> {
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
#[doc = "Reader of field `EN32K`"]
pub type EN32K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN32K`"]
pub struct EN32K_W<'a> {
    w: &'a mut W,
}
impl<'a> EN32K_W<'a> {
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
#[doc = "Reader of field `EN1K`"]
pub type EN1K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN1K`"]
pub struct EN1K_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1K_W<'a> {
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
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `SELCURR`"]
pub type SELCURR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SELCURR`"]
pub struct SELCURR_W<'a> {
    w: &'a mut W,
}
impl<'a> SELCURR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `STARTUP`"]
pub type STARTUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STARTUP`"]
pub struct STARTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 32 KHz Oscillator Enable"]
    #[inline(always)]
    pub fn osc32en(&self) -> OSC32EN_R {
        OSC32EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pins Select"]
    #[inline(always)]
    pub fn pinsel(&self) -> PINSEL_R {
        PINSEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 32 KHz output Enable"]
    #[inline(always)]
    pub fn en32k(&self) -> EN32K_R {
        EN32K_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 KHz output Enable"]
    #[inline(always)]
    pub fn en1k(&self) -> EN1K_R {
        EN1K_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:15 - Current selection"]
    #[inline(always)]
    pub fn selcurr(&self) -> SELCURR_R {
        SELCURR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Oscillator Start-up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 32 KHz Oscillator Enable"]
    #[inline(always)]
    pub fn osc32en(&mut self) -> OSC32EN_W {
        OSC32EN_W { w: self }
    }
    #[doc = "Bit 1 - Pins Select"]
    #[inline(always)]
    pub fn pinsel(&mut self) -> PINSEL_W {
        PINSEL_W { w: self }
    }
    #[doc = "Bit 2 - 32 KHz output Enable"]
    #[inline(always)]
    pub fn en32k(&mut self) -> EN32K_W {
        EN32K_W { w: self }
    }
    #[doc = "Bit 3 - 1 KHz output Enable"]
    #[inline(always)]
    pub fn en1k(&mut self) -> EN1K_W {
        EN1K_W { w: self }
    }
    #[doc = "Bits 8:10 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 12:15 - Current selection"]
    #[inline(always)]
    pub fn selcurr(&mut self) -> SELCURR_W {
        SELCURR_W { w: self }
    }
    #[doc = "Bits 16:18 - Oscillator Start-up Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W {
        STARTUP_W { w: self }
    }
}
