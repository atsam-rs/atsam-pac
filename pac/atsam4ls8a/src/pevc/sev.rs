#[doc = "Writer for register SEV"]
pub type W = crate::W<u32, super::SEV>;
#[doc = "Register SEV `reset()`'s with value 0"]
impl crate::ResetValue for super::SEV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SEV`"]
pub struct SEV_W<'a> {
    w: &'a mut W,
}
impl<'a> SEV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Software Event"]
    #[inline(always)]
    pub fn sev(&mut self) -> SEV_W {
        SEV_W { w: self }
    }
}
