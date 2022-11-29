#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HRMOD` reader - 12-/24-hour Mode"]
pub type HRMOD_R = crate::BitReader<bool>;
#[doc = "Field `HRMOD` writer - 12-/24-hour Mode"]
pub type HRMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `PERSIAN` reader - PERSIAN Calendar"]
pub type PERSIAN_R = crate::BitReader<bool>;
#[doc = "Field `PERSIAN` writer - PERSIAN Calendar"]
pub type PERSIAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `NEGPPM` reader - NEGative PPM Correction"]
pub type NEGPPM_R = crate::BitReader<bool>;
#[doc = "Field `NEGPPM` writer - NEGative PPM Correction"]
pub type NEGPPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `CORRECTION` reader - Slow Clock Correction"]
pub type CORRECTION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORRECTION` writer - Slow Clock Correction"]
pub type CORRECTION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 7, O>;
#[doc = "Field `HIGHPPM` reader - HIGH PPM Correction"]
pub type HIGHPPM_R = crate::BitReader<bool>;
#[doc = "Field `HIGHPPM` writer - HIGH PPM Correction"]
pub type HIGHPPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 12-/24-hour Mode"]
    #[inline(always)]
    pub fn hrmod(&self) -> HRMOD_R {
        HRMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PERSIAN Calendar"]
    #[inline(always)]
    pub fn persian(&self) -> PERSIAN_R {
        PERSIAN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - NEGative PPM Correction"]
    #[inline(always)]
    pub fn negppm(&self) -> NEGPPM_R {
        NEGPPM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Slow Clock Correction"]
    #[inline(always)]
    pub fn correction(&self) -> CORRECTION_R {
        CORRECTION_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - HIGH PPM Correction"]
    #[inline(always)]
    pub fn highppm(&self) -> HIGHPPM_R {
        HIGHPPM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 12-/24-hour Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hrmod(&mut self) -> HRMOD_W<0> {
        HRMOD_W::new(self)
    }
    #[doc = "Bit 1 - PERSIAN Calendar"]
    #[inline(always)]
    #[must_use]
    pub fn persian(&mut self) -> PERSIAN_W<1> {
        PERSIAN_W::new(self)
    }
    #[doc = "Bit 4 - NEGative PPM Correction"]
    #[inline(always)]
    #[must_use]
    pub fn negppm(&mut self) -> NEGPPM_W<4> {
        NEGPPM_W::new(self)
    }
    #[doc = "Bits 8:14 - Slow Clock Correction"]
    #[inline(always)]
    #[must_use]
    pub fn correction(&mut self) -> CORRECTION_W<8> {
        CORRECTION_W::new(self)
    }
    #[doc = "Bit 15 - HIGH PPM Correction"]
    #[inline(always)]
    #[must_use]
    pub fn highppm(&mut self) -> HIGHPPM_W<15> {
        HIGHPPM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
