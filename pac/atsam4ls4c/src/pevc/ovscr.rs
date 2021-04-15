#[doc = "Writer for register OVSCR"]
pub type W = crate::W<u32, super::OVSCR>;
#[doc = "Register OVSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::OVSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `OVSC`"]
pub struct OVSC_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Overrun Interrupt Status Clear"]
    #[inline(always)]
    pub fn ovsc(&mut self) -> OVSC_W {
        OVSC_W { w: self }
    }
}
