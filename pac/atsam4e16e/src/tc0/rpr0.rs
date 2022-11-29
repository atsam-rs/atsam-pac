#[doc = "Register `RPR0` reader"]
pub struct R(crate::R<RPR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RPR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RPR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPR0` writer"]
pub struct W(crate::W<RPR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPR0_SPEC>;
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
impl From<crate::W<RPR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RPR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXPTR` reader - Receive Pointer Register"]
pub type RXPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RXPTR` writer - Receive Pointer Register"]
pub type RXPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RPR0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Receive Pointer Register"]
    #[inline(always)]
    pub fn rxptr(&self) -> RXPTR_R {
        RXPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Pointer Register"]
    #[inline(always)]
    #[must_use]
    pub fn rxptr(&mut self) -> RXPTR_W<0> {
        RXPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Pointer Register (pdc = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpr0](index.html) module"]
pub struct RPR0_SPEC;
impl crate::RegisterSpec for RPR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpr0::R](R) reader structure"]
impl crate::Readable for RPR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpr0::W](W) writer structure"]
impl crate::Writable for RPR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RPR0 to value 0"]
impl crate::Resettable for RPR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
