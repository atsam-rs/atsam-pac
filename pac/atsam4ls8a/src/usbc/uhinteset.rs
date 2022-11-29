#[doc = "Register `UHINTESET` writer"]
pub struct W(crate::W<UHINTESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHINTESET_SPEC>;
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
impl From<crate::W<UHINTESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHINTESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCONNIES` writer - DCONNIE Set"]
pub type DCONNIES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTESET_SPEC, bool, O>;
#[doc = "Field `DDISCIES` writer - DDISCIE Set"]
pub type DDISCIES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTESET_SPEC, bool, O>;
#[doc = "Field `RSTIES` writer - RSTIE Set"]
pub type RSTIES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTESET_SPEC, bool, O>;
#[doc = "Field `RSMEDIES` writer - RSMEDIE Set"]
pub type RSMEDIES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTESET_SPEC, bool, O>;
#[doc = "Field `RXRSMIES` writer - RXRSMIE Set"]
pub type RXRSMIES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTESET_SPEC, bool, O>;
#[doc = "Field `HSOFIES` writer - HSOFIE Set"]
pub type HSOFIES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTESET_SPEC, bool, O>;
#[doc = "Field `HWUPIES` writer - HWUPIE Set"]
pub type HWUPIES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTESET_SPEC, bool, O>;
#[doc = "Field `P0INTES` writer - P0INTE Set"]
pub type P0INTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTESET_SPEC, bool, O>;
#[doc = "Field `P1INTES` writer - P1INTE Set"]
pub type P1INTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTESET_SPEC, bool, O>;
#[doc = "Field `P2INTES` writer - P2INTE Set"]
pub type P2INTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTESET_SPEC, bool, O>;
#[doc = "Field `P3INTES` writer - P3INTE Set"]
pub type P3INTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTESET_SPEC, bool, O>;
#[doc = "Field `P4INTES` writer - P4INTE Set"]
pub type P4INTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTESET_SPEC, bool, O>;
#[doc = "Field `P5INTES` writer - P5INTE Set"]
pub type P5INTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTESET_SPEC, bool, O>;
#[doc = "Field `P6INTES` writer - P6INTE Set"]
pub type P6INTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTESET_SPEC, bool, O>;
#[doc = "Field `P7INTES` writer - P7INTE Set"]
pub type P7INTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTESET_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - DCONNIE Set"]
    #[inline(always)]
    #[must_use]
    pub fn dconnies(&mut self) -> DCONNIES_W<0> {
        DCONNIES_W::new(self)
    }
    #[doc = "Bit 1 - DDISCIE Set"]
    #[inline(always)]
    #[must_use]
    pub fn ddiscies(&mut self) -> DDISCIES_W<1> {
        DDISCIES_W::new(self)
    }
    #[doc = "Bit 2 - RSTIE Set"]
    #[inline(always)]
    #[must_use]
    pub fn rsties(&mut self) -> RSTIES_W<2> {
        RSTIES_W::new(self)
    }
    #[doc = "Bit 3 - RSMEDIE Set"]
    #[inline(always)]
    #[must_use]
    pub fn rsmedies(&mut self) -> RSMEDIES_W<3> {
        RSMEDIES_W::new(self)
    }
    #[doc = "Bit 4 - RXRSMIE Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsmies(&mut self) -> RXRSMIES_W<4> {
        RXRSMIES_W::new(self)
    }
    #[doc = "Bit 5 - HSOFIE Set"]
    #[inline(always)]
    #[must_use]
    pub fn hsofies(&mut self) -> HSOFIES_W<5> {
        HSOFIES_W::new(self)
    }
    #[doc = "Bit 6 - HWUPIE Set"]
    #[inline(always)]
    #[must_use]
    pub fn hwupies(&mut self) -> HWUPIES_W<6> {
        HWUPIES_W::new(self)
    }
    #[doc = "Bit 8 - P0INTE Set"]
    #[inline(always)]
    #[must_use]
    pub fn p0intes(&mut self) -> P0INTES_W<8> {
        P0INTES_W::new(self)
    }
    #[doc = "Bit 9 - P1INTE Set"]
    #[inline(always)]
    #[must_use]
    pub fn p1intes(&mut self) -> P1INTES_W<9> {
        P1INTES_W::new(self)
    }
    #[doc = "Bit 10 - P2INTE Set"]
    #[inline(always)]
    #[must_use]
    pub fn p2intes(&mut self) -> P2INTES_W<10> {
        P2INTES_W::new(self)
    }
    #[doc = "Bit 11 - P3INTE Set"]
    #[inline(always)]
    #[must_use]
    pub fn p3intes(&mut self) -> P3INTES_W<11> {
        P3INTES_W::new(self)
    }
    #[doc = "Bit 12 - P4INTE Set"]
    #[inline(always)]
    #[must_use]
    pub fn p4intes(&mut self) -> P4INTES_W<12> {
        P4INTES_W::new(self)
    }
    #[doc = "Bit 13 - P5INTE Set"]
    #[inline(always)]
    #[must_use]
    pub fn p5intes(&mut self) -> P5INTES_W<13> {
        P5INTES_W::new(self)
    }
    #[doc = "Bit 14 - P6INTE Set"]
    #[inline(always)]
    #[must_use]
    pub fn p6intes(&mut self) -> P6INTES_W<14> {
        P6INTES_W::new(self)
    }
    #[doc = "Bit 15 - P7INTE Set"]
    #[inline(always)]
    #[must_use]
    pub fn p7intes(&mut self) -> P7INTES_W<15> {
        P7INTES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Global Interrupt Enable Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhinteset](index.html) module"]
pub struct UHINTESET_SPEC;
impl crate::RegisterSpec for UHINTESET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uhinteset::W](W) writer structure"]
impl crate::Writable for UHINTESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UHINTESET to value 0"]
impl crate::Resettable for UHINTESET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
