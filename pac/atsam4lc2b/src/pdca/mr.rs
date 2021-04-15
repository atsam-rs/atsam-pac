#[doc = "Reader of register MR%s"]
pub type R = crate::R<u32, super::MR>;
#[doc = "Writer for register MR%s"]
pub type W = crate::W<u32, super::MR>;
#[doc = "Register MR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::MR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transfer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "0: `0`"]
    BYTE = 0,
    #[doc = "1: `1`"]
    HALF_WORD = 1,
    #[doc = "2: `10`"]
    WORD = 2,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, SIZE_A>;
impl SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SIZE_A::BYTE),
            1 => Val(SIZE_A::HALF_WORD),
            2 => Val(SIZE_A::WORD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == SIZE_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALF_WORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == SIZE_A::HALF_WORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SIZE_A::WORD
    }
}
#[doc = "Write proxy for field `SIZE`"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(SIZE_A::BYTE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(SIZE_A::HALF_WORD)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(SIZE_A::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `ETRIG`"]
pub type ETRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETRIG`"]
pub struct ETRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRIG_W<'a> {
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
#[doc = "Reader of field `RING`"]
pub type RING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RING`"]
pub struct RING_W<'a> {
    w: &'a mut W,
}
impl<'a> RING_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Transfer size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Event trigger"]
    #[inline(always)]
    pub fn etrig(&self) -> ETRIG_R {
        ETRIG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Ring Buffer"]
    #[inline(always)]
    pub fn ring(&self) -> RING_R {
        RING_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transfer size"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bit 2 - Event trigger"]
    #[inline(always)]
    pub fn etrig(&mut self) -> ETRIG_W {
        ETRIG_W { w: self }
    }
    #[doc = "Bit 3 - Ring Buffer"]
    #[inline(always)]
    pub fn ring(&mut self) -> RING_W {
        RING_W { w: self }
    }
}
