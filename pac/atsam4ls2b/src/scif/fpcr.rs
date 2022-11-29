#[doc = "Register `FPCR` reader"]
pub struct R(crate::R<FPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPCR` writer"]
pub struct W(crate::W<FPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPCR_SPEC>;
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
impl From<crate::W<FPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPEN` reader - High Resolution Prescaler Enable"]
pub type FPEN_R = crate::BitReader<bool>;
#[doc = "Field `FPEN` writer - High Resolution Prescaler Enable"]
pub type FPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCR_SPEC, bool, O>;
#[doc = "Field `CKSEL` reader - Clock Input Selection"]
pub type CKSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKSEL` writer - Clock Input Selection"]
pub type CKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPCR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - High Resolution Prescaler Enable"]
    #[inline(always)]
    pub fn fpen(&self) -> FPEN_R {
        FPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Clock Input Selection"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - High Resolution Prescaler Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpen(&mut self) -> FPEN_W<0> {
        FPEN_W::new(self)
    }
    #[doc = "Bits 1:3 - Clock Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CKSEL_W<1> {
        CKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fractional Prescaler Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpcr](index.html) module"]
pub struct FPCR_SPEC;
impl crate::RegisterSpec for FPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpcr::R](R) reader structure"]
impl crate::Readable for FPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpcr::W](W) writer structure"]
impl crate::Writable for FPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPCR to value 0"]
impl crate::Resettable for FPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
