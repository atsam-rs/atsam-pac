#[doc = "Register `UHINTCLR` writer"]
pub struct W(crate::W<UHINTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHINTCLR_SPEC>;
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
impl From<crate::W<UHINTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHINTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCONNIC` writer - DCONNI Clear"]
pub type DCONNIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTCLR_SPEC, bool, O>;
#[doc = "Field `DDISCIC` writer - DDISCI Clear"]
pub type DDISCIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTCLR_SPEC, bool, O>;
#[doc = "Field `RSTIC` writer - RSTI Clear"]
pub type RSTIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTCLR_SPEC, bool, O>;
#[doc = "Field `RSMEDIC` writer - RSMEDI Clear"]
pub type RSMEDIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTCLR_SPEC, bool, O>;
#[doc = "Field `RXRSMIC` writer - RXRSMI Clear"]
pub type RXRSMIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTCLR_SPEC, bool, O>;
#[doc = "Field `HSOFIC` writer - HSOFI Clear"]
pub type HSOFIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTCLR_SPEC, bool, O>;
#[doc = "Field `HWUPIC` writer - HWUPI Clear"]
pub type HWUPIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTCLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - DCONNI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dconnic(&mut self) -> DCONNIC_W<0> {
        DCONNIC_W::new(self)
    }
    #[doc = "Bit 1 - DDISCI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ddiscic(&mut self) -> DDISCIC_W<1> {
        DDISCIC_W::new(self)
    }
    #[doc = "Bit 2 - RSTI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstic(&mut self) -> RSTIC_W<2> {
        RSTIC_W::new(self)
    }
    #[doc = "Bit 3 - RSMEDI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rsmedic(&mut self) -> RSMEDIC_W<3> {
        RSMEDIC_W::new(self)
    }
    #[doc = "Bit 4 - RXRSMI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsmic(&mut self) -> RXRSMIC_W<4> {
        RXRSMIC_W::new(self)
    }
    #[doc = "Bit 5 - HSOFI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsofic(&mut self) -> HSOFIC_W<5> {
        HSOFIC_W::new(self)
    }
    #[doc = "Bit 6 - HWUPI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hwupic(&mut self) -> HWUPIC_W<6> {
        HWUPIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Global Interrrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhintclr](index.html) module"]
pub struct UHINTCLR_SPEC;
impl crate::RegisterSpec for UHINTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uhintclr::W](W) writer structure"]
impl crate::Writable for UHINTCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UHINTCLR to value 0"]
impl crate::Resettable for UHINTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
