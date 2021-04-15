#[doc = "Reader of register TR"]
pub type R = crate::R<u32, super::TR>;
#[doc = "Writer for register TR"]
pub type W = crate::W<u32, super::TR>;
#[doc = "Register TR `reset()`'s with value 0"]
impl crate::ResetValue for super::TR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ACTEST0`"]
pub type ACTEST0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTEST0`"]
pub struct ACTEST0_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEST0_W<'a> {
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
#[doc = "Reader of field `ACTEST1`"]
pub type ACTEST1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTEST1`"]
pub struct ACTEST1_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEST1_W<'a> {
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
#[doc = "Reader of field `ACTEST2`"]
pub type ACTEST2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTEST2`"]
pub struct ACTEST2_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEST2_W<'a> {
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
#[doc = "Reader of field `ACTEST3`"]
pub type ACTEST3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTEST3`"]
pub struct ACTEST3_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEST3_W<'a> {
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
#[doc = "Reader of field `ACTEST4`"]
pub type ACTEST4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTEST4`"]
pub struct ACTEST4_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEST4_W<'a> {
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
#[doc = "Reader of field `ACTEST5`"]
pub type ACTEST5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTEST5`"]
pub struct ACTEST5_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEST5_W<'a> {
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
#[doc = "Reader of field `ACTEST6`"]
pub type ACTEST6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTEST6`"]
pub struct ACTEST6_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEST6_W<'a> {
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
#[doc = "Reader of field `ACTEST7`"]
pub type ACTEST7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTEST7`"]
pub struct ACTEST7_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEST7_W<'a> {
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
    #[doc = "Bit 0 - AC0 Output Override Value"]
    #[inline(always)]
    pub fn actest0(&self) -> ACTEST0_R {
        ACTEST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AC1 Output Override Value"]
    #[inline(always)]
    pub fn actest1(&self) -> ACTEST1_R {
        ACTEST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AC2 Output Override Value"]
    #[inline(always)]
    pub fn actest2(&self) -> ACTEST2_R {
        ACTEST2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AC3 Output Override Value"]
    #[inline(always)]
    pub fn actest3(&self) -> ACTEST3_R {
        ACTEST3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AC4 Output Override Value"]
    #[inline(always)]
    pub fn actest4(&self) -> ACTEST4_R {
        ACTEST4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AC5 Output Override Value"]
    #[inline(always)]
    pub fn actest5(&self) -> ACTEST5_R {
        ACTEST5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AC6 Output Override Value"]
    #[inline(always)]
    pub fn actest6(&self) -> ACTEST6_R {
        ACTEST6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AC7 Output Override Value"]
    #[inline(always)]
    pub fn actest7(&self) -> ACTEST7_R {
        ACTEST7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AC0 Output Override Value"]
    #[inline(always)]
    pub fn actest0(&mut self) -> ACTEST0_W {
        ACTEST0_W { w: self }
    }
    #[doc = "Bit 1 - AC1 Output Override Value"]
    #[inline(always)]
    pub fn actest1(&mut self) -> ACTEST1_W {
        ACTEST1_W { w: self }
    }
    #[doc = "Bit 2 - AC2 Output Override Value"]
    #[inline(always)]
    pub fn actest2(&mut self) -> ACTEST2_W {
        ACTEST2_W { w: self }
    }
    #[doc = "Bit 3 - AC3 Output Override Value"]
    #[inline(always)]
    pub fn actest3(&mut self) -> ACTEST3_W {
        ACTEST3_W { w: self }
    }
    #[doc = "Bit 4 - AC4 Output Override Value"]
    #[inline(always)]
    pub fn actest4(&mut self) -> ACTEST4_W {
        ACTEST4_W { w: self }
    }
    #[doc = "Bit 5 - AC5 Output Override Value"]
    #[inline(always)]
    pub fn actest5(&mut self) -> ACTEST5_W {
        ACTEST5_W { w: self }
    }
    #[doc = "Bit 6 - AC6 Output Override Value"]
    #[inline(always)]
    pub fn actest6(&mut self) -> ACTEST6_W {
        ACTEST6_W { w: self }
    }
    #[doc = "Bit 7 - AC7 Output Override Value"]
    #[inline(always)]
    pub fn actest7(&mut self) -> ACTEST7_W {
        ACTEST7_W { w: self }
    }
}
