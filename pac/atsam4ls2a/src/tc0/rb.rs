#[doc = "Reader of register RB%s"]
pub type R = crate::R<u32, super::RB>;
#[doc = "Writer for register RB%s"]
pub type W = crate::W<u32, super::RB>;
#[doc = "Register RB%s `reset()`'s with value 0"]
impl crate::ResetValue for super::RB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RB`"]
pub type RB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RB`"]
pub struct RB_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Register B"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Register B"]
    #[inline(always)]
    pub fn rb(&mut self) -> RB_W {
        RB_W { w: self }
    }
}
