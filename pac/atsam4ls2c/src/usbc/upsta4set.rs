#[doc = "Register `UPSTA4SET` writer"]
pub struct W(crate::W<UPSTA4SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPSTA4SET_SPEC>;
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
impl From<crate::W<UPSTA4SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPSTA4SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINIS` writer - RXINI Set"]
pub type RXINIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPSTA4SET_SPEC, bool, O>;
#[doc = "Field `TXOUTIS` writer - TXOUTI Set"]
pub type TXOUTIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPSTA4SET_SPEC, bool, O>;
#[doc = "Field `TXSTPIS` writer - TXSTPI Set"]
pub type TXSTPIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPSTA4SET_SPEC, bool, O>;
#[doc = "Field `PERRIS` writer - PERRI Set"]
pub type PERRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPSTA4SET_SPEC, bool, O>;
#[doc = "Field `NAKEDIS` writer - NAKEDI Set"]
pub type NAKEDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPSTA4SET_SPEC, bool, O>;
#[doc = "Field `ERRORFIS` writer - ERRORFI Set"]
pub type ERRORFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPSTA4SET_SPEC, bool, O>;
#[doc = "Field `RXSTALLDIS` writer - RXSTALLDI Set"]
pub type RXSTALLDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPSTA4SET_SPEC, bool, O>;
#[doc = "Field `RAMACERIS` writer - RAMACERI Set"]
pub type RAMACERIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPSTA4SET_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - RXINI Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxinis(&mut self) -> RXINIS_W<0> {
        RXINIS_W::new(self)
    }
    #[doc = "Bit 1 - TXOUTI Set"]
    #[inline(always)]
    #[must_use]
    pub fn txoutis(&mut self) -> TXOUTIS_W<1> {
        TXOUTIS_W::new(self)
    }
    #[doc = "Bit 2 - TXSTPI Set"]
    #[inline(always)]
    #[must_use]
    pub fn txstpis(&mut self) -> TXSTPIS_W<2> {
        TXSTPIS_W::new(self)
    }
    #[doc = "Bit 3 - PERRI Set"]
    #[inline(always)]
    #[must_use]
    pub fn perris(&mut self) -> PERRIS_W<3> {
        PERRIS_W::new(self)
    }
    #[doc = "Bit 4 - NAKEDI Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakedis(&mut self) -> NAKEDIS_W<4> {
        NAKEDIS_W::new(self)
    }
    #[doc = "Bit 5 - ERRORFI Set"]
    #[inline(always)]
    #[must_use]
    pub fn errorfis(&mut self) -> ERRORFIS_W<5> {
        ERRORFIS_W::new(self)
    }
    #[doc = "Bit 6 - RXSTALLDI Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldis(&mut self) -> RXSTALLDIS_W<6> {
        RXSTALLDIS_W::new(self)
    }
    #[doc = "Bit 10 - RAMACERI Set"]
    #[inline(always)]
    #[must_use]
    pub fn ramaceris(&mut self) -> RAMACERIS_W<10> {
        RAMACERIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pipe Status Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta4set](index.html) module"]
pub struct UPSTA4SET_SPEC;
impl crate::RegisterSpec for UPSTA4SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [upsta4set::W](W) writer structure"]
impl crate::Writable for UPSTA4SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UPSTA4SET to value 0"]
impl crate::Resettable for UPSTA4SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
