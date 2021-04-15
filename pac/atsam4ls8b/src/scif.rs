#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x04 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x08 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x0c - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x10 - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x14 - Power and Clocks Status Register"]
    pub pclksr: PCLKSR,
    #[doc = "0x18 - Unlock Register"]
    pub unlock: UNLOCK,
    #[doc = "0x1c - Chip Specific Configuration Register"]
    pub cscr: CSCR,
    #[doc = "0x20 - Oscillator Control Register"]
    pub oscctrl0: OSCCTRL0,
    #[doc = "0x24 - PLL0 Control Register"]
    pub pll: PLL,
    #[doc = "0x28 - DFLL0 Config Register"]
    pub dfll0conf: DFLL0CONF,
    #[doc = "0x2c - DFLL Value Register"]
    pub dfll0val: DFLL0VAL,
    #[doc = "0x30 - DFLL0 Multiplier Register"]
    pub dfll0mul: DFLL0MUL,
    #[doc = "0x34 - DFLL0 Step Register"]
    pub dfll0step: DFLL0STEP,
    #[doc = "0x38 - DFLL0 Spread Spectrum Generator Control Register"]
    pub dfll0ssg: DFLL0SSG,
    #[doc = "0x3c - DFLL0 Ratio Registe"]
    pub dfll0ratio: DFLL0RATIO,
    #[doc = "0x40 - DFLL0 Synchronization Register"]
    pub dfll0sync: DFLL0SYNC,
    #[doc = "0x44 - System RC Oscillator Calibration Register"]
    pub rccr: RCCR,
    #[doc = "0x48 - 4/8/12 MHz RC Oscillator Configuration Register"]
    pub rcfastcfg: RCFASTCFG,
    #[doc = "0x4c - 4/8/12 MHz RC Oscillator Status Register"]
    pub rcfastsr: RCFASTSR,
    #[doc = "0x50 - 80 MHz RC Oscillator Register"]
    pub rc80mcr: RC80MCR,
    _reserved21: [u8; 16usize],
    #[doc = "0x64 - High Resolution Prescaler Control Register"]
    pub hrpcr: HRPCR,
    #[doc = "0x68 - Fractional Prescaler Control Register"]
    pub fpcr: FPCR,
    #[doc = "0x6c - Fractional Prescaler Multiplier Register"]
    pub fpmul: FPMUL,
    #[doc = "0x70 - Fractional Prescaler DIVIDER Register"]
    pub fpdiv: FPDIV,
    #[doc = "0x74 - Generic Clock Control"]
    pub gcctrl: [GCCTRL; 12],
    _reserved26: [u8; 820usize],
    #[doc = "0x3d8 - 4/8/12 MHz RC Oscillator Version Register"]
    pub rcfastversion: RCFASTVERSION,
    #[doc = "0x3dc - Generic Clock Prescaler Version Register"]
    pub gclkprescversion: GCLKPRESCVERSION,
    #[doc = "0x3e0 - PLL Version Register"]
    pub pllifaversion: PLLIFAVERSION,
    #[doc = "0x3e4 - Oscillator 0 Version Register"]
    pub oscifaversion: OSCIFAVERSION,
    #[doc = "0x3e8 - DFLL Version Register"]
    pub dfllifbversion: DFLLIFBVERSION,
    #[doc = "0x3ec - System RC Oscillator Version Register"]
    pub rcoscifaversion: RCOSCIFAVERSION,
    #[doc = "0x3f0 - Frequency Locked Oscillator Version Register"]
    pub floversion: FLOVERSION,
    #[doc = "0x3f4 - 80MHz RC Oscillator Version Register"]
    pub rc80mversion: RC80MVERSION,
    #[doc = "0x3f8 - Generic Clock Version Register"]
    pub gclkifversion: GCLKIFVERSION,
    #[doc = "0x3fc - SCIF Version Register"]
    pub version: VERSION,
}
#[doc = "Chip Specific Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cscr](cscr) module"]
pub type CSCR = crate::Reg<u32, _CSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCR;
#[doc = "`read()` method returns [cscr::R](cscr::R) reader structure"]
impl crate::Readable for CSCR {}
#[doc = "`write(|w| ..)` method takes [cscr::W](cscr::W) writer structure"]
impl crate::Writable for CSCR {}
#[doc = "Chip Specific Configuration Register"]
pub mod cscr;
#[doc = "DFLL Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllifbversion](dfllifbversion) module"]
pub type DFLLIFBVERSION = crate::Reg<u32, _DFLLIFBVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLIFBVERSION;
#[doc = "`read()` method returns [dfllifbversion::R](dfllifbversion::R) reader structure"]
impl crate::Readable for DFLLIFBVERSION {}
#[doc = "DFLL Version Register"]
pub mod dfllifbversion;
#[doc = "DFLL0 Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfll0conf](dfll0conf) module"]
pub type DFLL0CONF = crate::Reg<u32, _DFLL0CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLL0CONF;
#[doc = "`read()` method returns [dfll0conf::R](dfll0conf::R) reader structure"]
impl crate::Readable for DFLL0CONF {}
#[doc = "`write(|w| ..)` method takes [dfll0conf::W](dfll0conf::W) writer structure"]
impl crate::Writable for DFLL0CONF {}
#[doc = "DFLL0 Config Register"]
pub mod dfll0conf;
#[doc = "DFLL0 Multiplier Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfll0mul](dfll0mul) module"]
pub type DFLL0MUL = crate::Reg<u32, _DFLL0MUL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLL0MUL;
#[doc = "`read()` method returns [dfll0mul::R](dfll0mul::R) reader structure"]
impl crate::Readable for DFLL0MUL {}
#[doc = "`write(|w| ..)` method takes [dfll0mul::W](dfll0mul::W) writer structure"]
impl crate::Writable for DFLL0MUL {}
#[doc = "DFLL0 Multiplier Register"]
pub mod dfll0mul;
#[doc = "DFLL0 Ratio Registe\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfll0ratio](dfll0ratio) module"]
pub type DFLL0RATIO = crate::Reg<u32, _DFLL0RATIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLL0RATIO;
#[doc = "`read()` method returns [dfll0ratio::R](dfll0ratio::R) reader structure"]
impl crate::Readable for DFLL0RATIO {}
#[doc = "DFLL0 Ratio Registe"]
pub mod dfll0ratio;
#[doc = "DFLL0 Spread Spectrum Generator Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfll0ssg](dfll0ssg) module"]
pub type DFLL0SSG = crate::Reg<u32, _DFLL0SSG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLL0SSG;
#[doc = "`write(|w| ..)` method takes [dfll0ssg::W](dfll0ssg::W) writer structure"]
impl crate::Writable for DFLL0SSG {}
#[doc = "DFLL0 Spread Spectrum Generator Control Register"]
pub mod dfll0ssg;
#[doc = "DFLL0 Step Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfll0step](dfll0step) module"]
pub type DFLL0STEP = crate::Reg<u32, _DFLL0STEP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLL0STEP;
#[doc = "`read()` method returns [dfll0step::R](dfll0step::R) reader structure"]
impl crate::Readable for DFLL0STEP {}
#[doc = "`write(|w| ..)` method takes [dfll0step::W](dfll0step::W) writer structure"]
impl crate::Writable for DFLL0STEP {}
#[doc = "DFLL0 Step Register"]
pub mod dfll0step;
#[doc = "DFLL0 Synchronization Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfll0sync](dfll0sync) module"]
pub type DFLL0SYNC = crate::Reg<u32, _DFLL0SYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLL0SYNC;
#[doc = "`write(|w| ..)` method takes [dfll0sync::W](dfll0sync::W) writer structure"]
impl crate::Writable for DFLL0SYNC {}
#[doc = "DFLL0 Synchronization Register"]
pub mod dfll0sync;
#[doc = "DFLL Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfll0val](dfll0val) module"]
pub type DFLL0VAL = crate::Reg<u32, _DFLL0VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLL0VAL;
#[doc = "`read()` method returns [dfll0val::R](dfll0val::R) reader structure"]
impl crate::Readable for DFLL0VAL {}
#[doc = "`write(|w| ..)` method takes [dfll0val::W](dfll0val::W) writer structure"]
impl crate::Writable for DFLL0VAL {}
#[doc = "DFLL Value Register"]
pub mod dfll0val;
#[doc = "Frequency Locked Oscillator Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [floversion](floversion) module"]
pub type FLOVERSION = crate::Reg<u32, _FLOVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLOVERSION;
#[doc = "`read()` method returns [floversion::R](floversion::R) reader structure"]
impl crate::Readable for FLOVERSION {}
#[doc = "Frequency Locked Oscillator Version Register"]
pub mod floversion;
#[doc = "Fractional Prescaler Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpcr](fpcr) module"]
pub type FPCR = crate::Reg<u32, _FPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPCR;
#[doc = "`read()` method returns [fpcr::R](fpcr::R) reader structure"]
impl crate::Readable for FPCR {}
#[doc = "`write(|w| ..)` method takes [fpcr::W](fpcr::W) writer structure"]
impl crate::Writable for FPCR {}
#[doc = "Fractional Prescaler Control Register"]
pub mod fpcr;
#[doc = "Fractional Prescaler DIVIDER Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpdiv](fpdiv) module"]
pub type FPDIV = crate::Reg<u32, _FPDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPDIV;
#[doc = "`read()` method returns [fpdiv::R](fpdiv::R) reader structure"]
impl crate::Readable for FPDIV {}
#[doc = "`write(|w| ..)` method takes [fpdiv::W](fpdiv::W) writer structure"]
impl crate::Writable for FPDIV {}
#[doc = "Fractional Prescaler DIVIDER Register"]
pub mod fpdiv;
#[doc = "Fractional Prescaler Multiplier Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpmul](fpmul) module"]
pub type FPMUL = crate::Reg<u32, _FPMUL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPMUL;
#[doc = "`read()` method returns [fpmul::R](fpmul::R) reader structure"]
impl crate::Readable for FPMUL {}
#[doc = "`write(|w| ..)` method takes [fpmul::W](fpmul::W) writer structure"]
impl crate::Writable for FPMUL {}
#[doc = "Fractional Prescaler Multiplier Register"]
pub mod fpmul;
#[doc = "Generic Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcctrl](gcctrl) module"]
pub type GCCTRL = crate::Reg<u32, _GCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCCTRL;
#[doc = "`read()` method returns [gcctrl::R](gcctrl::R) reader structure"]
impl crate::Readable for GCCTRL {}
#[doc = "`write(|w| ..)` method takes [gcctrl::W](gcctrl::W) writer structure"]
impl crate::Writable for GCCTRL {}
#[doc = "Generic Clock Control"]
pub mod gcctrl;
#[doc = "Generic Clock Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gclkifversion](gclkifversion) module"]
pub type GCLKIFVERSION = crate::Reg<u32, _GCLKIFVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCLKIFVERSION;
#[doc = "`read()` method returns [gclkifversion::R](gclkifversion::R) reader structure"]
impl crate::Readable for GCLKIFVERSION {}
#[doc = "Generic Clock Version Register"]
pub mod gclkifversion;
#[doc = "Generic Clock Prescaler Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gclkprescversion](gclkprescversion) module"]
pub type GCLKPRESCVERSION = crate::Reg<u32, _GCLKPRESCVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCLKPRESCVERSION;
#[doc = "`read()` method returns [gclkprescversion::R](gclkprescversion::R) reader structure"]
impl crate::Readable for GCLKPRESCVERSION {}
#[doc = "Generic Clock Prescaler Version Register"]
pub mod gclkprescversion;
#[doc = "High Resolution Prescaler Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrpcr](hrpcr) module"]
pub type HRPCR = crate::Reg<u32, _HRPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRPCR;
#[doc = "`read()` method returns [hrpcr::R](hrpcr::R) reader structure"]
impl crate::Readable for HRPCR {}
#[doc = "`write(|w| ..)` method takes [hrpcr::W](hrpcr::W) writer structure"]
impl crate::Writable for HRPCR {}
#[doc = "High Resolution Prescaler Control Register"]
pub mod hrpcr;
#[doc = "Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Oscillator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscctrl0](oscctrl0) module"]
pub type OSCCTRL0 = crate::Reg<u32, _OSCCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCCTRL0;
#[doc = "`read()` method returns [oscctrl0::R](oscctrl0::R) reader structure"]
impl crate::Readable for OSCCTRL0 {}
#[doc = "`write(|w| ..)` method takes [oscctrl0::W](oscctrl0::W) writer structure"]
impl crate::Writable for OSCCTRL0 {}
#[doc = "Oscillator Control Register"]
pub mod oscctrl0;
#[doc = "Oscillator 0 Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscifaversion](oscifaversion) module"]
pub type OSCIFAVERSION = crate::Reg<u32, _OSCIFAVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCIFAVERSION;
#[doc = "`read()` method returns [oscifaversion::R](oscifaversion::R) reader structure"]
impl crate::Readable for OSCIFAVERSION {}
#[doc = "Oscillator 0 Version Register"]
pub mod oscifaversion;
#[doc = "Power and Clocks Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclksr](pclksr) module"]
pub type PCLKSR = crate::Reg<u32, _PCLKSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCLKSR;
#[doc = "`read()` method returns [pclksr::R](pclksr::R) reader structure"]
impl crate::Readable for PCLKSR {}
#[doc = "Power and Clocks Status Register"]
pub mod pclksr;
#[doc = "PLL Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllifaversion](pllifaversion) module"]
pub type PLLIFAVERSION = crate::Reg<u32, _PLLIFAVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLIFAVERSION;
#[doc = "`read()` method returns [pllifaversion::R](pllifaversion::R) reader structure"]
impl crate::Readable for PLLIFAVERSION {}
#[doc = "PLL Version Register"]
pub mod pllifaversion;
#[doc = "PLL0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll](pll) module"]
pub type PLL = crate::Reg<u32, _PLL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL;
#[doc = "`read()` method returns [pll::R](pll::R) reader structure"]
impl crate::Readable for PLL {}
#[doc = "`write(|w| ..)` method takes [pll::W](pll::W) writer structure"]
impl crate::Writable for PLL {}
#[doc = "PLL0 Control Register"]
pub mod pll;
#[doc = "System RC Oscillator Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rccr](rccr) module"]
pub type RCCR = crate::Reg<u32, _RCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCCR;
#[doc = "`read()` method returns [rccr::R](rccr::R) reader structure"]
impl crate::Readable for RCCR {}
#[doc = "`write(|w| ..)` method takes [rccr::W](rccr::W) writer structure"]
impl crate::Writable for RCCR {}
#[doc = "System RC Oscillator Calibration Register"]
pub mod rccr;
#[doc = "4/8/12 MHz RC Oscillator Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcfastcfg](rcfastcfg) module"]
pub type RCFASTCFG = crate::Reg<u32, _RCFASTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCFASTCFG;
#[doc = "`read()` method returns [rcfastcfg::R](rcfastcfg::R) reader structure"]
impl crate::Readable for RCFASTCFG {}
#[doc = "`write(|w| ..)` method takes [rcfastcfg::W](rcfastcfg::W) writer structure"]
impl crate::Writable for RCFASTCFG {}
#[doc = "4/8/12 MHz RC Oscillator Configuration Register"]
pub mod rcfastcfg;
#[doc = "4/8/12 MHz RC Oscillator Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcfastsr](rcfastsr) module"]
pub type RCFASTSR = crate::Reg<u32, _RCFASTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCFASTSR;
#[doc = "`read()` method returns [rcfastsr::R](rcfastsr::R) reader structure"]
impl crate::Readable for RCFASTSR {}
#[doc = "`write(|w| ..)` method takes [rcfastsr::W](rcfastsr::W) writer structure"]
impl crate::Writable for RCFASTSR {}
#[doc = "4/8/12 MHz RC Oscillator Status Register"]
pub mod rcfastsr;
#[doc = "4/8/12 MHz RC Oscillator Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcfastversion](rcfastversion) module"]
pub type RCFASTVERSION = crate::Reg<u32, _RCFASTVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCFASTVERSION;
#[doc = "`read()` method returns [rcfastversion::R](rcfastversion::R) reader structure"]
impl crate::Readable for RCFASTVERSION {}
#[doc = "4/8/12 MHz RC Oscillator Version Register"]
pub mod rcfastversion;
#[doc = "System RC Oscillator Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcoscifaversion](rcoscifaversion) module"]
pub type RCOSCIFAVERSION = crate::Reg<u32, _RCOSCIFAVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCOSCIFAVERSION;
#[doc = "`read()` method returns [rcoscifaversion::R](rcoscifaversion::R) reader structure"]
impl crate::Readable for RCOSCIFAVERSION {}
#[doc = "System RC Oscillator Version Register"]
pub mod rcoscifaversion;
#[doc = "80 MHz RC Oscillator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc80mcr](rc80mcr) module"]
pub type RC80MCR = crate::Reg<u32, _RC80MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RC80MCR;
#[doc = "`read()` method returns [rc80mcr::R](rc80mcr::R) reader structure"]
impl crate::Readable for RC80MCR {}
#[doc = "`write(|w| ..)` method takes [rc80mcr::W](rc80mcr::W) writer structure"]
impl crate::Writable for RC80MCR {}
#[doc = "80 MHz RC Oscillator Register"]
pub mod rc80mcr;
#[doc = "80MHz RC Oscillator Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc80mversion](rc80mversion) module"]
pub type RC80MVERSION = crate::Reg<u32, _RC80MVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RC80MVERSION;
#[doc = "`read()` method returns [rc80mversion::R](rc80mversion::R) reader structure"]
impl crate::Readable for RC80MVERSION {}
#[doc = "80MHz RC Oscillator Version Register"]
pub mod rc80mversion;
#[doc = "Unlock Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unlock](unlock) module"]
pub type UNLOCK = crate::Reg<u32, _UNLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNLOCK;
#[doc = "`write(|w| ..)` method takes [unlock::W](unlock::W) writer structure"]
impl crate::Writable for UNLOCK {}
#[doc = "Unlock Register"]
pub mod unlock;
#[doc = "SCIF Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "SCIF Version Register"]
pub mod version;
