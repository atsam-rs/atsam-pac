#[doc = "Writer for register UHINTSET"]
pub type W = crate::W<u32, super::UHINTSET>;
#[doc = "Register UHINTSET `reset()`'s with value 0"]
impl crate::ResetValue for super::UHINTSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DCONNIS`"]
pub struct DCONNIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCONNIS_W<'a> {
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
#[doc = "Write proxy for field `DDISCIS`"]
pub struct DDISCIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DDISCIS_W<'a> {
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
#[doc = "Write proxy for field `RSTIS`"]
pub struct RSTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIS_W<'a> {
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
#[doc = "Write proxy for field `RSMEDIS`"]
pub struct RSMEDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RSMEDIS_W<'a> {
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
#[doc = "Write proxy for field `RXRSMIS`"]
pub struct RXRSMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRSMIS_W<'a> {
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
#[doc = "Write proxy for field `HSOFIS`"]
pub struct HSOFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HSOFIS_W<'a> {
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
#[doc = "Write proxy for field `HWUPIS`"]
pub struct HWUPIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HWUPIS_W<'a> {
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
    #[doc = "Bit 0 - DCONNI Set"]
    #[inline(always)]
    pub fn dconnis(&mut self) -> DCONNIS_W {
        DCONNIS_W { w: self }
    }
    #[doc = "Bit 1 - DDISCI Set"]
    #[inline(always)]
    pub fn ddiscis(&mut self) -> DDISCIS_W {
        DDISCIS_W { w: self }
    }
    #[doc = "Bit 2 - RSTI Set"]
    #[inline(always)]
    pub fn rstis(&mut self) -> RSTIS_W {
        RSTIS_W { w: self }
    }
    #[doc = "Bit 3 - RSMEDI Set"]
    #[inline(always)]
    pub fn rsmedis(&mut self) -> RSMEDIS_W {
        RSMEDIS_W { w: self }
    }
    #[doc = "Bit 4 - RXRSMI Set"]
    #[inline(always)]
    pub fn rxrsmis(&mut self) -> RXRSMIS_W {
        RXRSMIS_W { w: self }
    }
    #[doc = "Bit 5 - HSOFI Set"]
    #[inline(always)]
    pub fn hsofis(&mut self) -> HSOFIS_W {
        HSOFIS_W { w: self }
    }
    #[doc = "Bit 6 - HWUPI Set"]
    #[inline(always)]
    pub fn hwupis(&mut self) -> HWUPIS_W {
        HWUPIS_W { w: self }
    }
}
