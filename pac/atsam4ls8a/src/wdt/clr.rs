#[doc = "Register `CLR` writer"]
pub struct W(crate::W<CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLR_SPEC>;
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
impl From<crate::W<CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTCLR` writer - Clear WDT counter"]
pub type WDTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, bool, O>;
#[doc = "Field `KEY` writer - Key"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLR_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bit 0 - Clear WDT counter"]
    #[inline(always)]
    #[must_use]
    pub fn wdtclr(&mut self) -> WDTCLR_W<0> {
        WDTCLR_W::new(self)
    }
    #[doc = "Bits 24:31 - Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<24> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr](index.html) module"]
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clr::W](W) writer structure"]
impl crate::Writable for CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
