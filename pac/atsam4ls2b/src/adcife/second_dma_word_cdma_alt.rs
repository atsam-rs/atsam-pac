#[doc = "Register `CDMA_ALT` writer"]
pub struct W(crate::W<SECOND_DMA_WORD_CDMA_ALT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECOND_DMA_WORD_CDMA_ALT_SPEC>;
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
impl From<crate::W<SECOND_DMA_WORD_CDMA_ALT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECOND_DMA_WORD_CDMA_ALT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LT` writer - Low Threshold"]
pub type LT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECOND_DMA_WORD_CDMA_ALT_SPEC, u16, u16, 12, O>;
#[doc = "Field `WM` writer - Window Monitor Mode"]
pub type WM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECOND_DMA_WORD_CDMA_ALT_SPEC, u8, u8, 3, O>;
#[doc = "Field `HT` writer - High Threshold"]
pub type HT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECOND_DMA_WORD_CDMA_ALT_SPEC, u16, u16, 12, O>;
#[doc = "Field `DW` writer - Double Word transmitting"]
pub type DW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECOND_DMA_WORD_CDMA_ALT_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:11 - Low Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn lt(&mut self) -> LT_W<0> {
        LT_W::new(self)
    }
    #[doc = "Bits 12:14 - Window Monitor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wm(&mut self) -> WM_W<12> {
        WM_W::new(self)
    }
    #[doc = "Bits 16:27 - High Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<16> {
        HT_W::new(self)
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
#[doc = "Configuration Direct Memory Access Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [second_dma_word_cdma_alt](index.html) module"]
pub struct SECOND_DMA_WORD_CDMA_ALT_SPEC;
impl crate::RegisterSpec for SECOND_DMA_WORD_CDMA_ALT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [second_dma_word_cdma_alt::W](W) writer structure"]
impl crate::Writable for SECOND_DMA_WORD_CDMA_ALT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDMA_ALT to value 0"]
impl crate::Resettable for SECOND_DMA_WORD_CDMA_ALT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
