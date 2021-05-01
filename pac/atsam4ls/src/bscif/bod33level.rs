#[doc = "Reader of register BOD33LEVEL"]
pub type R = crate::R<u32, super::BOD33LEVEL>;
#[doc = "Writer for register BOD33LEVEL"]
pub type W = crate::W<u32, super::BOD33LEVEL>;
#[doc = "Register BOD33LEVEL `reset()`'s with value 0"]
impl crate::ResetValue for super::BOD33LEVEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VAL`"]
pub type VAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VAL`"]
pub struct VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - BOD Value"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - BOD Value"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W {
        VAL_W { w: self }
    }
}
