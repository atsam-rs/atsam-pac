#[doc = "Reader of register TCRR%s"]
pub type R = crate::R<u32, super::TCRR>;
#[doc = "Writer for register TCRR%s"]
pub type W = crate::W<u32, super::TCRR>;
#[doc = "Register TCRR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::TCRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TCRV`"]
pub type TCRV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TCRV`"]
pub struct TCRV_W<'a> {
    w: &'a mut W,
}
impl<'a> TCRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Transfer Counter Reload Value"]
    #[inline(always)]
    pub fn tcrv(&self) -> TCRV_R {
        TCRV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transfer Counter Reload Value"]
    #[inline(always)]
    pub fn tcrv(&mut self) -> TCRV_W {
        TCRV_W { w: self }
    }
}
