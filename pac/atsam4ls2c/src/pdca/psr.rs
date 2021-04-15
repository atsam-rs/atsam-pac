#[doc = "Reader of register PSR%s"]
pub type R = crate::R<u32, super::PSR>;
#[doc = "Writer for register PSR%s"]
pub type W = crate::W<u32, super::PSR>;
#[doc = "Register PSR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::PSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PID`"]
pub type PID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PID`"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Peripheral Identifier"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Peripheral Identifier"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
}
