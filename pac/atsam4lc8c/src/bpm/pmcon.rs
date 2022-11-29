#[doc = "Register `PMCON` reader"]
pub struct R(crate::R<PMCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMCON` writer"]
pub struct W(crate::W<PMCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMCON_SPEC>;
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
impl From<crate::W<PMCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PS` reader - Power Scaling Configuration Value"]
pub type PS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PS` writer - Power Scaling Configuration Value"]
pub type PS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMCON_SPEC, u8, u8, 2, O>;
#[doc = "Field `PSCREQ` reader - Power Scaling Change Request"]
pub type PSCREQ_R = crate::BitReader<bool>;
#[doc = "Field `PSCREQ` writer - Power Scaling Change Request"]
pub type PSCREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCON_SPEC, bool, O>;
#[doc = "Field `PSCM` reader - Power Scaling Change Mode"]
pub type PSCM_R = crate::BitReader<bool>;
#[doc = "Field `PSCM` writer - Power Scaling Change Mode"]
pub type PSCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCON_SPEC, bool, O>;
#[doc = "Field `BKUP` reader - BACKUP Mode"]
pub type BKUP_R = crate::BitReader<bool>;
#[doc = "Field `BKUP` writer - BACKUP Mode"]
pub type BKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCON_SPEC, bool, O>;
#[doc = "Field `RET` reader - RETENTION Mode"]
pub type RET_R = crate::BitReader<bool>;
#[doc = "Field `RET` writer - RETENTION Mode"]
pub type RET_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCON_SPEC, bool, O>;
#[doc = "Field `SLEEP` reader - SLEEP mode Configuration"]
pub type SLEEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLEEP` writer - SLEEP mode Configuration"]
pub type SLEEP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMCON_SPEC, u8, u8, 2, O>;
#[doc = "Field `CK32S` reader - 32Khz-1Khz Clock Source Selection"]
pub type CK32S_R = crate::BitReader<bool>;
#[doc = "Field `CK32S` writer - 32Khz-1Khz Clock Source Selection"]
pub type CK32S_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCON_SPEC, bool, O>;
#[doc = "Field `FASTWKUP` reader - Fast Wakeup"]
pub type FASTWKUP_R = crate::BitReader<bool>;
#[doc = "Field `FASTWKUP` writer - Fast Wakeup"]
pub type FASTWKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCON_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Power Scaling Configuration Value"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Power Scaling Change Request"]
    #[inline(always)]
    pub fn pscreq(&self) -> PSCREQ_R {
        PSCREQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power Scaling Change Mode"]
    #[inline(always)]
    pub fn pscm(&self) -> PSCM_R {
        PSCM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - BACKUP Mode"]
    #[inline(always)]
    pub fn bkup(&self) -> BKUP_R {
        BKUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RETENTION Mode"]
    #[inline(always)]
    pub fn ret(&self) -> RET_R {
        RET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:13 - SLEEP mode Configuration"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - 32Khz-1Khz Clock Source Selection"]
    #[inline(always)]
    pub fn ck32s(&self) -> CK32S_R {
        CK32S_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Fast Wakeup"]
    #[inline(always)]
    pub fn fastwkup(&self) -> FASTWKUP_R {
        FASTWKUP_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power Scaling Configuration Value"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<0> {
        PS_W::new(self)
    }
    #[doc = "Bit 2 - Power Scaling Change Request"]
    #[inline(always)]
    #[must_use]
    pub fn pscreq(&mut self) -> PSCREQ_W<2> {
        PSCREQ_W::new(self)
    }
    #[doc = "Bit 3 - Power Scaling Change Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pscm(&mut self) -> PSCM_W<3> {
        PSCM_W::new(self)
    }
    #[doc = "Bit 8 - BACKUP Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bkup(&mut self) -> BKUP_W<8> {
        BKUP_W::new(self)
    }
    #[doc = "Bit 9 - RETENTION Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ret(&mut self) -> RET_W<9> {
        RET_W::new(self)
    }
    #[doc = "Bits 12:13 - SLEEP mode Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<12> {
        SLEEP_W::new(self)
    }
    #[doc = "Bit 16 - 32Khz-1Khz Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ck32s(&mut self) -> CK32S_W<16> {
        CK32S_W::new(self)
    }
    #[doc = "Bit 24 - Fast Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn fastwkup(&mut self) -> FASTWKUP_W<24> {
        FASTWKUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmcon](index.html) module"]
pub struct PMCON_SPEC;
impl crate::RegisterSpec for PMCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmcon::R](R) reader structure"]
impl crate::Readable for PMCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmcon::W](W) writer structure"]
impl crate::Writable for PMCON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMCON to value 0"]
impl crate::Resettable for PMCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
