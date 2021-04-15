#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Clock Waveform Generator Register"]
    pub cwgr: CWGR,
    #[doc = "0x08 - SMBus Timing Register"]
    pub smbtr: SMBTR,
    #[doc = "0x0c - Command Register"]
    pub cmdr: CMDR,
    #[doc = "0x10 - Next Command Register"]
    pub ncmdr: NCMDR,
    #[doc = "0x14 - Receive Holding Register"]
    pub rhr: RHR,
    #[doc = "0x18 - Transmit Holding Register"]
    pub thr: THR,
    #[doc = "0x1c - Status Register"]
    pub sr: SR,
    #[doc = "0x20 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x24 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x28 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x2c - Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x30 - Parameter Register"]
    pub pr: PR,
    #[doc = "0x34 - Version Register"]
    pub vr: VR,
    #[doc = "0x38 - HS-mode Clock Waveform Generator"]
    pub hscwgr: HSCWGR,
    #[doc = "0x3c - Slew Rate Register"]
    pub srr: SRR,
    #[doc = "0x40 - HS-mode Slew Rate Register"]
    pub hssrr: HSSRR,
}
#[doc = "Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdr](cmdr) module"]
pub type CMDR = crate::Reg<u32, _CMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDR;
#[doc = "`read()` method returns [cmdr::R](cmdr::R) reader structure"]
impl crate::Readable for CMDR {}
#[doc = "`write(|w| ..)` method takes [cmdr::W](cmdr::W) writer structure"]
impl crate::Writable for CMDR {}
#[doc = "Command Register"]
pub mod cmdr;
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Clock Waveform Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwgr](cwgr) module"]
pub type CWGR = crate::Reg<u32, _CWGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CWGR;
#[doc = "`read()` method returns [cwgr::R](cwgr::R) reader structure"]
impl crate::Readable for CWGR {}
#[doc = "`write(|w| ..)` method takes [cwgr::W](cwgr::W) writer structure"]
impl crate::Writable for CWGR {}
#[doc = "Clock Waveform Generator Register"]
pub mod cwgr;
#[doc = "HS-mode Clock Waveform Generator\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hscwgr](hscwgr) module"]
pub type HSCWGR = crate::Reg<u32, _HSCWGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCWGR;
#[doc = "`read()` method returns [hscwgr::R](hscwgr::R) reader structure"]
impl crate::Readable for HSCWGR {}
#[doc = "`write(|w| ..)` method takes [hscwgr::W](hscwgr::W) writer structure"]
impl crate::Writable for HSCWGR {}
#[doc = "HS-mode Clock Waveform Generator"]
pub mod hscwgr;
#[doc = "HS-mode Slew Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hssrr](hssrr) module"]
pub type HSSRR = crate::Reg<u32, _HSSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSSRR;
#[doc = "`read()` method returns [hssrr::R](hssrr::R) reader structure"]
impl crate::Readable for HSSRR {}
#[doc = "`write(|w| ..)` method takes [hssrr::W](hssrr::W) writer structure"]
impl crate::Writable for HSSRR {}
#[doc = "HS-mode Slew Rate Register"]
pub mod hssrr;
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
#[doc = "Next Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncmdr](ncmdr) module"]
pub type NCMDR = crate::Reg<u32, _NCMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NCMDR;
#[doc = "`read()` method returns [ncmdr::R](ncmdr::R) reader structure"]
impl crate::Readable for NCMDR {}
#[doc = "`write(|w| ..)` method takes [ncmdr::W](ncmdr::W) writer structure"]
impl crate::Writable for NCMDR {}
#[doc = "Next Command Register"]
pub mod ncmdr;
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr](pr) module"]
pub type PR = crate::Reg<u32, _PR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR;
#[doc = "`read()` method returns [pr::R](pr::R) reader structure"]
impl crate::Readable for PR {}
#[doc = "Parameter Register"]
pub mod pr;
#[doc = "Receive Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rhr](rhr) module"]
pub type RHR = crate::Reg<u32, _RHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RHR;
#[doc = "`read()` method returns [rhr::R](rhr::R) reader structure"]
impl crate::Readable for RHR {}
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "Status Clear Register"]
pub mod scr;
#[doc = "SMBus Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smbtr](smbtr) module"]
pub type SMBTR = crate::Reg<u32, _SMBTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMBTR;
#[doc = "`read()` method returns [smbtr::R](smbtr::R) reader structure"]
impl crate::Readable for SMBTR {}
#[doc = "`write(|w| ..)` method takes [smbtr::W](smbtr::W) writer structure"]
impl crate::Writable for SMBTR {}
#[doc = "SMBus Timing Register"]
pub mod smbtr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Slew Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srr](srr) module"]
pub type SRR = crate::Reg<u32, _SRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRR;
#[doc = "`read()` method returns [srr::R](srr::R) reader structure"]
impl crate::Readable for SRR {}
#[doc = "`write(|w| ..)` method takes [srr::W](srr::W) writer structure"]
impl crate::Writable for SRR {}
#[doc = "Slew Rate Register"]
pub mod srr;
#[doc = "Transmit Holding Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr](thr) module"]
pub type THR = crate::Reg<u32, _THR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THR;
#[doc = "`write(|w| ..)` method takes [thr::W](thr::W) writer structure"]
impl crate::Writable for THR {}
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vr](vr) module"]
pub type VR = crate::Reg<u32, _VR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VR;
#[doc = "`read()` method returns [vr::R](vr::R) reader structure"]
impl crate::Readable for VR {}
#[doc = "Version Register"]
pub mod vr;
