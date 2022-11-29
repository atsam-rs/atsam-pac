#[doc = "Register `TRIER` writer"]
pub struct W(crate::W<TRIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIER_SPEC>;
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
impl From<crate::W<TRIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIE` writer - Trigger Interrupt Enable"]
pub type TRIE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIER_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Trigger Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trie(&mut self) -> TRIE_W<0> {
        TRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger Interrupt Mask Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trier](index.html) module"]
pub struct TRIER_SPEC;
impl crate::RegisterSpec for TRIER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [trier::W](W) writer structure"]
impl crate::Writable for TRIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIER to value 0"]
impl crate::Resettable for TRIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
