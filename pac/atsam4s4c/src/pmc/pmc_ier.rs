#[doc = "Register `PMC_IER` writer"]
pub struct W(crate::W<PMC_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_IER_SPEC>;
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
impl From<crate::W<PMC_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOSCXTS` writer - Main Crystal Oscillator Status Interrupt Enable"]
pub type MOSCXTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IER_SPEC, bool, O>;
#[doc = "Field `LOCKA` writer - PLLA Lock Interrupt Enable"]
pub type LOCKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IER_SPEC, bool, O>;
#[doc = "Field `LOCKB` writer - PLLB Lock Interrupt Enable"]
pub type LOCKB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IER_SPEC, bool, O>;
#[doc = "Field `MCKRDY` writer - Master Clock Ready Interrupt Enable"]
pub type MCKRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IER_SPEC, bool, O>;
#[doc = "Field `PCKRDY0` writer - Programmable Clock Ready 0 Interrupt Enable"]
pub type PCKRDY0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IER_SPEC, bool, O>;
#[doc = "Field `PCKRDY1` writer - Programmable Clock Ready 1 Interrupt Enable"]
pub type PCKRDY1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IER_SPEC, bool, O>;
#[doc = "Field `PCKRDY2` writer - Programmable Clock Ready 2 Interrupt Enable"]
pub type PCKRDY2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IER_SPEC, bool, O>;
#[doc = "Field `MOSCSELS` writer - Main Oscillator Selection Status Interrupt Enable"]
pub type MOSCSELS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IER_SPEC, bool, O>;
#[doc = "Field `MOSCRCS` writer - Main On-Chip RC Status Interrupt Enable"]
pub type MOSCRCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IER_SPEC, bool, O>;
#[doc = "Field `CFDEV` writer - Clock Failure Detector Event Interrupt Enable"]
pub type CFDEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscxts(&mut self) -> MOSCXTS_W<0> {
        MOSCXTS_W::new(self)
    }
    #[doc = "Bit 1 - PLLA Lock Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn locka(&mut self) -> LOCKA_W<1> {
        LOCKA_W::new(self)
    }
    #[doc = "Bit 2 - PLLB Lock Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lockb(&mut self) -> LOCKB_W<2> {
        LOCKB_W::new(self)
    }
    #[doc = "Bit 3 - Master Clock Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mckrdy(&mut self) -> MCKRDY_W<3> {
        MCKRDY_W::new(self)
    }
    #[doc = "Bit 8 - Programmable Clock Ready 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy0(&mut self) -> PCKRDY0_W<8> {
        PCKRDY0_W::new(self)
    }
    #[doc = "Bit 9 - Programmable Clock Ready 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy1(&mut self) -> PCKRDY1_W<9> {
        PCKRDY1_W::new(self)
    }
    #[doc = "Bit 10 - Programmable Clock Ready 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy2(&mut self) -> PCKRDY2_W<10> {
        PCKRDY2_W::new(self)
    }
    #[doc = "Bit 16 - Main Oscillator Selection Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscsels(&mut self) -> MOSCSELS_W<16> {
        MOSCSELS_W::new(self)
    }
    #[doc = "Bit 17 - Main On-Chip RC Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscrcs(&mut self) -> MOSCRCS_W<17> {
        MOSCRCS_W::new(self)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfdev(&mut self) -> CFDEV_W<18> {
        CFDEV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_ier](index.html) module"]
pub struct PMC_IER_SPEC;
impl crate::RegisterSpec for PMC_IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pmc_ier::W](W) writer structure"]
impl crate::Writable for PMC_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
