#[doc = "Register `DFLL0STEP` reader"]
pub struct R(crate::R<DFLL0STEP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLL0STEP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLL0STEP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLL0STEP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLL0STEP` writer"]
pub struct W(crate::W<DFLL0STEP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLL0STEP_SPEC>;
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
impl From<crate::W<DFLL0STEP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLL0STEP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSTEP` reader - Fine Maximum Step"]
pub type FSTEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSTEP` writer - Fine Maximum Step"]
pub type FSTEP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFLL0STEP_SPEC, u8, u8, 8, O>;
#[doc = "Field `CSTEP` reader - Coarse Maximum Step"]
pub type CSTEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSTEP` writer - Coarse Maximum Step"]
pub type CSTEP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFLL0STEP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:7 - Fine Maximum Step"]
    #[inline(always)]
    pub fn fstep(&self) -> FSTEP_R {
        FSTEP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - Coarse Maximum Step"]
    #[inline(always)]
    pub fn cstep(&self) -> CSTEP_R {
        CSTEP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fine Maximum Step"]
    #[inline(always)]
    #[must_use]
    pub fn fstep(&mut self) -> FSTEP_W<0> {
        FSTEP_W::new(self)
    }
    #[doc = "Bits 16:20 - Coarse Maximum Step"]
    #[inline(always)]
    #[must_use]
    pub fn cstep(&mut self) -> CSTEP_W<16> {
        CSTEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLL0 Step Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfll0step](index.html) module"]
pub struct DFLL0STEP_SPEC;
impl crate::RegisterSpec for DFLL0STEP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfll0step::R](R) reader structure"]
impl crate::Readable for DFLL0STEP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfll0step::W](W) writer structure"]
impl crate::Writable for DFLL0STEP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFLL0STEP to value 0"]
impl crate::Resettable for DFLL0STEP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
