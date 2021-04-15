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
    #[doc = "0x20 - Oscillator 32 Control Register"]
    pub oscctrl32: OSCCTRL32,
    #[doc = "0x24 - 32 kHz RC Oscillator Control Register"]
    pub rc32kcr: RC32KCR,
    #[doc = "0x28 - 32kHz RC Oscillator Tuning Register"]
    pub rc32ktune: RC32KTUNE,
    #[doc = "0x2c - BOD33 Control Register"]
    pub bod33ctrl: BOD33CTRL,
    #[doc = "0x30 - BOD33 Level Register"]
    pub bod33level: BOD33LEVEL,
    #[doc = "0x34 - BOD33 Sampling Control Register"]
    pub bod33sampling: BOD33SAMPLING,
    #[doc = "0x38 - BOD18 Control Register"]
    pub bod18ctrl: BOD18CTRL,
    #[doc = "0x3c - BOD18 Level Register"]
    pub bod18level: BOD18LEVEL,
    _reserved16: [u8; 4usize],
    #[doc = "0x44 - Voltage Regulator Configuration Register"]
    pub vregcr: VREGCR,
    _reserved17: [u8; 4usize],
    #[doc = "0x4c - Normal Mode Control and Status Register"]
    pub vregncsr: VREGNCSR,
    #[doc = "0x50 - LP Mode Control and Status Register"]
    pub vreglpcsr: VREGLPCSR,
    _reserved19: [u8; 4usize],
    #[doc = "0x58 - 1MHz RC Clock Configuration Register"]
    pub rc1mcr: RC1MCR,
    #[doc = "0x5c - Bandgap Calibration Register"]
    pub bgcr: BGCR,
    #[doc = "0x60 - Bandgap Control Register"]
    pub bgctrl: BGCTRL,
    #[doc = "0x64 - Bandgap Status Register"]
    pub bgsr: BGSR,
    _reserved23: [u8; 16usize],
    #[doc = "0x78 - Backup Register"]
    pub br: [BR; 4],
    _reserved24: [u8; 860usize],
    #[doc = "0x3e4 - Backup Register Interface Version Register"]
    pub brifbversion: BRIFBVERSION,
    #[doc = "0x3e8 - BGREFIFB Version Register"]
    pub bgrefifbversion: BGREFIFBVERSION,
    #[doc = "0x3ec - VREGIFA Version Register"]
    pub vregifgversion: VREGIFGVERSION,
    #[doc = "0x3f0 - BODIFC Version Register"]
    pub bodifcversion: BODIFCVERSION,
    #[doc = "0x3f4 - 32 kHz RC Oscillator Version Register"]
    pub rc32kifbversion: RC32KIFBVERSION,
    #[doc = "0x3f8 - 32 KHz Oscillator Version Register"]
    pub osc32ifaversion: OSC32IFAVERSION,
    #[doc = "0x3fc - BSCIF Version Register"]
    pub version: VERSION,
}
#[doc = "Bandgap Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgcr](bgcr) module"]
pub type BGCR = crate::Reg<u32, _BGCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGCR;
#[doc = "`read()` method returns [bgcr::R](bgcr::R) reader structure"]
impl crate::Readable for BGCR {}
#[doc = "`write(|w| ..)` method takes [bgcr::W](bgcr::W) writer structure"]
impl crate::Writable for BGCR {}
#[doc = "Bandgap Calibration Register"]
pub mod bgcr;
#[doc = "Bandgap Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgctrl](bgctrl) module"]
pub type BGCTRL = crate::Reg<u32, _BGCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGCTRL;
#[doc = "`read()` method returns [bgctrl::R](bgctrl::R) reader structure"]
impl crate::Readable for BGCTRL {}
#[doc = "`write(|w| ..)` method takes [bgctrl::W](bgctrl::W) writer structure"]
impl crate::Writable for BGCTRL {}
#[doc = "Bandgap Control Register"]
pub mod bgctrl;
#[doc = "BGREFIFB Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgrefifbversion](bgrefifbversion) module"]
pub type BGREFIFBVERSION = crate::Reg<u32, _BGREFIFBVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGREFIFBVERSION;
#[doc = "`read()` method returns [bgrefifbversion::R](bgrefifbversion::R) reader structure"]
impl crate::Readable for BGREFIFBVERSION {}
#[doc = "BGREFIFB Version Register"]
pub mod bgrefifbversion;
#[doc = "Bandgap Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgsr](bgsr) module"]
pub type BGSR = crate::Reg<u32, _BGSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGSR;
#[doc = "`read()` method returns [bgsr::R](bgsr::R) reader structure"]
impl crate::Readable for BGSR {}
#[doc = "Bandgap Status Register"]
pub mod bgsr;
#[doc = "BODIFC Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodifcversion](bodifcversion) module"]
pub type BODIFCVERSION = crate::Reg<u32, _BODIFCVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BODIFCVERSION;
#[doc = "`read()` method returns [bodifcversion::R](bodifcversion::R) reader structure"]
impl crate::Readable for BODIFCVERSION {}
#[doc = "BODIFC Version Register"]
pub mod bodifcversion;
#[doc = "BOD18 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod18ctrl](bod18ctrl) module"]
pub type BOD18CTRL = crate::Reg<u32, _BOD18CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOD18CTRL;
#[doc = "`read()` method returns [bod18ctrl::R](bod18ctrl::R) reader structure"]
impl crate::Readable for BOD18CTRL {}
#[doc = "`write(|w| ..)` method takes [bod18ctrl::W](bod18ctrl::W) writer structure"]
impl crate::Writable for BOD18CTRL {}
#[doc = "BOD18 Control Register"]
pub mod bod18ctrl;
#[doc = "BOD18 Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod18level](bod18level) module"]
pub type BOD18LEVEL = crate::Reg<u32, _BOD18LEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOD18LEVEL;
#[doc = "`read()` method returns [bod18level::R](bod18level::R) reader structure"]
impl crate::Readable for BOD18LEVEL {}
#[doc = "`write(|w| ..)` method takes [bod18level::W](bod18level::W) writer structure"]
impl crate::Writable for BOD18LEVEL {}
#[doc = "BOD18 Level Register"]
pub mod bod18level;
#[doc = "BOD33 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod33ctrl](bod33ctrl) module"]
pub type BOD33CTRL = crate::Reg<u32, _BOD33CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOD33CTRL;
#[doc = "`read()` method returns [bod33ctrl::R](bod33ctrl::R) reader structure"]
impl crate::Readable for BOD33CTRL {}
#[doc = "`write(|w| ..)` method takes [bod33ctrl::W](bod33ctrl::W) writer structure"]
impl crate::Writable for BOD33CTRL {}
#[doc = "BOD33 Control Register"]
pub mod bod33ctrl;
#[doc = "BOD33 Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod33level](bod33level) module"]
pub type BOD33LEVEL = crate::Reg<u32, _BOD33LEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOD33LEVEL;
#[doc = "`read()` method returns [bod33level::R](bod33level::R) reader structure"]
impl crate::Readable for BOD33LEVEL {}
#[doc = "`write(|w| ..)` method takes [bod33level::W](bod33level::W) writer structure"]
impl crate::Writable for BOD33LEVEL {}
#[doc = "BOD33 Level Register"]
pub mod bod33level;
#[doc = "BOD33 Sampling Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod33sampling](bod33sampling) module"]
pub type BOD33SAMPLING = crate::Reg<u32, _BOD33SAMPLING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOD33SAMPLING;
#[doc = "`read()` method returns [bod33sampling::R](bod33sampling::R) reader structure"]
impl crate::Readable for BOD33SAMPLING {}
#[doc = "`write(|w| ..)` method takes [bod33sampling::W](bod33sampling::W) writer structure"]
impl crate::Writable for BOD33SAMPLING {}
#[doc = "BOD33 Sampling Control Register"]
pub mod bod33sampling;
#[doc = "Backup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [br](br) module"]
pub type BR = crate::Reg<u32, _BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BR;
#[doc = "`read()` method returns [br::R](br::R) reader structure"]
impl crate::Readable for BR {}
#[doc = "`write(|w| ..)` method takes [br::W](br::W) writer structure"]
impl crate::Writable for BR {}
#[doc = "Backup Register"]
pub mod br;
#[doc = "Backup Register Interface Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brifbversion](brifbversion) module"]
pub type BRIFBVERSION = crate::Reg<u32, _BRIFBVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRIFBVERSION;
#[doc = "`read()` method returns [brifbversion::R](brifbversion::R) reader structure"]
impl crate::Readable for BRIFBVERSION {}
#[doc = "Backup Register Interface Version Register"]
pub mod brifbversion;
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
#[doc = "Oscillator 32 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscctrl32](oscctrl32) module"]
pub type OSCCTRL32 = crate::Reg<u32, _OSCCTRL32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCCTRL32;
#[doc = "`read()` method returns [oscctrl32::R](oscctrl32::R) reader structure"]
impl crate::Readable for OSCCTRL32 {}
#[doc = "`write(|w| ..)` method takes [oscctrl32::W](oscctrl32::W) writer structure"]
impl crate::Writable for OSCCTRL32 {}
#[doc = "Oscillator 32 Control Register"]
pub mod oscctrl32;
#[doc = "32 KHz Oscillator Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc32ifaversion](osc32ifaversion) module"]
pub type OSC32IFAVERSION = crate::Reg<u32, _OSC32IFAVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC32IFAVERSION;
#[doc = "`read()` method returns [osc32ifaversion::R](osc32ifaversion::R) reader structure"]
impl crate::Readable for OSC32IFAVERSION {}
#[doc = "32 KHz Oscillator Version Register"]
pub mod osc32ifaversion;
#[doc = "Power and Clocks Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclksr](pclksr) module"]
pub type PCLKSR = crate::Reg<u32, _PCLKSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCLKSR;
#[doc = "`read()` method returns [pclksr::R](pclksr::R) reader structure"]
impl crate::Readable for PCLKSR {}
#[doc = "Power and Clocks Status Register"]
pub mod pclksr;
#[doc = "1MHz RC Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc1mcr](rc1mcr) module"]
pub type RC1MCR = crate::Reg<u32, _RC1MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RC1MCR;
#[doc = "`read()` method returns [rc1mcr::R](rc1mcr::R) reader structure"]
impl crate::Readable for RC1MCR {}
#[doc = "`write(|w| ..)` method takes [rc1mcr::W](rc1mcr::W) writer structure"]
impl crate::Writable for RC1MCR {}
#[doc = "1MHz RC Clock Configuration Register"]
pub mod rc1mcr;
#[doc = "32 kHz RC Oscillator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc32kcr](rc32kcr) module"]
pub type RC32KCR = crate::Reg<u32, _RC32KCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RC32KCR;
#[doc = "`read()` method returns [rc32kcr::R](rc32kcr::R) reader structure"]
impl crate::Readable for RC32KCR {}
#[doc = "`write(|w| ..)` method takes [rc32kcr::W](rc32kcr::W) writer structure"]
impl crate::Writable for RC32KCR {}
#[doc = "32 kHz RC Oscillator Control Register"]
pub mod rc32kcr;
#[doc = "32 kHz RC Oscillator Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc32kifbversion](rc32kifbversion) module"]
pub type RC32KIFBVERSION = crate::Reg<u32, _RC32KIFBVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RC32KIFBVERSION;
#[doc = "`read()` method returns [rc32kifbversion::R](rc32kifbversion::R) reader structure"]
impl crate::Readable for RC32KIFBVERSION {}
#[doc = "32 kHz RC Oscillator Version Register"]
pub mod rc32kifbversion;
#[doc = "32kHz RC Oscillator Tuning Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc32ktune](rc32ktune) module"]
pub type RC32KTUNE = crate::Reg<u32, _RC32KTUNE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RC32KTUNE;
#[doc = "`read()` method returns [rc32ktune::R](rc32ktune::R) reader structure"]
impl crate::Readable for RC32KTUNE {}
#[doc = "`write(|w| ..)` method takes [rc32ktune::W](rc32ktune::W) writer structure"]
impl crate::Writable for RC32KTUNE {}
#[doc = "32kHz RC Oscillator Tuning Register"]
pub mod rc32ktune;
#[doc = "Unlock Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unlock](unlock) module"]
pub type UNLOCK = crate::Reg<u32, _UNLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNLOCK;
#[doc = "`write(|w| ..)` method takes [unlock::W](unlock::W) writer structure"]
impl crate::Writable for UNLOCK {}
#[doc = "Unlock Register"]
pub mod unlock;
#[doc = "BSCIF Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "BSCIF Version Register"]
pub mod version;
#[doc = "Voltage Regulator Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vregcr](vregcr) module"]
pub type VREGCR = crate::Reg<u32, _VREGCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREGCR;
#[doc = "`read()` method returns [vregcr::R](vregcr::R) reader structure"]
impl crate::Readable for VREGCR {}
#[doc = "`write(|w| ..)` method takes [vregcr::W](vregcr::W) writer structure"]
impl crate::Writable for VREGCR {}
#[doc = "Voltage Regulator Configuration Register"]
pub mod vregcr;
#[doc = "VREGIFA Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vregifgversion](vregifgversion) module"]
pub type VREGIFGVERSION = crate::Reg<u32, _VREGIFGVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREGIFGVERSION;
#[doc = "`read()` method returns [vregifgversion::R](vregifgversion::R) reader structure"]
impl crate::Readable for VREGIFGVERSION {}
#[doc = "VREGIFA Version Register"]
pub mod vregifgversion;
#[doc = "LP Mode Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vreglpcsr](vreglpcsr) module"]
pub type VREGLPCSR = crate::Reg<u32, _VREGLPCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREGLPCSR;
#[doc = "`read()` method returns [vreglpcsr::R](vreglpcsr::R) reader structure"]
impl crate::Readable for VREGLPCSR {}
#[doc = "`write(|w| ..)` method takes [vreglpcsr::W](vreglpcsr::W) writer structure"]
impl crate::Writable for VREGLPCSR {}
#[doc = "LP Mode Control and Status Register"]
pub mod vreglpcsr;
#[doc = "Normal Mode Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vregncsr](vregncsr) module"]
pub type VREGNCSR = crate::Reg<u32, _VREGNCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREGNCSR;
#[doc = "`read()` method returns [vregncsr::R](vregncsr::R) reader structure"]
impl crate::Readable for VREGNCSR {}
#[doc = "`write(|w| ..)` method takes [vregncsr::W](vregncsr::W) writer structure"]
impl crate::Writable for VREGNCSR {}
#[doc = "Normal Mode Control and Status Register"]
pub mod vregncsr;
