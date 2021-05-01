#[doc = "Writer for register UECON0CLR"]
pub type W = crate::W<u32, super::UECON0CLR>;
#[doc = "Register UECON0CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::UECON0CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TXINEC`"]
pub struct TXINEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINEC_W<'a> {
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
#[doc = "Write proxy for field `RXOUTEC`"]
pub struct RXOUTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOUTEC_W<'a> {
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
#[doc = "Write proxy for field `RXSTPEC`"]
pub struct RXSTPEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTPEC_W<'a> {
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
#[doc = "Write proxy for field `NAKOUTEC`"]
pub struct NAKOUTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKOUTEC_W<'a> {
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
#[doc = "Write proxy for field `NAKINEC`"]
pub struct NAKINEC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKINEC_W<'a> {
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
#[doc = "Write proxy for field `STALLEDEC`"]
pub struct STALLEDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLEDEC_W<'a> {
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
#[doc = "Write proxy for field `NREPLYC`"]
pub struct NREPLYC_W<'a> {
    w: &'a mut W,
}
impl<'a> NREPLYC_W<'a> {
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
#[doc = "Write proxy for field `RAMACEREC`"]
pub struct RAMACEREC_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMACEREC_W<'a> {
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
#[doc = "Write proxy for field `NBUSYBKEC`"]
pub struct NBUSYBKEC_W<'a> {
    w: &'a mut W,
}
impl<'a> NBUSYBKEC_W<'a> {
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
#[doc = "Write proxy for field `FIFOCONC`"]
pub struct FIFOCONC_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOCONC_W<'a> {
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
#[doc = "Write proxy for field `NYETDISC`"]
pub struct NYETDISC_W<'a> {
    w: &'a mut W,
}
impl<'a> NYETDISC_W<'a> {
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
#[doc = "Write proxy for field `STALLRQC`"]
pub struct STALLRQC_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLRQC_W<'a> {
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
#[doc = "Write proxy for field `BUSY0C`"]
pub struct BUSY0C_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY0C_W<'a> {
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
#[doc = "Write proxy for field `BUSY1C`"]
pub struct BUSY1C_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY1C_W<'a> {
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
    #[doc = "Bit 0 - TXINE Clear"]
    #[inline(always)]
    pub fn txinec(&mut self) -> TXINEC_W {
        TXINEC_W { w: self }
    }
    #[doc = "Bit 1 - RXOUTE Clear"]
    #[inline(always)]
    pub fn rxoutec(&mut self) -> RXOUTEC_W {
        RXOUTEC_W { w: self }
    }
    #[doc = "Bit 2 - RXSTPE Clear"]
    #[inline(always)]
    pub fn rxstpec(&mut self) -> RXSTPEC_W {
        RXSTPEC_W { w: self }
    }
    #[doc = "Bit 3 - NAKOUTE Clear"]
    #[inline(always)]
    pub fn nakoutec(&mut self) -> NAKOUTEC_W {
        NAKOUTEC_W { w: self }
    }
    #[doc = "Bit 4 - NAKINE Clear"]
    #[inline(always)]
    pub fn nakinec(&mut self) -> NAKINEC_W {
        NAKINEC_W { w: self }
    }
    #[doc = "Bit 6 - STALLEDE Clear"]
    #[inline(always)]
    pub fn stalledec(&mut self) -> STALLEDEC_W {
        STALLEDEC_W { w: self }
    }
    #[doc = "Bit 8 - NREPLY Clear"]
    #[inline(always)]
    pub fn nreplyc(&mut self) -> NREPLYC_W {
        NREPLYC_W { w: self }
    }
    #[doc = "Bit 11 - RAMACERE Clear"]
    #[inline(always)]
    pub fn ramacerec(&mut self) -> RAMACEREC_W {
        RAMACEREC_W { w: self }
    }
    #[doc = "Bit 12 - NBUSYBKE Clear"]
    #[inline(always)]
    pub fn nbusybkec(&mut self) -> NBUSYBKEC_W {
        NBUSYBKEC_W { w: self }
    }
    #[doc = "Bit 14 - FIFOCON Clear"]
    #[inline(always)]
    pub fn fifoconc(&mut self) -> FIFOCONC_W {
        FIFOCONC_W { w: self }
    }
    #[doc = "Bit 17 - NYETDIS Clear"]
    #[inline(always)]
    pub fn nyetdisc(&mut self) -> NYETDISC_W {
        NYETDISC_W { w: self }
    }
    #[doc = "Bit 19 - STALLRQ Clear"]
    #[inline(always)]
    pub fn stallrqc(&mut self) -> STALLRQC_W {
        STALLRQC_W { w: self }
    }
    #[doc = "Bit 24 - BUSY0 Clear"]
    #[inline(always)]
    pub fn busy0c(&mut self) -> BUSY0C_W {
        BUSY0C_W { w: self }
    }
    #[doc = "Bit 25 - BUSY1 Clear"]
    #[inline(always)]
    pub fn busy1c(&mut self) -> BUSY1C_W {
        BUSY1C_W { w: self }
    }
}
