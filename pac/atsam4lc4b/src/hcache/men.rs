#[doc = "Writer for register MEN"]
pub type W = crate::W<u32, super::MEN>;
#[doc = "Register MEN `reset()`'s with value 0"]
impl crate::ResetValue for super::MEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MENABLE_AW {
    #[doc = "0: Disable Monitor Counter"]
    DIS = 0,
    #[doc = "1: Enable Monitor Counter"]
    EN = 1,
}
impl From<MENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: MENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `MENABLE`"]
pub struct MENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MENABLE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Monitor Counter"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MENABLE_AW::DIS)
    }
    #[doc = "Enable Monitor Counter"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MENABLE_AW::EN)
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
    #[doc = "Bit 0 - Monitor Enable"]
    #[inline(always)]
    pub fn menable(&mut self) -> MENABLE_W {
        MENABLE_W { w: self }
    }
}
