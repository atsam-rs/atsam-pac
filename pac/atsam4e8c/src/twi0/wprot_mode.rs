#[doc = "Reader of register WPROT_MODE"]
pub type R = crate::R<u32, super::WPROT_MODE>;
#[doc = "Writer for register WPROT_MODE"]
pub type W = crate::W<u32, super::WPROT_MODE>;
#[doc = "Register WPROT_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::WPROT_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WPROT`"]
pub type WPROT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WPROT`"]
pub struct WPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> WPROT_W<'a> {
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
#[doc = "Reader of field `SECURITY_CODE`"]
pub type SECURITY_CODE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SECURITY_CODE`"]
pub struct SECURITY_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURITY_CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write protection bit"]
    #[inline(always)]
    pub fn wprot(&self) -> WPROT_R {
        WPROT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:31 - Write protection mode security code"]
    #[inline(always)]
    pub fn security_code(&self) -> SECURITY_CODE_R {
        SECURITY_CODE_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Write protection bit"]
    #[inline(always)]
    pub fn wprot(&mut self) -> WPROT_W {
        WPROT_W { w: self }
    }
    #[doc = "Bits 8:31 - Write protection mode security code"]
    #[inline(always)]
    pub fn security_code(&mut self) -> SECURITY_CODE_W {
        SECURITY_CODE_W { w: self }
    }
}
