#[doc = "Reader of register TIM"]
pub type R = crate::R<u32, super::TIM>;
#[doc = "Writer for register TIM"]
pub type W = crate::W<u32, super::TIM>;
#[doc = "Register TIM `reset()`'s with value 0"]
impl crate::ResetValue for super::TIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STARTUP`"]
pub type STARTUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STARTUP`"]
pub struct STARTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `ENSTUP`"]
pub type ENSTUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENSTUP`"]
pub struct ENSTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSTUP_W<'a> {
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
    #[doc = "Bits 0:4 - Startup time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Enable Startup"]
    #[inline(always)]
    pub fn enstup(&self) -> ENSTUP_R {
        ENSTUP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Startup time"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W {
        STARTUP_W { w: self }
    }
    #[doc = "Bit 8 - Enable Startup"]
    #[inline(always)]
    pub fn enstup(&mut self) -> ENSTUP_W {
        ENSTUP_W { w: self }
    }
}
