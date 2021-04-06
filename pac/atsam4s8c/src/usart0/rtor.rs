#[doc = "Reader of register RTOR"]
pub type R = crate::R<u32, super::RTOR>;
#[doc = "Writer for register RTOR"]
pub type W = crate::W<u32, super::RTOR>;
#[doc = "Register RTOR `reset()`'s with value 0"]
impl crate::ResetValue for super::RTOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TO`"]
pub type TO_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TO`"]
pub struct TO_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Time-out Value"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time-out Value"]
    #[inline(always)]
    pub fn to(&mut self) -> TO_W {
        TO_W { w: self }
    }
}
