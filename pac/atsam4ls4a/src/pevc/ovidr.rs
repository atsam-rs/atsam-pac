#[doc = "Writer for register OVIDR"]
pub type W = crate::W<u32, super::OVIDR>;
#[doc = "Register OVIDR `reset()`'s with value 0"]
impl crate::ResetValue for super::OVIDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `OVID`"]
pub struct OVID_W<'a> {
    w: &'a mut W,
}
impl<'a> OVID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Overrun Interrupt Disable"]
    #[inline(always)]
    pub fn ovid(&mut self) -> OVID_W {
        OVID_W { w: self }
    }
}
