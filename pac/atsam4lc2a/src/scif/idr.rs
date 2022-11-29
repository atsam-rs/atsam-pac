#[doc = "Register `IDR` writer"]
pub struct W(crate::W<IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR_SPEC>;
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
impl From<crate::W<IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSC0RDY` writer - OSC0 Ready"]
pub type OSC0RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `DFLL0LOCKC` writer - DFLL0 Lock Coarse"]
pub type DFLL0LOCKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `DFLL0LOCKF` writer - DFLL0 Lock Fine"]
pub type DFLL0LOCKF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `DFLL0RDY` writer - DFLL0 Ready"]
pub type DFLL0RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `DFLL0RCS` writer - DFLL0 Reference Clock Stopped"]
pub type DFLL0RCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `DFLL0OOB` writer - DFLL0 Out Of Bounds"]
pub type DFLL0OOB_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `PLL0LOCK` writer - PLL0 Lock"]
pub type PLL0LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `PLL0LOCKLOST` writer - PLL0 Lock Lost"]
pub type PLL0LOCKLOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `RCFASTLOCK` writer - RCFAST Lock"]
pub type RCFASTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `RCFASTLOCKLOST` writer - RCFAST Lock Lost"]
pub type RCFASTLOCKLOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `AE` writer - Access Error"]
pub type AE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - OSC0 Ready"]
    #[inline(always)]
    #[must_use]
    pub fn osc0rdy(&mut self) -> OSC0RDY_W<0> {
        OSC0RDY_W::new(self)
    }
    #[doc = "Bit 1 - DFLL0 Lock Coarse"]
    #[inline(always)]
    #[must_use]
    pub fn dfll0lockc(&mut self) -> DFLL0LOCKC_W<1> {
        DFLL0LOCKC_W::new(self)
    }
    #[doc = "Bit 2 - DFLL0 Lock Fine"]
    #[inline(always)]
    #[must_use]
    pub fn dfll0lockf(&mut self) -> DFLL0LOCKF_W<2> {
        DFLL0LOCKF_W::new(self)
    }
    #[doc = "Bit 3 - DFLL0 Ready"]
    #[inline(always)]
    #[must_use]
    pub fn dfll0rdy(&mut self) -> DFLL0RDY_W<3> {
        DFLL0RDY_W::new(self)
    }
    #[doc = "Bit 4 - DFLL0 Reference Clock Stopped"]
    #[inline(always)]
    #[must_use]
    pub fn dfll0rcs(&mut self) -> DFLL0RCS_W<4> {
        DFLL0RCS_W::new(self)
    }
    #[doc = "Bit 5 - DFLL0 Out Of Bounds"]
    #[inline(always)]
    #[must_use]
    pub fn dfll0oob(&mut self) -> DFLL0OOB_W<5> {
        DFLL0OOB_W::new(self)
    }
    #[doc = "Bit 6 - PLL0 Lock"]
    #[inline(always)]
    #[must_use]
    pub fn pll0lock(&mut self) -> PLL0LOCK_W<6> {
        PLL0LOCK_W::new(self)
    }
    #[doc = "Bit 7 - PLL0 Lock Lost"]
    #[inline(always)]
    #[must_use]
    pub fn pll0locklost(&mut self) -> PLL0LOCKLOST_W<7> {
        PLL0LOCKLOST_W::new(self)
    }
    #[doc = "Bit 13 - RCFAST Lock"]
    #[inline(always)]
    #[must_use]
    pub fn rcfastlock(&mut self) -> RCFASTLOCK_W<13> {
        RCFASTLOCK_W::new(self)
    }
    #[doc = "Bit 14 - RCFAST Lock Lost"]
    #[inline(always)]
    #[must_use]
    pub fn rcfastlocklost(&mut self) -> RCFASTLOCKLOST_W<14> {
        RCFASTLOCKLOST_W::new(self)
    }
    #[doc = "Bit 31 - Access Error"]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AE_W<31> {
        AE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr::W](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
