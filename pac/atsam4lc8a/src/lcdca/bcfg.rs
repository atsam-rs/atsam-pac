#[doc = "Reader of register BCFG"]
pub type R = crate::R<u32, super::BCFG>;
#[doc = "Writer for register BCFG"]
pub type W = crate::W<u32, super::BCFG>;
#[doc = "Register BCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::BCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `BSS0`"]
pub type BSS0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BSS0`"]
pub struct BSS0_W<'a> {
    w: &'a mut W,
}
impl<'a> BSS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `BSS1`"]
pub type BSS1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BSS1`"]
pub struct BSS1_W<'a> {
    w: &'a mut W,
}
impl<'a> BSS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Blinking Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Blink Segment Selection 0"]
    #[inline(always)]
    pub fn bss0(&self) -> BSS0_R {
        BSS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Blink Segment Selection 1"]
    #[inline(always)]
    pub fn bss1(&self) -> BSS1_R {
        BSS1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Blinking Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    pub fn fcs(&mut self) -> FCS_W {
        FCS_W { w: self }
    }
    #[doc = "Bits 8:11 - Blink Segment Selection 0"]
    #[inline(always)]
    pub fn bss0(&mut self) -> BSS0_W {
        BSS0_W { w: self }
    }
    #[doc = "Bits 12:15 - Blink Segment Selection 1"]
    #[inline(always)]
    pub fn bss1(&mut self) -> BSS1_W {
        BSS1_W { w: self }
    }
}
