#[doc = "Reader of register PLL"]
pub type R = crate::R<u32, super::PLL>;
#[doc = "Writer for register PLL"]
pub type W = crate::W<u32, super::PLL>;
#[doc = "Register PLL `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLLEN`"]
pub type PLLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLEN`"]
pub struct PLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLEN_W<'a> {
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
#[doc = "Reader of field `PLLOSC`"]
pub type PLLOSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLOSC`"]
pub struct PLLOSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLOSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `PLLOPT`"]
pub type PLLOPT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLOPT`"]
pub struct PLLOPT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLOPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `PLLDIV`"]
pub type PLLDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLDIV`"]
pub struct PLLDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PLLMUL`"]
pub type PLLMUL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLMUL`"]
pub struct PLLMUL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLMUL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PLLCOUNT`"]
pub type PLLCOUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLCOUNT`"]
pub struct PLLCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PLL Enable"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - PLL Oscillator Select"]
    #[inline(always)]
    pub fn pllosc(&self) -> PLLOSC_R {
        PLLOSC_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - PLL Option"]
    #[inline(always)]
    pub fn pllopt(&self) -> PLLOPT_R {
        PLLOPT_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - PLL Division Factor"]
    #[inline(always)]
    pub fn plldiv(&self) -> PLLDIV_R {
        PLLDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PLL Multiply Factor"]
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - PLL Count"]
    #[inline(always)]
    pub fn pllcount(&self) -> PLLCOUNT_R {
        PLLCOUNT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Enable"]
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W {
        PLLEN_W { w: self }
    }
    #[doc = "Bits 1:2 - PLL Oscillator Select"]
    #[inline(always)]
    pub fn pllosc(&mut self) -> PLLOSC_W {
        PLLOSC_W { w: self }
    }
    #[doc = "Bits 3:5 - PLL Option"]
    #[inline(always)]
    pub fn pllopt(&mut self) -> PLLOPT_W {
        PLLOPT_W { w: self }
    }
    #[doc = "Bits 8:11 - PLL Division Factor"]
    #[inline(always)]
    pub fn plldiv(&mut self) -> PLLDIV_W {
        PLLDIV_W { w: self }
    }
    #[doc = "Bits 16:19 - PLL Multiply Factor"]
    #[inline(always)]
    pub fn pllmul(&mut self) -> PLLMUL_W {
        PLLMUL_W { w: self }
    }
    #[doc = "Bits 24:29 - PLL Count"]
    #[inline(always)]
    pub fn pllcount(&mut self) -> PLLCOUNT_W {
        PLLCOUNT_W { w: self }
    }
}
