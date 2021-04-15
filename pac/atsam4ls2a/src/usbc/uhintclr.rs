#[doc = "Writer for register UHINTCLR"]
pub type W = crate::W<u32, super::UHINTCLR>;
#[doc = "Register UHINTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::UHINTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DCONNIC`"]
pub struct DCONNIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCONNIC_W<'a> {
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
#[doc = "Write proxy for field `DDISCIC`"]
pub struct DDISCIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DDISCIC_W<'a> {
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
#[doc = "Write proxy for field `RSTIC`"]
pub struct RSTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIC_W<'a> {
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
#[doc = "Write proxy for field `RSMEDIC`"]
pub struct RSMEDIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSMEDIC_W<'a> {
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
#[doc = "Write proxy for field `RXRSMIC`"]
pub struct RXRSMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRSMIC_W<'a> {
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
#[doc = "Write proxy for field `HSOFIC`"]
pub struct HSOFIC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSOFIC_W<'a> {
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
#[doc = "Write proxy for field `HWUPIC`"]
pub struct HWUPIC_W<'a> {
    w: &'a mut W,
}
impl<'a> HWUPIC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - DCONNI Clear"]
    #[inline(always)]
    pub fn dconnic(&mut self) -> DCONNIC_W {
        DCONNIC_W { w: self }
    }
    #[doc = "Bit 1 - DDISCI Clear"]
    #[inline(always)]
    pub fn ddiscic(&mut self) -> DDISCIC_W {
        DDISCIC_W { w: self }
    }
    #[doc = "Bit 2 - RSTI Clear"]
    #[inline(always)]
    pub fn rstic(&mut self) -> RSTIC_W {
        RSTIC_W { w: self }
    }
    #[doc = "Bit 3 - RSMEDI Clear"]
    #[inline(always)]
    pub fn rsmedic(&mut self) -> RSMEDIC_W {
        RSMEDIC_W { w: self }
    }
    #[doc = "Bit 4 - RXRSMI Clear"]
    #[inline(always)]
    pub fn rxrsmic(&mut self) -> RXRSMIC_W {
        RXRSMIC_W { w: self }
    }
    #[doc = "Bit 5 - HSOFI Clear"]
    #[inline(always)]
    pub fn hsofic(&mut self) -> HSOFIC_W {
        HSOFIC_W { w: self }
    }
    #[doc = "Bit 6 - HWUPI Clear"]
    #[inline(always)]
    pub fn hwupic(&mut self) -> HWUPIC_W {
        HWUPIC_W { w: self }
    }
}
