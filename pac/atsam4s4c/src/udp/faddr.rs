#[doc = "Reader of register FADDR"]
pub type R = crate::R<u32, super::FADDR>;
#[doc = "Writer for register FADDR"]
pub type W = crate::W<u32, super::FADDR>;
#[doc = "Register FADDR `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::FADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `FADD`"]
pub type FADD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FADD`"]
pub struct FADD_W<'a> {
    w: &'a mut W,
}
impl<'a> FADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `FEN`"]
pub type FEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEN`"]
pub struct FEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FEN_W<'a> {
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
impl R {
    #[doc = "Bits 0:6 - Function Address Value"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Function Enable"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Function Address Value"]
    #[inline(always)]
    pub fn fadd(&mut self) -> FADD_W {
        FADD_W { w: self }
    }
    #[doc = "Bit 8 - Function Enable"]
    #[inline(always)]
    pub fn fen(&mut self) -> FEN_W {
        FEN_W { w: self }
    }
}
