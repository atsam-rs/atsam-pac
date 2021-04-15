#[doc = "Reader of register PBBMASK"]
pub type R = crate::R<u32, super::PBBMASK>;
#[doc = "Writer for register PBBMASK"]
pub type W = crate::W<u32, super::PBBMASK>;
#[doc = "Register PBBMASK `reset()`'s with value 0x01"]
impl crate::ResetValue for super::PBBMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `HCACHE_`"]
pub type HCACHE__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HCACHE_`"]
pub struct HCACHE__W<'a> {
    w: &'a mut W,
}
impl<'a> HCACHE__W<'a> {
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
#[doc = "Reader of field `HMATRIX_`"]
pub type HMATRIX__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HMATRIX_`"]
pub struct HMATRIX__W<'a> {
    w: &'a mut W,
}
impl<'a> HMATRIX__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `PEVC_`"]
pub type PEVC__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEVC_`"]
pub struct PEVC__W<'a> {
    w: &'a mut W,
}
impl<'a> PEVC__W<'a> {
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
impl R {
    #[doc = "Bit 0 - HFLASHC APB Clock Enable"]
    #[inline(always)]
    pub fn hflashc_(&self) -> HFLASHC__R {
        HFLASHC__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HCACHE APB Clock Enable"]
    #[inline(always)]
    pub fn hcache_(&self) -> HCACHE__R {
        HCACHE__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HMATRIX APB Clock Enable"]
    #[inline(always)]
    pub fn hmatrix_(&self) -> HMATRIX__R {
        HMATRIX__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PDCA APB Clock Enable"]
    #[inline(always)]
    pub fn pdca_(&self) -> PDCA__R {
        PDCA__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRCCU APB Clock Enable"]
    #[inline(always)]
    pub fn crccu_(&self) -> CRCCU__R {
        CRCCU__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USBC APB Clock Enable"]
    #[inline(always)]
    pub fn usbc_(&self) -> USBC__R {
        USBC__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PEVC APB Clock Enable"]
    #[inline(always)]
    pub fn pevc_(&self) -> PEVC__R {
        PEVC__R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFLASHC APB Clock Enable"]
    #[inline(always)]
    pub fn hflashc_(&mut self) -> HFLASHC__W {
        HFLASHC__W { w: self }
    }
    #[doc = "Bit 1 - HCACHE APB Clock Enable"]
    #[inline(always)]
    pub fn hcache_(&mut self) -> HCACHE__W {
        HCACHE__W { w: self }
    }
    #[doc = "Bit 2 - HMATRIX APB Clock Enable"]
    #[inline(always)]
    pub fn hmatrix_(&mut self) -> HMATRIX__W {
        HMATRIX__W { w: self }
    }
    #[doc = "Bit 3 - PDCA APB Clock Enable"]
    #[inline(always)]
    pub fn pdca_(&mut self) -> PDCA__W {
        PDCA__W { w: self }
    }
    #[doc = "Bit 4 - CRCCU APB Clock Enable"]
    #[inline(always)]
    pub fn crccu_(&mut self) -> CRCCU__W {
        CRCCU__W { w: self }
    }
    #[doc = "Bit 5 - USBC APB Clock Enable"]
    #[inline(always)]
    pub fn usbc_(&mut self) -> USBC__W {
        USBC__W { w: self }
    }
    #[doc = "Bit 6 - PEVC APB Clock Enable"]
    #[inline(always)]
    pub fn pevc_(&mut self) -> PEVC__W {
        PEVC__W { w: self }
    }
}
