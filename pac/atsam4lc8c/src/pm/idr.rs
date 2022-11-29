#[doc = "Register `IDR` writer"]
pub struct W(crate::W<IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR_SPEC>;
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
impl From<crate::W<IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFD` writer - Clock Failure Detected Interrupt Disable"]
pub type CFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `CKRDY` writer - Clock Ready Interrupt Disable"]
pub type CKRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Wake up Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKESELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Disable Interrupt."]
    _1 = 1,
}
impl From<WAKESELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: WAKESELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKE` writer - Wake up Interrupt Disable"]
pub type WAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, WAKESELECT_AW, O>;
impl<'a, const O: u8> WAKE_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKESELECT_AW::_0)
    }
    #[doc = "Disable Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKESELECT_AW::_1)
    }
}
#[doc = "Field `AE` writer - Access Error Interrupt Disable"]
pub type AE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clock Failure Detected Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cfd(&mut self) -> CFD_W<0> {
        CFD_W::new(self)
    }
    #[doc = "Bit 5 - Clock Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ckrdy(&mut self) -> CKRDY_W<5> {
        CKRDY_W::new(self)
    }
    #[doc = "Bit 8 - Wake up Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WAKE_W<8> {
        WAKE_W::new(self)
    }
    #[doc = "Bit 31 - Access Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AE_W<31> {
        AE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr::W](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
