#[doc = "Register `IMR0` reader"]
pub struct R(crate::R<IMR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COVFS` reader - Counter Overflow"]
pub type COVFS_R = crate::BitReader<bool>;
#[doc = "Field `LOVRS` reader - Load Overrun"]
pub type LOVRS_R = crate::BitReader<bool>;
#[doc = "Field `CPAS` reader - RA Compare"]
pub type CPAS_R = crate::BitReader<bool>;
#[doc = "Field `CPBS` reader - RB Compare"]
pub type CPBS_R = crate::BitReader<bool>;
#[doc = "Field `CPCS` reader - RC Compare"]
pub type CPCS_R = crate::BitReader<bool>;
#[doc = "Field `LDRAS` reader - RA Loading"]
pub type LDRAS_R = crate::BitReader<bool>;
#[doc = "Field `LDRBS` reader - RB Loading"]
pub type LDRBS_R = crate::BitReader<bool>;
#[doc = "Field `ETRGS` reader - External Trigger"]
pub type ETRGS_R = crate::BitReader<bool>;
#[doc = "Field `ENDRX` reader - End of Receiver Transfer"]
pub type ENDRX_R = crate::BitReader<bool>;
#[doc = "Field `RXBUFF` reader - Reception Buffer Full"]
pub type RXBUFF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Counter Overflow"]
    #[inline(always)]
    pub fn covfs(&self) -> COVFS_R {
        COVFS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load Overrun"]
    #[inline(always)]
    pub fn lovrs(&self) -> LOVRS_R {
        LOVRS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RA Compare"]
    #[inline(always)]
    pub fn cpas(&self) -> CPAS_R {
        CPAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RB Compare"]
    #[inline(always)]
    pub fn cpbs(&self) -> CPBS_R {
        CPBS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RC Compare"]
    #[inline(always)]
    pub fn cpcs(&self) -> CPCS_R {
        CPCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RA Loading"]
    #[inline(always)]
    pub fn ldras(&self) -> LDRAS_R {
        LDRAS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RB Loading"]
    #[inline(always)]
    pub fn ldrbs(&self) -> LDRBS_R {
        LDRBS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    pub fn etrgs(&self) -> ETRGS_R {
        ETRGS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - End of Receiver Transfer"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reception Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr0](index.html) module"]
pub struct IMR0_SPEC;
impl crate::RegisterSpec for IMR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr0::R](R) reader structure"]
impl crate::Readable for IMR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR0 to value 0"]
impl crate::Resettable for IMR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
