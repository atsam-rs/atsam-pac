#[doc = "Reader of register PCONTROL"]
pub type R = crate::R<u32, super::PCONTROL>;
#[doc = "Writer for register PCONTROL"]
pub type W = crate::W<u32, super::PCONTROL>;
#[doc = "Register PCONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::PCONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0EN`"]
pub type CH0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0EN`"]
pub struct CH0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0EN_W<'a> {
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
#[doc = "Reader of field `CH1EN`"]
pub type CH1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1EN`"]
pub struct CH1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1EN_W<'a> {
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
#[doc = "Reader of field `CH0OF`"]
pub type CH0OF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0OF`"]
pub struct CH0OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0OF_W<'a> {
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
#[doc = "Reader of field `CH1OF`"]
pub type CH1OF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1OF`"]
pub struct CH1OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OF_W<'a> {
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
#[doc = "Reader of field `CH0RES`"]
pub type CH0RES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0RES`"]
pub struct CH0RES_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0RES_W<'a> {
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
#[doc = "Reader of field `CH1RES`"]
pub type CH1RES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1RES`"]
pub struct CH1RES_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1RES_W<'a> {
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
#[doc = "Reader of field `MON0CH`"]
pub type MON0CH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MON0CH`"]
pub struct MON0CH_W<'a> {
    w: &'a mut W,
}
impl<'a> MON0CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MON1CH`"]
pub type MON1CH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MON1CH`"]
pub struct MON1CH_W<'a> {
    w: &'a mut W,
}
impl<'a> MON1CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Enabled"]
    #[inline(always)]
    pub fn ch0en(&self) -> CH0EN_R {
        CH0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Enabled."]
    #[inline(always)]
    pub fn ch1en(&self) -> CH1EN_R {
        CH1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 0 Overflow Freeze"]
    #[inline(always)]
    pub fn ch0of(&self) -> CH0OF_R {
        CH0OF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 1 overflow freeze"]
    #[inline(always)]
    pub fn ch1of(&self) -> CH1OF_R {
        CH1OF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 0 counter reset"]
    #[inline(always)]
    pub fn ch0res(&self) -> CH0RES_R {
        CH0RES_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 1 counter reset"]
    #[inline(always)]
    pub fn ch1res(&self) -> CH1RES_R {
        CH1RES_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - PDCA Channel to monitor with counter 0"]
    #[inline(always)]
    pub fn mon0ch(&self) -> MON0CH_R {
        MON0CH_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - PDCA Channel to monitor with counter 1"]
    #[inline(always)]
    pub fn mon1ch(&self) -> MON1CH_R {
        MON1CH_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Enabled"]
    #[inline(always)]
    pub fn ch0en(&mut self) -> CH0EN_W {
        CH0EN_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Enabled."]
    #[inline(always)]
    pub fn ch1en(&mut self) -> CH1EN_W {
        CH1EN_W { w: self }
    }
    #[doc = "Bit 4 - Channel 0 Overflow Freeze"]
    #[inline(always)]
    pub fn ch0of(&mut self) -> CH0OF_W {
        CH0OF_W { w: self }
    }
    #[doc = "Bit 5 - Channel 1 overflow freeze"]
    #[inline(always)]
    pub fn ch1of(&mut self) -> CH1OF_W {
        CH1OF_W { w: self }
    }
    #[doc = "Bit 8 - Channel 0 counter reset"]
    #[inline(always)]
    pub fn ch0res(&mut self) -> CH0RES_W {
        CH0RES_W { w: self }
    }
    #[doc = "Bit 9 - Channel 1 counter reset"]
    #[inline(always)]
    pub fn ch1res(&mut self) -> CH1RES_W {
        CH1RES_W { w: self }
    }
    #[doc = "Bits 16:21 - PDCA Channel to monitor with counter 0"]
    #[inline(always)]
    pub fn mon0ch(&mut self) -> MON0CH_W {
        MON0CH_W { w: self }
    }
    #[doc = "Bits 24:29 - PDCA Channel to monitor with counter 1"]
    #[inline(always)]
    pub fn mon1ch(&mut self) -> MON1CH_W {
        MON1CH_W { w: self }
    }
}
