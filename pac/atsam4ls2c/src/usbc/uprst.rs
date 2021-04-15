#[doc = "Reader of register UPRST"]
pub type R = crate::R<u32, super::UPRST>;
#[doc = "Writer for register UPRST"]
pub type W = crate::W<u32, super::UPRST>;
#[doc = "Register UPRST `reset()`'s with value 0"]
impl crate::ResetValue for super::UPRST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PEN0`"]
pub type PEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEN0`"]
pub struct PEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN0_W<'a> {
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
#[doc = "Reader of field `PEN1`"]
pub type PEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEN1`"]
pub struct PEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN1_W<'a> {
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
#[doc = "Reader of field `PEN2`"]
pub type PEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEN2`"]
pub struct PEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN2_W<'a> {
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
#[doc = "Reader of field `PEN3`"]
pub type PEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEN3`"]
pub struct PEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN3_W<'a> {
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
#[doc = "Reader of field `PEN4`"]
pub type PEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEN4`"]
pub struct PEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN4_W<'a> {
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
#[doc = "Reader of field `PEN5`"]
pub type PEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEN5`"]
pub struct PEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN5_W<'a> {
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
#[doc = "Reader of field `PEN6`"]
pub type PEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEN6`"]
pub struct PEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN6_W<'a> {
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
#[doc = "Reader of field `PEN7`"]
pub type PEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEN7`"]
pub struct PEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Pipe0 Enable"]
    #[inline(always)]
    pub fn pen0(&self) -> PEN0_R {
        PEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pipe1 Enable"]
    #[inline(always)]
    pub fn pen1(&self) -> PEN1_R {
        PEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pipe2 Enable"]
    #[inline(always)]
    pub fn pen2(&self) -> PEN2_R {
        PEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pipe3 Enable"]
    #[inline(always)]
    pub fn pen3(&self) -> PEN3_R {
        PEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pipe4 Enable"]
    #[inline(always)]
    pub fn pen4(&self) -> PEN4_R {
        PEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pipe5 Enable"]
    #[inline(always)]
    pub fn pen5(&self) -> PEN5_R {
        PEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pipe6 Enable"]
    #[inline(always)]
    pub fn pen6(&self) -> PEN6_R {
        PEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pipe7 Enable"]
    #[inline(always)]
    pub fn pen7(&self) -> PEN7_R {
        PEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pipe0 Enable"]
    #[inline(always)]
    pub fn pen0(&mut self) -> PEN0_W {
        PEN0_W { w: self }
    }
    #[doc = "Bit 1 - Pipe1 Enable"]
    #[inline(always)]
    pub fn pen1(&mut self) -> PEN1_W {
        PEN1_W { w: self }
    }
    #[doc = "Bit 2 - Pipe2 Enable"]
    #[inline(always)]
    pub fn pen2(&mut self) -> PEN2_W {
        PEN2_W { w: self }
    }
    #[doc = "Bit 3 - Pipe3 Enable"]
    #[inline(always)]
    pub fn pen3(&mut self) -> PEN3_W {
        PEN3_W { w: self }
    }
    #[doc = "Bit 4 - Pipe4 Enable"]
    #[inline(always)]
    pub fn pen4(&mut self) -> PEN4_W {
        PEN4_W { w: self }
    }
    #[doc = "Bit 5 - Pipe5 Enable"]
    #[inline(always)]
    pub fn pen5(&mut self) -> PEN5_W {
        PEN5_W { w: self }
    }
    #[doc = "Bit 6 - Pipe6 Enable"]
    #[inline(always)]
    pub fn pen6(&mut self) -> PEN6_W {
        PEN6_W { w: self }
    }
    #[doc = "Bit 7 - Pipe7 Enable"]
    #[inline(always)]
    pub fn pen7(&mut self) -> PEN7_W {
        PEN7_W { w: self }
    }
}
