#[doc = "Register `HRPCR` reader"]
pub struct R(crate::R<HRPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HRPCR` writer"]
pub struct W(crate::W<HRPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRPCR_SPEC>;
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
impl From<crate::W<HRPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HRPEN` reader - High Resolution Prescaler Enable"]
pub type HRPEN_R = crate::BitReader<bool>;
#[doc = "Field `HRPEN` writer - High Resolution Prescaler Enable"]
pub type HRPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRPCR_SPEC, bool, O>;
#[doc = "Field `CKSEL` reader - Clock Input Selection"]
pub type CKSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKSEL` writer - Clock Input Selection"]
pub type CKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HRPCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `HRCOUNT` reader - High Resolution Counter"]
pub type HRCOUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HRCOUNT` writer - High Resolution Counter"]
pub type HRCOUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HRPCR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 0 - High Resolution Prescaler Enable"]
    #[inline(always)]
    pub fn hrpen(&self) -> HRPEN_R {
        HRPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Clock Input Selection"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 8:31 - High Resolution Counter"]
    #[inline(always)]
    pub fn hrcount(&self) -> HRCOUNT_R {
        HRCOUNT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - High Resolution Prescaler Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hrpen(&mut self) -> HRPEN_W<0> {
        HRPEN_W::new(self)
    }
    #[doc = "Bits 1:3 - Clock Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CKSEL_W<1> {
        CKSEL_W::new(self)
    }
    #[doc = "Bits 8:31 - High Resolution Counter"]
    #[inline(always)]
    #[must_use]
    pub fn hrcount(&mut self) -> HRCOUNT_W<8> {
        HRCOUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Resolution Prescaler Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrpcr](index.html) module"]
pub struct HRPCR_SPEC;
impl crate::RegisterSpec for HRPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrpcr::R](R) reader structure"]
impl crate::Readable for HRPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hrpcr::W](W) writer structure"]
impl crate::Writable for HRPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HRPCR to value 0"]
impl crate::Resettable for HRPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
