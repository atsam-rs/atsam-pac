#[doc = "Reader of register RCFASTSR"]
pub type R = crate::R<u32, super::RCFASTSR>;
#[doc = "Writer for register RCFASTSR"]
pub type W = crate::W<u32, super::RCFASTSR>;
#[doc = "Register RCFASTSR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCFASTSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CURTRIM`"]
pub type CURTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CURTRIM`"]
pub struct CURTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CURTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `CNTERR`"]
pub type CNTERR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNTERR`"]
pub struct CNTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTERR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `SIGN`"]
pub type SIGN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIGN`"]
pub struct SIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `LOCKLOST`"]
pub type LOCKLOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCKLOST`"]
pub struct LOCKLOST_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKLOST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `UPDATED`"]
pub type UPDATED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPDATED`"]
pub struct UPDATED_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDATED_W<'a> {
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
    #[doc = "Bits 0:6 - Current Trim Value"]
    #[inline(always)]
    pub fn curtrim(&self) -> CURTRIM_R {
        CURTRIM_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:20 - Current Count Error"]
    #[inline(always)]
    pub fn cnterr(&self) -> CNTERR_R {
        CNTERR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Sign of Current Count Error"]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Lock Lost"]
    #[inline(always)]
    pub fn locklost(&self) -> LOCKLOST_R {
        LOCKLOST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Current Trim Value Updated"]
    #[inline(always)]
    pub fn updated(&self) -> UPDATED_R {
        UPDATED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Current Trim Value"]
    #[inline(always)]
    pub fn curtrim(&mut self) -> CURTRIM_W {
        CURTRIM_W { w: self }
    }
    #[doc = "Bits 16:20 - Current Count Error"]
    #[inline(always)]
    pub fn cnterr(&mut self) -> CNTERR_W {
        CNTERR_W { w: self }
    }
    #[doc = "Bit 21 - Sign of Current Count Error"]
    #[inline(always)]
    pub fn sign(&mut self) -> SIGN_W {
        SIGN_W { w: self }
    }
    #[doc = "Bit 24 - Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Bit 25 - Lock Lost"]
    #[inline(always)]
    pub fn locklost(&mut self) -> LOCKLOST_W {
        LOCKLOST_W { w: self }
    }
    #[doc = "Bit 31 - Current Trim Value Updated"]
    #[inline(always)]
    pub fn updated(&mut self) -> UPDATED_W {
        UPDATED_W { w: self }
    }
}
