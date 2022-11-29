#[doc = "Register `CDMA` writer"]
pub struct W(crate::W<FIRST_DMA_WORD_CDMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIRST_DMA_WORD_CDMA_SPEC>;
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
impl From<crate::W<FIRST_DMA_WORD_CDMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIRST_DMA_WORD_CDMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HWLA` writer - Half word left adjust"]
pub type HWLA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIRST_DMA_WORD_CDMA_SPEC, bool, O>;
#[doc = "Field `BIPOLAR` writer - Bipolar Mode"]
pub type BIPOLAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIRST_DMA_WORD_CDMA_SPEC, bool, O>;
#[doc = "Field `STRIG` writer - Sequencer Trigger Event"]
pub type STRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIRST_DMA_WORD_CDMA_SPEC, bool, O>;
#[doc = "Field `GAIN` writer - Gain factor"]
pub type GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FIRST_DMA_WORD_CDMA_SPEC, u8, u8, 3, O>;
#[doc = "Field `GCOMP` writer - Gain Compensation"]
pub type GCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIRST_DMA_WORD_CDMA_SPEC, bool, O>;
#[doc = "Field `ENSTUP` writer - Enable Start-Up Time"]
pub type ENSTUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIRST_DMA_WORD_CDMA_SPEC, bool, O>;
#[doc = "Field `RES` writer - Resolution"]
pub type RES_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIRST_DMA_WORD_CDMA_SPEC, bool, O>;
#[doc = "Field `TSS` writer - Internal timer start or stop bit"]
pub type TSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIRST_DMA_WORD_CDMA_SPEC, bool, O>;
#[doc = "Field `INTERNAL` writer - Internal Voltage Source Selection"]
pub type INTERNAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FIRST_DMA_WORD_CDMA_SPEC, u8, u8, 2, O>;
#[doc = "Field `MUXPOS` writer - MUX selection on Positive ADC input channel"]
pub type MUXPOS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FIRST_DMA_WORD_CDMA_SPEC, u8, u8, 4, O>;
#[doc = "Field `MUXNEG` writer - MUX selection on Negative ADC input channel"]
pub type MUXNEG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FIRST_DMA_WORD_CDMA_SPEC, u8, u8, 3, O>;
#[doc = "Field `ZOOMRANGE` writer - Zoom shift/unipolar reference source selection"]
pub type ZOOMRANGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FIRST_DMA_WORD_CDMA_SPEC, u8, u8, 3, O>;
#[doc = "Field `DW` writer - Double Word transmitting"]
pub type DW_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIRST_DMA_WORD_CDMA_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Half word left adjust"]
    #[inline(always)]
    #[must_use]
    pub fn hwla(&mut self) -> HWLA_W<0> {
        HWLA_W::new(self)
    }
    #[doc = "Bit 2 - Bipolar Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bipolar(&mut self) -> BIPOLAR_W<2> {
        BIPOLAR_W::new(self)
    }
    #[doc = "Bit 3 - Sequencer Trigger Event"]
    #[inline(always)]
    #[must_use]
    pub fn strig(&mut self) -> STRIG_W<3> {
        STRIG_W::new(self)
    }
    #[doc = "Bits 4:6 - Gain factor"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GAIN_W<4> {
        GAIN_W::new(self)
    }
    #[doc = "Bit 7 - Gain Compensation"]
    #[inline(always)]
    #[must_use]
    pub fn gcomp(&mut self) -> GCOMP_W<7> {
        GCOMP_W::new(self)
    }
    #[doc = "Bit 8 - Enable Start-Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn enstup(&mut self) -> ENSTUP_W<8> {
        ENSTUP_W::new(self)
    }
    #[doc = "Bit 12 - Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<12> {
        RES_W::new(self)
    }
    #[doc = "Bit 13 - Internal timer start or stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TSS_W<13> {
        TSS_W::new(self)
    }
    #[doc = "Bits 14:15 - Internal Voltage Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn internal(&mut self) -> INTERNAL_W<14> {
        INTERNAL_W::new(self)
    }
    #[doc = "Bits 16:19 - MUX selection on Positive ADC input channel"]
    #[inline(always)]
    #[must_use]
    pub fn muxpos(&mut self) -> MUXPOS_W<16> {
        MUXPOS_W::new(self)
    }
    #[doc = "Bits 20:22 - MUX selection on Negative ADC input channel"]
    #[inline(always)]
    #[must_use]
    pub fn muxneg(&mut self) -> MUXNEG_W<20> {
        MUXNEG_W::new(self)
    }
    #[doc = "Bits 28:30 - Zoom shift/unipolar reference source selection"]
    #[inline(always)]
    #[must_use]
    pub fn zoomrange(&mut self) -> ZOOMRANGE_W<28> {
        ZOOMRANGE_W::new(self)
    }
    #[doc = "Bit 31 - Double Word transmitting"]
    #[inline(always)]
    #[must_use]
    pub fn dw(&mut self) -> DW_W<31> {
        DW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Direct Memory Access Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [first_dma_word_cdma](index.html) module"]
pub struct FIRST_DMA_WORD_CDMA_SPEC;
impl crate::RegisterSpec for FIRST_DMA_WORD_CDMA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [first_dma_word_cdma::W](W) writer structure"]
impl crate::Writable for FIRST_DMA_WORD_CDMA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDMA to value 0"]
impl crate::Resettable for FIRST_DMA_WORD_CDMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
