#[doc = "Register `ACMDR` writer"]
pub struct W(crate::W<ACMDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACMDR_SPEC>;
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
impl From<crate::W<ACMDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACMDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASCII` writer - ASCII Code"]
pub type ASCII_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACMDR_SPEC, u8, u8, 7, O>;
impl W {
    #[doc = "Bits 0:6 - ASCII Code"]
    #[inline(always)]
    #[must_use]
    pub fn ascii(&mut self) -> ASCII_W<0> {
        ASCII_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Automated Character Mapping Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmdr](index.html) module"]
pub struct ACMDR_SPEC;
impl crate::RegisterSpec for ACMDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [acmdr::W](W) writer structure"]
impl crate::Writable for ACMDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACMDR to value 0"]
impl crate::Resettable for ACMDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
