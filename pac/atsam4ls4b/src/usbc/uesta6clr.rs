#[doc = "Register `UESTA6CLR` writer"]
pub struct W(crate::W<UESTA6CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UESTA6CLR_SPEC>;
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
impl From<crate::W<UESTA6CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UESTA6CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINIC` writer - TXINI Clear"]
pub type TXINIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UESTA6CLR_SPEC, bool, O>;
#[doc = "Field `RXOUTIC` writer - RXOUTI Clear"]
pub type RXOUTIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UESTA6CLR_SPEC, bool, O>;
#[doc = "Field `RXSTPIC` writer - RXSTPI Clear"]
pub type RXSTPIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UESTA6CLR_SPEC, bool, O>;
#[doc = "Field `NAKOUTIC` writer - NAKOUTI Clear"]
pub type NAKOUTIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UESTA6CLR_SPEC, bool, O>;
#[doc = "Field `NAKINIC` writer - NAKINI Clear"]
pub type NAKINIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UESTA6CLR_SPEC, bool, O>;
#[doc = "Field `STALLEDIC` writer - STALLEDI Clear"]
pub type STALLEDIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UESTA6CLR_SPEC, bool, O>;
#[doc = "Field `RAMACERIC` writer - RAMACERI Clear"]
pub type RAMACERIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UESTA6CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - TXINI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txinic(&mut self) -> TXINIC_W<0> {
        TXINIC_W::new(self)
    }
    #[doc = "Bit 1 - RXOUTI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutic(&mut self) -> RXOUTIC_W<1> {
        RXOUTIC_W::new(self)
    }
    #[doc = "Bit 2 - RXSTPI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpic(&mut self) -> RXSTPIC_W<2> {
        RXSTPIC_W::new(self)
    }
    #[doc = "Bit 3 - NAKOUTI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutic(&mut self) -> NAKOUTIC_W<3> {
        NAKOUTIC_W::new(self)
    }
    #[doc = "Bit 4 - NAKINI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakinic(&mut self) -> NAKINIC_W<4> {
        NAKINIC_W::new(self)
    }
    #[doc = "Bit 6 - STALLEDI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stalledic(&mut self) -> STALLEDIC_W<6> {
        STALLEDIC_W::new(self)
    }
    #[doc = "Bit 11 - RAMACERI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ramaceric(&mut self) -> RAMACERIC_W<11> {
        RAMACERIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta6clr](index.html) module"]
pub struct UESTA6CLR_SPEC;
impl crate::RegisterSpec for UESTA6CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uesta6clr::W](W) writer structure"]
impl crate::Writable for UESTA6CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UESTA6CLR to value 0"]
impl crate::Resettable for UESTA6CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
