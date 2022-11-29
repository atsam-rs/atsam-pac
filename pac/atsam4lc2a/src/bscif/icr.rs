#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSC32RDY` writer - 32kHz Oscillator Ready"]
pub type OSC32RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RC32KRDY` writer - 32kHz RC Oscillator Ready"]
pub type RC32KRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RC32KLOCK` writer - 32kHz RC Oscillator Lock"]
pub type RC32KLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RC32KREFE` writer - 32kHz RC Oscillator Reference Error"]
pub type RC32KREFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RC32KSAT` writer - 32kHz RC Oscillator Saturation"]
pub type RC32KSAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `BOD33DET` writer - BOD33 Detected"]
pub type BOD33DET_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `BOD18DET` writer - BOD18 Detected"]
pub type BOD18DET_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `BOD33SYNRDY` writer - BOD33 Synchronization Ready"]
pub type BOD33SYNRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `BOD18SYNRDY` writer - BOD18 Synchronization Ready"]
pub type BOD18SYNRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `SSWRDY` writer - VREG Stop Switching Ready"]
pub type SSWRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `VREGOK` writer - Main VREG OK"]
pub type VREGOK_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `LPBGRDY` writer - Low Power Bandgap Voltage Reference Ready"]
pub type LPBGRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `AE` writer - Access Error"]
pub type AE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - 32kHz Oscillator Ready"]
    #[inline(always)]
    #[must_use]
    pub fn osc32rdy(&mut self) -> OSC32RDY_W<0> {
        OSC32RDY_W::new(self)
    }
    #[doc = "Bit 1 - 32kHz RC Oscillator Ready"]
    #[inline(always)]
    #[must_use]
    pub fn rc32krdy(&mut self) -> RC32KRDY_W<1> {
        RC32KRDY_W::new(self)
    }
    #[doc = "Bit 2 - 32kHz RC Oscillator Lock"]
    #[inline(always)]
    #[must_use]
    pub fn rc32klock(&mut self) -> RC32KLOCK_W<2> {
        RC32KLOCK_W::new(self)
    }
    #[doc = "Bit 3 - 32kHz RC Oscillator Reference Error"]
    #[inline(always)]
    #[must_use]
    pub fn rc32krefe(&mut self) -> RC32KREFE_W<3> {
        RC32KREFE_W::new(self)
    }
    #[doc = "Bit 4 - 32kHz RC Oscillator Saturation"]
    #[inline(always)]
    #[must_use]
    pub fn rc32ksat(&mut self) -> RC32KSAT_W<4> {
        RC32KSAT_W::new(self)
    }
    #[doc = "Bit 5 - BOD33 Detected"]
    #[inline(always)]
    #[must_use]
    pub fn bod33det(&mut self) -> BOD33DET_W<5> {
        BOD33DET_W::new(self)
    }
    #[doc = "Bit 6 - BOD18 Detected"]
    #[inline(always)]
    #[must_use]
    pub fn bod18det(&mut self) -> BOD18DET_W<6> {
        BOD18DET_W::new(self)
    }
    #[doc = "Bit 7 - BOD33 Synchronization Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bod33synrdy(&mut self) -> BOD33SYNRDY_W<7> {
        BOD33SYNRDY_W::new(self)
    }
    #[doc = "Bit 8 - BOD18 Synchronization Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bod18synrdy(&mut self) -> BOD18SYNRDY_W<8> {
        BOD18SYNRDY_W::new(self)
    }
    #[doc = "Bit 9 - VREG Stop Switching Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sswrdy(&mut self) -> SSWRDY_W<9> {
        SSWRDY_W::new(self)
    }
    #[doc = "Bit 10 - Main VREG OK"]
    #[inline(always)]
    #[must_use]
    pub fn vregok(&mut self) -> VREGOK_W<10> {
        VREGOK_W::new(self)
    }
    #[doc = "Bit 12 - Low Power Bandgap Voltage Reference Ready"]
    #[inline(always)]
    #[must_use]
    pub fn lpbgrdy(&mut self) -> LPBGRDY_W<12> {
        LPBGRDY_W::new(self)
    }
    #[doc = "Bit 31 - Access Error"]
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
#[doc = "Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
