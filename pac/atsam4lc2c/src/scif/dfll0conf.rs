#[doc = "Register `DFLL0CONF` reader"]
pub struct R(crate::R<DFLL0CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLL0CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLL0CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLL0CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLL0CONF` writer"]
pub struct W(crate::W<DFLL0CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLL0CONF_SPEC>;
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
impl From<crate::W<DFLL0CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLL0CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFLL0CONF_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Mode Selection"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - Mode Selection"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFLL0CONF_SPEC, bool, O>;
#[doc = "Field `STABLE` reader - Stable DFLL Frequency"]
pub type STABLE_R = crate::BitReader<bool>;
#[doc = "Field `STABLE` writer - Stable DFLL Frequency"]
pub type STABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFLL0CONF_SPEC, bool, O>;
#[doc = "Field `LLAW` reader - Lose Lock After Wake"]
pub type LLAW_R = crate::BitReader<bool>;
#[doc = "Field `LLAW` writer - Lose Lock After Wake"]
pub type LLAW_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFLL0CONF_SPEC, bool, O>;
#[doc = "Field `CCDIS` reader - Chill Cycle Disable"]
pub type CCDIS_R = crate::BitReader<bool>;
#[doc = "Field `CCDIS` writer - Chill Cycle Disable"]
pub type CCDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFLL0CONF_SPEC, bool, O>;
#[doc = "Field `QLDIS` reader - Quick Lock Disable"]
pub type QLDIS_R = crate::BitReader<bool>;
#[doc = "Field `QLDIS` writer - Quick Lock Disable"]
pub type QLDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFLL0CONF_SPEC, bool, O>;
#[doc = "Field `RANGE` reader - Range Value"]
pub type RANGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RANGE` writer - Range Value"]
pub type RANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFLL0CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `FCD` reader - Fuse Calibration Done"]
pub type FCD_R = crate::BitReader<bool>;
#[doc = "Field `FCD` writer - Fuse Calibration Done"]
pub type FCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFLL0CONF_SPEC, bool, O>;
#[doc = "Field `CALIB` reader - Calibration Value"]
pub type CALIB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALIB` writer - Calibration Value"]
pub type CALIB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFLL0CONF_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode Selection"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stable DFLL Frequency"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lose Lock After Wake"]
    #[inline(always)]
    pub fn llaw(&self) -> LLAW_R {
        LLAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Chill Cycle Disable"]
    #[inline(always)]
    pub fn ccdis(&self) -> CCDIS_R {
        CCDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Quick Lock Disable"]
    #[inline(always)]
    pub fn qldis(&self) -> QLDIS_R {
        QLDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Range Value"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 23 - Fuse Calibration Done"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Calibration Value"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<1> {
        MODE_W::new(self)
    }
    #[doc = "Bit 2 - Stable DFLL Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn stable(&mut self) -> STABLE_W<2> {
        STABLE_W::new(self)
    }
    #[doc = "Bit 3 - Lose Lock After Wake"]
    #[inline(always)]
    #[must_use]
    pub fn llaw(&mut self) -> LLAW_W<3> {
        LLAW_W::new(self)
    }
    #[doc = "Bit 5 - Chill Cycle Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ccdis(&mut self) -> CCDIS_W<5> {
        CCDIS_W::new(self)
    }
    #[doc = "Bit 6 - Quick Lock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn qldis(&mut self) -> QLDIS_W<6> {
        QLDIS_W::new(self)
    }
    #[doc = "Bits 16:17 - Range Value"]
    #[inline(always)]
    #[must_use]
    pub fn range(&mut self) -> RANGE_W<16> {
        RANGE_W::new(self)
    }
    #[doc = "Bit 23 - Fuse Calibration Done"]
    #[inline(always)]
    #[must_use]
    pub fn fcd(&mut self) -> FCD_W<23> {
        FCD_W::new(self)
    }
    #[doc = "Bits 24:27 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn calib(&mut self) -> CALIB_W<24> {
        CALIB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLL0 Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfll0conf](index.html) module"]
pub struct DFLL0CONF_SPEC;
impl crate::RegisterSpec for DFLL0CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfll0conf::R](R) reader structure"]
impl crate::Readable for DFLL0CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfll0conf::W](W) writer structure"]
impl crate::Writable for DFLL0CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFLL0CONF to value 0"]
impl crate::Resettable for DFLL0CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
