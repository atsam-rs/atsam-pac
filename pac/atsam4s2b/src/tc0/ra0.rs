#[doc = "Register `RA0` reader"]
pub struct R(crate::R<RA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RA0` writer"]
pub struct W(crate::W<RA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RA0_SPEC>;
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
impl From<crate::W<RA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RA` reader - Register A"]
pub type RA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RA` writer - Register A"]
pub type RA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RA0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Register A"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register A"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<0> {
        RA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register A (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ra0](index.html) module"]
pub struct RA0_SPEC;
impl crate::RegisterSpec for RA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ra0::R](R) reader structure"]
impl crate::Readable for RA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ra0::W](W) writer structure"]
impl crate::Writable for RA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RA0 to value 0"]
impl crate::Resettable for RA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
