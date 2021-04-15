#[doc = "Reader of register FPDIV"]
pub type R = crate::R<u32, super::FPDIV>;
#[doc = "Writer for register FPDIV"]
pub type W = crate::W<u32, super::FPDIV>;
#[doc = "Register FPDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::FPDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FPDIV`"]
pub type FPDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FPDIV`"]
pub struct FPDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FPDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Fractional Prescaler Division Factor"]
    #[inline(always)]
    pub fn fpdiv(&self) -> FPDIV_R {
        FPDIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Fractional Prescaler Division Factor"]
    #[inline(always)]
    pub fn fpdiv(&mut self) -> FPDIV_W {
        FPDIV_W { w: self }
    }
}
