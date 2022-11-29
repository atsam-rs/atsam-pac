#[doc = "Register `UESTA6SET` writer"]
pub struct W(crate::W<UESTA6SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UESTA6SET_SPEC>;
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
impl From<crate::W<UESTA6SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UESTA6SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINIS` writer - TXINI Set"]
pub type TXINIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UESTA6SET_SPEC, bool, O>;
#[doc = "Field `RXOUTIS` writer - RXOUTI Set"]
pub type RXOUTIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UESTA6SET_SPEC, bool, O>;
#[doc = "Field `RXSTPIS` writer - RXSTPI Set"]
pub type RXSTPIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UESTA6SET_SPEC, bool, O>;
#[doc = "Field `NAKOUTIS` writer - NAKOUTI Set"]
pub type NAKOUTIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UESTA6SET_SPEC, bool, O>;
#[doc = "Field `NAKINIS` writer - NAKINI Set"]
pub type NAKINIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UESTA6SET_SPEC, bool, O>;
#[doc = "Field `STALLEDIS` writer - STALLEDI Set"]
pub type STALLEDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UESTA6SET_SPEC, bool, O>;
#[doc = "Field `RAMACERIS` writer - RAMACERI Set"]
pub type RAMACERIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UESTA6SET_SPEC, bool, O>;
#[doc = "Field `NBUSYBKS` writer - NBUSYBK Set"]
pub type NBUSYBKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UESTA6SET_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - TXINI Set"]
    #[inline(always)]
    #[must_use]
    pub fn txinis(&mut self) -> TXINIS_W<0> {
        TXINIS_W::new(self)
    }
    #[doc = "Bit 1 - RXOUTI Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutis(&mut self) -> RXOUTIS_W<1> {
        RXOUTIS_W::new(self)
    }
    #[doc = "Bit 2 - RXSTPI Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpis(&mut self) -> RXSTPIS_W<2> {
        RXSTPIS_W::new(self)
    }
    #[doc = "Bit 3 - NAKOUTI Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutis(&mut self) -> NAKOUTIS_W<3> {
        NAKOUTIS_W::new(self)
    }
    #[doc = "Bit 4 - NAKINI Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakinis(&mut self) -> NAKINIS_W<4> {
        NAKINIS_W::new(self)
    }
    #[doc = "Bit 6 - STALLEDI Set"]
    #[inline(always)]
    #[must_use]
    pub fn stalledis(&mut self) -> STALLEDIS_W<6> {
        STALLEDIS_W::new(self)
    }
    #[doc = "Bit 11 - RAMACERI Set"]
    #[inline(always)]
    #[must_use]
    pub fn ramaceris(&mut self) -> RAMACERIS_W<11> {
        RAMACERIS_W::new(self)
    }
    #[doc = "Bit 12 - NBUSYBK Set"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybks(&mut self) -> NBUSYBKS_W<12> {
        NBUSYBKS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Status Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta6set](index.html) module"]
pub struct UESTA6SET_SPEC;
impl crate::RegisterSpec for UESTA6SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uesta6set::W](W) writer structure"]
impl crate::Writable for UESTA6SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UESTA6SET to value 0"]
impl crate::Resettable for UESTA6SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
