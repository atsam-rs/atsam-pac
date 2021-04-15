#[doc = "Reader of register DTR"]
pub type R = crate::R<u32, super::DTR>;
#[doc = "Writer for register DTR"]
pub type W = crate::W<u32, super::DTR>;
#[doc = "Register DTR `reset()`'s with value 0"]
impl crate::ResetValue for super::DTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXP`"]
pub type EXP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXP`"]
pub struct EXP_W<'a> {
    w: &'a mut W,
}
impl<'a> EXP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `ADD`"]
pub type ADD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADD`"]
pub struct ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_W<'a> {
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
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VALUE`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - EXP"]
    #[inline(always)]
    pub fn exp(&self) -> EXP_R {
        EXP_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - ADD"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - VALUE"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - EXP"]
    #[inline(always)]
    pub fn exp(&mut self) -> EXP_W {
        EXP_W { w: self }
    }
    #[doc = "Bit 5 - ADD"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W { w: self }
    }
    #[doc = "Bits 8:15 - VALUE"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
