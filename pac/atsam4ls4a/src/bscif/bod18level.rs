#[doc = "Reader of register BOD18LEVEL"]
pub type R = crate::R<u32, super::BOD18LEVEL>;
#[doc = "Writer for register BOD18LEVEL"]
pub type W = crate::W<u32, super::BOD18LEVEL>;
#[doc = "Register BOD18LEVEL `reset()`'s with value 0"]
impl crate::ResetValue for super::BOD18LEVEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VAL`"]
pub type VAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VAL`"]
pub struct VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `RANGE`"]
pub type RANGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RANGE`"]
pub struct RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - BOD Value"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 31 - BOD Threshold Range"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - BOD Value"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W {
        VAL_W { w: self }
    }
    #[doc = "Bit 31 - BOD Threshold Range"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W {
        RANGE_W { w: self }
    }
}
