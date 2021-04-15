#[doc = "Writer for register INITVECT%s"]
pub type W = crate::W<u32, super::INITVECT>;
#[doc = "Register INITVECT%s `reset()`'s with value 0"]
impl crate::ResetValue for super::INITVECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `INITVECT0`"]
pub struct INITVECT0_W<'a> {
    w: &'a mut W,
}
impl<'a> INITVECT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization Vector Word 0"]
    #[inline(always)]
    pub fn initvect0(&mut self) -> INITVECT0_W {
        INITVECT0_W { w: self }
    }
}
