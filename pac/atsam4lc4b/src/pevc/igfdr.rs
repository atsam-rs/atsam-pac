#[doc = "Reader of register IGFDR"]
pub type R = crate::R<u32, super::IGFDR>;
#[doc = "Writer for register IGFDR"]
pub type W = crate::W<u32, super::IGFDR>;
#[doc = "Register IGFDR `reset()`'s with value 0"]
impl crate::ResetValue for super::IGFDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IGFDR`"]
pub type IGFDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IGFDR`"]
pub struct IGFDR_W<'a> {
    w: &'a mut W,
}
impl<'a> IGFDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Input Glitch Filter Divider Register"]
    #[inline(always)]
    pub fn igfdr(&self) -> IGFDR_R {
        IGFDR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input Glitch Filter Divider Register"]
    #[inline(always)]
    pub fn igfdr(&mut self) -> IGFDR_W {
        IGFDR_W { w: self }
    }
}
