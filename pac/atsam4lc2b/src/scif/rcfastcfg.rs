#[doc = "Reader of register RCFASTCFG"]
pub type R = crate::R<u32, super::RCFASTCFG>;
#[doc = "Writer for register RCFASTCFG"]
pub type W = crate::W<u32, super::RCFASTCFG>;
#[doc = "Register RCFASTCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::RCFASTCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Reader of field `TUNEEN`"]
pub type TUNEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TUNEEN`"]
pub struct TUNEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNEEN_W<'a> {
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
#[doc = "Reader of field `JITMODE`"]
pub type JITMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JITMODE`"]
pub struct JITMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> JITMODE_W<'a> {
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
#[doc = "Reader of field `NBPERIODS`"]
pub type NBPERIODS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NBPERIODS`"]
pub struct NBPERIODS_W<'a> {
    w: &'a mut W,
}
impl<'a> NBPERIODS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `FRANGE`"]
pub type FRANGE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRANGE`"]
pub struct FRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRANGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `LOCKMARGIN`"]
pub type LOCKMARGIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LOCKMARGIN`"]
pub struct LOCKMARGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKMARGIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
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
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Oscillator Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tuner Enable"]
    #[inline(always)]
    pub fn tuneen(&self) -> TUNEEN_R {
        TUNEEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Jitter Mode"]
    #[inline(always)]
    pub fn jitmode(&self) -> JITMODE_R {
        JITMODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Number of 32kHz Periods"]
    #[inline(always)]
    pub fn nbperiods(&self) -> NBPERIODS_R {
        NBPERIODS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - RCFAST Fuse Calibration Done"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Frequency Range"]
    #[inline(always)]
    pub fn frange(&self) -> FRANGE_R {
        FRANGE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:15 - Accepted Count Error for Lock"]
    #[inline(always)]
    pub fn lockmargin(&self) -> LOCKMARGIN_R {
        LOCKMARGIN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:22 - Oscillator Calibration Value"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Oscillator Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Tuner Enable"]
    #[inline(always)]
    pub fn tuneen(&mut self) -> TUNEEN_W {
        TUNEEN_W { w: self }
    }
    #[doc = "Bit 2 - Jitter Mode"]
    #[inline(always)]
    pub fn jitmode(&mut self) -> JITMODE_W {
        JITMODE_W { w: self }
    }
    #[doc = "Bits 4:6 - Number of 32kHz Periods"]
    #[inline(always)]
    pub fn nbperiods(&mut self) -> NBPERIODS_W {
        NBPERIODS_W { w: self }
    }
    #[doc = "Bit 7 - RCFAST Fuse Calibration Done"]
    #[inline(always)]
    pub fn fcd(&mut self) -> FCD_W {
        FCD_W { w: self }
    }
    #[doc = "Bits 8:9 - Frequency Range"]
    #[inline(always)]
    pub fn frange(&mut self) -> FRANGE_W {
        FRANGE_W { w: self }
    }
    #[doc = "Bits 12:15 - Accepted Count Error for Lock"]
    #[inline(always)]
    pub fn lockmargin(&mut self) -> LOCKMARGIN_W {
        LOCKMARGIN_W { w: self }
    }
    #[doc = "Bits 16:22 - Oscillator Calibration Value"]
    #[inline(always)]
    pub fn calib(&mut self) -> CALIB_W {
        CALIB_W { w: self }
    }
}
