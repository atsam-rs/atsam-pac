#[doc = "Reader of register SEQCFG"]
pub type R = crate::R<u32, super::SEQCFG>;
#[doc = "Writer for register SEQCFG"]
pub type W = crate::W<u32, super::SEQCFG>;
#[doc = "Register SEQCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SEQCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HWLA`"]
pub type HWLA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HWLA`"]
pub struct HWLA_W<'a> {
    w: &'a mut W,
}
impl<'a> HWLA_W<'a> {
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
#[doc = "Reader of field `BIPOLAR`"]
pub type BIPOLAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIPOLAR`"]
pub struct BIPOLAR_W<'a> {
    w: &'a mut W,
}
impl<'a> BIPOLAR_W<'a> {
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
#[doc = "Reader of field `GAIN`"]
pub type GAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GAIN`"]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `GCOMP`"]
pub type GCOMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCOMP`"]
pub struct GCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> GCOMP_W<'a> {
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
#[doc = "Reader of field `TRGSEL`"]
pub type TRGSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRGSEL`"]
pub struct TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `RES`"]
pub type RES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RES`"]
pub struct RES_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `INTERNAL`"]
pub type INTERNAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTERNAL`"]
pub struct INTERNAL_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `MUXPOS`"]
pub type MUXPOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MUXPOS`"]
pub struct MUXPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MUXNEG`"]
pub type MUXNEG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MUXNEG`"]
pub struct MUXNEG_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXNEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `ZOOMRANGE`"]
pub type ZOOMRANGE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ZOOMRANGE`"]
pub struct ZOOMRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> ZOOMRANGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Half word left adjust"]
    #[inline(always)]
    pub fn hwla(&self) -> HWLA_R {
        HWLA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bipolar Mode"]
    #[inline(always)]
    pub fn bipolar(&self) -> BIPOLAR_R {
        BIPOLAR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Gain factor"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Gain Compensation"]
    #[inline(always)]
    pub fn gcomp(&self) -> GCOMP_R {
        GCOMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Trigger selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Resolution"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Internal Voltage Source Selection"]
    #[inline(always)]
    pub fn internal(&self) -> INTERNAL_R {
        INTERNAL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - MUX selection on Positive ADC input channel"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - MUX selection on Negative ADC input channel"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - Zoom shift/unipolar reference source selection"]
    #[inline(always)]
    pub fn zoomrange(&self) -> ZOOMRANGE_R {
        ZOOMRANGE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Half word left adjust"]
    #[inline(always)]
    pub fn hwla(&mut self) -> HWLA_W {
        HWLA_W { w: self }
    }
    #[doc = "Bit 2 - Bipolar Mode"]
    #[inline(always)]
    pub fn bipolar(&mut self) -> BIPOLAR_W {
        BIPOLAR_W { w: self }
    }
    #[doc = "Bits 4:6 - Gain factor"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W { w: self }
    }
    #[doc = "Bit 7 - Gain Compensation"]
    #[inline(always)]
    pub fn gcomp(&mut self) -> GCOMP_W {
        GCOMP_W { w: self }
    }
    #[doc = "Bits 8:10 - Trigger selection"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TRGSEL_W {
        TRGSEL_W { w: self }
    }
    #[doc = "Bit 12 - Resolution"]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W {
        RES_W { w: self }
    }
    #[doc = "Bits 14:15 - Internal Voltage Source Selection"]
    #[inline(always)]
    pub fn internal(&mut self) -> INTERNAL_W {
        INTERNAL_W { w: self }
    }
    #[doc = "Bits 16:19 - MUX selection on Positive ADC input channel"]
    #[inline(always)]
    pub fn muxpos(&mut self) -> MUXPOS_W {
        MUXPOS_W { w: self }
    }
    #[doc = "Bits 20:22 - MUX selection on Negative ADC input channel"]
    #[inline(always)]
    pub fn muxneg(&mut self) -> MUXNEG_W {
        MUXNEG_W { w: self }
    }
    #[doc = "Bits 28:30 - Zoom shift/unipolar reference source selection"]
    #[inline(always)]
    pub fn zoomrange(&mut self) -> ZOOMRANGE_W {
        ZOOMRANGE_W { w: self }
    }
}
