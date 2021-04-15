#[doc = "Reader of register PPCR"]
pub type R = crate::R<u32, super::PPCR>;
#[doc = "Writer for register PPCR"]
pub type W = crate::W<u32, super::PPCR>;
#[doc = "Register PPCR `reset()`'s with value 0x01fe"]
impl crate::ResetValue for super::PPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01fe
    }
}
#[doc = "Reader of field `RSTPUN`"]
pub type RSTPUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTPUN`"]
pub struct RSTPUN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTPUN_W<'a> {
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
#[doc = "Reader of field `CATBRCMASK`"]
pub type CATBRCMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CATBRCMASK`"]
pub struct CATBRCMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CATBRCMASK_W<'a> {
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
#[doc = "Reader of field `ACIFCRCMASK`"]
pub type ACIFCRCMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACIFCRCMASK`"]
pub struct ACIFCRCMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACIFCRCMASK_W<'a> {
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
#[doc = "Reader of field `ASTRCMASK`"]
pub type ASTRCMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASTRCMASK`"]
pub struct ASTRCMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ASTRCMASK_W<'a> {
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
#[doc = "Reader of field `TWIS0RCMASK`"]
pub type TWIS0RCMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TWIS0RCMASK`"]
pub struct TWIS0RCMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TWIS0RCMASK_W<'a> {
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
#[doc = "Reader of field `TWIS1RCMASK`"]
pub type TWIS1RCMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TWIS1RCMASK`"]
pub struct TWIS1RCMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TWIS1RCMASK_W<'a> {
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
#[doc = "Reader of field `PEVCRCMASK`"]
pub type PEVCRCMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEVCRCMASK`"]
pub struct PEVCRCMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PEVCRCMASK_W<'a> {
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
#[doc = "Reader of field `ADCIFERCMASK`"]
pub type ADCIFERCMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCIFERCMASK`"]
pub struct ADCIFERCMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCIFERCMASK_W<'a> {
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
#[doc = "Reader of field `VREGRCMASK`"]
pub type VREGRCMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREGRCMASK`"]
pub struct VREGRCMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> VREGRCMASK_W<'a> {
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
#[doc = "Reader of field `FWBGREF`"]
pub type FWBGREF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FWBGREF`"]
pub struct FWBGREF_W<'a> {
    w: &'a mut W,
}
impl<'a> FWBGREF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `FWBOD18`"]
pub type FWBOD18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FWBOD18`"]
pub struct FWBOD18_W<'a> {
    w: &'a mut W,
}
impl<'a> FWBOD18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Reset Pullup"]
    #[inline(always)]
    pub fn rstpun(&self) -> RSTPUN_R {
        RSTPUN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CAT Request Clock Mask"]
    #[inline(always)]
    pub fn catbrcmask(&self) -> CATBRCMASK_R {
        CATBRCMASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ACIFC Request Clock Mask"]
    #[inline(always)]
    pub fn acifcrcmask(&self) -> ACIFCRCMASK_R {
        ACIFCRCMASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AST Request Clock Mask"]
    #[inline(always)]
    pub fn astrcmask(&self) -> ASTRCMASK_R {
        ASTRCMASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TWIS0 Request Clock Mask"]
    #[inline(always)]
    pub fn twis0rcmask(&self) -> TWIS0RCMASK_R {
        TWIS0RCMASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TWIS1 Request Clock Mask"]
    #[inline(always)]
    pub fn twis1rcmask(&self) -> TWIS1RCMASK_R {
        TWIS1RCMASK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PEVC Request Clock Mask"]
    #[inline(always)]
    pub fn pevcrcmask(&self) -> PEVCRCMASK_R {
        PEVCRCMASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADCIFE Request Clock Mask"]
    #[inline(always)]
    pub fn adcifercmask(&self) -> ADCIFERCMASK_R {
        ADCIFERCMASK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - VREG Request Clock Mask"]
    #[inline(always)]
    pub fn vregrcmask(&self) -> VREGRCMASK_R {
        VREGRCMASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Flash Wait BGREF"]
    #[inline(always)]
    pub fn fwbgref(&self) -> FWBGREF_R {
        FWBGREF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Flash Wait BOD18"]
    #[inline(always)]
    pub fn fwbod18(&self) -> FWBOD18_R {
        FWBOD18_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset Pullup"]
    #[inline(always)]
    pub fn rstpun(&mut self) -> RSTPUN_W {
        RSTPUN_W { w: self }
    }
    #[doc = "Bit 1 - CAT Request Clock Mask"]
    #[inline(always)]
    pub fn catbrcmask(&mut self) -> CATBRCMASK_W {
        CATBRCMASK_W { w: self }
    }
    #[doc = "Bit 2 - ACIFC Request Clock Mask"]
    #[inline(always)]
    pub fn acifcrcmask(&mut self) -> ACIFCRCMASK_W {
        ACIFCRCMASK_W { w: self }
    }
    #[doc = "Bit 3 - AST Request Clock Mask"]
    #[inline(always)]
    pub fn astrcmask(&mut self) -> ASTRCMASK_W {
        ASTRCMASK_W { w: self }
    }
    #[doc = "Bit 4 - TWIS0 Request Clock Mask"]
    #[inline(always)]
    pub fn twis0rcmask(&mut self) -> TWIS0RCMASK_W {
        TWIS0RCMASK_W { w: self }
    }
    #[doc = "Bit 5 - TWIS1 Request Clock Mask"]
    #[inline(always)]
    pub fn twis1rcmask(&mut self) -> TWIS1RCMASK_W {
        TWIS1RCMASK_W { w: self }
    }
    #[doc = "Bit 6 - PEVC Request Clock Mask"]
    #[inline(always)]
    pub fn pevcrcmask(&mut self) -> PEVCRCMASK_W {
        PEVCRCMASK_W { w: self }
    }
    #[doc = "Bit 7 - ADCIFE Request Clock Mask"]
    #[inline(always)]
    pub fn adcifercmask(&mut self) -> ADCIFERCMASK_W {
        ADCIFERCMASK_W { w: self }
    }
    #[doc = "Bit 8 - VREG Request Clock Mask"]
    #[inline(always)]
    pub fn vregrcmask(&mut self) -> VREGRCMASK_W {
        VREGRCMASK_W { w: self }
    }
    #[doc = "Bit 9 - Flash Wait BGREF"]
    #[inline(always)]
    pub fn fwbgref(&mut self) -> FWBGREF_W {
        FWBGREF_W { w: self }
    }
    #[doc = "Bit 10 - Flash Wait BOD18"]
    #[inline(always)]
    pub fn fwbod18(&mut self) -> FWBOD18_W {
        FWBOD18_W { w: self }
    }
}
