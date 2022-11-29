#[doc = "Register `RCFASTCFG` reader"]
pub struct R(crate::R<RCFASTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCFASTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCFASTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCFASTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCFASTCFG` writer"]
pub struct W(crate::W<RCFASTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCFASTCFG_SPEC>;
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
impl From<crate::W<RCFASTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCFASTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Oscillator Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Oscillator Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCFASTCFG_SPEC, bool, O>;
#[doc = "Field `TUNEEN` reader - Tuner Enable"]
pub type TUNEEN_R = crate::BitReader<bool>;
#[doc = "Field `TUNEEN` writer - Tuner Enable"]
pub type TUNEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCFASTCFG_SPEC, bool, O>;
#[doc = "Field `JITMODE` reader - Jitter Mode"]
pub type JITMODE_R = crate::BitReader<bool>;
#[doc = "Field `JITMODE` writer - Jitter Mode"]
pub type JITMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCFASTCFG_SPEC, bool, O>;
#[doc = "Field `NBPERIODS` reader - Number of 32kHz Periods"]
pub type NBPERIODS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBPERIODS` writer - Number of 32kHz Periods"]
pub type NBPERIODS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCFASTCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `FCD` reader - RCFAST Fuse Calibration Done"]
pub type FCD_R = crate::BitReader<bool>;
#[doc = "Field `FCD` writer - RCFAST Fuse Calibration Done"]
pub type FCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCFASTCFG_SPEC, bool, O>;
#[doc = "Field `FRANGE` reader - Frequency Range"]
pub type FRANGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRANGE` writer - Frequency Range"]
pub type FRANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCFASTCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `LOCKMARGIN` reader - Accepted Count Error for Lock"]
pub type LOCKMARGIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCKMARGIN` writer - Accepted Count Error for Lock"]
pub type LOCKMARGIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCFASTCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `CALIB` reader - Oscillator Calibration Value"]
pub type CALIB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALIB` writer - Oscillator Calibration Value"]
pub type CALIB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCFASTCFG_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Oscillator Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tuner Enable"]
    #[inline(always)]
    pub fn tuneen(&self) -> TUNEEN_R {
        TUNEEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Jitter Mode"]
    #[inline(always)]
    pub fn jitmode(&self) -> JITMODE_R {
        JITMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Number of 32kHz Periods"]
    #[inline(always)]
    pub fn nbperiods(&self) -> NBPERIODS_R {
        NBPERIODS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - RCFAST Fuse Calibration Done"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Frequency Range"]
    #[inline(always)]
    pub fn frange(&self) -> FRANGE_R {
        FRANGE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Accepted Count Error for Lock"]
    #[inline(always)]
    pub fn lockmargin(&self) -> LOCKMARGIN_R {
        LOCKMARGIN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:22 - Oscillator Calibration Value"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Tuner Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tuneen(&mut self) -> TUNEEN_W<1> {
        TUNEEN_W::new(self)
    }
    #[doc = "Bit 2 - Jitter Mode"]
    #[inline(always)]
    #[must_use]
    pub fn jitmode(&mut self) -> JITMODE_W<2> {
        JITMODE_W::new(self)
    }
    #[doc = "Bits 4:6 - Number of 32kHz Periods"]
    #[inline(always)]
    #[must_use]
    pub fn nbperiods(&mut self) -> NBPERIODS_W<4> {
        NBPERIODS_W::new(self)
    }
    #[doc = "Bit 7 - RCFAST Fuse Calibration Done"]
    #[inline(always)]
    #[must_use]
    pub fn fcd(&mut self) -> FCD_W<7> {
        FCD_W::new(self)
    }
    #[doc = "Bits 8:9 - Frequency Range"]
    #[inline(always)]
    #[must_use]
    pub fn frange(&mut self) -> FRANGE_W<8> {
        FRANGE_W::new(self)
    }
    #[doc = "Bits 12:15 - Accepted Count Error for Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lockmargin(&mut self) -> LOCKMARGIN_W<12> {
        LOCKMARGIN_W::new(self)
    }
    #[doc = "Bits 16:22 - Oscillator Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn calib(&mut self) -> CALIB_W<16> {
        CALIB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "4/8/12 MHz RC Oscillator Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcfastcfg](index.html) module"]
pub struct RCFASTCFG_SPEC;
impl crate::RegisterSpec for RCFASTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcfastcfg::R](R) reader structure"]
impl crate::Readable for RCFASTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcfastcfg::W](W) writer structure"]
impl crate::Writable for RCFASTCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCFASTCFG to value 0"]
impl crate::Resettable for RCFASTCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
