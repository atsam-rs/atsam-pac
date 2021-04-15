#[doc = "Reader of register RC32KTUNE"]
pub type R = crate::R<u32, super::RC32KTUNE>;
#[doc = "Writer for register RC32KTUNE"]
pub type W = crate::W<u32, super::RC32KTUNE>;
#[doc = "Register RC32KTUNE `reset()`'s with value 0"]
impl crate::ResetValue for super::RC32KTUNE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FINE`"]
pub type FINE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FINE`"]
pub struct FINE_W<'a> {
    w: &'a mut W,
}
impl<'a> FINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `COARSE`"]
pub type COARSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COARSE`"]
pub struct COARSE_W<'a> {
    w: &'a mut W,
}
impl<'a> COARSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Fine value"]
    #[inline(always)]
    pub fn fine(&self) -> FINE_R {
        FINE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - Coarse Value"]
    #[inline(always)]
    pub fn coarse(&self) -> COARSE_R {
        COARSE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Fine value"]
    #[inline(always)]
    pub fn fine(&mut self) -> FINE_W {
        FINE_W { w: self }
    }
    #[doc = "Bits 16:22 - Coarse Value"]
    #[inline(always)]
    pub fn coarse(&mut self) -> COARSE_W {
        COARSE_W { w: self }
    }
}
