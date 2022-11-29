#[doc = "Register `KEY%s` writer"]
pub struct W(crate::W<KEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY_SPEC>;
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
impl From<crate::W<KEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY0` writer - Key Word 0"]
pub type KEY0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEY_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Key Word 0"]
    #[inline(always)]
    #[must_use]
    pub fn key0(&mut self) -> KEY0_W<0> {
        KEY0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key](index.html) module"]
pub struct KEY_SPEC;
impl crate::RegisterSpec for KEY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [key::W](W) writer structure"]
impl crate::Writable for KEY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY%s to value 0"]
impl crate::Resettable for KEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
