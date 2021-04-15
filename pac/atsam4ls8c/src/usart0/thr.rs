#[doc = "Writer for register THR"]
pub type W = crate::W<u32, super::THR>;
#[doc = "Register THR `reset()`'s with value 0"]
impl crate::ResetValue for super::THR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TXCHR`"]
pub struct TXCHR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Sync Field to be transmitted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSYNH_AW {
    #[doc = "0: The next character sent is encoded as a data. Start Frame Delimiter is DATA SYNC"]
    _0 = 0,
    #[doc = "1: The next character sent is encoded as a command. Start Frame Delimiter is COMMAND SYNC"]
    _1 = 1,
}
impl From<TXSYNH_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSYNH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TXSYNH`"]
pub struct TXSYNH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSYNH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSYNH_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The next character sent is encoded as a data. Start Frame Delimiter is DATA SYNC"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXSYNH_AW::_0)
    }
    #[doc = "The next character sent is encoded as a command. Start Frame Delimiter is COMMAND SYNC"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXSYNH_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:8 - Character to be Transmitted"]
    #[inline(always)]
    pub fn txchr(&mut self) -> TXCHR_W {
        TXCHR_W { w: self }
    }
    #[doc = "Bit 15 - Sync Field to be transmitted"]
    #[inline(always)]
    pub fn txsynh(&mut self) -> TXSYNH_W {
        TXSYNH_W { w: self }
    }
}
