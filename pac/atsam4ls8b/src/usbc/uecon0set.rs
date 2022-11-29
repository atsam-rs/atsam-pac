#[doc = "Register `UECON0SET` writer"]
pub struct W(crate::W<UECON0SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UECON0SET_SPEC>;
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
impl From<crate::W<UECON0SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UECON0SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINES` writer - TXINE Set"]
pub type TXINES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON0SET_SPEC, bool, O>;
#[doc = "Field `RXOUTES` writer - RXOUTE Set"]
pub type RXOUTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON0SET_SPEC, bool, O>;
#[doc = "Field `RXSTPES` writer - RXSTPE Set"]
pub type RXSTPES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON0SET_SPEC, bool, O>;
#[doc = "Field `NAKOUTES` writer - NAKOUTE Set"]
pub type NAKOUTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON0SET_SPEC, bool, O>;
#[doc = "Field `NAKINES` writer - NAKINE Set"]
pub type NAKINES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON0SET_SPEC, bool, O>;
#[doc = "Field `STALLEDES` writer - STALLEDE Set"]
pub type STALLEDES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON0SET_SPEC, bool, O>;
#[doc = "Field `NREPLYS` writer - NREPLY Set"]
pub type NREPLYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON0SET_SPEC, bool, O>;
#[doc = "Field `RAMACERES` writer - RAMACERE Set"]
pub type RAMACERES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON0SET_SPEC, bool, O>;
#[doc = "Field `NBUSYBKES` writer - NBUSYBKE Set"]
pub type NBUSYBKES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON0SET_SPEC, bool, O>;
#[doc = "Field `KILLBKS` writer - KILLBK Set"]
pub type KILLBKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON0SET_SPEC, bool, O>;
#[doc = "Field `NYETDISS` writer - NYETDIS Set"]
pub type NYETDISS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON0SET_SPEC, bool, O>;
#[doc = "Field `RSTDTS` writer - RSTDT Set"]
pub type RSTDTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON0SET_SPEC, bool, O>;
#[doc = "Field `STALLRQS` writer - STALLRQ Set"]
pub type STALLRQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON0SET_SPEC, bool, O>;
#[doc = "Field `BUSY0S` writer - BUSY0 Set"]
pub type BUSY0S_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON0SET_SPEC, bool, O>;
#[doc = "Field `BUSY1S` writer - BUSY1 Set"]
pub type BUSY1S_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON0SET_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - TXINE Set"]
    #[inline(always)]
    #[must_use]
    pub fn txines(&mut self) -> TXINES_W<0> {
        TXINES_W::new(self)
    }
    #[doc = "Bit 1 - RXOUTE Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutes(&mut self) -> RXOUTES_W<1> {
        RXOUTES_W::new(self)
    }
    #[doc = "Bit 2 - RXSTPE Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpes(&mut self) -> RXSTPES_W<2> {
        RXSTPES_W::new(self)
    }
    #[doc = "Bit 3 - NAKOUTE Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutes(&mut self) -> NAKOUTES_W<3> {
        NAKOUTES_W::new(self)
    }
    #[doc = "Bit 4 - NAKINE Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakines(&mut self) -> NAKINES_W<4> {
        NAKINES_W::new(self)
    }
    #[doc = "Bit 6 - STALLEDE Set"]
    #[inline(always)]
    #[must_use]
    pub fn stalledes(&mut self) -> STALLEDES_W<6> {
        STALLEDES_W::new(self)
    }
    #[doc = "Bit 8 - NREPLY Set"]
    #[inline(always)]
    #[must_use]
    pub fn nreplys(&mut self) -> NREPLYS_W<8> {
        NREPLYS_W::new(self)
    }
    #[doc = "Bit 11 - RAMACERE Set"]
    #[inline(always)]
    #[must_use]
    pub fn ramaceres(&mut self) -> RAMACERES_W<11> {
        RAMACERES_W::new(self)
    }
    #[doc = "Bit 12 - NBUSYBKE Set"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkes(&mut self) -> NBUSYBKES_W<12> {
        NBUSYBKES_W::new(self)
    }
    #[doc = "Bit 13 - KILLBK Set"]
    #[inline(always)]
    #[must_use]
    pub fn killbks(&mut self) -> KILLBKS_W<13> {
        KILLBKS_W::new(self)
    }
    #[doc = "Bit 17 - NYETDIS Set"]
    #[inline(always)]
    #[must_use]
    pub fn nyetdiss(&mut self) -> NYETDISS_W<17> {
        NYETDISS_W::new(self)
    }
    #[doc = "Bit 18 - RSTDT Set"]
    #[inline(always)]
    #[must_use]
    pub fn rstdts(&mut self) -> RSTDTS_W<18> {
        RSTDTS_W::new(self)
    }
    #[doc = "Bit 19 - STALLRQ Set"]
    #[inline(always)]
    #[must_use]
    pub fn stallrqs(&mut self) -> STALLRQS_W<19> {
        STALLRQS_W::new(self)
    }
    #[doc = "Bit 24 - BUSY0 Set"]
    #[inline(always)]
    #[must_use]
    pub fn busy0s(&mut self) -> BUSY0S_W<24> {
        BUSY0S_W::new(self)
    }
    #[doc = "Bit 25 - BUSY1 Set"]
    #[inline(always)]
    #[must_use]
    pub fn busy1s(&mut self) -> BUSY1S_W<25> {
        BUSY1S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Control Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon0set](index.html) module"]
pub struct UECON0SET_SPEC;
impl crate::RegisterSpec for UECON0SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uecon0set::W](W) writer structure"]
impl crate::Writable for UECON0SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UECON0SET to value 0"]
impl crate::Resettable for UECON0SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
