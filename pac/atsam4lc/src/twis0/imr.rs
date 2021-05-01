#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `RXRDY`"]
pub type RXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRDY`"]
pub type TXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCOMP`"]
pub type TCOMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `URUN`"]
pub type URUN_R = crate::R<bool, bool>;
#[doc = "Reader of field `ORUN`"]
pub type ORUN_R = crate::R<bool, bool>;
#[doc = "Reader of field `NAK`"]
pub type NAK_R = crate::R<bool, bool>;
#[doc = "Reader of field `SMBTOUT`"]
pub type SMBTOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SMBPECERR`"]
pub type SMBPECERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSERR`"]
pub type BUSERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SAM`"]
pub type SAM_R = crate::R<bool, bool>;
#[doc = "Reader of field `GCM`"]
pub type GCM_R = crate::R<bool, bool>;
#[doc = "Reader of field `SMBALERTM`"]
pub type SMBALERTM_R = crate::R<bool, bool>;
#[doc = "Reader of field `SMBHHM`"]
pub type SMBHHM_R = crate::R<bool, bool>;
#[doc = "Reader of field `SMBDAM`"]
pub type SMBDAM_R = crate::R<bool, bool>;
#[doc = "Reader of field `STO`"]
pub type STO_R = crate::R<bool, bool>;
#[doc = "Reader of field `REP`"]
pub type REP_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTF`"]
pub type BTF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - RX Buffer Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TX Buffer Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmission Complete"]
    #[inline(always)]
    pub fn tcomp(&self) -> TCOMP_R {
        TCOMP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Underrun"]
    #[inline(always)]
    pub fn urun(&self) -> URUN_R {
        URUN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Overrun"]
    #[inline(always)]
    pub fn orun(&self) -> ORUN_R {
        ORUN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - NAK Received"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SMBus Timeout"]
    #[inline(always)]
    pub fn smbtout(&self) -> SMBTOUT_R {
        SMBTOUT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SMBus PEC Error"]
    #[inline(always)]
    pub fn smbpecerr(&self) -> SMBPECERR_R {
        SMBPECERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Bus Error"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Slave Address Match"]
    #[inline(always)]
    pub fn sam(&self) -> SAM_R {
        SAM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - General Call Match"]
    #[inline(always)]
    pub fn gcm(&self) -> GCM_R {
        GCM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SMBus Alert Response Address Match"]
    #[inline(always)]
    pub fn smbalertm(&self) -> SMBALERTM_R {
        SMBALERTM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SMBus Host Header Address Match"]
    #[inline(always)]
    pub fn smbhhm(&self) -> SMBHHM_R {
        SMBHHM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SMBus Default Address Match"]
    #[inline(always)]
    pub fn smbdam(&self) -> SMBDAM_R {
        SMBDAM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Stop Received"]
    #[inline(always)]
    pub fn sto(&self) -> STO_R {
        STO_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Repeated Start Received"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Byte Transfer Finished"]
    #[inline(always)]
    pub fn btf(&self) -> BTF_R {
        BTF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
