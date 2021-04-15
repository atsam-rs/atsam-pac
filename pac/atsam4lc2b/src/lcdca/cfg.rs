#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XBIAS`"]
pub type XBIAS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XBIAS`"]
pub struct XBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> XBIAS_W<'a> {
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
#[doc = "Reader of field `WMOD`"]
pub type WMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WMOD`"]
pub struct WMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> WMOD_W<'a> {
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
#[doc = "Reader of field `BLANK`"]
pub type BLANK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLANK`"]
pub struct BLANK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLANK_W<'a> {
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
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
#[doc = "Reader of field `DUTY`"]
pub type DUTY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DUTY`"]
pub struct DUTY_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `FCST`"]
pub type FCST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FCST`"]
pub struct FCST_W<'a> {
    w: &'a mut W,
}
impl<'a> FCST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `NSU`"]
pub type NSU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NSU`"]
pub struct NSU_W<'a> {
    w: &'a mut W,
}
impl<'a> NSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - External Bias Generation"]
    #[inline(always)]
    pub fn xbias(&self) -> XBIAS_R {
        XBIAS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Waveform Mode"]
    #[inline(always)]
    pub fn wmod(&self) -> WMOD_R {
        WMOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Blank LCD"]
    #[inline(always)]
    pub fn blank(&self) -> BLANK_R {
        BLANK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Duty Select"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:21 - Fine Contrast"]
    #[inline(always)]
    pub fn fcst(&self) -> FCST_R {
        FCST_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Number of Segment Terminals in Use"]
    #[inline(always)]
    pub fn nsu(&self) -> NSU_R {
        NSU_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Bias Generation"]
    #[inline(always)]
    pub fn xbias(&mut self) -> XBIAS_W {
        XBIAS_W { w: self }
    }
    #[doc = "Bit 1 - Waveform Mode"]
    #[inline(always)]
    pub fn wmod(&mut self) -> WMOD_W {
        WMOD_W { w: self }
    }
    #[doc = "Bit 2 - Blank LCD"]
    #[inline(always)]
    pub fn blank(&mut self) -> BLANK_W {
        BLANK_W { w: self }
    }
    #[doc = "Bit 3 - Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Bits 8:9 - Duty Select"]
    #[inline(always)]
    pub fn duty(&mut self) -> DUTY_W {
        DUTY_W { w: self }
    }
    #[doc = "Bits 16:21 - Fine Contrast"]
    #[inline(always)]
    pub fn fcst(&mut self) -> FCST_W {
        FCST_W { w: self }
    }
    #[doc = "Bits 24:29 - Number of Segment Terminals in Use"]
    #[inline(always)]
    pub fn nsu(&mut self) -> NSU_W {
        NSU_W { w: self }
    }
}
