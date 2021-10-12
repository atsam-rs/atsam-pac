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
pub struct LT_W<'a> {
    w: &'a mut W,
}
impl<'a> LT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `WM` writer - Window Monitor Mode"]
pub struct WM_W<'a> {
    w: &'a mut W,
}
impl<'a> WM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `HT` writer - High Threshold"]
pub struct HT_W<'a> {
    w: &'a mut W,
}
impl<'a> HT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
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
    #[doc = "Bits 0:11 - Low Threshold"]
    #[inline(always)]
    pub fn lt(&mut self) -> LT_W {
        LT_W { w: self }
    }
    #[doc = "Bits 12:14 - Window Monitor Mode"]
    #[inline(always)]
    pub fn wm(&mut self) -> WM_W {
        WM_W { w: self }
    }
    #[doc = "Bits 16:27 - High Threshold"]
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W {
        HT_W { w: self }
    }
    #[doc = "Bit 31 - Double Word transmitting"]
    #[inline(always)]
    pub fn dw(&mut self) -> DW_W {
        DW_W { w: self }
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
}
#[doc = "`reset()` method sets CDMA_ALT to value 0"]
impl crate::Resettable for SECOND_DMA_WORD_CDMA_ALT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
