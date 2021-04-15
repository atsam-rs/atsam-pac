#[doc = "Reader of register OSCCTRL0"]
pub type R = crate::R<u32, super::OSCCTRL0>;
#[doc = "Writer for register OSCCTRL0"]
pub type W = crate::W<u32, super::OSCCTRL0>;
#[doc = "Register OSCCTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::OSCCTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
#[doc = "Reader of field `GAIN`"]
pub type GAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GAIN`"]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `AGC`"]
pub type AGC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AGC`"]
pub struct AGC_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `OSCEN`"]
pub type OSCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSCEN`"]
pub struct OSCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Automatic Gain Control"]
    #[inline(always)]
    pub fn agc(&self) -> AGC_R {
        AGC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Oscillator Start-up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Oscillator Enable"]
    #[inline(always)]
    pub fn oscen(&self) -> OSCEN_R {
        OSCEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 1:2 - Gain"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W { w: self }
    }
    #[doc = "Bit 3 - Automatic Gain Control"]
    #[inline(always)]
    pub fn agc(&mut self) -> AGC_W {
        AGC_W { w: self }
    }
    #[doc = "Bits 8:11 - Oscillator Start-up Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W {
        STARTUP_W { w: self }
    }
    #[doc = "Bit 16 - Oscillator Enable"]
    #[inline(always)]
    pub fn oscen(&mut self) -> OSCEN_W {
        OSCEN_W { w: self }
    }
}
