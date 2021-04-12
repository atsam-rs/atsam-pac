#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GCLKDIS`"]
pub type GCLKDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCLKDIS`"]
pub struct GCLKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> GCLKDIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Disable Clock Gating"]
    #[inline(always)]
    pub fn gclkdis(&self) -> GCLKDIS_R {
        GCLKDIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Clock Gating"]
    #[inline(always)]
    pub fn gclkdis(&mut self) -> GCLKDIS_W {
        GCLKDIS_W { w: self }
    }
}
