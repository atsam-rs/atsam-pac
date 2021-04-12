#[doc = "Writer for register CUPD1"]
pub type W = crate::W<u32, super::CUPD1>;
#[doc = "Write proxy for field `CUPD`"]
pub struct CUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cupd(&mut self) -> CUPD_W {
        CUPD_W { w: self }
    }
}
