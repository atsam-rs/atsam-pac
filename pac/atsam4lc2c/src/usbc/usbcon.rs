#[doc = "Reader of register USBCON"]
pub type R = crate::R<u32, super::USBCON>;
#[doc = "Writer for register USBCON"]
pub type W = crate::W<u32, super::USBCON>;
#[doc = "Register USBCON `reset()`'s with value 0x0100_4000"]
impl crate::ResetValue for super::USBCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100_4000
    }
}
#[doc = "Reader of field `FRZCLK`"]
pub type FRZCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRZCLK`"]
pub struct FRZCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `USBE`"]
pub type USBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBE`"]
pub struct USBE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBE_W<'a> {
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
#[doc = "Reader of field `UIMOD`"]
pub type UIMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UIMOD`"]
pub struct UIMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> UIMOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&self) -> FRZCLK_R {
        FRZCLK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - USBC Enable"]
    #[inline(always)]
    pub fn usbe(&self) -> USBE_R {
        USBE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 24 - USBC Mode"]
    #[inline(always)]
    pub fn uimod(&self) -> UIMOD_R {
        UIMOD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&mut self) -> FRZCLK_W {
        FRZCLK_W { w: self }
    }
    #[doc = "Bit 15 - USBC Enable"]
    #[inline(always)]
    pub fn usbe(&mut self) -> USBE_W {
        USBE_W { w: self }
    }
    #[doc = "Bit 24 - USBC Mode"]
    #[inline(always)]
    pub fn uimod(&mut self) -> UIMOD_W {
        UIMOD_W { w: self }
    }
}
