#[doc = "Reader of register SRR"]
pub type R = crate::R<u32, super::SRR>;
#[doc = "Writer for register SRR"]
pub type W = crate::W<u32, super::SRR>;
#[doc = "Register SRR `reset()`'s with value 0"]
impl crate::ResetValue for super::SRR {
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
    #[doc = "Bits 28:29 - Input Spike Filter Control"]
    #[inline(always)]
    pub fn filter(&mut self) -> FILTER_W {
        FILTER_W { w: self }
    }
}
