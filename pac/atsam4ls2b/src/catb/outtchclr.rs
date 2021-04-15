#[doc = "Writer for register OUTTCHCLR%s"]
pub type W = crate::W<u32, super::OUTTCHCLR>;
#[doc = "Register OUTTCHCLR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTTCHCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `OUTTCHCLR`"]
pub struct OUTTCHCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTCHCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Out of Touch"]
    #[inline(always)]
    pub fn outtchclr(&mut self) -> OUTTCHCLR_W {
        OUTTCHCLR_W { w: self }
    }
}
