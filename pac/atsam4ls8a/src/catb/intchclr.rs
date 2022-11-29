#[doc = "Register `INTCHCLR%s` writer"]
pub struct W(crate::W<INTCHCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCHCLR_SPEC>;
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
impl From<crate::W<INTCHCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTCHCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTCHCLR` writer - In-Touch Clear"]
pub type INTCHCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTCHCLR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - In-Touch Clear"]
    #[inline(always)]
    #[must_use]
    pub fn intchclr(&mut self) -> INTCHCLR_W<0> {
        INTCHCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "In-Touch Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intchclr](index.html) module"]
pub struct INTCHCLR_SPEC;
impl crate::RegisterSpec for INTCHCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intchclr::W](W) writer structure"]
impl crate::Writable for INTCHCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCHCLR%s to value 0"]
impl crate::Resettable for INTCHCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
