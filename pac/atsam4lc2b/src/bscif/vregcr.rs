#[doc = "Reader of register VREGCR"]
pub type R = crate::R<u32, super::VREGCR>;
#[doc = "Writer for register VREGCR"]
pub type W = crate::W<u32, super::VREGCR>;
#[doc = "Register VREGCR `reset()`'s with value 0"]
impl crate::ResetValue for super::VREGCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIS`"]
pub type DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS`"]
pub struct DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_W<'a> {
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
#[doc = "Reader of field `SSG`"]
pub type SSG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSG`"]
pub struct SSG_W<'a> {
    w: &'a mut W,
}
impl<'a> SSG_W<'a> {
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
#[doc = "Reader of field `SSW`"]
pub type SSW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSW`"]
pub struct SSW_W<'a> {
    w: &'a mut W,
}
impl<'a> SSW_W<'a> {
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
#[doc = "Reader of field `SSWEVT`"]
pub type SSWEVT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSWEVT`"]
pub struct SSWEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> SSWEVT_W<'a> {
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
#[doc = "Reader of field `SFV`"]
pub type SFV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SFV`"]
pub struct SFV_W<'a> {
    w: &'a mut W,
}
impl<'a> SFV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Voltage Regulator disable"]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Spread Spectrum Generator Enable"]
    #[inline(always)]
    pub fn ssg(&self) -> SSG_R {
        SSG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Stop Switching"]
    #[inline(always)]
    pub fn ssw(&self) -> SSW_R {
        SSW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Stop Switching On Event Enable"]
    #[inline(always)]
    pub fn sswevt(&self) -> SSWEVT_R {
        SSWEVT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Store Final Value"]
    #[inline(always)]
    pub fn sfv(&self) -> SFV_R {
        SFV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Regulator disable"]
    #[inline(always)]
    pub fn dis(&mut self) -> DIS_W {
        DIS_W { w: self }
    }
    #[doc = "Bit 8 - Spread Spectrum Generator Enable"]
    #[inline(always)]
    pub fn ssg(&mut self) -> SSG_W {
        SSG_W { w: self }
    }
    #[doc = "Bit 9 - Stop Switching"]
    #[inline(always)]
    pub fn ssw(&mut self) -> SSW_W {
        SSW_W { w: self }
    }
    #[doc = "Bit 10 - Stop Switching On Event Enable"]
    #[inline(always)]
    pub fn sswevt(&mut self) -> SSWEVT_W {
        SSWEVT_W { w: self }
    }
    #[doc = "Bit 31 - Store Final Value"]
    #[inline(always)]
    pub fn sfv(&mut self) -> SFV_W {
        SFV_W { w: self }
    }
}
