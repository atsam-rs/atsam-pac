#[doc = "Register `DFLL0MUL` reader"]
pub struct R(crate::R<DFLL0MUL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLL0MUL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLL0MUL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLL0MUL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLL0MUL` writer"]
pub struct W(crate::W<DFLL0MUL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLL0MUL_SPEC>;
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
impl From<crate::W<DFLL0MUL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLL0MUL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUL` reader - DFLL Multiply Factor"]
pub type MUL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MUL` writer - DFLL Multiply Factor"]
pub type MUL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFLL0MUL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - DFLL Multiply Factor"]
    #[inline(always)]
    pub fn mul(&self) -> MUL_R {
        MUL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DFLL Multiply Factor"]
    #[inline(always)]
    #[must_use]
    pub fn mul(&mut self) -> MUL_W<0> {
        MUL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLL0 Multiplier Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfll0mul](index.html) module"]
pub struct DFLL0MUL_SPEC;
impl crate::RegisterSpec for DFLL0MUL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfll0mul::R](R) reader structure"]
impl crate::Readable for DFLL0MUL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfll0mul::W](W) writer structure"]
impl crate::Writable for DFLL0MUL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFLL0MUL to value 0"]
impl crate::Resettable for DFLL0MUL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
