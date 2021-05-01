#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Main Clock Control"]
    pub mcctrl: MCCTRL,
    #[doc = "0x04 - CPU Clock Select"]
    pub cpusel: CPUSEL,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - PBA Clock Select"]
    pub pbasel: PBASEL,
    #[doc = "0x10 - PBB Clock Select"]
    pub pbbsel: PBBSEL,
    #[doc = "0x14 - PBC Clock Select"]
    pub pbcsel: PBCSEL,
    #[doc = "0x18 - PBD Clock Select"]
    pub pbdsel: PBDSEL,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - CPU Mask"]
    pub cpumask: CPUMASK,
    #[doc = "0x24 - HSB Mask"]
    pub hsbmask: HSBMASK,
    #[doc = "0x28 - PBA Mask"]
    pub pbamask: PBAMASK,
    #[doc = "0x2c - PBB Mask"]
    pub pbbmask: PBBMASK,
    #[doc = "0x30 - PBC Mask"]
    pub pbcmask: PBCMASK,
    #[doc = "0x34 - PBD Mask"]
    pub pbdmask: PBDMASK,
    _reserved12: [u8; 8usize],
    #[doc = "0x40 - PBA Divided Clock Mask"]
    pub pbadivmask: PBADIVMASK,
    _reserved13: [u8; 16usize],
    #[doc = "0x54 - Clock Failure Detector Control"]
    pub cfdctrl: CFDCTRL,
    #[doc = "0x58 - Unlock Register"]
    pub unlock: UNLOCK,
    _reserved15: [u8; 100usize],
    #[doc = "0xc0 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0xc4 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0xc8 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0xcc - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0xd0 - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0xd4 - Status Register"]
    pub sr: SR,
    _reserved21: [u8; 136usize],
    #[doc = "0x160 - Peripheral Power Control Register"]
    pub ppcr: PPCR,
    _reserved22: [u8; 28usize],
    #[doc = "0x180 - Reset Cause Register"]
    pub rcause: RCAUSE,
    #[doc = "0x184 - Wake Cause Register"]
    pub wcause: WCAUSE,
    #[doc = "0x188 - Asynchronous Wake Enable"]
    pub awen: AWEN,
    _reserved25: [u8; 4usize],
    #[doc = "0x190 - Obsvervability"]
    pub obs: OBS,
    #[doc = "0x194 - Fast Sleep Register"]
    pub fastsleep: FASTSLEEP,
    _reserved27: [u8; 608usize],
    #[doc = "0x3f8 - Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x3fc - Version Register"]
    pub version: VERSION,
}
#[doc = "Asynchronous Wake Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awen](awen) module"]
pub type AWEN = crate::Reg<u32, _AWEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWEN;
#[doc = "`read()` method returns [awen::R](awen::R) reader structure"]
impl crate::Readable for AWEN {}
#[doc = "`write(|w| ..)` method takes [awen::W](awen::W) writer structure"]
impl crate::Writable for AWEN {}
#[doc = "Asynchronous Wake Enable"]
pub mod awen;
#[doc = "Clock Failure Detector Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdctrl](cfdctrl) module"]
pub type CFDCTRL = crate::Reg<u32, _CFDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFDCTRL;
#[doc = "`read()` method returns [cfdctrl::R](cfdctrl::R) reader structure"]
impl crate::Readable for CFDCTRL {}
#[doc = "`write(|w| ..)` method takes [cfdctrl::W](cfdctrl::W) writer structure"]
impl crate::Writable for CFDCTRL {}
#[doc = "Clock Failure Detector Control"]
pub mod cfdctrl;
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "Configuration Register"]
pub mod config;
#[doc = "CPU Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpumask](cpumask) module"]
pub type CPUMASK = crate::Reg<u32, _CPUMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUMASK;
#[doc = "`read()` method returns [cpumask::R](cpumask::R) reader structure"]
impl crate::Readable for CPUMASK {}
#[doc = "`write(|w| ..)` method takes [cpumask::W](cpumask::W) writer structure"]
impl crate::Writable for CPUMASK {}
#[doc = "CPU Mask"]
pub mod cpumask;
#[doc = "CPU Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpusel](cpusel) module"]
pub type CPUSEL = crate::Reg<u32, _CPUSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUSEL;
#[doc = "`read()` method returns [cpusel::R](cpusel::R) reader structure"]
impl crate::Readable for CPUSEL {}
#[doc = "`write(|w| ..)` method takes [cpusel::W](cpusel::W) writer structure"]
impl crate::Writable for CPUSEL {}
#[doc = "CPU Clock Select"]
pub mod cpusel;
#[doc = "Fast Sleep Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fastsleep](fastsleep) module"]
pub type FASTSLEEP = crate::Reg<u32, _FASTSLEEP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FASTSLEEP;
#[doc = "`read()` method returns [fastsleep::R](fastsleep::R) reader structure"]
impl crate::Readable for FASTSLEEP {}
#[doc = "`write(|w| ..)` method takes [fastsleep::W](fastsleep::W) writer structure"]
impl crate::Writable for FASTSLEEP {}
#[doc = "Fast Sleep Register"]
pub mod fastsleep;
#[doc = "HSB Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsbmask](hsbmask) module"]
pub type HSBMASK = crate::Reg<u32, _HSBMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSBMASK;
#[doc = "`read()` method returns [hsbmask::R](hsbmask::R) reader structure"]
impl crate::Readable for HSBMASK {}
#[doc = "`write(|w| ..)` method takes [hsbmask::W](hsbmask::W) writer structure"]
impl crate::Writable for HSBMASK {}
#[doc = "HSB Mask"]
pub mod hsbmask;
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
#[doc = "Main Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcctrl](mcctrl) module"]
pub type MCCTRL = crate::Reg<u32, _MCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCCTRL;
#[doc = "`read()` method returns [mcctrl::R](mcctrl::R) reader structure"]
impl crate::Readable for MCCTRL {}
#[doc = "`write(|w| ..)` method takes [mcctrl::W](mcctrl::W) writer structure"]
impl crate::Writable for MCCTRL {}
#[doc = "Main Clock Control"]
pub mod mcctrl;
#[doc = "Obsvervability\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obs](obs) module"]
pub type OBS = crate::Reg<u32, _OBS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBS;
#[doc = "`read()` method returns [obs::R](obs::R) reader structure"]
impl crate::Readable for OBS {}
#[doc = "`write(|w| ..)` method takes [obs::W](obs::W) writer structure"]
impl crate::Writable for OBS {}
#[doc = "Obsvervability"]
pub mod obs;
#[doc = "PBA Divided Clock Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbadivmask](pbadivmask) module"]
pub type PBADIVMASK = crate::Reg<u32, _PBADIVMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBADIVMASK;
#[doc = "`read()` method returns [pbadivmask::R](pbadivmask::R) reader structure"]
impl crate::Readable for PBADIVMASK {}
#[doc = "`write(|w| ..)` method takes [pbadivmask::W](pbadivmask::W) writer structure"]
impl crate::Writable for PBADIVMASK {}
#[doc = "PBA Divided Clock Mask"]
pub mod pbadivmask;
#[doc = "PBA Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbamask](pbamask) module"]
pub type PBAMASK = crate::Reg<u32, _PBAMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBAMASK;
#[doc = "`read()` method returns [pbamask::R](pbamask::R) reader structure"]
impl crate::Readable for PBAMASK {}
#[doc = "`write(|w| ..)` method takes [pbamask::W](pbamask::W) writer structure"]
impl crate::Writable for PBAMASK {}
#[doc = "PBA Mask"]
pub mod pbamask;
#[doc = "PBA Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbasel](pbasel) module"]
pub type PBASEL = crate::Reg<u32, _PBASEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBASEL;
#[doc = "`read()` method returns [pbasel::R](pbasel::R) reader structure"]
impl crate::Readable for PBASEL {}
#[doc = "`write(|w| ..)` method takes [pbasel::W](pbasel::W) writer structure"]
impl crate::Writable for PBASEL {}
#[doc = "PBA Clock Select"]
pub mod pbasel;
#[doc = "PBB Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbbmask](pbbmask) module"]
pub type PBBMASK = crate::Reg<u32, _PBBMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBBMASK;
#[doc = "`read()` method returns [pbbmask::R](pbbmask::R) reader structure"]
impl crate::Readable for PBBMASK {}
#[doc = "`write(|w| ..)` method takes [pbbmask::W](pbbmask::W) writer structure"]
impl crate::Writable for PBBMASK {}
#[doc = "PBB Mask"]
pub mod pbbmask;
#[doc = "PBB Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbbsel](pbbsel) module"]
pub type PBBSEL = crate::Reg<u32, _PBBSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBBSEL;
#[doc = "`read()` method returns [pbbsel::R](pbbsel::R) reader structure"]
impl crate::Readable for PBBSEL {}
#[doc = "`write(|w| ..)` method takes [pbbsel::W](pbbsel::W) writer structure"]
impl crate::Writable for PBBSEL {}
#[doc = "PBB Clock Select"]
pub mod pbbsel;
#[doc = "PBC Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbcmask](pbcmask) module"]
pub type PBCMASK = crate::Reg<u32, _PBCMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBCMASK;
#[doc = "`read()` method returns [pbcmask::R](pbcmask::R) reader structure"]
impl crate::Readable for PBCMASK {}
#[doc = "`write(|w| ..)` method takes [pbcmask::W](pbcmask::W) writer structure"]
impl crate::Writable for PBCMASK {}
#[doc = "PBC Mask"]
pub mod pbcmask;
#[doc = "PBC Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbcsel](pbcsel) module"]
pub type PBCSEL = crate::Reg<u32, _PBCSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBCSEL;
#[doc = "`read()` method returns [pbcsel::R](pbcsel::R) reader structure"]
impl crate::Readable for PBCSEL {}
#[doc = "`write(|w| ..)` method takes [pbcsel::W](pbcsel::W) writer structure"]
impl crate::Writable for PBCSEL {}
#[doc = "PBC Clock Select"]
pub mod pbcsel;
#[doc = "PBD Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbdmask](pbdmask) module"]
pub type PBDMASK = crate::Reg<u32, _PBDMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBDMASK;
#[doc = "`read()` method returns [pbdmask::R](pbdmask::R) reader structure"]
impl crate::Readable for PBDMASK {}
#[doc = "`write(|w| ..)` method takes [pbdmask::W](pbdmask::W) writer structure"]
impl crate::Writable for PBDMASK {}
#[doc = "PBD Mask"]
pub mod pbdmask;
#[doc = "PBD Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbdsel](pbdsel) module"]
pub type PBDSEL = crate::Reg<u32, _PBDSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBDSEL;
#[doc = "`read()` method returns [pbdsel::R](pbdsel::R) reader structure"]
impl crate::Readable for PBDSEL {}
#[doc = "`write(|w| ..)` method takes [pbdsel::W](pbdsel::W) writer structure"]
impl crate::Writable for PBDSEL {}
#[doc = "PBD Clock Select"]
pub mod pbdsel;
#[doc = "Peripheral Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppcr](ppcr) module"]
pub type PPCR = crate::Reg<u32, _PPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPCR;
#[doc = "`read()` method returns [ppcr::R](ppcr::R) reader structure"]
impl crate::Readable for PPCR {}
#[doc = "`write(|w| ..)` method takes [ppcr::W](ppcr::W) writer structure"]
impl crate::Writable for PPCR {}
#[doc = "Peripheral Power Control Register"]
pub mod ppcr;
#[doc = "Reset Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcause](rcause) module"]
pub type RCAUSE = crate::Reg<u32, _RCAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCAUSE;
#[doc = "`read()` method returns [rcause::R](rcause::R) reader structure"]
impl crate::Readable for RCAUSE {}
#[doc = "Reset Cause Register"]
pub mod rcause;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Unlock Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unlock](unlock) module"]
pub type UNLOCK = crate::Reg<u32, _UNLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNLOCK;
#[doc = "`write(|w| ..)` method takes [unlock::W](unlock::W) writer structure"]
impl crate::Writable for UNLOCK {}
#[doc = "Unlock Register"]
pub mod unlock;
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "Version Register"]
pub mod version;
#[doc = "Wake Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcause](wcause) module"]
pub type WCAUSE = crate::Reg<u32, _WCAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WCAUSE;
#[doc = "`read()` method returns [wcause::R](wcause::R) reader structure"]
impl crate::Readable for WCAUSE {}
#[doc = "Wake Cause Register"]
pub mod wcause;
