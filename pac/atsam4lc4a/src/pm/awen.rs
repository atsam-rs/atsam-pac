#[doc = "Reader of register AWEN"]
pub type R = crate::R<u32, super::AWEN>;
#[doc = "Writer for register AWEN"]
pub type W = crate::W<u32, super::AWEN>;
#[doc = "Register AWEN `reset()`'s with value 0"]
impl crate::ResetValue for super::AWEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AWEN`"]
pub type AWEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AWEN`"]
pub struct AWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AWEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Asynchronous Wake Up"]
    #[inline(always)]
    pub fn awen(&self) -> AWEN_R {
        AWEN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Asynchronous Wake Up"]
    #[inline(always)]
    pub fn awen(&mut self) -> AWEN_W {
        AWEN_W { w: self }
    }
}
