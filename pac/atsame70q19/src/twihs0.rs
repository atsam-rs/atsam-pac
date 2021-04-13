#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Master Mode Register"]
    pub mmr: MMR,
    #[doc = "0x08 - Slave Mode Register"]
    pub smr: SMR,
    #[doc = "0x0c - Internal Address Register"]
    pub iadr: IADR,
    #[doc = "0x10 - Clock Waveform Generator Register"]
    pub cwgr: CWGR,
    _reserved5: [u8; 12usize],
    #[doc = "0x20 - Status Register"]
    pub sr: SR,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x30 - Receive Holding Register"]
    pub rhr: RHR,
    #[doc = "0x34 - Transmit Holding Register"]
    pub thr: THR,
    #[doc = "0x38 - SMBus Timing Register"]
    pub smbtr: SMBTR,
    _reserved12: [u8; 8usize],
    #[doc = "0x44 - Filter Register"]
    pub filtr: FILTR,
    _reserved13: [u8; 4usize],
    #[doc = "0x4c - SleepWalking Matching Register"]
    pub swmr: SWMR,
    _reserved14: [u8; 148usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: WPSR,
}
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Master Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmr](mmr) module"]
pub type MMR = crate::Reg<u32, _MMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMR;
#[doc = "`read()` method returns [mmr::R](mmr::R) reader structure"]
impl crate::Readable for MMR {}
#[doc = "`write(|w| ..)` method takes [mmr::W](mmr::W) writer structure"]
impl crate::Writable for MMR {}
#[doc = "Master Mode Register"]
pub mod mmr;
#[doc = "Slave Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smr](smr) module"]
pub type SMR = crate::Reg<u32, _SMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMR;
#[doc = "`read()` method returns [smr::R](smr::R) reader structure"]
impl crate::Readable for SMR {}
#[doc = "`write(|w| ..)` method takes [smr::W](smr::W) writer structure"]
impl crate::Writable for SMR {}
#[doc = "Slave Mode Register"]
pub mod smr;
#[doc = "Internal Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iadr](iadr) module"]
pub type IADR = crate::Reg<u32, _IADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IADR;
#[doc = "`read()` method returns [iadr::R](iadr::R) reader structure"]
impl crate::Readable for IADR {}
#[doc = "`write(|w| ..)` method takes [iadr::W](iadr::W) writer structure"]
impl crate::Writable for IADR {}
#[doc = "Internal Address Register"]
pub mod iadr;
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
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Receive Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rhr](rhr) module"]
pub type RHR = crate::Reg<u32, _RHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RHR;
#[doc = "`read()` method returns [rhr::R](rhr::R) reader structure"]
impl crate::Readable for RHR {}
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "Transmit Holding Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr](thr) module"]
pub type THR = crate::Reg<u32, _THR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THR;
#[doc = "`write(|w| ..)` method takes [thr::W](thr::W) writer structure"]
impl crate::Writable for THR {}
#[doc = "Transmit Holding Register"]
pub mod thr;
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
#[doc = "Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filtr](filtr) module"]
pub type FILTR = crate::Reg<u32, _FILTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILTR;
#[doc = "`read()` method returns [filtr::R](filtr::R) reader structure"]
impl crate::Readable for FILTR {}
#[doc = "`write(|w| ..)` method takes [filtr::W](filtr::W) writer structure"]
impl crate::Writable for FILTR {}
#[doc = "Filter Register"]
pub mod filtr;
#[doc = "SleepWalking Matching Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swmr](swmr) module"]
pub type SWMR = crate::Reg<u32, _SWMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWMR;
#[doc = "`read()` method returns [swmr::R](swmr::R) reader structure"]
impl crate::Readable for SWMR {}
#[doc = "`write(|w| ..)` method takes [swmr::W](swmr::W) writer structure"]
impl crate::Writable for SWMR {}
#[doc = "SleepWalking Matching Register"]
pub mod swmr;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpmr](wpmr) module"]
pub type WPMR = crate::Reg<u32, _WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPMR;
#[doc = "`read()` method returns [wpmr::R](wpmr::R) reader structure"]
impl crate::Readable for WPMR {}
#[doc = "`write(|w| ..)` method takes [wpmr::W](wpmr::W) writer structure"]
impl crate::Writable for WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](wpsr) module"]
pub type WPSR = crate::Reg<u32, _WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSR;
#[doc = "`read()` method returns [wpsr::R](wpsr::R) reader structure"]
impl crate::Readable for WPSR {}
#[doc = "Write Protection Status Register"]
pub mod wpsr;
