#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OSC0RDY` reader - OSC0 Ready"]
pub type OSC0RDY_R = crate::BitReader<bool>;
#[doc = "Field `DFLL0LOCKC` reader - DFLL0 Lock Coarse"]
pub type DFLL0LOCKC_R = crate::BitReader<bool>;
#[doc = "Field `DFLL0LOCKF` reader - DFLL0 Lock Fine"]
pub type DFLL0LOCKF_R = crate::BitReader<bool>;
#[doc = "Field `DFLL0RDY` reader - DFLL0 Ready"]
pub type DFLL0RDY_R = crate::BitReader<bool>;
#[doc = "Field `DFLL0RCS` reader - DFLL0 Reference Clock Stopped"]
pub type DFLL0RCS_R = crate::BitReader<bool>;
#[doc = "Field `DFLL0OOB` reader - DFLL0 Out Of Bounds"]
pub type DFLL0OOB_R = crate::BitReader<bool>;
#[doc = "Field `PLL0LOCK` reader - PLL0 Lock"]
pub type PLL0LOCK_R = crate::BitReader<bool>;
#[doc = "Field `PLL0LOCKLOST` reader - PLL0 Lock Lost"]
pub type PLL0LOCKLOST_R = crate::BitReader<bool>;
#[doc = "Field `RCFASTLOCK` reader - RCFAST Lock"]
pub type RCFASTLOCK_R = crate::BitReader<bool>;
#[doc = "Field `RCFASTLOCKLOST` reader - RCFAST Lock Lost"]
pub type RCFASTLOCKLOST_R = crate::BitReader<bool>;
#[doc = "Field `AE` reader - Access Error"]
pub type AE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - OSC0 Ready"]
    #[inline(always)]
    pub fn osc0rdy(&self) -> OSC0RDY_R {
        OSC0RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DFLL0 Lock Coarse"]
    #[inline(always)]
    pub fn dfll0lockc(&self) -> DFLL0LOCKC_R {
        DFLL0LOCKC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DFLL0 Lock Fine"]
    #[inline(always)]
    pub fn dfll0lockf(&self) -> DFLL0LOCKF_R {
        DFLL0LOCKF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DFLL0 Ready"]
    #[inline(always)]
    pub fn dfll0rdy(&self) -> DFLL0RDY_R {
        DFLL0RDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DFLL0 Reference Clock Stopped"]
    #[inline(always)]
    pub fn dfll0rcs(&self) -> DFLL0RCS_R {
        DFLL0RCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DFLL0 Out Of Bounds"]
    #[inline(always)]
    pub fn dfll0oob(&self) -> DFLL0OOB_R {
        DFLL0OOB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL0 Lock"]
    #[inline(always)]
    pub fn pll0lock(&self) -> PLL0LOCK_R {
        PLL0LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL0 Lock Lost"]
    #[inline(always)]
    pub fn pll0locklost(&self) -> PLL0LOCKLOST_R {
        PLL0LOCKLOST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - RCFAST Lock"]
    #[inline(always)]
    pub fn rcfastlock(&self) -> RCFASTLOCK_R {
        RCFASTLOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RCFAST Lock Lost"]
    #[inline(always)]
    pub fn rcfastlocklost(&self) -> RCFASTLOCKLOST_R {
        RCFASTLOCKLOST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 31 - Access Error"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
