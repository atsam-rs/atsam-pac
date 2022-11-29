#[doc = "Register `UECON4CLR` writer"]
pub struct W(crate::W<UECON4CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UECON4CLR_SPEC>;
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
impl From<crate::W<UECON4CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UECON4CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINEC` writer - TXINE Clear"]
pub type TXINEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON4CLR_SPEC, bool, O>;
#[doc = "Field `RXOUTEC` writer - RXOUTE Clear"]
pub type RXOUTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON4CLR_SPEC, bool, O>;
#[doc = "Field `RXSTPEC` writer - RXOUTE Clear"]
pub type RXSTPEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON4CLR_SPEC, bool, O>;
#[doc = "Field `NAKOUTEC` writer - NAKOUTE Clear"]
pub type NAKOUTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON4CLR_SPEC, bool, O>;
#[doc = "Field `NAKINEC` writer - NAKINE Clear"]
pub type NAKINEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON4CLR_SPEC, bool, O>;
#[doc = "Field `STALLEDEC` writer - RXSTPE Clear"]
pub type STALLEDEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON4CLR_SPEC, bool, O>;
#[doc = "Field `NREPLYC` writer - NREPLY Clear"]
pub type NREPLYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON4CLR_SPEC, bool, O>;
#[doc = "Field `RAMACEREC` writer - RAMACERE Clear"]
pub type RAMACEREC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON4CLR_SPEC, bool, O>;
#[doc = "Field `NBUSYBKEC` writer - NBUSYBKE Clear"]
pub type NBUSYBKEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON4CLR_SPEC, bool, O>;
#[doc = "Field `FIFOCONC` writer - FIFOCON Clear"]
pub type FIFOCONC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON4CLR_SPEC, bool, O>;
#[doc = "Field `NYETDISC` writer - NYETDIS Clear"]
pub type NYETDISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON4CLR_SPEC, bool, O>;
#[doc = "Field `STALLRQC` writer - STALLEDE Clear"]
pub type STALLRQC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON4CLR_SPEC, bool, O>;
#[doc = "Field `BUSY0C` writer - BUSY0 Clear"]
pub type BUSY0C_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON4CLR_SPEC, bool, O>;
#[doc = "Field `BUSY1C` writer - BUSY1 Clear"]
pub type BUSY1C_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECON4CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - TXINE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txinec(&mut self) -> TXINEC_W<0> {
        TXINEC_W::new(self)
    }
    #[doc = "Bit 1 - RXOUTE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutec(&mut self) -> RXOUTEC_W<1> {
        RXOUTEC_W::new(self)
    }
    #[doc = "Bit 2 - RXOUTE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpec(&mut self) -> RXSTPEC_W<2> {
        RXSTPEC_W::new(self)
    }
    #[doc = "Bit 3 - NAKOUTE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutec(&mut self) -> NAKOUTEC_W<3> {
        NAKOUTEC_W::new(self)
    }
    #[doc = "Bit 4 - NAKINE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakinec(&mut self) -> NAKINEC_W<4> {
        NAKINEC_W::new(self)
    }
    #[doc = "Bit 6 - RXSTPE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stalledec(&mut self) -> STALLEDEC_W<6> {
        STALLEDEC_W::new(self)
    }
    #[doc = "Bit 8 - NREPLY Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nreplyc(&mut self) -> NREPLYC_W<8> {
        NREPLYC_W::new(self)
    }
    #[doc = "Bit 11 - RAMACERE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ramacerec(&mut self) -> RAMACEREC_W<11> {
        RAMACEREC_W::new(self)
    }
    #[doc = "Bit 12 - NBUSYBKE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkec(&mut self) -> NBUSYBKEC_W<12> {
        NBUSYBKEC_W::new(self)
    }
    #[doc = "Bit 14 - FIFOCON Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoconc(&mut self) -> FIFOCONC_W<14> {
        FIFOCONC_W::new(self)
    }
    #[doc = "Bit 17 - NYETDIS Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nyetdisc(&mut self) -> NYETDISC_W<17> {
        NYETDISC_W::new(self)
    }
    #[doc = "Bit 19 - STALLEDE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stallrqc(&mut self) -> STALLRQC_W<19> {
        STALLRQC_W::new(self)
    }
    #[doc = "Bit 24 - BUSY0 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn busy0c(&mut self) -> BUSY0C_W<24> {
        BUSY0C_W::new(self)
    }
    #[doc = "Bit 25 - BUSY1 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn busy1c(&mut self) -> BUSY1C_W<25> {
        BUSY1C_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TXINE Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon4clr](index.html) module"]
pub struct UECON4CLR_SPEC;
impl crate::RegisterSpec for UECON4CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uecon4clr::W](W) writer structure"]
impl crate::Writable for UECON4CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UECON4CLR to value 0"]
impl crate::Resettable for UECON4CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
