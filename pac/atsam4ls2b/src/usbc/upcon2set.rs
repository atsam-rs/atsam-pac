#[doc = "Register `UPCON2SET` writer"]
pub struct W(crate::W<UPCON2SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPCON2SET_SPEC>;
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
impl From<crate::W<UPCON2SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPCON2SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINES` writer - RXINE Set"]
pub type RXINES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON2SET_SPEC, bool, O>;
#[doc = "Field `TXOUTES` writer - TXOUTE Set"]
pub type TXOUTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON2SET_SPEC, bool, O>;
#[doc = "Field `TXSTPES` writer - TXSTPE Set"]
pub type TXSTPES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON2SET_SPEC, bool, O>;
#[doc = "Field `PERRES` writer - PERRE Set"]
pub type PERRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON2SET_SPEC, bool, O>;
#[doc = "Field `NAKEDES` writer - NAKEDE Set"]
pub type NAKEDES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON2SET_SPEC, bool, O>;
#[doc = "Field `ERRORFIES` writer - ERRORFIE Set"]
pub type ERRORFIES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON2SET_SPEC, bool, O>;
#[doc = "Field `RXSTALLDES` writer - RXSTALLDE Set"]
pub type RXSTALLDES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON2SET_SPEC, bool, O>;
#[doc = "Field `RAMACERES` writer - RAMACERE Set"]
pub type RAMACERES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON2SET_SPEC, bool, O>;
#[doc = "Field `NBUSYBKES` writer - NBUSYBKE Set"]
pub type NBUSYBKES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON2SET_SPEC, bool, O>;
#[doc = "Field `FIFOCONS` writer - FIFOCON Set"]
pub type FIFOCONS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON2SET_SPEC, bool, O>;
#[doc = "Field `PFREEZES` writer - PFREEZE Set"]
pub type PFREEZES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON2SET_SPEC, bool, O>;
#[doc = "Field `INITDTGLS` writer - INITDTGL Set"]
pub type INITDTGLS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON2SET_SPEC, bool, O>;
#[doc = "Field `INITBKS` writer - INITBK Set"]
pub type INITBKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON2SET_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - RXINE Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxines(&mut self) -> RXINES_W<0> {
        RXINES_W::new(self)
    }
    #[doc = "Bit 1 - TXOUTE Set"]
    #[inline(always)]
    #[must_use]
    pub fn txoutes(&mut self) -> TXOUTES_W<1> {
        TXOUTES_W::new(self)
    }
    #[doc = "Bit 2 - TXSTPE Set"]
    #[inline(always)]
    #[must_use]
    pub fn txstpes(&mut self) -> TXSTPES_W<2> {
        TXSTPES_W::new(self)
    }
    #[doc = "Bit 3 - PERRE Set"]
    #[inline(always)]
    #[must_use]
    pub fn perres(&mut self) -> PERRES_W<3> {
        PERRES_W::new(self)
    }
    #[doc = "Bit 4 - NAKEDE Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakedes(&mut self) -> NAKEDES_W<4> {
        NAKEDES_W::new(self)
    }
    #[doc = "Bit 5 - ERRORFIE Set"]
    #[inline(always)]
    #[must_use]
    pub fn errorfies(&mut self) -> ERRORFIES_W<5> {
        ERRORFIES_W::new(self)
    }
    #[doc = "Bit 6 - RXSTALLDE Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldes(&mut self) -> RXSTALLDES_W<6> {
        RXSTALLDES_W::new(self)
    }
    #[doc = "Bit 10 - RAMACERE Set"]
    #[inline(always)]
    #[must_use]
    pub fn ramaceres(&mut self) -> RAMACERES_W<10> {
        RAMACERES_W::new(self)
    }
    #[doc = "Bit 12 - NBUSYBKE Set"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkes(&mut self) -> NBUSYBKES_W<12> {
        NBUSYBKES_W::new(self)
    }
    #[doc = "Bit 14 - FIFOCON Set"]
    #[inline(always)]
    #[must_use]
    pub fn fifocons(&mut self) -> FIFOCONS_W<14> {
        FIFOCONS_W::new(self)
    }
    #[doc = "Bit 17 - PFREEZE Set"]
    #[inline(always)]
    #[must_use]
    pub fn pfreezes(&mut self) -> PFREEZES_W<17> {
        PFREEZES_W::new(self)
    }
    #[doc = "Bit 18 - INITDTGL Set"]
    #[inline(always)]
    #[must_use]
    pub fn initdtgls(&mut self) -> INITDTGLS_W<18> {
        INITDTGLS_W::new(self)
    }
    #[doc = "Bit 19 - INITBK Set"]
    #[inline(always)]
    #[must_use]
    pub fn initbks(&mut self) -> INITBKS_W<19> {
        INITBKS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pipe Control Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon2set](index.html) module"]
pub struct UPCON2SET_SPEC;
impl crate::RegisterSpec for UPCON2SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [upcon2set::W](W) writer structure"]
impl crate::Writable for UPCON2SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UPCON2SET to value 0"]
impl crate::Resettable for UPCON2SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
