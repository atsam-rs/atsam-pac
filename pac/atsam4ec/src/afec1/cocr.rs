#[doc = "Reader of register COCR"]
pub type R = crate::R<u32, super::COCR>;
#[doc = "Writer for register COCR"]
pub type W = crate::W<u32, super::COCR>;
#[doc = "Register COCR `reset()`'s with value 0"]
impl crate::ResetValue for super::COCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AOFF`"]
pub type AOFF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AOFF`"]
pub struct AOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> AOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Analog Offset"]
    #[inline(always)]
    pub fn aoff(&self) -> AOFF_R {
        AOFF_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog Offset"]
    #[inline(always)]
    pub fn aoff(&mut self) -> AOFF_W {
        AOFF_W { w: self }
    }
}
