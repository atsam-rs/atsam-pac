#[doc = "Register `DMADIS` writer"]
pub struct W(crate::W<DMADIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMADIS_SPEC>;
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
impl From<crate::W<DMADIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMADIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMADIS` writer - DMA Disable"]
pub type DMADIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMADIS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - DMA Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dmadis(&mut self) -> DMADIS_W<0> {
        DMADIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmadis](index.html) module"]
pub struct DMADIS_SPEC;
impl crate::RegisterSpec for DMADIS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dmadis::W](W) writer structure"]
impl crate::Writable for DMADIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMADIS to value 0"]
impl crate::Resettable for DMADIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
