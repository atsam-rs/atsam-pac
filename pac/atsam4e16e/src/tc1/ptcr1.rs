#[doc = "Register `PTCR1` writer"]
pub struct W(crate::W<PTCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTCR1_SPEC>;
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
impl From<crate::W<PTCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXTEN` writer - Receiver Transfer Enable"]
pub type RXTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTCR1_SPEC, bool, O>;
#[doc = "Field `RXTDIS` writer - Receiver Transfer Disable"]
pub type RXTDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTCR1_SPEC, bool, O>;
#[doc = "Field `TXTEN` writer - Transmitter Transfer Enable"]
pub type TXTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTCR1_SPEC, bool, O>;
#[doc = "Field `TXTDIS` writer - Transmitter Transfer Disable"]
pub type TXTDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTCR1_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Receiver Transfer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxten(&mut self) -> RXTEN_W<0> {
        RXTEN_W::new(self)
    }
    #[doc = "Bit 1 - Receiver Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxtdis(&mut self) -> RXTDIS_W<1> {
        RXTDIS_W::new(self)
    }
    #[doc = "Bit 8 - Transmitter Transfer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txten(&mut self) -> TXTEN_W<8> {
        TXTEN_W::new(self)
    }
    #[doc = "Bit 9 - Transmitter Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txtdis(&mut self) -> TXTDIS_W<9> {
        TXTDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer Control Register (pdc = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptcr1](index.html) module"]
pub struct PTCR1_SPEC;
impl crate::RegisterSpec for PTCR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ptcr1::W](W) writer structure"]
impl crate::Writable for PTCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTCR1 to value 0"]
impl crate::Resettable for PTCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
