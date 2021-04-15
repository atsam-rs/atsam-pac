#[doc = "Reader of register CMCFG"]
pub type R = crate::R<u32, super::CMCFG>;
#[doc = "Writer for register CMCFG"]
pub type W = crate::W<u32, super::CMCFG>;
#[doc = "Register CMCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CMCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DREV`"]
pub type DREV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DREV`"]
pub struct DREV_W<'a> {
    w: &'a mut W,
}
impl<'a> DREV_W<'a> {
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
#[doc = "Reader of field `TDG`"]
pub type TDG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDG`"]
pub struct TDG_W<'a> {
    w: &'a mut W,
}
impl<'a> TDG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `STSEG`"]
pub type STSEG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STSEG`"]
pub struct STSEG_W<'a> {
    w: &'a mut W,
}
impl<'a> STSEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Digit Reverse Mode"]
    #[inline(always)]
    pub fn drev(&self) -> DREV_R {
        DREV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Type of Digit"]
    #[inline(always)]
    pub fn tdg(&self) -> TDG_R {
        TDG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 8:13 - Start Segment"]
    #[inline(always)]
    pub fn stseg(&self) -> STSEG_R {
        STSEG_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Digit Reverse Mode"]
    #[inline(always)]
    pub fn drev(&mut self) -> DREV_W {
        DREV_W { w: self }
    }
    #[doc = "Bits 1:2 - Type of Digit"]
    #[inline(always)]
    pub fn tdg(&mut self) -> TDG_W {
        TDG_W { w: self }
    }
    #[doc = "Bits 8:13 - Start Segment"]
    #[inline(always)]
    pub fn stseg(&mut self) -> STSEG_W {
        STSEG_W { w: self }
    }
}
