#[doc = "Register `RCCR` reader"]
pub struct R(crate::R<RCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCCR` writer"]
pub struct W(crate::W<RCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCCR_SPEC>;
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
impl From<crate::W<RCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALIB` reader - Calibration Value"]
pub type CALIB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CALIB` writer - Calibration Value"]
pub type CALIB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCCR_SPEC, u16, u16, 10, O>;
#[doc = "Field `FCD` reader - Flash Calibration Done"]
pub type FCD_R = crate::BitReader<bool>;
#[doc = "Field `FCD` writer - Flash Calibration Done"]
pub type FCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - Calibration Value"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Flash Calibration Done"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn calib(&mut self) -> CALIB_W<0> {
        CALIB_W::new(self)
    }
    #[doc = "Bit 16 - Flash Calibration Done"]
    #[inline(always)]
    #[must_use]
    pub fn fcd(&mut self) -> FCD_W<16> {
        FCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System RC Oscillator Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rccr](index.html) module"]
pub struct RCCR_SPEC;
impl crate::RegisterSpec for RCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rccr::R](R) reader structure"]
impl crate::Readable for RCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rccr::W](W) writer structure"]
impl crate::Writable for RCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCCR to value 0"]
impl crate::Resettable for RCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
