#[doc = "Register `MAINT1` writer"]
pub struct W(crate::W<MAINT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAINT1_SPEC>;
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
impl From<crate::W<MAINT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAINT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INDEX` writer - Invalidate Index"]
pub type INDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAINT1_SPEC, u8, u8, 4, O>;
impl W {
    #[doc = "Bits 4:7 - Invalidate Index"]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> INDEX_W<4> {
        INDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Maintenance Register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maint1](index.html) module"]
pub struct MAINT1_SPEC;
impl crate::RegisterSpec for MAINT1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [maint1::W](W) writer structure"]
impl crate::Writable for MAINT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAINT1 to value 0"]
impl crate::Resettable for MAINT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
