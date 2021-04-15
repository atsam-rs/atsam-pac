#[doc = "Reader of register CONFW%s"]
pub type R = crate::R<u32, super::CONFW>;
#[doc = "Writer for register CONFW%s"]
pub type W = crate::W<u32, super::CONFW>;
#[doc = "Register CONFW%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WIS`"]
pub type WIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WIS`"]
pub struct WIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `WEVSRC`"]
pub type WEVSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WEVSRC`"]
pub struct WEVSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> WEVSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `WEVEN`"]
pub type WEVEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WEVEN`"]
pub struct WEVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WEVEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `WFEN`"]
pub type WFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WFEN`"]
pub struct WFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WFEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Window Mode Interrupt Settings"]
    #[inline(always)]
    pub fn wis(&self) -> WIS_R {
        WIS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Peripheral Event Sourse Selection for Window Mode"]
    #[inline(always)]
    pub fn wevsrc(&self) -> WEVSRC_R {
        WEVSRC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Window Peripheral Event Enable"]
    #[inline(always)]
    pub fn weven(&self) -> WEVEN_R {
        WEVEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Window Mode Enable"]
    #[inline(always)]
    pub fn wfen(&self) -> WFEN_R {
        WFEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Window Mode Interrupt Settings"]
    #[inline(always)]
    pub fn wis(&mut self) -> WIS_W {
        WIS_W { w: self }
    }
    #[doc = "Bits 8:10 - Peripheral Event Sourse Selection for Window Mode"]
    #[inline(always)]
    pub fn wevsrc(&mut self) -> WEVSRC_W {
        WEVSRC_W { w: self }
    }
    #[doc = "Bit 11 - Window Peripheral Event Enable"]
    #[inline(always)]
    pub fn weven(&mut self) -> WEVEN_W {
        WEVEN_W { w: self }
    }
    #[doc = "Bit 16 - Window Mode Enable"]
    #[inline(always)]
    pub fn wfen(&mut self) -> WFEN_W {
        WFEN_W { w: self }
    }
}
