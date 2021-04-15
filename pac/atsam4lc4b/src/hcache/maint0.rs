#[doc = "Writer for register MAINT0"]
pub type W = crate::W<u32, super::MAINT0>;
#[doc = "Register MAINT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MAINT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Cache Controller Invalidate All\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVALL_AW {
    #[doc = "0: No effect"]
    NO = 0,
    #[doc = "1: Invalidate all cache entries"]
    YES = 1,
}
impl From<INVALL_AW> for bool {
    #[inline(always)]
    fn from(variant: INVALL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `INVALL`"]
pub struct INVALL_W<'a> {
    w: &'a mut W,
}
impl<'a> INVALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVALL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(INVALL_AW::NO)
    }
    #[doc = "Invalidate all cache entries"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(INVALL_AW::YES)
    }
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
impl W {
    #[doc = "Bit 0 - Cache Controller Invalidate All"]
    #[inline(always)]
    pub fn invall(&mut self) -> INVALL_W {
        INVALL_W { w: self }
    }
}
