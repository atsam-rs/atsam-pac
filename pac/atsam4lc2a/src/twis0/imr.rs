#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IMR_SPEC>> for R {
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRDY` reader - RX Buffer Ready"]
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
#[doc = "Field `TXRDY` reader - TX Buffer Ready"]
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
#[doc = "Field `TCOMP` reader - Transmission Complete"]
pub struct TCOMP_R(crate::FieldReader<bool, bool>);
impl TCOMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCOMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URUN` reader - Underrun"]
pub struct URUN_R(crate::FieldReader<bool, bool>);
impl URUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        URUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ORUN` reader - Overrun"]
pub struct ORUN_R(crate::FieldReader<bool, bool>);
impl ORUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ORUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ORUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAK` reader - NAK Received"]
pub struct NAK_R(crate::FieldReader<bool, bool>);
impl NAK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMBTOUT` reader - SMBus Timeout"]
pub struct SMBTOUT_R(crate::FieldReader<bool, bool>);
impl SMBTOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMBTOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMBTOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMBPECERR` reader - SMBus PEC Error"]
pub struct SMBPECERR_R(crate::FieldReader<bool, bool>);
impl SMBPECERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMBPECERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMBPECERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSERR` reader - Bus Error"]
pub struct BUSERR_R(crate::FieldReader<bool, bool>);
impl BUSERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAM` reader - Slave Address Match"]
pub struct SAM_R(crate::FieldReader<bool, bool>);
impl SAM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCM` reader - General Call Match"]
pub struct GCM_R(crate::FieldReader<bool, bool>);
impl GCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        GCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMBALERTM` reader - SMBus Alert Response Address Match"]
pub struct SMBALERTM_R(crate::FieldReader<bool, bool>);
impl SMBALERTM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMBALERTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMBALERTM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMBHHM` reader - SMBus Host Header Address Match"]
pub struct SMBHHM_R(crate::FieldReader<bool, bool>);
impl SMBHHM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMBHHM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMBHHM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMBDAM` reader - SMBus Default Address Match"]
pub struct SMBDAM_R(crate::FieldReader<bool, bool>);
impl SMBDAM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMBDAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMBDAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STO` reader - Stop Received"]
pub struct STO_R(crate::FieldReader<bool, bool>);
impl STO_R {
    pub(crate) fn new(bits: bool) -> Self {
        STO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REP` reader - Repeated Start Received"]
pub struct REP_R(crate::FieldReader<bool, bool>);
impl REP_R {
    pub(crate) fn new(bits: bool) -> Self {
        REP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTF` reader - Byte Transfer Finished"]
pub struct BTF_R(crate::FieldReader<bool, bool>);
impl BTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
