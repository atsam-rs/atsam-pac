#[doc = "Writer for register UESTA7SET"]
pub type W = crate::W<u32, super::UESTA7SET>;
#[doc = "Register UESTA7SET `reset()`'s with value 0"]
impl crate::ResetValue for super::UESTA7SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TXINIS`"]
pub struct TXINIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINIS_W<'a> {
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
#[doc = "Write proxy for field `RXOUTIS`"]
pub struct RXOUTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOUTIS_W<'a> {
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
#[doc = "Write proxy for field `RXSTPIS`"]
pub struct RXSTPIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTPIS_W<'a> {
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
#[doc = "Write proxy for field `NAKOUTIS`"]
pub struct NAKOUTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKOUTIS_W<'a> {
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
#[doc = "Write proxy for field `NAKINIS`"]
pub struct NAKINIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKINIS_W<'a> {
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
#[doc = "Write proxy for field `STALLEDIS`"]
pub struct STALLEDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLEDIS_W<'a> {
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
#[doc = "Write proxy for field `RAMACERIS`"]
pub struct RAMACERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMACERIS_W<'a> {
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
#[doc = "Write proxy for field `NBUSYBKS`"]
pub struct NBUSYBKS_W<'a> {
    w: &'a mut W,
}
impl<'a> NBUSYBKS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - TXINI Set"]
    #[inline(always)]
    pub fn txinis(&mut self) -> TXINIS_W {
        TXINIS_W { w: self }
    }
    #[doc = "Bit 1 - RXOUTI Set"]
    #[inline(always)]
    pub fn rxoutis(&mut self) -> RXOUTIS_W {
        RXOUTIS_W { w: self }
    }
    #[doc = "Bit 2 - RXSTPI Set"]
    #[inline(always)]
    pub fn rxstpis(&mut self) -> RXSTPIS_W {
        RXSTPIS_W { w: self }
    }
    #[doc = "Bit 3 - NAKOUTI Set"]
    #[inline(always)]
    pub fn nakoutis(&mut self) -> NAKOUTIS_W {
        NAKOUTIS_W { w: self }
    }
    #[doc = "Bit 4 - NAKINI Set"]
    #[inline(always)]
    pub fn nakinis(&mut self) -> NAKINIS_W {
        NAKINIS_W { w: self }
    }
    #[doc = "Bit 6 - STALLEDI Set"]
    #[inline(always)]
    pub fn stalledis(&mut self) -> STALLEDIS_W {
        STALLEDIS_W { w: self }
    }
    #[doc = "Bit 11 - RAMACERI Set"]
    #[inline(always)]
    pub fn ramaceris(&mut self) -> RAMACERIS_W {
        RAMACERIS_W { w: self }
    }
    #[doc = "Bit 12 - NBUSYBK Set"]
    #[inline(always)]
    pub fn nbusybks(&mut self) -> NBUSYBKS_W {
        NBUSYBKS_W { w: self }
    }
}
