#[doc = "Register `ITIMER` reader"]
pub struct R(crate::R<ITIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ITIMER` writer"]
pub struct W(crate::W<ITIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ITIMER_SPEC>;
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
impl From<crate::W<ITIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ITIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITMC` reader - Internal timer max counter"]
pub type ITMC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ITMC` writer - Internal timer max counter"]
pub type ITMC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ITIMER_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Internal timer max counter"]
    #[inline(always)]
    pub fn itmc(&self) -> ITMC_R {
        ITMC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Internal timer max counter"]
    #[inline(always)]
    #[must_use]
    pub fn itmc(&mut self) -> ITMC_W<0> {
        ITMC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itimer](index.html) module"]
pub struct ITIMER_SPEC;
impl crate::RegisterSpec for ITIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itimer::R](R) reader structure"]
impl crate::Readable for ITIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [itimer::W](W) writer structure"]
impl crate::Writable for ITIMER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ITIMER to value 0"]
impl crate::Resettable for ITIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
