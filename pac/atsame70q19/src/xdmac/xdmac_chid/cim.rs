#[doc = "Writer for register CIM"]
pub type W = crate::W<u32, super::CIM>;
#[doc = "Register CIM `reset()`'s with value 0"]
impl crate::ResetValue for super::CIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `BIM`"]
pub struct BIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BIM_W<'a> {
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
#[doc = "Write proxy for field `LIM`"]
pub struct LIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LIM_W<'a> {
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
#[doc = "Write proxy for field `DIM`"]
pub struct DIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIM_W<'a> {
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
#[doc = "Write proxy for field `FIM`"]
pub struct FIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FIM_W<'a> {
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
#[doc = "Write proxy for field `RBEIM`"]
pub struct RBEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RBEIM_W<'a> {
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
#[doc = "Write proxy for field `WBEIM`"]
pub struct WBEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> WBEIM_W<'a> {
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
#[doc = "Write proxy for field `ROIM`"]
pub struct ROIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROIM_W<'a> {
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
impl W {
    #[doc = "Bit 0 - End of Block Interrupt Mask Bit"]
    #[inline(always)]
    pub fn bim(&mut self) -> BIM_W {
        BIM_W { w: self }
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Mask Bit"]
    #[inline(always)]
    pub fn lim(&mut self) -> LIM_W {
        LIM_W { w: self }
    }
    #[doc = "Bit 2 - End of Disable Interrupt Mask Bit"]
    #[inline(always)]
    pub fn dim(&mut self) -> DIM_W {
        DIM_W { w: self }
    }
    #[doc = "Bit 3 - End of Flush Interrupt Mask Bit"]
    #[inline(always)]
    pub fn fim(&mut self) -> FIM_W {
        FIM_W { w: self }
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn rbeim(&mut self) -> RBEIM_W {
        RBEIM_W { w: self }
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn wbeim(&mut self) -> WBEIM_W {
        WBEIM_W { w: self }
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn roim(&mut self) -> ROIM_W {
        ROIM_W { w: self }
    }
}
