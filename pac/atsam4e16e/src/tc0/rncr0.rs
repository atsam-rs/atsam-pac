#[doc = "Register `RNCR0` reader"]
pub struct R(crate::R<RNCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNCR0` writer"]
pub struct W(crate::W<RNCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNCR0_SPEC>;
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
impl From<crate::W<RNCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXNCTR` reader - Receive Next Counter"]
pub type RXNCTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXNCTR` writer - Receive Next Counter"]
pub type RXNCTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RNCR0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Receive Next Counter"]
    #[inline(always)]
    pub fn rxnctr(&self) -> RXNCTR_R {
        RXNCTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Next Counter"]
    #[inline(always)]
    #[must_use]
    pub fn rxnctr(&mut self) -> RXNCTR_W<0> {
        RXNCTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Next Counter Register (pdc = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rncr0](index.html) module"]
pub struct RNCR0_SPEC;
impl crate::RegisterSpec for RNCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rncr0::R](R) reader structure"]
impl crate::Readable for RNCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rncr0::W](W) writer structure"]
impl crate::Writable for RNCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RNCR0 to value 0"]
impl crate::Resettable for RNCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
