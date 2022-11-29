#[doc = "Register `WCFG` reader"]
pub struct R(crate::R<WCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WCFG` writer"]
pub struct W(crate::W<WCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WCFG_SPEC>;
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
impl From<crate::W<WCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WM` reader - Window Monitor Mode"]
pub type WM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WM` writer - Window Monitor Mode"]
pub type WM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WCFG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 12:14 - Window Monitor Mode"]
    #[inline(always)]
    pub fn wm(&self) -> WM_R {
        WM_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 12:14 - Window Monitor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wm(&mut self) -> WM_W<12> {
        WM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window Monitor Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcfg](index.html) module"]
pub struct WCFG_SPEC;
impl crate::RegisterSpec for WCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wcfg::R](R) reader structure"]
impl crate::Readable for WCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wcfg::W](W) writer structure"]
impl crate::Writable for WCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WCFG to value 0"]
impl crate::Resettable for WCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
