#[doc = "Register `UPCON4CLR` writer"]
pub struct W(crate::W<UPCON4CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPCON4CLR_SPEC>;
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
impl From<crate::W<UPCON4CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPCON4CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINEC` writer - RXINE Clear"]
pub type RXINEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON4CLR_SPEC, bool, O>;
#[doc = "Field `TXOUTEC` writer - TXOUTE Clear"]
pub type TXOUTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON4CLR_SPEC, bool, O>;
#[doc = "Field `TXSTPEC` writer - TXSTPE Clear"]
pub type TXSTPEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON4CLR_SPEC, bool, O>;
#[doc = "Field `PERREC` writer - PERRE Clear"]
pub type PERREC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON4CLR_SPEC, bool, O>;
#[doc = "Field `NAKEDEC` writer - NAKEDE Clear"]
pub type NAKEDEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON4CLR_SPEC, bool, O>;
#[doc = "Field `ERRORFIEC` writer - ERRORFIE Clear"]
pub type ERRORFIEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON4CLR_SPEC, bool, O>;
#[doc = "Field `RXSTALLDEC` writer - RXTALLDE Clear"]
pub type RXSTALLDEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON4CLR_SPEC, bool, O>;
#[doc = "Field `RAMACEREC` writer - RAMACERE Clear"]
pub type RAMACEREC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON4CLR_SPEC, bool, O>;
#[doc = "Field `NBUSYBKEC` writer - NBUSYBKE Clear"]
pub type NBUSYBKEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON4CLR_SPEC, bool, O>;
#[doc = "Field `FIFOCONC` writer - FIFOCON Clear"]
pub type FIFOCONC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON4CLR_SPEC, bool, O>;
#[doc = "Field `PFREEZEC` writer - PFREEZE Clear"]
pub type PFREEZEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON4CLR_SPEC, bool, O>;
#[doc = "Field `INITBKC` writer - INITBK Clear"]
pub type INITBKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCON4CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - RXINE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxinec(&mut self) -> RXINEC_W<0> {
        RXINEC_W::new(self)
    }
    #[doc = "Bit 1 - TXOUTE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txoutec(&mut self) -> TXOUTEC_W<1> {
        TXOUTEC_W::new(self)
    }
    #[doc = "Bit 2 - TXSTPE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txstpec(&mut self) -> TXSTPEC_W<2> {
        TXSTPEC_W::new(self)
    }
    #[doc = "Bit 3 - PERRE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn perrec(&mut self) -> PERREC_W<3> {
        PERREC_W::new(self)
    }
    #[doc = "Bit 4 - NAKEDE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakedec(&mut self) -> NAKEDEC_W<4> {
        NAKEDEC_W::new(self)
    }
    #[doc = "Bit 5 - ERRORFIE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn errorfiec(&mut self) -> ERRORFIEC_W<5> {
        ERRORFIEC_W::new(self)
    }
    #[doc = "Bit 6 - RXTALLDE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldec(&mut self) -> RXSTALLDEC_W<6> {
        RXSTALLDEC_W::new(self)
    }
    #[doc = "Bit 10 - RAMACERE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ramacerec(&mut self) -> RAMACEREC_W<10> {
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
    #[doc = "Bit 17 - PFREEZE Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pfreezec(&mut self) -> PFREEZEC_W<17> {
        PFREEZEC_W::new(self)
    }
    #[doc = "Bit 19 - INITBK Clear"]
    #[inline(always)]
    #[must_use]
    pub fn initbkc(&mut self) -> INITBKC_W<19> {
        INITBKC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pipe Control Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon4clr](index.html) module"]
pub struct UPCON4CLR_SPEC;
impl crate::RegisterSpec for UPCON4CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [upcon4clr::W](W) writer structure"]
impl crate::Writable for UPCON4CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UPCON4CLR to value 0"]
impl crate::Resettable for UPCON4CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
