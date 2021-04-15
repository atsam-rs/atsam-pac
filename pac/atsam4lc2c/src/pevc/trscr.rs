#[doc = "Writer for register TRSCR"]
pub type W = crate::W<u32, super::TRSCR>;
#[doc = "Register TRSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TRSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TRSC`"]
pub struct TRSC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Trigger Interrupt Status Clear"]
    #[inline(always)]
    pub fn trsc(&mut self) -> TRSC_W {
        TRSC_W { w: self }
    }
}
