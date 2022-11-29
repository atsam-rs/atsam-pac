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
#[doc = "Field `RXRDY` reader - RX Buffer Ready"]
pub type RXRDY_R = crate::BitReader<bool>;
#[doc = "Field `TXRDY` reader - TX Buffer Ready"]
pub type TXRDY_R = crate::BitReader<bool>;
#[doc = "Field `TCOMP` reader - Transmission Complete"]
pub type TCOMP_R = crate::BitReader<bool>;
#[doc = "Field `URUN` reader - Underrun"]
pub type URUN_R = crate::BitReader<bool>;
#[doc = "Field `ORUN` reader - Overrun"]
pub type ORUN_R = crate::BitReader<bool>;
#[doc = "Field `NAK` reader - NAK Received"]
pub type NAK_R = crate::BitReader<bool>;
#[doc = "Field `SMBTOUT` reader - SMBus Timeout"]
pub type SMBTOUT_R = crate::BitReader<bool>;
#[doc = "Field `SMBPECERR` reader - SMBus PEC Error"]
pub type SMBPECERR_R = crate::BitReader<bool>;
#[doc = "Field `BUSERR` reader - Bus Error"]
pub type BUSERR_R = crate::BitReader<bool>;
#[doc = "Field `SAM` reader - Slave Address Match"]
pub type SAM_R = crate::BitReader<bool>;
#[doc = "Field `GCM` reader - General Call Match"]
pub type GCM_R = crate::BitReader<bool>;
#[doc = "Field `SMBALERTM` reader - SMBus Alert Response Address Match"]
pub type SMBALERTM_R = crate::BitReader<bool>;
#[doc = "Field `SMBHHM` reader - SMBus Host Header Address Match"]
pub type SMBHHM_R = crate::BitReader<bool>;
#[doc = "Field `SMBDAM` reader - SMBus Default Address Match"]
pub type SMBDAM_R = crate::BitReader<bool>;
#[doc = "Field `STO` reader - Stop Received"]
pub type STO_R = crate::BitReader<bool>;
#[doc = "Field `REP` reader - Repeated Start Received"]
pub type REP_R = crate::BitReader<bool>;
#[doc = "Field `BTF` reader - Byte Transfer Finished"]
pub type BTF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - RX Buffer Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Buffer Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission Complete"]
    #[inline(always)]
    pub fn tcomp(&self) -> TCOMP_R {
        TCOMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Underrun"]
    #[inline(always)]
    pub fn urun(&self) -> URUN_R {
        URUN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Overrun"]
    #[inline(always)]
    pub fn orun(&self) -> ORUN_R {
        ORUN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NAK Received"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - SMBus Timeout"]
    #[inline(always)]
    pub fn smbtout(&self) -> SMBTOUT_R {
        SMBTOUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus PEC Error"]
    #[inline(always)]
    pub fn smbpecerr(&self) -> SMBPECERR_R {
        SMBPECERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bus Error"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave Address Match"]
    #[inline(always)]
    pub fn sam(&self) -> SAM_R {
        SAM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - General Call Match"]
    #[inline(always)]
    pub fn gcm(&self) -> GCM_R {
        GCM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SMBus Alert Response Address Match"]
    #[inline(always)]
    pub fn smbalertm(&self) -> SMBALERTM_R {
        SMBALERTM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SMBus Host Header Address Match"]
    #[inline(always)]
    pub fn smbhhm(&self) -> SMBHHM_R {
        SMBHHM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SMBus Default Address Match"]
    #[inline(always)]
    pub fn smbdam(&self) -> SMBDAM_R {
        SMBDAM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Stop Received"]
    #[inline(always)]
    pub fn sto(&self) -> STO_R {
        STO_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Repeated Start Received"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Byte Transfer Finished"]
    #[inline(always)]
    pub fn btf(&self) -> BTF_R {
        BTF_R::new(((self.bits >> 23) & 1) != 0)
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
