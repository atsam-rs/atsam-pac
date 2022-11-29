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
#[doc = "Field `RXRDY` reader - RHR Data Ready"]
pub type RXRDY_R = crate::BitReader<bool>;
#[doc = "Field `TXRDY` reader - THR Data Ready"]
pub type TXRDY_R = crate::BitReader<bool>;
#[doc = "Field `CRDY` reader - Ready for More Commands"]
pub type CRDY_R = crate::BitReader<bool>;
#[doc = "Field `CCOMP` reader - Command Complete"]
pub type CCOMP_R = crate::BitReader<bool>;
#[doc = "Field `IDLE` reader - Master Interface is Idle"]
pub type IDLE_R = crate::BitReader<bool>;
#[doc = "Field `BUSFREE` reader - Two-wire Bus is Free"]
pub type BUSFREE_R = crate::BitReader<bool>;
#[doc = "Field `ANAK` reader - NAK in Address Phase Received"]
pub type ANAK_R = crate::BitReader<bool>;
#[doc = "Field `DNAK` reader - NAK in Data Phase Received"]
pub type DNAK_R = crate::BitReader<bool>;
#[doc = "Field `ARBLST` reader - Arbitration Lost"]
pub type ARBLST_R = crate::BitReader<bool>;
#[doc = "Field `SMBALERT` reader - SMBus Alert"]
pub type SMBALERT_R = crate::BitReader<bool>;
#[doc = "Field `TOUT` reader - Timeout"]
pub type TOUT_R = crate::BitReader<bool>;
#[doc = "Field `PECERR` reader - PEC Error"]
pub type PECERR_R = crate::BitReader<bool>;
#[doc = "Field `STOP` reader - Stop Request Accepted"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `HSMCACK` reader - ACK in HS-mode Master Code Phase Received"]
pub type HSMCACK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - RHR Data Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - THR Data Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ready for More Commands"]
    #[inline(always)]
    pub fn crdy(&self) -> CRDY_R {
        CRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command Complete"]
    #[inline(always)]
    pub fn ccomp(&self) -> CCOMP_R {
        CCOMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master Interface is Idle"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Two-wire Bus is Free"]
    #[inline(always)]
    pub fn busfree(&self) -> BUSFREE_R {
        BUSFREE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - NAK in Address Phase Received"]
    #[inline(always)]
    pub fn anak(&self) -> ANAK_R {
        ANAK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NAK in Data Phase Received"]
    #[inline(always)]
    pub fn dnak(&self) -> DNAK_R {
        DNAK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Arbitration Lost"]
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SMBus Alert"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timeout"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PEC Error"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop Request Accepted"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - ACK in HS-mode Master Code Phase Received"]
    #[inline(always)]
    pub fn hsmcack(&self) -> HSMCACK_R {
        HSMCACK_R::new(((self.bits >> 17) & 1) != 0)
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
