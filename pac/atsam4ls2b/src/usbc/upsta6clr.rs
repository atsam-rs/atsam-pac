#[doc = "Writer for register UPSTA6CLR"]
pub type W = crate::W<u32, super::UPSTA6CLR>;
#[doc = "Register UPSTA6CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::UPSTA6CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RXINIC`"]
pub struct RXINIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINIC_W<'a> {
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
#[doc = "Write proxy for field `TXOUTIC`"]
pub struct TXOUTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOUTIC_W<'a> {
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
#[doc = "Write proxy for field `TXSTPIC`"]
pub struct TXSTPIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTPIC_W<'a> {
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
#[doc = "Write proxy for field `PERRIC`"]
pub struct PERRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> PERRIC_W<'a> {
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
#[doc = "Write proxy for field `NAKEDIC`"]
pub struct NAKEDIC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKEDIC_W<'a> {
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
#[doc = "Write proxy for field `ERRORFIC`"]
pub struct ERRORFIC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRORFIC_W<'a> {
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
#[doc = "Write proxy for field `RXSTALLDIC`"]
pub struct RXSTALLDIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTALLDIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - RXINI Clear"]
    #[inline(always)]
    pub fn rxinic(&mut self) -> RXINIC_W {
        RXINIC_W { w: self }
    }
    #[doc = "Bit 1 - TXOUTI Clear"]
    #[inline(always)]
    pub fn txoutic(&mut self) -> TXOUTIC_W {
        TXOUTIC_W { w: self }
    }
    #[doc = "Bit 2 - TXSTPI Clear"]
    #[inline(always)]
    pub fn txstpic(&mut self) -> TXSTPIC_W {
        TXSTPIC_W { w: self }
    }
    #[doc = "Bit 3 - PERRI Clear"]
    #[inline(always)]
    pub fn perric(&mut self) -> PERRIC_W {
        PERRIC_W { w: self }
    }
    #[doc = "Bit 4 - NAKEDI Clear"]
    #[inline(always)]
    pub fn nakedic(&mut self) -> NAKEDIC_W {
        NAKEDIC_W { w: self }
    }
    #[doc = "Bit 5 - ERRORFI Clear"]
    #[inline(always)]
    pub fn errorfic(&mut self) -> ERRORFIC_W {
        ERRORFIC_W { w: self }
    }
    #[doc = "Bit 6 - RXSTALLDI Clear"]
    #[inline(always)]
    pub fn rxstalldic(&mut self) -> RXSTALLDIC_W {
        RXSTALLDIC_W { w: self }
    }
    #[doc = "Bit 10 - RAMACERI Clear"]
    #[inline(always)]
    pub fn ramaceric(&mut self) -> RAMACERIC_W {
        RAMACERIC_W { w: self }
    }
}
