#[doc = "Writer for register MCTRL"]
pub type W = crate::W<u32, super::MCTRL>;
#[doc = "Register MCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Monitor Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRST_AW {
    #[doc = "0: No effect"]
    NO = 0,
    #[doc = "1: Reset event counter register"]
    YES = 1,
}
impl From<SWRST_AW> for bool {
    #[inline(always)]
    fn from(variant: SWRST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SWRST`"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRST_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(SWRST_AW::NO)
    }
    #[doc = "Reset event counter register"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(SWRST_AW::YES)
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
    #[doc = "Bit 0 - Monitor Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
}
