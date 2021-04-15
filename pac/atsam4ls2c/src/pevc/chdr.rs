#[doc = "Writer for register CHDR"]
pub type W = crate::W<u32, super::CHDR>;
#[doc = "Register CHDR `reset()`'s with value 0"]
impl crate::ResetValue for super::CHDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CHD`"]
pub struct CHD_W<'a> {
    w: &'a mut W,
}
impl<'a> CHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Disable"]
    #[inline(always)]
    pub fn chd(&mut self) -> CHD_W {
        CHD_W { w: self }
    }
}
