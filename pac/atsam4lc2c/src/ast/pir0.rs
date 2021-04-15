#[doc = "Reader of register PIR0"]
pub type R = crate::R<u32, super::PIR0>;
#[doc = "Writer for register PIR0"]
pub type W = crate::W<u32, super::PIR0>;
#[doc = "Register PIR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PIR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INSEL`"]
pub type INSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSEL`"]
pub struct INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Interval Select"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Interval Select"]
    #[inline(always)]
    pub fn insel(&mut self) -> INSEL_W {
        INSEL_W { w: self }
    }
}
