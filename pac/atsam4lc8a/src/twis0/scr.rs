#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCOMP` writer - Transmission Complete"]
pub type TCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `URUN` writer - Underrun"]
pub type URUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `ORUN` writer - Overrun"]
pub type ORUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `NAK` writer - NAK Received"]
pub type NAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `SMBTOUT` writer - SMBus Timeout"]
pub type SMBTOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `SMBPECERR` writer - SMBus PEC Error"]
pub type SMBPECERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `BUSERR` writer - Bus Error"]
pub type BUSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `SAM` writer - Slave Address Match"]
pub type SAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `GCM` writer - General Call Match"]
pub type GCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `SMBALERTM` writer - SMBus Alert Response Address Match"]
pub type SMBALERTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `SMBHHM` writer - SMBus Host Header Address Match"]
pub type SMBHHM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `SMBDAM` writer - SMBus Default Address Match"]
pub type SMBDAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `STO` writer - Stop Received"]
pub type STO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `REP` writer - Repeated Start Received"]
pub type REP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `BTF` writer - Byte Transfer Finished"]
pub type BTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 3 - Transmission Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tcomp(&mut self) -> TCOMP_W<3> {
        TCOMP_W::new(self)
    }
    #[doc = "Bit 6 - Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn urun(&mut self) -> URUN_W<6> {
        URUN_W::new(self)
    }
    #[doc = "Bit 7 - Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn orun(&mut self) -> ORUN_W<7> {
        ORUN_W::new(self)
    }
    #[doc = "Bit 8 - NAK Received"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<8> {
        NAK_W::new(self)
    }
    #[doc = "Bit 12 - SMBus Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn smbtout(&mut self) -> SMBTOUT_W<12> {
        SMBTOUT_W::new(self)
    }
    #[doc = "Bit 13 - SMBus PEC Error"]
    #[inline(always)]
    #[must_use]
    pub fn smbpecerr(&mut self) -> SMBPECERR_W<13> {
        SMBPECERR_W::new(self)
    }
    #[doc = "Bit 14 - Bus Error"]
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BUSERR_W<14> {
        BUSERR_W::new(self)
    }
    #[doc = "Bit 16 - Slave Address Match"]
    #[inline(always)]
    #[must_use]
    pub fn sam(&mut self) -> SAM_W<16> {
        SAM_W::new(self)
    }
    #[doc = "Bit 17 - General Call Match"]
    #[inline(always)]
    #[must_use]
    pub fn gcm(&mut self) -> GCM_W<17> {
        GCM_W::new(self)
    }
    #[doc = "Bit 18 - SMBus Alert Response Address Match"]
    #[inline(always)]
    #[must_use]
    pub fn smbalertm(&mut self) -> SMBALERTM_W<18> {
        SMBALERTM_W::new(self)
    }
    #[doc = "Bit 19 - SMBus Host Header Address Match"]
    #[inline(always)]
    #[must_use]
    pub fn smbhhm(&mut self) -> SMBHHM_W<19> {
        SMBHHM_W::new(self)
    }
    #[doc = "Bit 20 - SMBus Default Address Match"]
    #[inline(always)]
    #[must_use]
    pub fn smbdam(&mut self) -> SMBDAM_W<20> {
        SMBDAM_W::new(self)
    }
    #[doc = "Bit 21 - Stop Received"]
    #[inline(always)]
    #[must_use]
    pub fn sto(&mut self) -> STO_W<21> {
        STO_W::new(self)
    }
    #[doc = "Bit 22 - Repeated Start Received"]
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> REP_W<22> {
        REP_W::new(self)
    }
    #[doc = "Bit 23 - Byte Transfer Finished"]
    #[inline(always)]
    #[must_use]
    pub fn btf(&mut self) -> BTF_W<23> {
        BTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
