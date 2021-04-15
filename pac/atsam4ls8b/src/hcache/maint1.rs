#[doc = "Writer for register MAINT1"]
pub type W = crate::W<u32, super::MAINT1>;
#[doc = "Register MAINT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MAINT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `INDEX`"]
pub struct INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl W {
    #[doc = "Bits 4:7 - Invalidate Index"]
    #[inline(always)]
    pub fn index(&mut self) -> INDEX_W {
        INDEX_W { w: self }
    }
}
