#[doc = "Reader of register TRUTH%s"]
pub type R = crate::R<u32, super::TRUTH>;
#[doc = "Writer for register TRUTH%s"]
pub type W = crate::W<u32, super::TRUTH>;
#[doc = "Register TRUTH%s `reset()`'s with value 0"]
impl crate::ResetValue for super::TRUTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRUTH`"]
pub type TRUTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TRUTH`"]
pub struct TRUTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TRUTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Truth"]
    #[inline(always)]
    pub fn truth(&self) -> TRUTH_R {
        TRUTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Truth"]
    #[inline(always)]
    pub fn truth(&mut self) -> TRUTH_W {
        TRUTH_W { w: self }
    }
}
