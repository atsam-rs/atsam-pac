#[doc = "Register `RC80MCR` reader"]
pub struct R(crate::R<RC80MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC80MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RC80MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RC80MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RC80MCR` writer"]
pub struct W(crate::W<RC80MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC80MCR_SPEC>;
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
impl From<crate::W<RC80MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RC80MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC80MCR_SPEC, bool, O>;
#[doc = "Field `FCD` reader - Flash Calibration Done"]
pub type FCD_R = crate::BitReader<bool>;
#[doc = "Field `FCD` writer - Flash Calibration Done"]
pub type FCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC80MCR_SPEC, bool, O>;
#[doc = "Field `CALIB` reader - Calibration Value"]
pub type CALIB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALIB` writer - Calibration Value"]
pub type CALIB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RC80MCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Flash Calibration Done"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Calibration Value"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 7 - Flash Calibration Done"]
    #[inline(always)]
    #[must_use]
    pub fn fcd(&mut self) -> FCD_W<7> {
        FCD_W::new(self)
    }
    #[doc = "Bits 16:17 - Calibration Value"]
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
#[doc = "80 MHz RC Oscillator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc80mcr](index.html) module"]
pub struct RC80MCR_SPEC;
impl crate::RegisterSpec for RC80MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc80mcr::R](R) reader structure"]
impl crate::Readable for RC80MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc80mcr::W](W) writer structure"]
impl crate::Writable for RC80MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RC80MCR to value 0"]
impl crate::Resettable for RC80MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
