#[doc = "Register `SEQCFG` reader"]
pub struct R(crate::R<SEQCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQCFG` writer"]
pub struct W(crate::W<SEQCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQCFG_SPEC>;
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
impl From<crate::W<SEQCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HWLA` reader - Half word left adjust"]
pub struct HWLA_R(crate::FieldReader<bool, bool>);
impl HWLA_R {
    pub(crate) fn new(bits: bool) -> Self {
        HWLA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HWLA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `BIPOLAR` reader - Bipolar Mode"]
pub struct BIPOLAR_R(crate::FieldReader<bool, bool>);
impl BIPOLAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIPOLAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIPOLAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `GAIN` reader - Gain factor"]
pub struct GAIN_R(crate::FieldReader<u8, u8>);
impl GAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `GCOMP` reader - Gain Compensation"]
pub struct GCOMP_R(crate::FieldReader<bool, bool>);
impl GCOMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        GCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCOMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `TRGSEL` reader - Trigger selection"]
pub struct TRGSEL_R(crate::FieldReader<u8, u8>);
impl TRGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSEL` writer - Trigger selection"]
pub struct TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `RES` reader - Resolution"]
pub struct RES_R(crate::FieldReader<bool, bool>);
impl RES_R {
    pub(crate) fn new(bits: bool) -> Self {
        RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `INTERNAL` reader - Internal Voltage Source Selection"]
pub struct INTERNAL_R(crate::FieldReader<u8, u8>);
impl INTERNAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        INTERNAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERNAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `MUXPOS` reader - MUX selection on Positive ADC input channel"]
pub struct MUXPOS_R(crate::FieldReader<u8, u8>);
impl MUXPOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        MUXPOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUXPOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `MUXNEG` reader - MUX selection on Negative ADC input channel"]
pub struct MUXNEG_R(crate::FieldReader<u8, u8>);
impl MUXNEG_R {
    pub(crate) fn new(bits: u8) -> Self {
        MUXNEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUXNEG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `ZOOMRANGE` reader - Zoom shift/unipolar reference source selection"]
pub struct ZOOMRANGE_R(crate::FieldReader<u8, u8>);
impl ZOOMRANGE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ZOOMRANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZOOMRANGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequencer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqcfg](index.html) module"]
pub struct SEQCFG_SPEC;
impl crate::RegisterSpec for SEQCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seqcfg::R](R) reader structure"]
impl crate::Readable for SEQCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seqcfg::W](W) writer structure"]
impl crate::Writable for SEQCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQCFG to value 0"]
impl crate::Resettable for SEQCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
