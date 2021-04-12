#[doc = "Reader of register RPR2"]
pub type R = crate::R<u32, super::RPR2>;
#[doc = "Writer for register RPR2"]
pub type W = crate::W<u32, super::RPR2>;
#[doc = "Register RPR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXPTR`"]
pub type RXPTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RXPTR`"]
pub struct RXPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Receive Pointer Register"]
    #[inline(always)]
    pub fn rxptr(&self) -> RXPTR_R {
        RXPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Pointer Register"]
    #[inline(always)]
    pub fn rxptr(&mut self) -> RXPTR_W {
        RXPTR_W { w: self }
    }
}