#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOURCE`"]
pub type SOURCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SOURCE`"]
pub struct SOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOURCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `ACTION`"]
pub type ACTION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTION`"]
pub struct ACTION_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTION_W<'a> {
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
#[doc = "Reader of field `MATCH`"]
pub type MATCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MATCH`"]
pub struct MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Source Enable Mode"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Action to perform"]
    #[inline(always)]
    pub fn action(&self) -> ACTION_R {
        ACTION_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Data Match"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Source Enable Mode"]
    #[inline(always)]
    pub fn source(&mut self) -> SOURCE_W {
        SOURCE_W { w: self }
    }
    #[doc = "Bit 2 - Action to perform"]
    #[inline(always)]
    pub fn action(&mut self) -> ACTION_W {
        ACTION_W { w: self }
    }
    #[doc = "Bits 8:15 - Data Match"]
    #[inline(always)]
    pub fn match_(&mut self) -> MATCH_W {
        MATCH_W { w: self }
    }
}
