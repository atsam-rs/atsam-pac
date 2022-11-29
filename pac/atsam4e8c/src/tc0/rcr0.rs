#[doc = "Register `RCR0` reader"]
pub struct R(crate::R<RCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCR0` writer"]
pub struct W(crate::W<RCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR0_SPEC>;
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
impl From<crate::W<RCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXCTR` reader - Receive Counter Register"]
pub type RXCTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXCTR` writer - Receive Counter Register"]
pub type RXCTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCR0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Receive Counter Register"]
    #[inline(always)]
    pub fn rxctr(&self) -> RXCTR_R {
        RXCTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Counter Register"]
    #[inline(always)]
    #[must_use]
    pub fn rxctr(&mut self) -> RXCTR_W<0> {
        RXCTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Counter Register (pdc = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr0](index.html) module"]
pub struct RCR0_SPEC;
impl crate::RegisterSpec for RCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcr0::R](R) reader structure"]
impl crate::Readable for RCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcr0::W](W) writer structure"]
impl crate::Writable for RCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCR0 to value 0"]
impl crate::Resettable for RCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
