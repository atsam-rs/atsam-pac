#[doc = "Writer for register UPSTA3SET"]
pub type W = crate::W<u32, super::UPSTA3SET>;
#[doc = "Register UPSTA3SET `reset()`'s with value 0"]
impl crate::ResetValue for super::UPSTA3SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RXINIS`"]
pub struct RXINIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINIS_W<'a> {
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
#[doc = "Write proxy for field `TXOUTIS`"]
pub struct TXOUTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOUTIS_W<'a> {
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
#[doc = "Write proxy for field `TXSTPIS`"]
pub struct TXSTPIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTPIS_W<'a> {
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
#[doc = "Write proxy for field `PERRIS`"]
pub struct PERRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PERRIS_W<'a> {
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
#[doc = "Write proxy for field `NAKEDIS`"]
pub struct NAKEDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKEDIS_W<'a> {
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
#[doc = "Write proxy for field `ERRORFIS`"]
pub struct ERRORFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRORFIS_W<'a> {
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
#[doc = "Write proxy for field `RXSTALLDIS`"]
pub struct RXSTALLDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTALLDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - RXINI Set"]
    #[inline(always)]
    pub fn rxinis(&mut self) -> RXINIS_W {
        RXINIS_W { w: self }
    }
    #[doc = "Bit 1 - TXOUTI Set"]
    #[inline(always)]
    pub fn txoutis(&mut self) -> TXOUTIS_W {
        TXOUTIS_W { w: self }
    }
    #[doc = "Bit 2 - TXSTPI Set"]
    #[inline(always)]
    pub fn txstpis(&mut self) -> TXSTPIS_W {
        TXSTPIS_W { w: self }
    }
    #[doc = "Bit 3 - PERRI Set"]
    #[inline(always)]
    pub fn perris(&mut self) -> PERRIS_W {
        PERRIS_W { w: self }
    }
    #[doc = "Bit 4 - NAKEDI Set"]
    #[inline(always)]
    pub fn nakedis(&mut self) -> NAKEDIS_W {
        NAKEDIS_W { w: self }
    }
    #[doc = "Bit 5 - ERRORFI Set"]
    #[inline(always)]
    pub fn errorfis(&mut self) -> ERRORFIS_W {
        ERRORFIS_W { w: self }
    }
    #[doc = "Bit 6 - RXSTALLDI Set"]
    #[inline(always)]
    pub fn rxstalldis(&mut self) -> RXSTALLDIS_W {
        RXSTALLDIS_W { w: self }
    }
    #[doc = "Bit 10 - RAMACERI Set"]
    #[inline(always)]
    pub fn ramaceris(&mut self) -> RAMACERIS_W {
        RAMACERIS_W { w: self }
    }
}
