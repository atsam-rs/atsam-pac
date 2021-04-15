#[doc = "Reader of register PBCMASK"]
pub type R = crate::R<u32, super::PBCMASK>;
#[doc = "Writer for register PBCMASK"]
pub type W = crate::W<u32, super::PBCMASK>;
#[doc = "Register PBCMASK `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::PBCMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `PM_`"]
pub type PM__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PM_`"]
pub struct PM__W<'a> {
    w: &'a mut W,
}
impl<'a> PM__W<'a> {
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
#[doc = "Reader of field `CHIPID_`"]
pub type CHIPID__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHIPID_`"]
pub struct CHIPID__W<'a> {
    w: &'a mut W,
}
impl<'a> CHIPID__W<'a> {
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
#[doc = "Reader of field `SCIF_`"]
pub type SCIF__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCIF_`"]
pub struct SCIF__W<'a> {
    w: &'a mut W,
}
impl<'a> SCIF__W<'a> {
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
#[doc = "Reader of field `FREQM_`"]
pub type FREQM__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FREQM_`"]
pub struct FREQM__W<'a> {
    w: &'a mut W,
}
impl<'a> FREQM__W<'a> {
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
#[doc = "Reader of field `GPIO_`"]
pub type GPIO__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_`"]
pub struct GPIO__W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO__W<'a> {
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
impl R {
    #[doc = "Bit 0 - PM APB Clock Enable"]
    #[inline(always)]
    pub fn pm_(&self) -> PM__R {
        PM__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CHIPID APB Clock Enable"]
    #[inline(always)]
    pub fn chipid_(&self) -> CHIPID__R {
        CHIPID__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SCIF APB Clock Enable"]
    #[inline(always)]
    pub fn scif_(&self) -> SCIF__R {
        SCIF__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FREQM APB Clock Enable"]
    #[inline(always)]
    pub fn freqm_(&self) -> FREQM__R {
        FREQM__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO APB Clock Enable"]
    #[inline(always)]
    pub fn gpio_(&self) -> GPIO__R {
        GPIO__R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PM APB Clock Enable"]
    #[inline(always)]
    pub fn pm_(&mut self) -> PM__W {
        PM__W { w: self }
    }
    #[doc = "Bit 1 - CHIPID APB Clock Enable"]
    #[inline(always)]
    pub fn chipid_(&mut self) -> CHIPID__W {
        CHIPID__W { w: self }
    }
    #[doc = "Bit 2 - SCIF APB Clock Enable"]
    #[inline(always)]
    pub fn scif_(&mut self) -> SCIF__W {
        SCIF__W { w: self }
    }
    #[doc = "Bit 3 - FREQM APB Clock Enable"]
    #[inline(always)]
    pub fn freqm_(&mut self) -> FREQM__W {
        FREQM__W { w: self }
    }
    #[doc = "Bit 4 - GPIO APB Clock Enable"]
    #[inline(always)]
    pub fn gpio_(&mut self) -> GPIO__W {
        GPIO__W { w: self }
    }
}
