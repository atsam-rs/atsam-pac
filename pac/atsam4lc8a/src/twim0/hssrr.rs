#[doc = "Reader of register HSSRR"]
pub type R = crate::R<u32, super::HSSRR>;
#[doc = "Writer for register HSSRR"]
pub type W = crate::W<u32, super::HSSRR>;
#[doc = "Register HSSRR `reset()`'s with value 0"]
impl crate::ResetValue for super::HSSRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DADRIVEL`"]
pub type DADRIVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DADRIVEL`"]
pub struct DADRIVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DADRIVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `DASLEW`"]
pub type DASLEW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DASLEW`"]
pub struct DASLEW_W<'a> {
    w: &'a mut W,
}
impl<'a> DASLEW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CLDRIVEL`"]
pub type CLDRIVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLDRIVEL`"]
pub struct CLDRIVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLDRIVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `CLDRIVEH`"]
pub type CLDRIVEH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLDRIVEH`"]
pub struct CLDRIVEH_W<'a> {
    w: &'a mut W,
}
impl<'a> CLDRIVEH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `CLSLEW`"]
pub type CLSLEW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLSLEW`"]
pub struct CLSLEW_W<'a> {
    w: &'a mut W,
}
impl<'a> CLSLEW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `FILTER`"]
pub type FILTER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILTER`"]
pub struct FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Data Drive Strength LOW"]
    #[inline(always)]
    pub fn dadrivel(&self) -> DADRIVEL_R {
        DADRIVEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Data Slew Limit"]
    #[inline(always)]
    pub fn daslew(&self) -> DASLEW_R {
        DASLEW_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - Clock Drive Strength LOW"]
    #[inline(always)]
    pub fn cldrivel(&self) -> CLDRIVEL_R {
        CLDRIVEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:21 - Clock Drive Strength HIGH"]
    #[inline(always)]
    pub fn cldriveh(&self) -> CLDRIVEH_R {
        CLDRIVEH_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Clock Slew Limit"]
    #[inline(always)]
    pub fn clslew(&self) -> CLSLEW_R {
        CLSLEW_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Input Spike Filter Control"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Data Drive Strength LOW"]
    #[inline(always)]
    pub fn dadrivel(&mut self) -> DADRIVEL_W {
        DADRIVEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Data Slew Limit"]
    #[inline(always)]
    pub fn daslew(&mut self) -> DASLEW_W {
        DASLEW_W { w: self }
    }
    #[doc = "Bits 16:18 - Clock Drive Strength LOW"]
    #[inline(always)]
    pub fn cldrivel(&mut self) -> CLDRIVEL_W {
        CLDRIVEL_W { w: self }
    }
    #[doc = "Bits 20:21 - Clock Drive Strength HIGH"]
    #[inline(always)]
    pub fn cldriveh(&mut self) -> CLDRIVEH_W {
        CLDRIVEH_W { w: self }
    }
    #[doc = "Bits 24:25 - Clock Slew Limit"]
    #[inline(always)]
    pub fn clslew(&mut self) -> CLSLEW_W {
        CLSLEW_W { w: self }
    }
    #[doc = "Bits 28:29 - Input Spike Filter Control"]
    #[inline(always)]
    pub fn filter(&mut self) -> FILTER_W {
        FILTER_W { w: self }
    }
}