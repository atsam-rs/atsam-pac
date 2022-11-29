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
#[doc = "Field `CCOMP` writer - Command Complete"]
pub type CCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `ANAK` writer - NAK in Address Phase Received"]
pub type ANAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `DNAK` writer - NAK in Data Phase Received"]
pub type DNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `ARBLST` writer - Arbitration Lost"]
pub type ARBLST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `SMBALERT` writer - SMBus Alert"]
pub type SMBALERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `TOUT` writer - Timeout"]
pub type TOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `PECERR` writer - PEC Error"]
pub type PECERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `STOP` writer - Stop Request Accepted"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `HSMCACK` writer - ACK in HS-mode Master Code Phase Received"]
pub type HSMCACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 3 - Command Complete"]
    #[inline(always)]
    #[must_use]
    pub fn ccomp(&mut self) -> CCOMP_W<3> {
        CCOMP_W::new(self)
    }
    #[doc = "Bit 8 - NAK in Address Phase Received"]
    #[inline(always)]
    #[must_use]
    pub fn anak(&mut self) -> ANAK_W<8> {
        ANAK_W::new(self)
    }
    #[doc = "Bit 9 - NAK in Data Phase Received"]
    #[inline(always)]
    #[must_use]
    pub fn dnak(&mut self) -> DNAK_W<9> {
        DNAK_W::new(self)
    }
    #[doc = "Bit 10 - Arbitration Lost"]
    #[inline(always)]
    #[must_use]
    pub fn arblst(&mut self) -> ARBLST_W<10> {
        ARBLST_W::new(self)
    }
    #[doc = "Bit 11 - SMBus Alert"]
    #[inline(always)]
    #[must_use]
    pub fn smbalert(&mut self) -> SMBALERT_W<11> {
        SMBALERT_W::new(self)
    }
    #[doc = "Bit 12 - Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> TOUT_W<12> {
        TOUT_W::new(self)
    }
    #[doc = "Bit 13 - PEC Error"]
    #[inline(always)]
    #[must_use]
    pub fn pecerr(&mut self) -> PECERR_W<13> {
        PECERR_W::new(self)
    }
    #[doc = "Bit 14 - Stop Request Accepted"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<14> {
        STOP_W::new(self)
    }
    #[doc = "Bit 17 - ACK in HS-mode Master Code Phase Received"]
    #[inline(always)]
    #[must_use]
    pub fn hsmcack(&mut self) -> HSMCACK_W<17> {
        HSMCACK_W::new(self)
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
