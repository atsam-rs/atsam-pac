#[doc = "Register `UPSTA5CLR` writer"]
pub struct W(crate::W<UPSTA5CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPSTA5CLR_SPEC>;
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
impl From<crate::W<UPSTA5CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPSTA5CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINIC` writer - RXINI Clear"]
pub type RXINIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPSTA5CLR_SPEC, bool, O>;
#[doc = "Field `TXOUTIC` writer - TXOUTI Clear"]
pub type TXOUTIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPSTA5CLR_SPEC, bool, O>;
#[doc = "Field `TXSTPIC` writer - TXSTPI Clear"]
pub type TXSTPIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPSTA5CLR_SPEC, bool, O>;
#[doc = "Field `PERRIC` writer - PERRI Clear"]
pub type PERRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPSTA5CLR_SPEC, bool, O>;
#[doc = "Field `NAKEDIC` writer - NAKEDI Clear"]
pub type NAKEDIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPSTA5CLR_SPEC, bool, O>;
#[doc = "Field `ERRORFIC` writer - ERRORFI Clear"]
pub type ERRORFIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPSTA5CLR_SPEC, bool, O>;
#[doc = "Field `RXSTALLDIC` writer - RXSTALLDI Clear"]
pub type RXSTALLDIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPSTA5CLR_SPEC, bool, O>;
#[doc = "Field `RAMACERIC` writer - RAMACERI Clear"]
pub type RAMACERIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPSTA5CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - RXINI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxinic(&mut self) -> RXINIC_W<0> {
        RXINIC_W::new(self)
    }
    #[doc = "Bit 1 - TXOUTI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txoutic(&mut self) -> TXOUTIC_W<1> {
        TXOUTIC_W::new(self)
    }
    #[doc = "Bit 2 - TXSTPI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txstpic(&mut self) -> TXSTPIC_W<2> {
        TXSTPIC_W::new(self)
    }
    #[doc = "Bit 3 - PERRI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn perric(&mut self) -> PERRIC_W<3> {
        PERRIC_W::new(self)
    }
    #[doc = "Bit 4 - NAKEDI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakedic(&mut self) -> NAKEDIC_W<4> {
        NAKEDIC_W::new(self)
    }
    #[doc = "Bit 5 - ERRORFI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn errorfic(&mut self) -> ERRORFIC_W<5> {
        ERRORFIC_W::new(self)
    }
    #[doc = "Bit 6 - RXSTALLDI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldic(&mut self) -> RXSTALLDIC_W<6> {
        RXSTALLDIC_W::new(self)
    }
    #[doc = "Bit 10 - RAMACERI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ramaceric(&mut self) -> RAMACERIC_W<10> {
        RAMACERIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pipe Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta5clr](index.html) module"]
pub struct UPSTA5CLR_SPEC;
impl crate::RegisterSpec for UPSTA5CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [upsta5clr::W](W) writer structure"]
impl crate::Writable for UPSTA5CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UPSTA5CLR to value 0"]
impl crate::Resettable for UPSTA5CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
