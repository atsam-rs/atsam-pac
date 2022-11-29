#[doc = "Register `UHINTECLR` writer"]
pub struct W(crate::W<UHINTECLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHINTECLR_SPEC>;
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
impl From<crate::W<UHINTECLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHINTECLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCONNIEC` writer - DCONNIE Clear"]
pub type DCONNIEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTECLR_SPEC, bool, O>;
#[doc = "Field `DDISCIEC` writer - DDISCIE Clear"]
pub type DDISCIEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTECLR_SPEC, bool, O>;
#[doc = "Field `RSTIEC` writer - RSTIE Clear"]
pub type RSTIEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTECLR_SPEC, bool, O>;
#[doc = "Field `RSMEDIEC` writer - RSMEDIE Clear"]
pub type RSMEDIEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTECLR_SPEC, bool, O>;
#[doc = "Field `RXRSMIEC` writer - RXRSMIE Clear"]
pub type RXRSMIEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTECLR_SPEC, bool, O>;
#[doc = "Field `HSOFIEC` writer - HSOFIE Clear"]
pub type HSOFIEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTECLR_SPEC, bool, O>;
#[doc = "Field `HWUPIEC` writer - HWUPIE Clear"]
pub type HWUPIEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTECLR_SPEC, bool, O>;
#[doc = "Field `P0INTEC` writer - P0INTE Clear"]
pub type P0INTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTECLR_SPEC, bool, O>;
#[doc = "Field `P1INTEC` writer - P1INTE Clear"]
pub type P1INTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTECLR_SPEC, bool, O>;
#[doc = "Field `P2INTEC` writer - P2INTE Clear"]
pub type P2INTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTECLR_SPEC, bool, O>;
#[doc = "Field `P3INTEC` writer - P3INTE Clear"]
pub type P3INTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTECLR_SPEC, bool, O>;
#[doc = "Field `P4INTEC` writer - P4INTE Clear"]
pub type P4INTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTECLR_SPEC, bool, O>;
#[doc = "Field `P5INTEC` writer - P5INTE Clear"]
pub type P5INTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTECLR_SPEC, bool, O>;
#[doc = "Field `P6INTEC` writer - P6INTE Clear"]
pub type P6INTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTECLR_SPEC, bool, O>;
#[doc = "Field `P7INTEC` writer - P7INTE Clear"]
pub type P7INTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHINTECLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - DCONNIE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dconniec(&mut self) -> DCONNIEC_W<0> {
        DCONNIEC_W::new(self)
    }
    #[doc = "Bit 1 - DDISCIE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ddisciec(&mut self) -> DDISCIEC_W<1> {
        DDISCIEC_W::new(self)
    }
    #[doc = "Bit 2 - RSTIE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstiec(&mut self) -> RSTIEC_W<2> {
        RSTIEC_W::new(self)
    }
    #[doc = "Bit 3 - RSMEDIE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rsmediec(&mut self) -> RSMEDIEC_W<3> {
        RSMEDIEC_W::new(self)
    }
    #[doc = "Bit 4 - RXRSMIE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsmiec(&mut self) -> RXRSMIEC_W<4> {
        RXRSMIEC_W::new(self)
    }
    #[doc = "Bit 5 - HSOFIE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsofiec(&mut self) -> HSOFIEC_W<5> {
        HSOFIEC_W::new(self)
    }
    #[doc = "Bit 6 - HWUPIE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hwupiec(&mut self) -> HWUPIEC_W<6> {
        HWUPIEC_W::new(self)
    }
    #[doc = "Bit 8 - P0INTE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn p0intec(&mut self) -> P0INTEC_W<8> {
        P0INTEC_W::new(self)
    }
    #[doc = "Bit 9 - P1INTE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn p1intec(&mut self) -> P1INTEC_W<9> {
        P1INTEC_W::new(self)
    }
    #[doc = "Bit 10 - P2INTE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn p2intec(&mut self) -> P2INTEC_W<10> {
        P2INTEC_W::new(self)
    }
    #[doc = "Bit 11 - P3INTE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn p3intec(&mut self) -> P3INTEC_W<11> {
        P3INTEC_W::new(self)
    }
    #[doc = "Bit 12 - P4INTE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn p4intec(&mut self) -> P4INTEC_W<12> {
        P4INTEC_W::new(self)
    }
    #[doc = "Bit 13 - P5INTE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn p5intec(&mut self) -> P5INTEC_W<13> {
        P5INTEC_W::new(self)
    }
    #[doc = "Bit 14 - P6INTE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn p6intec(&mut self) -> P6INTEC_W<14> {
        P6INTEC_W::new(self)
    }
    #[doc = "Bit 15 - P7INTE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn p7intec(&mut self) -> P7INTEC_W<15> {
        P7INTEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Global Interrupt Enable Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhinteclr](index.html) module"]
pub struct UHINTECLR_SPEC;
impl crate::RegisterSpec for UHINTECLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uhinteclr::W](W) writer structure"]
impl crate::Writable for UHINTECLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UHINTECLR to value 0"]
impl crate::Resettable for UHINTECLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
