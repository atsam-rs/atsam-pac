#[doc = "Writer for register CDMA"]
pub type W = crate::W<u32, super::CDMA>;
#[doc = "Register CDMA `reset()`'s with value 0"]
impl crate::ResetValue for super::CDMA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
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
#[doc = "Write proxy for field `STRIG`"]
pub struct STRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> STRIG_W<'a> {
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
#[doc = "Write proxy for field `ENSTUP`"]
pub struct ENSTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSTUP_W<'a> {
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
#[doc = "Write proxy for field `TSS`"]
pub struct TSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
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
#[doc = "Write proxy for field `DW`"]
pub struct DW_W<'a> {
    w: &'a mut W,
}
impl<'a> DW_W<'a> {
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
    #[doc = "Bit 3 - Sequencer Trigger Event"]
    #[inline(always)]
    pub fn strig(&mut self) -> STRIG_W {
        STRIG_W { w: self }
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
    #[doc = "Bit 8 - Enable Start-Up Time"]
    #[inline(always)]
    pub fn enstup(&mut self) -> ENSTUP_W {
        ENSTUP_W { w: self }
    }
    #[doc = "Bit 12 - Resolution"]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W {
        RES_W { w: self }
    }
    #[doc = "Bit 13 - Internal timer start or stop bit"]
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W {
        TSS_W { w: self }
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
    #[doc = "Bit 31 - Double Word transmitting"]
    #[inline(always)]
    pub fn dw(&mut self) -> DW_W {
        DW_W { w: self }
    }
}
