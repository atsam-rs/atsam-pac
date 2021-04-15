#[doc = "Register `DMAIER` writer"]
pub struct W(crate::W<DMAIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAIER_SPEC>;
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
impl core::convert::From<crate::W<DMAIER_SPEC>> for W {
    fn from(writer: crate::W<DMAIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAIER` writer - DMA Interrupt Enable"]
pub struct DMAIER_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAIER_W<'a> {
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
impl W {
    #[doc = "Bit 0 - DMA Interrupt Enable"]
    #[inline(always)]
    pub fn dmaier(&mut self) -> DMAIER_W {
        DMAIER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaier](index.html) module"]
pub struct DMAIER_SPEC;
impl crate::RegisterSpec for DMAIER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dmaier::W](W) writer structure"]
impl crate::Writable for DMAIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAIER to value 0"]
impl crate::Resettable for DMAIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
