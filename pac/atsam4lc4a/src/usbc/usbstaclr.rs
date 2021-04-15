#[doc = "Writer for register USBSTACLR"]
pub type W = crate::W<u32, super::USBSTACLR>;
#[doc = "Register USBSTACLR `reset()`'s with value 0"]
impl crate::ResetValue for super::USBSTACLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RAMACERIC`"]
pub struct RAMACERIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMACERIC_W<'a> {
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
#[doc = "Write proxy for field `VBUSRQC`"]
pub struct VBUSRQC_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSRQC_W<'a> {
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
impl W {
    #[doc = "Bit 8 - RAMACERI Clear"]
    #[inline(always)]
    pub fn ramaceric(&mut self) -> RAMACERIC_W {
        RAMACERIC_W { w: self }
    }
    #[doc = "Bit 9 - VBUSRQ Clear"]
    #[inline(always)]
    pub fn vbusrqc(&mut self) -> VBUSRQC_W {
        VBUSRQC_W { w: self }
    }
}
