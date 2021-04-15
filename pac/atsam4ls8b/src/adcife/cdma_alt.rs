#[doc = "Writer for register CDMA_ALT"]
pub type W = crate::W<u32, super::CDMA_ALT>;
#[doc = "Register CDMA_ALT `reset()`'s with value 0"]
impl crate::ResetValue for super::CDMA_ALT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `LT`"]
pub struct LT_W<'a> {
    w: &'a mut W,
}
impl<'a> LT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Write proxy for field `WM`"]
pub struct WM_W<'a> {
    w: &'a mut W,
}
impl<'a> WM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `HT`"]
pub struct HT_W<'a> {
    w: &'a mut W,
}
impl<'a> HT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `DW`"]
pub struct DW_W<'a> {
    w: &'a mut W,
}
impl<'a> DW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:11 - Low Threshold"]
    #[inline(always)]
    pub fn lt(&mut self) -> LT_W {
        LT_W { w: self }
    }
    #[doc = "Bits 12:14 - Window Monitor Mode"]
    #[inline(always)]
    pub fn wm(&mut self) -> WM_W {
        WM_W { w: self }
    }
    #[doc = "Bits 16:27 - High Threshold"]
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W {
        HT_W { w: self }
    }
    #[doc = "Bit 31 - Double Word transmitting"]
    #[inline(always)]
    pub fn dw(&mut self) -> DW_W {
        DW_W { w: self }
    }
}
