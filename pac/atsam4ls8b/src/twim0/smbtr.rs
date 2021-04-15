#[doc = "Reader of register SMBTR"]
pub type R = crate::R<u32, super::SMBTR>;
#[doc = "Writer for register SMBTR"]
pub type W = crate::W<u32, super::SMBTR>;
#[doc = "Register SMBTR `reset()`'s with value 0"]
impl crate::ResetValue for super::SMBTR {
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
#[doc = "Reader of field `TLOWM`"]
pub type TLOWM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TLOWM`"]
pub struct TLOWM_W<'a> {
    w: &'a mut W,
}
impl<'a> TLOWM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `THMAX`"]
pub type THMAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `THMAX`"]
pub struct THMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> THMAX_W<'a> {
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
    #[doc = "Bits 0:7 - Slave Clock stretch maximum cycles"]
    #[inline(always)]
    pub fn tlows(&self) -> TLOWS_R {
        TLOWS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Master Clock stretch maximum cycles"]
    #[inline(always)]
    pub fn tlowm(&self) -> TLOWM_R {
        TLOWM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Clock High maximum cycles"]
    #[inline(always)]
    pub fn thmax(&self) -> THMAX_R {
        THMAX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - SMBus Timeout Clock prescaler"]
    #[inline(always)]
    pub fn exp(&self) -> EXP_R {
        EXP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave Clock stretch maximum cycles"]
    #[inline(always)]
    pub fn tlows(&mut self) -> TLOWS_W {
        TLOWS_W { w: self }
    }
    #[doc = "Bits 8:15 - Master Clock stretch maximum cycles"]
    #[inline(always)]
    pub fn tlowm(&mut self) -> TLOWM_W {
        TLOWM_W { w: self }
    }
    #[doc = "Bits 16:23 - Clock High maximum cycles"]
    #[inline(always)]
    pub fn thmax(&mut self) -> THMAX_W {
        THMAX_W { w: self }
    }
    #[doc = "Bits 28:31 - SMBus Timeout Clock prescaler"]
    #[inline(always)]
    pub fn exp(&mut self) -> EXP_W {
        EXP_W { w: self }
    }
}
