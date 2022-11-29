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
impl From<crate::W<DMAIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAIER` writer - DMA Interrupt Enable"]
pub type DMAIER_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - DMA Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaier(&mut self) -> DMAIER_W<0> {
        DMAIER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAIER to value 0"]
impl crate::Resettable for DMAIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
