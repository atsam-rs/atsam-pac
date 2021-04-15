#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `OSC32RDY`"]
pub type OSC32RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RC32KRDY`"]
pub type RC32KRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RC32KLOCK`"]
pub type RC32KLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `RC32KREFE`"]
pub type RC32KREFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RC32KSAT`"]
pub type RC32KSAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOD33DET`"]
pub type BOD33DET_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOD18DET`"]
pub type BOD18DET_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOD33SYNRDY`"]
pub type BOD33SYNRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOD18SYNRDY`"]
pub type BOD18SYNRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `SSWRDY`"]
pub type SSWRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `VREGOK`"]
pub type VREGOK_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPBGRDY`"]
pub type LPBGRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE`"]
pub type AE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - 32kHz Oscillator Ready"]
    #[inline(always)]
    pub fn osc32rdy(&self) -> OSC32RDY_R {
        OSC32RDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 32kHz RC Oscillator Ready"]
    #[inline(always)]
    pub fn rc32krdy(&self) -> RC32KRDY_R {
        RC32KRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 32kHz RC Oscillator Lock"]
    #[inline(always)]
    pub fn rc32klock(&self) -> RC32KLOCK_R {
        RC32KLOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 32kHz RC Oscillator Reference Error"]
    #[inline(always)]
    pub fn rc32krefe(&self) -> RC32KREFE_R {
        RC32KREFE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 32kHz RC Oscillator Saturation"]
    #[inline(always)]
    pub fn rc32ksat(&self) -> RC32KSAT_R {
        RC32KSAT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BOD33 Detected"]
    #[inline(always)]
    pub fn bod33det(&self) -> BOD33DET_R {
        BOD33DET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BOD18 Detected"]
    #[inline(always)]
    pub fn bod18det(&self) -> BOD18DET_R {
        BOD18DET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn bod33synrdy(&self) -> BOD33SYNRDY_R {
        BOD33SYNRDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BOD18 Synchronization Ready"]
    #[inline(always)]
    pub fn bod18synrdy(&self) -> BOD18SYNRDY_R {
        BOD18SYNRDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VREG Stop Switching Ready"]
    #[inline(always)]
    pub fn sswrdy(&self) -> SSWRDY_R {
        SSWRDY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Main VREG OK"]
    #[inline(always)]
    pub fn vregok(&self) -> VREGOK_R {
        VREGOK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Low Power Bandgap Voltage Reference Ready"]
    #[inline(always)]
    pub fn lpbgrdy(&self) -> LPBGRDY_R {
        LPBGRDY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Access Error"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
