#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `OSC0RDY`"]
pub type OSC0RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLL0LOCKC`"]
pub type DFLL0LOCKC_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLL0LOCKF`"]
pub type DFLL0LOCKF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLL0RDY`"]
pub type DFLL0RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLL0RCS`"]
pub type DFLL0RCS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLL0OOB`"]
pub type DFLL0OOB_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLL0LOCK`"]
pub type PLL0LOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLL0LOCKLOST`"]
pub type PLL0LOCKLOST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RCFASTLOCK`"]
pub type RCFASTLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `RCFASTLOCKLOST`"]
pub type RCFASTLOCKLOST_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE`"]
pub type AE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - OSC0 Ready"]
    #[inline(always)]
    pub fn osc0rdy(&self) -> OSC0RDY_R {
        OSC0RDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DFLL0 Lock Coarse"]
    #[inline(always)]
    pub fn dfll0lockc(&self) -> DFLL0LOCKC_R {
        DFLL0LOCKC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DFLL0 Lock Fine"]
    #[inline(always)]
    pub fn dfll0lockf(&self) -> DFLL0LOCKF_R {
        DFLL0LOCKF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DFLL0 Ready"]
    #[inline(always)]
    pub fn dfll0rdy(&self) -> DFLL0RDY_R {
        DFLL0RDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DFLL0 Reference Clock Stopped"]
    #[inline(always)]
    pub fn dfll0rcs(&self) -> DFLL0RCS_R {
        DFLL0RCS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DFLL0 Out Of Bounds"]
    #[inline(always)]
    pub fn dfll0oob(&self) -> DFLL0OOB_R {
        DFLL0OOB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL0 Lock"]
    #[inline(always)]
    pub fn pll0lock(&self) -> PLL0LOCK_R {
        PLL0LOCK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PLL0 Lock Lost"]
    #[inline(always)]
    pub fn pll0locklost(&self) -> PLL0LOCKLOST_R {
        PLL0LOCKLOST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RCFAST Lock"]
    #[inline(always)]
    pub fn rcfastlock(&self) -> RCFASTLOCK_R {
        RCFASTLOCK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RCFAST Lock Lost"]
    #[inline(always)]
    pub fn rcfastlocklost(&self) -> RCFASTLOCKLOST_R {
        RCFASTLOCKLOST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Access Error"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
