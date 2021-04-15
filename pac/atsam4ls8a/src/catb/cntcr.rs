#[doc = "Reader of register CNTCR"]
pub type R = crate::R<u32, super::CNTCR>;
#[doc = "Writer for register CNTCR"]
pub type W = crate::W<u32, super::CNTCR>;
#[doc = "Register CNTCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CNTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOP`"]
pub type TOP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TOP`"]
pub struct TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Reader of field `SPREAD`"]
pub type SPREAD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPREAD`"]
pub struct SPREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPREAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `REPEAT`"]
pub type REPEAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REPEAT`"]
pub struct REPEAT_W<'a> {
    w: &'a mut W,
}
impl<'a> REPEAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Counter Top Value"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:27 - Spread Spectrum"]
    #[inline(always)]
    pub fn spread(&self) -> SPREAD_R {
        SPREAD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Repeat Measurements"]
    #[inline(always)]
    pub fn repeat(&self) -> REPEAT_R {
        REPEAT_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Counter Top Value"]
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W {
        TOP_W { w: self }
    }
    #[doc = "Bits 24:27 - Spread Spectrum"]
    #[inline(always)]
    pub fn spread(&mut self) -> SPREAD_W {
        SPREAD_W { w: self }
    }
    #[doc = "Bits 28:30 - Repeat Measurements"]
    #[inline(always)]
    pub fn repeat(&mut self) -> REPEAT_W {
        REPEAT_W { w: self }
    }
}
