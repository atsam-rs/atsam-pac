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
#[doc = "Reader of field `TLOWS`"]
pub type TLOWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TLOWS`"]
pub struct TLOWS_W<'a> {
    w: &'a mut W,
}
impl<'a> TLOWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TTOUT`"]
pub type TTOUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TTOUT`"]
pub struct TTOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TTOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SUDAT`"]
pub type SUDAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SUDAT`"]
pub struct SUDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SUDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
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
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SMBus Tlow:sext Cycles"]
    #[inline(always)]
    pub fn tlows(&self) -> TLOWS_R {
        TLOWS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SMBus Ttimeout Cycles"]
    #[inline(always)]
    pub fn ttout(&self) -> TTOUT_R {
        TTOUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data Setup Cycles"]
    #[inline(always)]
    pub fn sudat(&self) -> SUDAT_R {
        SUDAT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - Clock Prescaler"]
    #[inline(always)]
    pub fn exp(&self) -> EXP_R {
        EXP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SMBus Tlow:sext Cycles"]
    #[inline(always)]
    pub fn tlows(&mut self) -> TLOWS_W {
        TLOWS_W { w: self }
    }
    #[doc = "Bits 8:15 - SMBus Ttimeout Cycles"]
    #[inline(always)]
    pub fn ttout(&mut self) -> TTOUT_W {
        TTOUT_W { w: self }
    }
    #[doc = "Bits 16:23 - Data Setup Cycles"]
    #[inline(always)]
    pub fn sudat(&mut self) -> SUDAT_W {
        SUDAT_W { w: self }
    }
    #[doc = "Bits 28:31 - Clock Prescaler"]
    #[inline(always)]
    pub fn exp(&mut self) -> EXP_W {
        EXP_W { w: self }
    }
}
