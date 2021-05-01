#[doc = "Writer for register UNLOCK"]
pub type W = crate::W<u32, super::UNLOCK>;
#[doc = "Register UNLOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::UNLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:9 - Unlock Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bits 24:31 - Unlock Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
