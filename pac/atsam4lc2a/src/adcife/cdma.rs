#[doc = "Register `CDMA` writer"]
pub struct W(crate::W<CDMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<CDMA_SPEC>> for W {
    fn from(writer: crate::W<CDMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HWLA` writer - Half word left adjust"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `BIPOLAR` writer - Bipolar Mode"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `STRIG` writer - Sequencer Trigger Event"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `GAIN` writer - Gain factor"]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `GCOMP` writer - Gain Compensation"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `ENSTUP` writer - Enable Start-Up Time"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `RES` writer - Resolution"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TSS` writer - Internal timer start or stop bit"]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `INTERNAL` writer - Internal Voltage Source Selection"]
pub struct INTERNAL_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `MUXPOS` writer - MUX selection on Positive ADC input channel"]
pub struct MUXPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `MUXNEG` writer - MUX selection on Negative ADC input channel"]
pub struct MUXNEG_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXNEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `ZOOMRANGE` writer - Zoom shift/unipolar reference source selection"]
pub struct ZOOMRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> ZOOMRANGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `DW` writer - Double Word transmitting"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Direct Memory Access Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdma](index.html) module"]
pub struct CDMA_SPEC;
impl crate::RegisterSpec for CDMA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cdma::W](W) writer structure"]
impl crate::Writable for CDMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CDMA to value 0"]
impl crate::Resettable for CDMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
