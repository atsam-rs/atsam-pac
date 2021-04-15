#[doc = "Writer for register BCR"]
pub type W = crate::W<u32, super::BCR>;
#[doc = "Register BCR `reset()`'s with value 0"]
impl crate::ResetValue for super::BCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Synchro Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Asserts the SYNC signal which generates a software trigger simultaneously for each of the channels."]
    _1 = 1,
}
impl From<SYNC_AW> for bool {
    #[inline(always)]
    fn from(variant: SYNC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SYNC`"]
pub struct SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNC_AW::_0)
    }
    #[doc = "Asserts the SYNC signal which generates a software trigger simultaneously for each of the channels."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNC_AW::_1)
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
    #[doc = "Bit 0 - Synchro Command"]
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W {
        SYNC_W { w: self }
    }
}
