#[doc = "Register `PIR0` reader"]
pub struct R(crate::R<PIR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIR0` writer"]
pub struct W(crate::W<PIR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIR0_SPEC>;
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
impl From<crate::W<PIR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSEL` reader - Interval Select"]
pub type INSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSEL` writer - Interval Select"]
pub type INSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIR0_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Interval Select"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Interval Select"]
    #[inline(always)]
    #[must_use]
    pub fn insel(&mut self) -> INSEL_W<0> {
        INSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Periodic Interval Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pir0](index.html) module"]
pub struct PIR0_SPEC;
impl crate::RegisterSpec for PIR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pir0::R](R) reader structure"]
impl crate::Readable for PIR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pir0::W](W) writer structure"]
impl crate::Writable for PIR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIR0 to value 0"]
impl crate::Resettable for PIR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
