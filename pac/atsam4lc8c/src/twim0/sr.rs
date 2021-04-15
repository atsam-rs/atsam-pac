#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `RXRDY`"]
pub type RXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRDY`"]
pub type TXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRDY`"]
pub type CRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCOMP`"]
pub type CCOMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDLE`"]
pub type IDLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSFREE`"]
pub type BUSFREE_R = crate::R<bool, bool>;
#[doc = "Reader of field `ANAK`"]
pub type ANAK_R = crate::R<bool, bool>;
#[doc = "Reader of field `DNAK`"]
pub type DNAK_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARBLST`"]
pub type ARBLST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SMBALERT`"]
pub type SMBALERT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOUT`"]
pub type TOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `PECERR`"]
pub type PECERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `STOP`"]
pub type STOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `MENB`"]
pub type MENB_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSMCACK`"]
pub type HSMCACK_R = crate::R<bool, bool>;
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
    #[doc = "Bit 16 - Master Interface Enable"]
    #[inline(always)]
    pub fn menb(&self) -> MENB_R {
        MENB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ACK in HS-mode Master Code Phase Received"]
    #[inline(always)]
    pub fn hsmcack(&self) -> HSMCACK_R {
        HSMCACK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
