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
pub struct FSTEP_R(crate::FieldReader<u8, u8>);
impl FSTEP_R {
    pub(crate) fn new(bits: u8) -> Self {
        FSTEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTEP` writer - Fine Maximum Step"]
pub struct FSTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CSTEP` reader - Coarse Maximum Step"]
pub struct CSTEP_R(crate::FieldReader<u8, u8>);
impl CSTEP_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSTEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSTEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSTEP` writer - Coarse Maximum Step"]
pub struct CSTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
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
    pub fn fstep(&mut self) -> FSTEP_W {
        FSTEP_W { w: self }
    }
    #[doc = "Bits 16:20 - Coarse Maximum Step"]
    #[inline(always)]
    pub fn cstep(&mut self) -> CSTEP_W {
        CSTEP_W { w: self }
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
}
#[doc = "`reset()` method sets DFLL0STEP to value 0"]
impl crate::Resettable for DFLL0STEP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
