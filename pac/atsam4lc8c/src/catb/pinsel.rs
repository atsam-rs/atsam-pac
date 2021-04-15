#[doc = "Reader of register PINSEL"]
pub type R = crate::R<u32, super::PINSEL>;
#[doc = "Writer for register PINSEL"]
pub type W = crate::W<u32, super::PINSEL>;
#[doc = "Register PINSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::PINSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PINSEL`"]
pub type PINSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PINSEL`"]
pub struct PINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Pin Select"]
    #[inline(always)]
    pub fn pinsel(&self) -> PINSEL_R {
        PINSEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pin Select"]
    #[inline(always)]
    pub fn pinsel(&mut self) -> PINSEL_W {
        PINSEL_W { w: self }
    }
}
