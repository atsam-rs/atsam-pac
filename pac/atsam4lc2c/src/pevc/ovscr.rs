#[doc = "Register `OVSCR` writer"]
pub struct W(crate::W<OVSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OVSCR_SPEC>;
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
impl From<crate::W<OVSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OVSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVSC` writer - Overrun Interrupt Status Clear"]
pub type OVSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OVSCR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Overrun Interrupt Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovsc(&mut self) -> OVSC_W<0> {
        OVSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Overrun Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovscr](index.html) module"]
pub struct OVSCR_SPEC;
impl crate::RegisterSpec for OVSCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ovscr::W](W) writer structure"]
impl crate::Writable for OVSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OVSCR to value 0"]
impl crate::Resettable for OVSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
