#[doc = "Register `PMC_PMMR` reader"]
pub struct R(crate::R<PMC_PMMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_PMMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_PMMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_PMMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC_PMMR` writer"]
pub struct W(crate::W<PMC_PMMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_PMMR_SPEC>;
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
impl From<crate::W<PMC_PMMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_PMMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLA_MMAX` reader - PLLA Maximum Allowed Multiplier Value"]
pub type PLLA_MMAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PLLA_MMAX` writer - PLLA Maximum Allowed Multiplier Value"]
pub type PLLA_MMAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMC_PMMR_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - PLLA Maximum Allowed Multiplier Value"]
    #[inline(always)]
    pub fn plla_mmax(&self) -> PLLA_MMAX_R {
        PLLA_MMAX_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - PLLA Maximum Allowed Multiplier Value"]
    #[inline(always)]
    #[must_use]
    pub fn plla_mmax(&mut self) -> PLLA_MMAX_W<0> {
        PLLA_MMAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Maximum Multiplier Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pmmr](index.html) module"]
pub struct PMC_PMMR_SPEC;
impl crate::RegisterSpec for PMC_PMMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_pmmr::R](R) reader structure"]
impl crate::Readable for PMC_PMMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_pmmr::W](W) writer structure"]
impl crate::Writable for PMC_PMMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMC_PMMR to value 0x07ff_07ff"]
impl crate::Resettable for PMC_PMMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07ff_07ff;
}
