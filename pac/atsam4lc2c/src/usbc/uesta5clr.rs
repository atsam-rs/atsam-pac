#[doc = "Writer for register UESTA5CLR"]
pub type W = crate::W<u32, super::UESTA5CLR>;
#[doc = "Register UESTA5CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::UESTA5CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TXINIC`"]
pub struct TXINIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINIC_W<'a> {
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
#[doc = "Write proxy for field `RXOUTIC`"]
pub struct RXOUTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOUTIC_W<'a> {
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
#[doc = "Write proxy for field `RXSTPIC`"]
pub struct RXSTPIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTPIC_W<'a> {
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
#[doc = "Write proxy for field `NAKOUTIC`"]
pub struct NAKOUTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKOUTIC_W<'a> {
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
#[doc = "Write proxy for field `NAKINIC`"]
pub struct NAKINIC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKINIC_W<'a> {
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
#[doc = "Write proxy for field `STALLEDIC`"]
pub struct STALLEDIC_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLEDIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - TXINI Clear"]
    #[inline(always)]
    pub fn txinic(&mut self) -> TXINIC_W {
        TXINIC_W { w: self }
    }
    #[doc = "Bit 1 - RXOUTI Clear"]
    #[inline(always)]
    pub fn rxoutic(&mut self) -> RXOUTIC_W {
        RXOUTIC_W { w: self }
    }
    #[doc = "Bit 2 - RXSTPI Clear"]
    #[inline(always)]
    pub fn rxstpic(&mut self) -> RXSTPIC_W {
        RXSTPIC_W { w: self }
    }
    #[doc = "Bit 3 - NAKOUTI Clear"]
    #[inline(always)]
    pub fn nakoutic(&mut self) -> NAKOUTIC_W {
        NAKOUTIC_W { w: self }
    }
    #[doc = "Bit 4 - NAKINI Clear"]
    #[inline(always)]
    pub fn nakinic(&mut self) -> NAKINIC_W {
        NAKINIC_W { w: self }
    }
    #[doc = "Bit 6 - STALLEDI Clear"]
    #[inline(always)]
    pub fn stalledic(&mut self) -> STALLEDIC_W {
        STALLEDIC_W { w: self }
    }
    #[doc = "Bit 11 - RAMACERI Clear"]
    #[inline(always)]
    pub fn ramaceric(&mut self) -> RAMACERIC_W {
        RAMACERIC_W { w: self }
    }
}
