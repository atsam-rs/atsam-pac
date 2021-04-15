#[doc = "Reader of register CALIB"]
pub type R = crate::R<u32, super::CALIB>;
#[doc = "Writer for register CALIB"]
pub type W = crate::W<u32, super::CALIB>;
#[doc = "Register CALIB `reset()`'s with value 0"]
impl crate::ResetValue for super::CALIB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALIB`"]
pub type CALIB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CALIB`"]
pub struct CALIB_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `BIASSEL`"]
pub type BIASSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIASSEL`"]
pub struct BIASSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASSEL_W<'a> {
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
#[doc = "Reader of field `BIASCAL`"]
pub type BIASCAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BIASCAL`"]
pub struct BIASCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `FCD`"]
pub type FCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCD`"]
pub struct FCD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Calibration Value"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Select bias mode"]
    #[inline(always)]
    pub fn biassel(&self) -> BIASSEL_R {
        BIASSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Bias Calibration"]
    #[inline(always)]
    pub fn biascal(&self) -> BIASCAL_R {
        BIASCAL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Flash Calibration Done"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calibration Value"]
    #[inline(always)]
    pub fn calib(&mut self) -> CALIB_W {
        CALIB_W { w: self }
    }
    #[doc = "Bit 8 - Select bias mode"]
    #[inline(always)]
    pub fn biassel(&mut self) -> BIASSEL_W {
        BIASSEL_W { w: self }
    }
    #[doc = "Bits 12:15 - Bias Calibration"]
    #[inline(always)]
    pub fn biascal(&mut self) -> BIASCAL_W {
        BIASCAL_W { w: self }
    }
    #[doc = "Bit 16 - Flash Calibration Done"]
    #[inline(always)]
    pub fn fcd(&mut self) -> FCD_W {
        FCD_W { w: self }
    }
}
