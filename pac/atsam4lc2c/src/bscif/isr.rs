#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OSC32RDY` reader - 32kHz Oscillator Ready"]
pub type OSC32RDY_R = crate::BitReader<bool>;
#[doc = "Field `RC32KRDY` reader - 32kHz RC Oscillator Ready"]
pub type RC32KRDY_R = crate::BitReader<bool>;
#[doc = "Field `RC32KLOCK` reader - 32kHz RC Oscillator Lock"]
pub type RC32KLOCK_R = crate::BitReader<bool>;
#[doc = "Field `RC32KREFE` reader - 32kHz RC Oscillator Reference Error"]
pub type RC32KREFE_R = crate::BitReader<bool>;
#[doc = "Field `RC32KSAT` reader - 32kHz RC Oscillator Saturation"]
pub type RC32KSAT_R = crate::BitReader<bool>;
#[doc = "Field `BOD33DET` reader - BOD33 Detected"]
pub type BOD33DET_R = crate::BitReader<bool>;
#[doc = "Field `BOD18DET` reader - BOD18 Detected"]
pub type BOD18DET_R = crate::BitReader<bool>;
#[doc = "Field `BOD33SYNRDY` reader - BOD33 Synchronization Ready"]
pub type BOD33SYNRDY_R = crate::BitReader<bool>;
#[doc = "Field `BOD18SYNRDY` reader - BOD18 Synchronization Ready"]
pub type BOD18SYNRDY_R = crate::BitReader<bool>;
#[doc = "Field `SSWRDY` reader - VREG Stop Switching Ready"]
pub type SSWRDY_R = crate::BitReader<bool>;
#[doc = "Field `VREGOK` reader - Main VREG OK"]
pub type VREGOK_R = crate::BitReader<bool>;
#[doc = "Field `LPBGRDY` reader - Low Power Bandgap Voltage Reference Ready"]
pub type LPBGRDY_R = crate::BitReader<bool>;
#[doc = "Field `AE` reader - Access Error"]
pub type AE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - 32kHz Oscillator Ready"]
    #[inline(always)]
    pub fn osc32rdy(&self) -> OSC32RDY_R {
        OSC32RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 32kHz RC Oscillator Ready"]
    #[inline(always)]
    pub fn rc32krdy(&self) -> RC32KRDY_R {
        RC32KRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 32kHz RC Oscillator Lock"]
    #[inline(always)]
    pub fn rc32klock(&self) -> RC32KLOCK_R {
        RC32KLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 32kHz RC Oscillator Reference Error"]
    #[inline(always)]
    pub fn rc32krefe(&self) -> RC32KREFE_R {
        RC32KREFE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 32kHz RC Oscillator Saturation"]
    #[inline(always)]
    pub fn rc32ksat(&self) -> RC32KSAT_R {
        RC32KSAT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BOD33 Detected"]
    #[inline(always)]
    pub fn bod33det(&self) -> BOD33DET_R {
        BOD33DET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BOD18 Detected"]
    #[inline(always)]
    pub fn bod18det(&self) -> BOD18DET_R {
        BOD18DET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn bod33synrdy(&self) -> BOD33SYNRDY_R {
        BOD33SYNRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BOD18 Synchronization Ready"]
    #[inline(always)]
    pub fn bod18synrdy(&self) -> BOD18SYNRDY_R {
        BOD18SYNRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VREG Stop Switching Ready"]
    #[inline(always)]
    pub fn sswrdy(&self) -> SSWRDY_R {
        SSWRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Main VREG OK"]
    #[inline(always)]
    pub fn vregok(&self) -> VREGOK_R {
        VREGOK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Low Power Bandgap Voltage Reference Ready"]
    #[inline(always)]
    pub fn lpbgrdy(&self) -> LPBGRDY_R {
        LPBGRDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 31 - Access Error"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
