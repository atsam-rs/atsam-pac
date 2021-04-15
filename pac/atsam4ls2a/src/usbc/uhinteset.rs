#[doc = "Writer for register UHINTESET"]
pub type W = crate::W<u32, super::UHINTESET>;
#[doc = "Register UHINTESET `reset()`'s with value 0"]
impl crate::ResetValue for super::UHINTESET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DCONNIES`"]
pub struct DCONNIES_W<'a> {
    w: &'a mut W,
}
impl<'a> DCONNIES_W<'a> {
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
#[doc = "Write proxy for field `DDISCIES`"]
pub struct DDISCIES_W<'a> {
    w: &'a mut W,
}
impl<'a> DDISCIES_W<'a> {
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
#[doc = "Write proxy for field `RSTIES`"]
pub struct RSTIES_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIES_W<'a> {
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
#[doc = "Write proxy for field `RSMEDIES`"]
pub struct RSMEDIES_W<'a> {
    w: &'a mut W,
}
impl<'a> RSMEDIES_W<'a> {
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
#[doc = "Write proxy for field `RXRSMIES`"]
pub struct RXRSMIES_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRSMIES_W<'a> {
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
#[doc = "Write proxy for field `HSOFIES`"]
pub struct HSOFIES_W<'a> {
    w: &'a mut W,
}
impl<'a> HSOFIES_W<'a> {
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
#[doc = "Write proxy for field `HWUPIES`"]
pub struct HWUPIES_W<'a> {
    w: &'a mut W,
}
impl<'a> HWUPIES_W<'a> {
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
#[doc = "Write proxy for field `P0INTES`"]
pub struct P0INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> P0INTES_W<'a> {
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
#[doc = "Write proxy for field `P1INTES`"]
pub struct P1INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> P1INTES_W<'a> {
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
#[doc = "Write proxy for field `P2INTES`"]
pub struct P2INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> P2INTES_W<'a> {
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
#[doc = "Write proxy for field `P3INTES`"]
pub struct P3INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> P3INTES_W<'a> {
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
#[doc = "Write proxy for field `P4INTES`"]
pub struct P4INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> P4INTES_W<'a> {
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
#[doc = "Write proxy for field `P5INTES`"]
pub struct P5INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> P5INTES_W<'a> {
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
#[doc = "Write proxy for field `P6INTES`"]
pub struct P6INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> P6INTES_W<'a> {
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
#[doc = "Write proxy for field `P7INTES`"]
pub struct P7INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> P7INTES_W<'a> {
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
    #[doc = "Bit 0 - DCONNIE Set"]
    #[inline(always)]
    pub fn dconnies(&mut self) -> DCONNIES_W {
        DCONNIES_W { w: self }
    }
    #[doc = "Bit 1 - DDISCIE Set"]
    #[inline(always)]
    pub fn ddiscies(&mut self) -> DDISCIES_W {
        DDISCIES_W { w: self }
    }
    #[doc = "Bit 2 - RSTIE Set"]
    #[inline(always)]
    pub fn rsties(&mut self) -> RSTIES_W {
        RSTIES_W { w: self }
    }
    #[doc = "Bit 3 - RSMEDIE Set"]
    #[inline(always)]
    pub fn rsmedies(&mut self) -> RSMEDIES_W {
        RSMEDIES_W { w: self }
    }
    #[doc = "Bit 4 - RXRSMIE Set"]
    #[inline(always)]
    pub fn rxrsmies(&mut self) -> RXRSMIES_W {
        RXRSMIES_W { w: self }
    }
    #[doc = "Bit 5 - HSOFIE Set"]
    #[inline(always)]
    pub fn hsofies(&mut self) -> HSOFIES_W {
        HSOFIES_W { w: self }
    }
    #[doc = "Bit 6 - HWUPIE Set"]
    #[inline(always)]
    pub fn hwupies(&mut self) -> HWUPIES_W {
        HWUPIES_W { w: self }
    }
    #[doc = "Bit 8 - P0INTE Set"]
    #[inline(always)]
    pub fn p0intes(&mut self) -> P0INTES_W {
        P0INTES_W { w: self }
    }
    #[doc = "Bit 9 - P1INTE Set"]
    #[inline(always)]
    pub fn p1intes(&mut self) -> P1INTES_W {
        P1INTES_W { w: self }
    }
    #[doc = "Bit 10 - P2INTE Set"]
    #[inline(always)]
    pub fn p2intes(&mut self) -> P2INTES_W {
        P2INTES_W { w: self }
    }
    #[doc = "Bit 11 - P3INTE Set"]
    #[inline(always)]
    pub fn p3intes(&mut self) -> P3INTES_W {
        P3INTES_W { w: self }
    }
    #[doc = "Bit 12 - P4INTE Set"]
    #[inline(always)]
    pub fn p4intes(&mut self) -> P4INTES_W {
        P4INTES_W { w: self }
    }
    #[doc = "Bit 13 - P5INTE Set"]
    #[inline(always)]
    pub fn p5intes(&mut self) -> P5INTES_W {
        P5INTES_W { w: self }
    }
    #[doc = "Bit 14 - P6INTE Set"]
    #[inline(always)]
    pub fn p6intes(&mut self) -> P6INTES_W {
        P6INTES_W { w: self }
    }
    #[doc = "Bit 15 - P7INTE Set"]
    #[inline(always)]
    pub fn p7intes(&mut self) -> P7INTES_W {
        P7INTES_W { w: self }
    }
}
