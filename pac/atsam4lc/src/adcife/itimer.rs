#[doc = "Reader of register ITIMER"]
pub type R = crate::R<u32, super::ITIMER>;
#[doc = "Writer for register ITIMER"]
pub type W = crate::W<u32, super::ITIMER>;
#[doc = "Register ITIMER `reset()`'s with value 0"]
impl crate::ResetValue for super::ITIMER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ITMC`"]
pub type ITMC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ITMC`"]
pub struct ITMC_W<'a> {
    w: &'a mut W,
}
impl<'a> ITMC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Internal timer max counter"]
    #[inline(always)]
    pub fn itmc(&self) -> ITMC_R {
        ITMC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Internal timer max counter"]
    #[inline(always)]
    pub fn itmc(&mut self) -> ITMC_W {
        ITMC_W { w: self }
    }
}
