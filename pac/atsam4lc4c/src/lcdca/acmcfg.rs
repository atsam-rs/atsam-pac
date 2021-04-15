#[doc = "Reader of register ACMCFG"]
pub type R = crate::R<u32, super::ACMCFG>;
#[doc = "Writer for register ACMCFG"]
pub type W = crate::W<u32, super::ACMCFG>;
#[doc = "Register ACMCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::ACMCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Reader of field `FCS`"]
pub type FCS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FCS`"]
pub struct FCS_W<'a> {
    w: &'a mut W,
}
impl<'a> FCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
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
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
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
#[doc = "Reader of field `STEPS`"]
pub type STEPS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STEPS`"]
pub struct STEPS_W<'a> {
    w: &'a mut W,
}
impl<'a> STEPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DIGN`"]
pub type DIGN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIGN`"]
pub struct DIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIGN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Mode (sequential or scrolling)"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Digit Reverse"]
    #[inline(always)]
    pub fn drev(&self) -> DREV_R {
        DREV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Type of Digit"]
    #[inline(always)]
    pub fn tdg(&self) -> TDG_R {
        TDG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 8:13 - Start Segment"]
    #[inline(always)]
    pub fn stseg(&self) -> STSEG_R {
        STSEG_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - Scrolling Steps"]
    #[inline(always)]
    pub fn steps(&self) -> STEPS_R {
        STEPS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Digit Number"]
    #[inline(always)]
    pub fn dign(&self) -> DIGN_R {
        DIGN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    pub fn fcs(&mut self) -> FCS_W {
        FCS_W { w: self }
    }
    #[doc = "Bit 3 - Mode (sequential or scrolling)"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 4 - Digit Reverse"]
    #[inline(always)]
    pub fn drev(&mut self) -> DREV_W {
        DREV_W { w: self }
    }
    #[doc = "Bits 5:6 - Type of Digit"]
    #[inline(always)]
    pub fn tdg(&mut self) -> TDG_W {
        TDG_W { w: self }
    }
    #[doc = "Bits 8:13 - Start Segment"]
    #[inline(always)]
    pub fn stseg(&mut self) -> STSEG_W {
        STSEG_W { w: self }
    }
    #[doc = "Bits 16:23 - Scrolling Steps"]
    #[inline(always)]
    pub fn steps(&mut self) -> STEPS_W {
        STEPS_W { w: self }
    }
    #[doc = "Bits 24:27 - Digit Number"]
    #[inline(always)]
    pub fn dign(&mut self) -> DIGN_W {
        DIGN_W { w: self }
    }
}
