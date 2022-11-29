#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEOC` writer - Sequencer end of conversion Interrupt Enable"]
pub type SEOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `LOVR` writer - Sequencer last converted value overrun Interrupt Enable"]
pub type LOVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `WM` writer - Window monitor Interrupt Enable"]
pub type WM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `SMTRG` writer - Sequencer missed trigger event Interrupt Enable"]
pub type SMTRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TTO` writer - Timer time-out Interrupt Enable"]
pub type TTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Sequencer end of conversion Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn seoc(&mut self) -> SEOC_W<0> {
        SEOC_W::new(self)
    }
    #[doc = "Bit 1 - Sequencer last converted value overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lovr(&mut self) -> LOVR_W<1> {
        LOVR_W::new(self)
    }
    #[doc = "Bit 2 - Window monitor Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wm(&mut self) -> WM_W<2> {
        WM_W::new(self)
    }
    #[doc = "Bit 3 - Sequencer missed trigger event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smtrg(&mut self) -> SMTRG_W<3> {
        SMTRG_W::new(self)
    }
    #[doc = "Bit 5 - Timer time-out Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tto(&mut self) -> TTO_W<5> {
        TTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
