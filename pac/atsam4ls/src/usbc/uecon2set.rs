#[doc = "Writer for register UECON2SET"]
pub type W = crate::W<u32, super::UECON2SET>;
#[doc = "Register UECON2SET `reset()`'s with value 0"]
impl crate::ResetValue for super::UECON2SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TXINES`"]
pub struct TXINES_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINES_W<'a> {
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
#[doc = "Write proxy for field `RXOUTES`"]
pub struct RXOUTES_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOUTES_W<'a> {
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
#[doc = "Write proxy for field `RXSTPES`"]
pub struct RXSTPES_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTPES_W<'a> {
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
#[doc = "Write proxy for field `NAKOUTES`"]
pub struct NAKOUTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKOUTES_W<'a> {
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
#[doc = "Write proxy for field `NAKINES`"]
pub struct NAKINES_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKINES_W<'a> {
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
#[doc = "Write proxy for field `STALLEDES`"]
pub struct STALLEDES_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLEDES_W<'a> {
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
#[doc = "Write proxy for field `NREPLYS`"]
pub struct NREPLYS_W<'a> {
    w: &'a mut W,
}
impl<'a> NREPLYS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
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
#[doc = "Write proxy for field `KILLBKS`"]
pub struct KILLBKS_W<'a> {
    w: &'a mut W,
}
impl<'a> KILLBKS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `NYETDISS`"]
pub struct NYETDISS_W<'a> {
    w: &'a mut W,
}
impl<'a> NYETDISS_W<'a> {
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
#[doc = "Write proxy for field `RSTDTS`"]
pub struct RSTDTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTDTS_W<'a> {
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
#[doc = "Write proxy for field `STALLRQS`"]
pub struct STALLRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLRQS_W<'a> {
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
#[doc = "Write proxy for field `BUSY0S`"]
pub struct BUSY0S_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY0S_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Write proxy for field `BUSY1S`"]
pub struct BUSY1S_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY1S_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - TXINE Set"]
    #[inline(always)]
    pub fn txines(&mut self) -> TXINES_W {
        TXINES_W { w: self }
    }
    #[doc = "Bit 1 - RXOUTE Set"]
    #[inline(always)]
    pub fn rxoutes(&mut self) -> RXOUTES_W {
        RXOUTES_W { w: self }
    }
    #[doc = "Bit 2 - RXSTPE Set"]
    #[inline(always)]
    pub fn rxstpes(&mut self) -> RXSTPES_W {
        RXSTPES_W { w: self }
    }
    #[doc = "Bit 3 - NAKOUTE Set"]
    #[inline(always)]
    pub fn nakoutes(&mut self) -> NAKOUTES_W {
        NAKOUTES_W { w: self }
    }
    #[doc = "Bit 4 - NAKINE Set"]
    #[inline(always)]
    pub fn nakines(&mut self) -> NAKINES_W {
        NAKINES_W { w: self }
    }
    #[doc = "Bit 6 - STALLEDE Set"]
    #[inline(always)]
    pub fn stalledes(&mut self) -> STALLEDES_W {
        STALLEDES_W { w: self }
    }
    #[doc = "Bit 8 - NREPLY Set"]
    #[inline(always)]
    pub fn nreplys(&mut self) -> NREPLYS_W {
        NREPLYS_W { w: self }
    }
    #[doc = "Bit 11 - RAMACERE Set"]
    #[inline(always)]
    pub fn ramaceres(&mut self) -> RAMACERES_W {
        RAMACERES_W { w: self }
    }
    #[doc = "Bit 12 - NBUSYBKE Set"]
    #[inline(always)]
    pub fn nbusybkes(&mut self) -> NBUSYBKES_W {
        NBUSYBKES_W { w: self }
    }
    #[doc = "Bit 13 - KILLBK Set"]
    #[inline(always)]
    pub fn killbks(&mut self) -> KILLBKS_W {
        KILLBKS_W { w: self }
    }
    #[doc = "Bit 17 - NYETDIS Set"]
    #[inline(always)]
    pub fn nyetdiss(&mut self) -> NYETDISS_W {
        NYETDISS_W { w: self }
    }
    #[doc = "Bit 18 - RSTDT Set"]
    #[inline(always)]
    pub fn rstdts(&mut self) -> RSTDTS_W {
        RSTDTS_W { w: self }
    }
    #[doc = "Bit 19 - STALLRQ Set"]
    #[inline(always)]
    pub fn stallrqs(&mut self) -> STALLRQS_W {
        STALLRQS_W { w: self }
    }
    #[doc = "Bit 24 - BUSY0 Set"]
    #[inline(always)]
    pub fn busy0s(&mut self) -> BUSY0S_W {
        BUSY0S_W { w: self }
    }
    #[doc = "Bit 25 - BUSY1 Set"]
    #[inline(always)]
    pub fn busy1s(&mut self) -> BUSY1S_W {
        BUSY1S_W { w: self }
    }
}
