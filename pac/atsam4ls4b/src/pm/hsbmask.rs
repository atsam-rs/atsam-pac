#[doc = "Reader of register HSBMASK"]
pub type R = crate::R<u32, super::HSBMASK>;
#[doc = "Writer for register HSBMASK"]
pub type W = crate::W<u32, super::HSBMASK>;
#[doc = "Register HSBMASK `reset()`'s with value 0x01e2"]
impl crate::ResetValue for super::HSBMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01e2
    }
}
#[doc = "Reader of field `PDCA_`"]
pub type PDCA__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDCA_`"]
pub struct PDCA__W<'a> {
    w: &'a mut W,
}
impl<'a> PDCA__W<'a> {
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
#[doc = "Reader of field `HFLASHC_`"]
pub type HFLASHC__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFLASHC_`"]
pub struct HFLASHC__W<'a> {
    w: &'a mut W,
}
impl<'a> HFLASHC__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `HRAMC1_`"]
pub type HRAMC1__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HRAMC1_`"]
pub struct HRAMC1__W<'a> {
    w: &'a mut W,
}
impl<'a> HRAMC1__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `USBC_`"]
pub type USBC__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBC_`"]
pub struct USBC__W<'a> {
    w: &'a mut W,
}
impl<'a> USBC__W<'a> {
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
#[doc = "Reader of field `CRCCU_`"]
pub type CRCCU__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCCU_`"]
pub struct CRCCU__W<'a> {
    w: &'a mut W,
}
impl<'a> CRCCU__W<'a> {
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
#[doc = "Reader of field `HTOP0_`"]
pub type HTOP0__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTOP0_`"]
pub struct HTOP0__W<'a> {
    w: &'a mut W,
}
impl<'a> HTOP0__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `HTOP1_`"]
pub type HTOP1__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTOP1_`"]
pub struct HTOP1__W<'a> {
    w: &'a mut W,
}
impl<'a> HTOP1__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `HTOP2_`"]
pub type HTOP2__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTOP2_`"]
pub struct HTOP2__W<'a> {
    w: &'a mut W,
}
impl<'a> HTOP2__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `HTOP3_`"]
pub type HTOP3__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTOP3_`"]
pub struct HTOP3__W<'a> {
    w: &'a mut W,
}
impl<'a> HTOP3__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PDCA HSB Clock Mask"]
    #[inline(always)]
    pub fn pdca_(&self) -> PDCA__R {
        PDCA__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HFLASHC HSB Clock Mask"]
    #[inline(always)]
    pub fn hflashc_(&self) -> HFLASHC__R {
        HFLASHC__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HRAMC1 HSB Clock Mask"]
    #[inline(always)]
    pub fn hramc1_(&self) -> HRAMC1__R {
        HRAMC1__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USBC HSB Clock Mask"]
    #[inline(always)]
    pub fn usbc_(&self) -> USBC__R {
        USBC__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRCCU HSB Clock Mask"]
    #[inline(always)]
    pub fn crccu_(&self) -> CRCCU__R {
        CRCCU__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HTOP0 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop0_(&self) -> HTOP0__R {
        HTOP0__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HTOP1 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop1_(&self) -> HTOP1__R {
        HTOP1__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - HTOP2 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop2_(&self) -> HTOP2__R {
        HTOP2__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HTOP3 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop3_(&self) -> HTOP3__R {
        HTOP3__R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDCA HSB Clock Mask"]
    #[inline(always)]
    pub fn pdca_(&mut self) -> PDCA__W {
        PDCA__W { w: self }
    }
    #[doc = "Bit 1 - HFLASHC HSB Clock Mask"]
    #[inline(always)]
    pub fn hflashc_(&mut self) -> HFLASHC__W {
        HFLASHC__W { w: self }
    }
    #[doc = "Bit 2 - HRAMC1 HSB Clock Mask"]
    #[inline(always)]
    pub fn hramc1_(&mut self) -> HRAMC1__W {
        HRAMC1__W { w: self }
    }
    #[doc = "Bit 3 - USBC HSB Clock Mask"]
    #[inline(always)]
    pub fn usbc_(&mut self) -> USBC__W {
        USBC__W { w: self }
    }
    #[doc = "Bit 4 - CRCCU HSB Clock Mask"]
    #[inline(always)]
    pub fn crccu_(&mut self) -> CRCCU__W {
        CRCCU__W { w: self }
    }
    #[doc = "Bit 5 - HTOP0 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop0_(&mut self) -> HTOP0__W {
        HTOP0__W { w: self }
    }
    #[doc = "Bit 6 - HTOP1 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop1_(&mut self) -> HTOP1__W {
        HTOP1__W { w: self }
    }
    #[doc = "Bit 7 - HTOP2 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop2_(&mut self) -> HTOP2__W {
        HTOP2__W { w: self }
    }
    #[doc = "Bit 8 - HTOP3 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop3_(&mut self) -> HTOP3__W {
        HTOP3__W { w: self }
    }
}
