#[doc = "Writer for register UPCON3SET"]
pub type W = crate::W<u32, super::UPCON3SET>;
#[doc = "Register UPCON3SET `reset()`'s with value 0"]
impl crate::ResetValue for super::UPCON3SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RXINES`"]
pub struct RXINES_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINES_W<'a> {
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
#[doc = "Write proxy for field `TXOUTES`"]
pub struct TXOUTES_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOUTES_W<'a> {
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
#[doc = "Write proxy for field `TXSTPES`"]
pub struct TXSTPES_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTPES_W<'a> {
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
#[doc = "Write proxy for field `PERRES`"]
pub struct PERRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PERRES_W<'a> {
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
#[doc = "Write proxy for field `NAKEDES`"]
pub struct NAKEDES_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKEDES_W<'a> {
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
#[doc = "Write proxy for field `ERRORFIES`"]
pub struct ERRORFIES_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRORFIES_W<'a> {
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
#[doc = "Write proxy for field `RXSTALLDES`"]
pub struct RXSTALLDES_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTALLDES_W<'a> {
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
#[doc = "Write proxy for field `RAMACERES`"]
pub struct RAMACERES_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMACERES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `NBUSYBKES`"]
pub struct NBUSYBKES_W<'a> {
    w: &'a mut W,
}
impl<'a> NBUSYBKES_W<'a> {
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
#[doc = "Write proxy for field `FIFOCONS`"]
pub struct FIFOCONS_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOCONS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `PFREEZES`"]
pub struct PFREEZES_W<'a> {
    w: &'a mut W,
}
impl<'a> PFREEZES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Write proxy for field `INITDTGLS`"]
pub struct INITDTGLS_W<'a> {
    w: &'a mut W,
}
impl<'a> INITDTGLS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Write proxy for field `INITBKS`"]
pub struct INITBKS_W<'a> {
    w: &'a mut W,
}
impl<'a> INITBKS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - RXINE Set"]
    #[inline(always)]
    pub fn rxines(&mut self) -> RXINES_W {
        RXINES_W { w: self }
    }
    #[doc = "Bit 1 - TXOUTE Set"]
    #[inline(always)]
    pub fn txoutes(&mut self) -> TXOUTES_W {
        TXOUTES_W { w: self }
    }
    #[doc = "Bit 2 - TXSTPE Set"]
    #[inline(always)]
    pub fn txstpes(&mut self) -> TXSTPES_W {
        TXSTPES_W { w: self }
    }
    #[doc = "Bit 3 - PERRE Set"]
    #[inline(always)]
    pub fn perres(&mut self) -> PERRES_W {
        PERRES_W { w: self }
    }
    #[doc = "Bit 4 - NAKEDE Set"]
    #[inline(always)]
    pub fn nakedes(&mut self) -> NAKEDES_W {
        NAKEDES_W { w: self }
    }
    #[doc = "Bit 5 - ERRORFIE Set"]
    #[inline(always)]
    pub fn errorfies(&mut self) -> ERRORFIES_W {
        ERRORFIES_W { w: self }
    }
    #[doc = "Bit 6 - RXSTALLDE Set"]
    #[inline(always)]
    pub fn rxstalldes(&mut self) -> RXSTALLDES_W {
        RXSTALLDES_W { w: self }
    }
    #[doc = "Bit 10 - RAMACERE Set"]
    #[inline(always)]
    pub fn ramaceres(&mut self) -> RAMACERES_W {
        RAMACERES_W { w: self }
    }
    #[doc = "Bit 12 - NBUSYBKE Set"]
    #[inline(always)]
    pub fn nbusybkes(&mut self) -> NBUSYBKES_W {
        NBUSYBKES_W { w: self }
    }
    #[doc = "Bit 14 - FIFOCON Set"]
    #[inline(always)]
    pub fn fifocons(&mut self) -> FIFOCONS_W {
        FIFOCONS_W { w: self }
    }
    #[doc = "Bit 17 - PFREEZE Set"]
    #[inline(always)]
    pub fn pfreezes(&mut self) -> PFREEZES_W {
        PFREEZES_W { w: self }
    }
    #[doc = "Bit 18 - INITDTGL Set"]
    #[inline(always)]
    pub fn initdtgls(&mut self) -> INITDTGLS_W {
        INITDTGLS_W { w: self }
    }
    #[doc = "Bit 19 - INITBK Set"]
    #[inline(always)]
    pub fn initbks(&mut self) -> INITBKS_W {
        INITBKS_W { w: self }
    }
}
