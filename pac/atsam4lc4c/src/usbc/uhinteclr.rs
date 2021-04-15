#[doc = "Writer for register UHINTECLR"]
pub type W = crate::W<u32, super::UHINTECLR>;
#[doc = "Register UHINTECLR `reset()`'s with value 0"]
impl crate::ResetValue for super::UHINTECLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DCONNIEC`"]
pub struct DCONNIEC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCONNIEC_W<'a> {
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
#[doc = "Write proxy for field `DDISCIEC`"]
pub struct DDISCIEC_W<'a> {
    w: &'a mut W,
}
impl<'a> DDISCIEC_W<'a> {
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
#[doc = "Write proxy for field `RSTIEC`"]
pub struct RSTIEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIEC_W<'a> {
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
#[doc = "Write proxy for field `RSMEDIEC`"]
pub struct RSMEDIEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSMEDIEC_W<'a> {
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
#[doc = "Write proxy for field `RXRSMIEC`"]
pub struct RXRSMIEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRSMIEC_W<'a> {
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
#[doc = "Write proxy for field `HSOFIEC`"]
pub struct HSOFIEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSOFIEC_W<'a> {
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
#[doc = "Write proxy for field `HWUPIEC`"]
pub struct HWUPIEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HWUPIEC_W<'a> {
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
#[doc = "Write proxy for field `P0INTEC`"]
pub struct P0INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> P0INTEC_W<'a> {
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
#[doc = "Write proxy for field `P1INTEC`"]
pub struct P1INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> P1INTEC_W<'a> {
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
#[doc = "Write proxy for field `P2INTEC`"]
pub struct P2INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> P2INTEC_W<'a> {
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
#[doc = "Write proxy for field `P3INTEC`"]
pub struct P3INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> P3INTEC_W<'a> {
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
#[doc = "Write proxy for field `P4INTEC`"]
pub struct P4INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> P4INTEC_W<'a> {
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
#[doc = "Write proxy for field `P5INTEC`"]
pub struct P5INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> P5INTEC_W<'a> {
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
#[doc = "Write proxy for field `P6INTEC`"]
pub struct P6INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> P6INTEC_W<'a> {
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
#[doc = "Write proxy for field `P7INTEC`"]
pub struct P7INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> P7INTEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - DCONNIE Clear"]
    #[inline(always)]
    pub fn dconniec(&mut self) -> DCONNIEC_W {
        DCONNIEC_W { w: self }
    }
    #[doc = "Bit 1 - DDISCIE Clear"]
    #[inline(always)]
    pub fn ddisciec(&mut self) -> DDISCIEC_W {
        DDISCIEC_W { w: self }
    }
    #[doc = "Bit 2 - RSTIE Clear"]
    #[inline(always)]
    pub fn rstiec(&mut self) -> RSTIEC_W {
        RSTIEC_W { w: self }
    }
    #[doc = "Bit 3 - RSMEDIE Clear"]
    #[inline(always)]
    pub fn rsmediec(&mut self) -> RSMEDIEC_W {
        RSMEDIEC_W { w: self }
    }
    #[doc = "Bit 4 - RXRSMIE Clear"]
    #[inline(always)]
    pub fn rxrsmiec(&mut self) -> RXRSMIEC_W {
        RXRSMIEC_W { w: self }
    }
    #[doc = "Bit 5 - HSOFIE Clear"]
    #[inline(always)]
    pub fn hsofiec(&mut self) -> HSOFIEC_W {
        HSOFIEC_W { w: self }
    }
    #[doc = "Bit 6 - HWUPIE Clear"]
    #[inline(always)]
    pub fn hwupiec(&mut self) -> HWUPIEC_W {
        HWUPIEC_W { w: self }
    }
    #[doc = "Bit 8 - P0INTE Clear"]
    #[inline(always)]
    pub fn p0intec(&mut self) -> P0INTEC_W {
        P0INTEC_W { w: self }
    }
    #[doc = "Bit 9 - P1INTE Clear"]
    #[inline(always)]
    pub fn p1intec(&mut self) -> P1INTEC_W {
        P1INTEC_W { w: self }
    }
    #[doc = "Bit 10 - P2INTE Clear"]
    #[inline(always)]
    pub fn p2intec(&mut self) -> P2INTEC_W {
        P2INTEC_W { w: self }
    }
    #[doc = "Bit 11 - P3INTE Clear"]
    #[inline(always)]
    pub fn p3intec(&mut self) -> P3INTEC_W {
        P3INTEC_W { w: self }
    }
    #[doc = "Bit 12 - P4INTE Clear"]
    #[inline(always)]
    pub fn p4intec(&mut self) -> P4INTEC_W {
        P4INTEC_W { w: self }
    }
    #[doc = "Bit 13 - P5INTE Clear"]
    #[inline(always)]
    pub fn p5intec(&mut self) -> P5INTEC_W {
        P5INTEC_W { w: self }
    }
    #[doc = "Bit 14 - P6INTE Clear"]
    #[inline(always)]
    pub fn p6intec(&mut self) -> P6INTEC_W {
        P6INTEC_W { w: self }
    }
    #[doc = "Bit 15 - P7INTE Clear"]
    #[inline(always)]
    pub fn p7intec(&mut self) -> P7INTEC_W {
        P7INTEC_W { w: self }
    }
}
