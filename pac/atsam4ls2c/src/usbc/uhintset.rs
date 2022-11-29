#[doc = "Register `UHINTSET` writer"]
pub struct W(crate::W<UHINTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHINTSET_SPEC>;
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
impl From<crate::W<UHINTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHINTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCONNIS` writer - DCONNI Set"]
pub type DCONNIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTSET_SPEC, bool, O>;
#[doc = "Field `DDISCIS` writer - DDISCI Set"]
pub type DDISCIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTSET_SPEC, bool, O>;
#[doc = "Field `RSTIS` writer - RSTI Set"]
pub type RSTIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTSET_SPEC, bool, O>;
#[doc = "Field `RSMEDIS` writer - RSMEDI Set"]
pub type RSMEDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTSET_SPEC, bool, O>;
#[doc = "Field `RXRSMIS` writer - RXRSMI Set"]
pub type RXRSMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTSET_SPEC, bool, O>;
#[doc = "Field `HSOFIS` writer - HSOFI Set"]
pub type HSOFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTSET_SPEC, bool, O>;
#[doc = "Field `HWUPIS` writer - HWUPI Set"]
pub type HWUPIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTSET_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - DCONNI Set"]
    #[inline(always)]
    #[must_use]
    pub fn dconnis(&mut self) -> DCONNIS_W<0> {
        DCONNIS_W::new(self)
    }
    #[doc = "Bit 1 - DDISCI Set"]
    #[inline(always)]
    #[must_use]
    pub fn ddiscis(&mut self) -> DDISCIS_W<1> {
        DDISCIS_W::new(self)
    }
    #[doc = "Bit 2 - RSTI Set"]
    #[inline(always)]
    #[must_use]
    pub fn rstis(&mut self) -> RSTIS_W<2> {
        RSTIS_W::new(self)
    }
    #[doc = "Bit 3 - RSMEDI Set"]
    #[inline(always)]
    #[must_use]
    pub fn rsmedis(&mut self) -> RSMEDIS_W<3> {
        RSMEDIS_W::new(self)
    }
    #[doc = "Bit 4 - RXRSMI Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsmis(&mut self) -> RXRSMIS_W<4> {
        RXRSMIS_W::new(self)
    }
    #[doc = "Bit 5 - HSOFI Set"]
    #[inline(always)]
    #[must_use]
    pub fn hsofis(&mut self) -> HSOFIS_W<5> {
        HSOFIS_W::new(self)
    }
    #[doc = "Bit 6 - HWUPI Set"]
    #[inline(always)]
    #[must_use]
    pub fn hwupis(&mut self) -> HWUPIS_W<6> {
        HWUPIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Global Interrupt Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhintset](index.html) module"]
pub struct UHINTSET_SPEC;
impl crate::RegisterSpec for UHINTSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uhintset::W](W) writer structure"]
impl crate::Writable for UHINTSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UHINTSET to value 0"]
impl crate::Resettable for UHINTSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
