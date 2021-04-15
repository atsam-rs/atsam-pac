#[doc = "Reader of register DFLL0MUL"]
pub type R = crate::R<u32, super::DFLL0MUL>;
#[doc = "Writer for register DFLL0MUL"]
pub type W = crate::W<u32, super::DFLL0MUL>;
#[doc = "Register DFLL0MUL `reset()`'s with value 0"]
impl crate::ResetValue for super::DFLL0MUL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MUL`"]
pub type MUL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MUL`"]
pub struct MUL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DFLL Multiply Factor"]
    #[inline(always)]
    pub fn mul(&self) -> MUL_R {
        MUL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DFLL Multiply Factor"]
    #[inline(always)]
    pub fn mul(&mut self) -> MUL_W {
        MUL_W { w: self }
    }
}
