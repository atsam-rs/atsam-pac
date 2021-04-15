#[doc = "Writer for register INTCHCLR%s"]
pub type W = crate::W<u32, super::INTCHCLR>;
#[doc = "Register INTCHCLR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCHCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `INTCHCLR`"]
pub struct INTCHCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCHCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - In-Touch Clear"]
    #[inline(always)]
    pub fn intchclr(&mut self) -> INTCHCLR_W {
        INTCHCLR_W { w: self }
    }
}
