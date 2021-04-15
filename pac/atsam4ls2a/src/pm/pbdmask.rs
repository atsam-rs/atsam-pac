#[doc = "Reader of register PBDMASK"]
pub type R = crate::R<u32, super::PBDMASK>;
#[doc = "Writer for register PBDMASK"]
pub type W = crate::W<u32, super::PBDMASK>;
#[doc = "Register PBDMASK `reset()`'s with value 0x3f"]
impl crate::ResetValue for super::PBDMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f
    }
}
#[doc = "Reader of field `BPM_`"]
pub type BPM__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BPM_`"]
pub struct BPM__W<'a> {
    w: &'a mut W,
}
impl<'a> BPM__W<'a> {
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
#[doc = "Reader of field `BSCIF_`"]
pub type BSCIF__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSCIF_`"]
pub struct BSCIF__W<'a> {
    w: &'a mut W,
}
impl<'a> BSCIF__W<'a> {
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
#[doc = "Reader of field `AST_`"]
pub type AST__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AST_`"]
pub struct AST__W<'a> {
    w: &'a mut W,
}
impl<'a> AST__W<'a> {
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
#[doc = "Reader of field `WDT_`"]
pub type WDT__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_`"]
pub struct WDT__W<'a> {
    w: &'a mut W,
}
impl<'a> WDT__W<'a> {
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
#[doc = "Reader of field `EIC_`"]
pub type EIC__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EIC_`"]
pub struct EIC__W<'a> {
    w: &'a mut W,
}
impl<'a> EIC__W<'a> {
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
#[doc = "Reader of field `PICOUART_`"]
pub type PICOUART__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PICOUART_`"]
pub struct PICOUART__W<'a> {
    w: &'a mut W,
}
impl<'a> PICOUART__W<'a> {
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
impl R {
    #[doc = "Bit 0 - BPM APB Clock Enable"]
    #[inline(always)]
    pub fn bpm_(&self) -> BPM__R {
        BPM__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BSCIF APB Clock Enable"]
    #[inline(always)]
    pub fn bscif_(&self) -> BSCIF__R {
        BSCIF__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AST APB Clock Enable"]
    #[inline(always)]
    pub fn ast_(&self) -> AST__R {
        AST__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WDT APB Clock Enable"]
    #[inline(always)]
    pub fn wdt_(&self) -> WDT__R {
        WDT__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EIC APB Clock Enable"]
    #[inline(always)]
    pub fn eic_(&self) -> EIC__R {
        EIC__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PICOUART APB Clock Enable"]
    #[inline(always)]
    pub fn picouart_(&self) -> PICOUART__R {
        PICOUART__R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BPM APB Clock Enable"]
    #[inline(always)]
    pub fn bpm_(&mut self) -> BPM__W {
        BPM__W { w: self }
    }
    #[doc = "Bit 1 - BSCIF APB Clock Enable"]
    #[inline(always)]
    pub fn bscif_(&mut self) -> BSCIF__W {
        BSCIF__W { w: self }
    }
    #[doc = "Bit 2 - AST APB Clock Enable"]
    #[inline(always)]
    pub fn ast_(&mut self) -> AST__W {
        AST__W { w: self }
    }
    #[doc = "Bit 3 - WDT APB Clock Enable"]
    #[inline(always)]
    pub fn wdt_(&mut self) -> WDT__W {
        WDT__W { w: self }
    }
    #[doc = "Bit 4 - EIC APB Clock Enable"]
    #[inline(always)]
    pub fn eic_(&mut self) -> EIC__W {
        EIC__W { w: self }
    }
    #[doc = "Bit 5 - PICOUART APB Clock Enable"]
    #[inline(always)]
    pub fn picouart_(&mut self) -> PICOUART__W {
        PICOUART__W { w: self }
    }
}
