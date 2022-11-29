#[doc = "Register `THR` writer"]
pub struct W(crate::W<THR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THR_SPEC>;
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
impl From<crate::W<THR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXCHR` writer - Character to be Transmitted"]
pub type TXCHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, THR_SPEC, u16, u16, 9, O>;
#[doc = "Sync Field to be transmitted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXSYNHSELECT_AW {
    #[doc = "0: The next character sent is encoded as a data. Start Frame Delimiter is DATA SYNC"]
    _0 = 0,
    #[doc = "1: The next character sent is encoded as a command. Start Frame Delimiter is COMMAND SYNC"]
    _1 = 1,
}
impl From<TXSYNHSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSYNHSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSYNH` writer - Sync Field to be transmitted"]
pub type TXSYNH_W<'a, const O: u8> = crate::BitWriter<'a, u32, THR_SPEC, TXSYNHSELECT_AW, O>;
impl<'a, const O: u8> TXSYNH_W<'a, O> {
    #[doc = "The next character sent is encoded as a data. Start Frame Delimiter is DATA SYNC"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXSYNHSELECT_AW::_0)
    }
    #[doc = "The next character sent is encoded as a command. Start Frame Delimiter is COMMAND SYNC"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXSYNHSELECT_AW::_1)
    }
}
impl W {
    #[doc = "Bits 0:8 - Character to be Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn txchr(&mut self) -> TXCHR_W<0> {
        TXCHR_W::new(self)
    }
    #[doc = "Bit 15 - Sync Field to be transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn txsynh(&mut self) -> TXSYNH_W<15> {
        TXSYNH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter Holding Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr](index.html) module"]
pub struct THR_SPEC;
impl crate::RegisterSpec for THR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [thr::W](W) writer structure"]
impl crate::Writable for THR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets THR to value 0"]
impl crate::Resettable for THR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
