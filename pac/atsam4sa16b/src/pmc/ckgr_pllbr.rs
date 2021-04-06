#[doc = "Reader of register CKGR_PLLBR"]
pub type R = crate::R<u32, super::CKGR_PLLBR>;
#[doc = "Writer for register CKGR_PLLBR"]
pub type W = crate::W<u32, super::CKGR_PLLBR>;
#[doc = "Register CKGR_PLLBR `reset()`'s with value 0x3f00"]
impl crate::ResetValue for super::CKGR_PLLBR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f00
    }
}
#[doc = "Reader of field `DIVB`"]
pub type DIVB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVB`"]
pub struct DIVB_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `PLLBCOUNT`"]
pub type PLLBCOUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLBCOUNT`"]
pub struct PLLBCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLBCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MULB`"]
pub type MULB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MULB`"]
pub struct MULB_W<'a> {
    w: &'a mut W,
}
impl<'a> MULB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PLLB Front-End Divider"]
    #[inline(always)]
    pub fn divb(&self) -> DIVB_R {
        DIVB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - PLLB Counter"]
    #[inline(always)]
    pub fn pllbcount(&self) -> PLLBCOUNT_R {
        PLLBCOUNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:26 - PLLB Multiplier"]
    #[inline(always)]
    pub fn mulb(&self) -> MULB_R {
        MULB_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - PLLB Front-End Divider"]
    #[inline(always)]
    pub fn divb(&mut self) -> DIVB_W {
        DIVB_W { w: self }
    }
    #[doc = "Bits 8:13 - PLLB Counter"]
    #[inline(always)]
    pub fn pllbcount(&mut self) -> PLLBCOUNT_W {
        PLLBCOUNT_W { w: self }
    }
    #[doc = "Bits 16:26 - PLLB Multiplier"]
    #[inline(always)]
    pub fn mulb(&mut self) -> MULB_W {
        MULB_W { w: self }
    }
}
