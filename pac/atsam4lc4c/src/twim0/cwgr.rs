#[doc = "Reader of register CWGR"]
pub type R = crate::R<u32, super::CWGR>;
#[doc = "Writer for register CWGR"]
pub type W = crate::W<u32, super::CWGR>;
#[doc = "Register CWGR `reset()`'s with value 0"]
impl crate::ResetValue for super::CWGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOW`"]
pub type LOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LOW`"]
pub struct LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `HIGH`"]
pub type HIGH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HIGH`"]
pub struct HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `STASTO`"]
pub type STASTO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STASTO`"]
pub struct STASTO_W<'a> {
    w: &'a mut W,
}
impl<'a> STASTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
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
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock Low Cycles"]
    #[inline(always)]
    pub fn low(&self) -> LOW_R {
        LOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock High Cycles"]
    #[inline(always)]
    pub fn high(&self) -> HIGH_R {
        HIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - START and STOP Cycles"]
    #[inline(always)]
    pub fn stasto(&self) -> STASTO_R {
        STASTO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Data Setup and Hold Cycles"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Clock Prescaler"]
    #[inline(always)]
    pub fn exp(&self) -> EXP_R {
        EXP_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Low Cycles"]
    #[inline(always)]
    pub fn low(&mut self) -> LOW_W {
        LOW_W { w: self }
    }
    #[doc = "Bits 8:15 - Clock High Cycles"]
    #[inline(always)]
    pub fn high(&mut self) -> HIGH_W {
        HIGH_W { w: self }
    }
    #[doc = "Bits 16:23 - START and STOP Cycles"]
    #[inline(always)]
    pub fn stasto(&mut self) -> STASTO_W {
        STASTO_W { w: self }
    }
    #[doc = "Bits 24:27 - Data Setup and Hold Cycles"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Bits 28:30 - Clock Prescaler"]
    #[inline(always)]
    pub fn exp(&mut self) -> EXP_W {
        EXP_W { w: self }
    }
}
