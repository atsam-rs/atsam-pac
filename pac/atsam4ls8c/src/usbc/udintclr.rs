#[doc = "Register `UDINTCLR` writer"]
pub struct W(crate::W<UDINTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDINTCLR_SPEC>;
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
impl From<crate::W<UDINTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDINTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSPC` writer - SUSP Interrupt Clear"]
pub type SUSPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTCLR_SPEC, bool, O>;
#[doc = "Field `MSOFC` writer - MSOF Interrupt Clear"]
pub type MSOFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTCLR_SPEC, bool, O>;
#[doc = "Field `SOFC` writer - SOF Interrupt Clear"]
pub type SOFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTCLR_SPEC, bool, O>;
#[doc = "Field `EORSTC` writer - EORST Interrupt Clear"]
pub type EORSTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTCLR_SPEC, bool, O>;
#[doc = "Field `WAKEUPC` writer - WAKEUP Interrupt Clear"]
pub type WAKEUPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTCLR_SPEC, bool, O>;
#[doc = "Field `EORSMC` writer - EORSM Interrupt Clear"]
pub type EORSMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTCLR_SPEC, bool, O>;
#[doc = "Field `UPRSMC` writer - UPRSM Interrupt Clear"]
pub type UPRSMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTCLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - SUSP Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn suspc(&mut self) -> SUSPC_W<0> {
        SUSPC_W::new(self)
    }
    #[doc = "Bit 1 - MSOF Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn msofc(&mut self) -> MSOFC_W<1> {
        MSOFC_W::new(self)
    }
    #[doc = "Bit 2 - SOF Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn sofc(&mut self) -> SOFC_W<2> {
        SOFC_W::new(self)
    }
    #[doc = "Bit 3 - EORST Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eorstc(&mut self) -> EORSTC_W<3> {
        EORSTC_W::new(self)
    }
    #[doc = "Bit 4 - WAKEUP Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupc(&mut self) -> WAKEUPC_W<4> {
        WAKEUPC_W::new(self)
    }
    #[doc = "Bit 5 - EORSM Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eorsmc(&mut self) -> EORSMC_W<5> {
        EORSMC_W::new(self)
    }
    #[doc = "Bit 6 - UPRSM Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn uprsmc(&mut self) -> UPRSMC_W<6> {
        UPRSMC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Global Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udintclr](index.html) module"]
pub struct UDINTCLR_SPEC;
impl crate::RegisterSpec for UDINTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [udintclr::W](W) writer structure"]
impl crate::Writable for UDINTCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDINTCLR to value 0"]
impl crate::Resettable for UDINTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
