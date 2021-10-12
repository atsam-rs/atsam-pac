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
pub struct RXRDY_R(crate::FieldReader<bool, bool>);
impl RXRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRDY` reader - THR Data Ready"]
pub struct TXRDY_R(crate::FieldReader<bool, bool>);
impl TXRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRDY` reader - Ready for More Commands"]
pub struct CRDY_R(crate::FieldReader<bool, bool>);
impl CRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCOMP` reader - Command Complete"]
pub struct CCOMP_R(crate::FieldReader<bool, bool>);
impl CCOMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCOMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE` reader - Master Interface is Idle"]
pub struct IDLE_R(crate::FieldReader<bool, bool>);
impl IDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSFREE` reader - Two-wire Bus is Free"]
pub struct BUSFREE_R(crate::FieldReader<bool, bool>);
impl BUSFREE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSFREE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSFREE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANAK` reader - NAK in Address Phase Received"]
pub struct ANAK_R(crate::FieldReader<bool, bool>);
impl ANAK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNAK` reader - NAK in Data Phase Received"]
pub struct DNAK_R(crate::FieldReader<bool, bool>);
impl DNAK_R {
    pub(crate) fn new(bits: bool) -> Self {
        DNAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DNAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBLST` reader - Arbitration Lost"]
pub struct ARBLST_R(crate::FieldReader<bool, bool>);
impl ARBLST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARBLST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARBLST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMBALERT` reader - SMBus Alert"]
pub struct SMBALERT_R(crate::FieldReader<bool, bool>);
impl SMBALERT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMBALERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMBALERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUT` reader - Timeout"]
pub struct TOUT_R(crate::FieldReader<bool, bool>);
impl TOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PECERR` reader - PEC Error"]
pub struct PECERR_R(crate::FieldReader<bool, bool>);
impl PECERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PECERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PECERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP` reader - Stop Request Accepted"]
pub struct STOP_R(crate::FieldReader<bool, bool>);
impl STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSMCACK` reader - ACK in HS-mode Master Code Phase Received"]
pub struct HSMCACK_R(crate::FieldReader<bool, bool>);
impl HSMCACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSMCACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSMCACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - RHR Data Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - THR Data Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Ready for More Commands"]
    #[inline(always)]
    pub fn crdy(&self) -> CRDY_R {
        CRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Command Complete"]
    #[inline(always)]
    pub fn ccomp(&self) -> CCOMP_R {
        CCOMP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master Interface is Idle"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Two-wire Bus is Free"]
    #[inline(always)]
    pub fn busfree(&self) -> BUSFREE_R {
        BUSFREE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - NAK in Address Phase Received"]
    #[inline(always)]
    pub fn anak(&self) -> ANAK_R {
        ANAK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - NAK in Data Phase Received"]
    #[inline(always)]
    pub fn dnak(&self) -> DNAK_R {
        DNAK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Arbitration Lost"]
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SMBus Alert"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Timeout"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PEC Error"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Stop Request Accepted"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ACK in HS-mode Master Code Phase Received"]
    #[inline(always)]
    pub fn hsmcack(&self) -> HSMCACK_R {
        HSMCACK_R::new(((self.bits >> 17) & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
