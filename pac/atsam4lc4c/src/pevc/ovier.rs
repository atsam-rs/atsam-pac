#[doc = "Writer for register OVIER"]
pub type W = crate::W<u32, super::OVIER>;
#[doc = "Register OVIER `reset()`'s with value 0"]
impl crate::ResetValue for super::OVIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `OVIE`"]
pub struct OVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVIE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovie(&mut self) -> OVIE_W {
        OVIE_W { w: self }
    }
}
