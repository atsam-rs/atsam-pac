#[doc = "Reader of register FPMUL"]
pub type R = crate::R<u32, super::FPMUL>;
#[doc = "Writer for register FPMUL"]
pub type W = crate::W<u32, super::FPMUL>;
#[doc = "Register FPMUL `reset()`'s with value 0"]
impl crate::ResetValue for super::FPMUL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FPMUL`"]
pub type FPMUL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FPMUL`"]
pub struct FPMUL_W<'a> {
    w: &'a mut W,
}
impl<'a> FPMUL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Fractional Prescaler Multiplication Factor"]
    #[inline(always)]
    pub fn fpmul(&self) -> FPMUL_R {
        FPMUL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Fractional Prescaler Multiplication Factor"]
    #[inline(always)]
    pub fn fpmul(&mut self) -> FPMUL_W {
        FPMUL_W { w: self }
    }
}
