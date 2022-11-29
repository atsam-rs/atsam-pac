#[doc = "Register `INITVECT%s` writer"]
pub struct W(crate::W<INITVECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INITVECT_SPEC>;
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
impl From<crate::W<INITVECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INITVECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INITVECT0` writer - Initialization Vector Word 0"]
pub type INITVECT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INITVECT_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Initialization Vector Word 0"]
    #[inline(always)]
    #[must_use]
    pub fn initvect0(&mut self) -> INITVECT0_W<0> {
        INITVECT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Initialization Vector Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [initvect](index.html) module"]
pub struct INITVECT_SPEC;
impl crate::RegisterSpec for INITVECT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [initvect::W](W) writer structure"]
impl crate::Writable for INITVECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INITVECT%s to value 0"]
impl crate::Resettable for INITVECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
