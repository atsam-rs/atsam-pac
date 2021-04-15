#[doc = "Reader of register BKUPPMUX"]
pub type R = crate::R<u32, super::BKUPPMUX>;
#[doc = "Writer for register BKUPPMUX"]
pub type W = crate::W<u32, super::BKUPPMUX>;
#[doc = "Register BKUPPMUX `reset()`'s with value 0"]
impl crate::ResetValue for super::BKUPPMUX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BKUPPMUX`"]
pub type BKUPPMUX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BKUPPMUX`"]
pub struct BKUPPMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> BKUPPMUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Backup Pin Muxing"]
    #[inline(always)]
    pub fn bkuppmux(&self) -> BKUPPMUX_R {
        BKUPPMUX_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Backup Pin Muxing"]
    #[inline(always)]
    pub fn bkuppmux(&mut self) -> BKUPPMUX_W {
        BKUPPMUX_W { w: self }
    }
}
