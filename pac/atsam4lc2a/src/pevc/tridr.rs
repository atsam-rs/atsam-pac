#[doc = "Register `TRIDR` writer"]
pub struct W(crate::W<TRIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIDR_SPEC>;
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
impl From<crate::W<TRIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRID` writer - Trigger Interrupt Disable"]
pub type TRID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIDR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Trigger Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trid(&mut self) -> TRID_W<0> {
        TRID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger Interrupt Mask Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tridr](index.html) module"]
pub struct TRIDR_SPEC;
impl crate::RegisterSpec for TRIDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tridr::W](W) writer structure"]
impl crate::Writable for TRIDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIDR to value 0"]
impl crate::Resettable for TRIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
