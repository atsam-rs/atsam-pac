#[doc = "Reader of register THRESH"]
pub type R = crate::R<u32, super::THRESH>;
#[doc = "Writer for register THRESH"]
pub type W = crate::W<u32, super::THRESH>;
#[doc = "Register THRESH `reset()`'s with value 0"]
impl crate::ResetValue for super::THRESH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FTHRESH`"]
pub type FTHRESH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FTHRESH`"]
pub struct FTHRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> FTHRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `RTHRESH`"]
pub type RTHRESH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTHRESH`"]
pub struct RTHRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTHRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | (((value as u32) & 0xff) << 12);
        self.w
    }
}
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `LENGTH`"]
pub type LENGTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LENGTH`"]
pub struct LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Fractional part of Threshold Value"]
    #[inline(always)]
    pub fn fthresh(&self) -> FTHRESH_R {
        FTHRESH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - Rational part of Threshold Value"]
    #[inline(always)]
    pub fn rthresh(&self) -> RTHRESH_R {
        RTHRESH_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 23 - Threshold Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - Threshold Length"]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Fractional part of Threshold Value"]
    #[inline(always)]
    pub fn fthresh(&mut self) -> FTHRESH_W {
        FTHRESH_W { w: self }
    }
    #[doc = "Bits 12:19 - Rational part of Threshold Value"]
    #[inline(always)]
    pub fn rthresh(&mut self) -> RTHRESH_W {
        RTHRESH_W { w: self }
    }
    #[doc = "Bit 23 - Threshold Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bits 24:28 - Threshold Length"]
    #[inline(always)]
    pub fn length(&mut self) -> LENGTH_W {
        LENGTH_W { w: self }
    }
}
