#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Clock Enable Register"]
    pub scer: SCER,
    #[doc = "0x04 - System Clock Disable Register"]
    pub scdr: SCDR,
    #[doc = "0x08 - System Clock Status Register"]
    pub scsr: SCSR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Peripheral Clock Enable Register 0"]
    pub pcer0: PCER0,
    #[doc = "0x14 - Peripheral Clock Disable Register 0"]
    pub pcdr0: PCDR0,
    #[doc = "0x18 - Peripheral Clock Status Register 0"]
    pub pcsr0: PCSR0,
    #[doc = "0x1c - UTMI Clock Register"]
    pub ckgr_uckr: CKGR_UCKR,
    #[doc = "0x20 - Main Oscillator Register"]
    pub ckgr_mor: CKGR_MOR,
    #[doc = "0x24 - Main Clock Frequency Register"]
    pub ckgr_mcfr: CKGR_MCFR,
    #[doc = "0x28 - PLLA Register"]
    pub ckgr_pllar: CKGR_PLLAR,
    _reserved10: [u8; 4usize],
    #[doc = "0x30 - Master Clock Register"]
    pub mckr: MCKR,
    _reserved11: [u8; 4usize],
    #[doc = "0x38 - USB Clock Register"]
    pub usb: USB,
    _reserved12: [u8; 4usize],
    #[doc = "0x40 - Programmable Clock Register"]
    pub pck: [PCK; 8],
    #[doc = "0x60 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x64 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x68 - Status Register"]
    pub sr: SR,
    #[doc = "0x6c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x70 - Fast Startup Mode Register"]
    pub fsmr: FSMR,
    #[doc = "0x74 - Fast Startup Polarity Register"]
    pub fspr: FSPR,
    #[doc = "0x78 - Fault Output Clear Register"]
    pub focr: FOCR,
    _reserved20: [u8; 104usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: WPSR,
    _reserved22: [u8; 20usize],
    #[doc = "0x100 - Peripheral Clock Enable Register 1"]
    pub pcer1: PCER1,
    #[doc = "0x104 - Peripheral Clock Disable Register 1"]
    pub pcdr1: PCDR1,
    #[doc = "0x108 - Peripheral Clock Status Register 1"]
    pub pcsr1: PCSR1,
    #[doc = "0x10c - Peripheral Control Register"]
    pub pcr: PCR,
    #[doc = "0x110 - Oscillator Calibration Register"]
    pub ocr: OCR,
    #[doc = "0x114 - SleepWalking Enable Register 0"]
    pub slpwk_er0: SLPWK_ER0,
    #[doc = "0x118 - SleepWalking Disable Register 0"]
    pub slpwk_dr0: SLPWK_DR0,
    #[doc = "0x11c - SleepWalking Status Register 0"]
    pub slpwk_sr0: SLPWK_SR0,
    #[doc = "0x120 - SleepWalking Activity Status Register 0"]
    pub slpwk_asr0: SLPWK_ASR0,
    _reserved31: [u8; 12usize],
    #[doc = "0x130 - PLL Maximum Multiplier Value Register"]
    pub pmmr: PMMR,
    #[doc = "0x134 - SleepWalking Enable Register 1"]
    pub slpwk_er1: SLPWK_ER1,
    #[doc = "0x138 - SleepWalking Disable Register 1"]
    pub slpwk_dr1: SLPWK_DR1,
    #[doc = "0x13c - SleepWalking Status Register 1"]
    pub slpwk_sr1: SLPWK_SR1,
    #[doc = "0x140 - SleepWalking Activity Status Register 1"]
    pub slpwk_asr1: SLPWK_ASR1,
    #[doc = "0x144 - SleepWalking Activity In Progress Register"]
    pub slpwk_aipr: SLPWK_AIPR,
}
#[doc = "System Clock Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scer](scer) module"]
pub type SCER = crate::Reg<u32, _SCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCER;
#[doc = "`write(|w| ..)` method takes [scer::W](scer::W) writer structure"]
impl crate::Writable for SCER {}
#[doc = "System Clock Enable Register"]
pub mod scer;
#[doc = "System Clock Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scdr](scdr) module"]
pub type SCDR = crate::Reg<u32, _SCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCDR;
#[doc = "`write(|w| ..)` method takes [scdr::W](scdr::W) writer structure"]
impl crate::Writable for SCDR {}
#[doc = "System Clock Disable Register"]
pub mod scdr;
#[doc = "System Clock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scsr](scsr) module"]
pub type SCSR = crate::Reg<u32, _SCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCSR;
#[doc = "`read()` method returns [scsr::R](scsr::R) reader structure"]
impl crate::Readable for SCSR {}
#[doc = "System Clock Status Register"]
pub mod scsr;
#[doc = "Peripheral Clock Enable Register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcer0](pcer0) module"]
pub type PCER0 = crate::Reg<u32, _PCER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCER0;
#[doc = "`write(|w| ..)` method takes [pcer0::W](pcer0::W) writer structure"]
impl crate::Writable for PCER0 {}
#[doc = "Peripheral Clock Enable Register 0"]
pub mod pcer0;
#[doc = "Peripheral Clock Disable Register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcdr0](pcdr0) module"]
pub type PCDR0 = crate::Reg<u32, _PCDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCDR0;
#[doc = "`write(|w| ..)` method takes [pcdr0::W](pcdr0::W) writer structure"]
impl crate::Writable for PCDR0 {}
#[doc = "Peripheral Clock Disable Register 0"]
pub mod pcdr0;
#[doc = "Peripheral Clock Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsr0](pcsr0) module"]
pub type PCSR0 = crate::Reg<u32, _PCSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCSR0;
#[doc = "`read()` method returns [pcsr0::R](pcsr0::R) reader structure"]
impl crate::Readable for PCSR0 {}
#[doc = "Peripheral Clock Status Register 0"]
pub mod pcsr0;
#[doc = "UTMI Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_uckr](ckgr_uckr) module"]
pub type CKGR_UCKR = crate::Reg<u32, _CKGR_UCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKGR_UCKR;
#[doc = "`read()` method returns [ckgr_uckr::R](ckgr_uckr::R) reader structure"]
impl crate::Readable for CKGR_UCKR {}
#[doc = "`write(|w| ..)` method takes [ckgr_uckr::W](ckgr_uckr::W) writer structure"]
impl crate::Writable for CKGR_UCKR {}
#[doc = "UTMI Clock Register"]
pub mod ckgr_uckr;
#[doc = "Main Oscillator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_mor](ckgr_mor) module"]
pub type CKGR_MOR = crate::Reg<u32, _CKGR_MOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKGR_MOR;
#[doc = "`read()` method returns [ckgr_mor::R](ckgr_mor::R) reader structure"]
impl crate::Readable for CKGR_MOR {}
#[doc = "`write(|w| ..)` method takes [ckgr_mor::W](ckgr_mor::W) writer structure"]
impl crate::Writable for CKGR_MOR {}
#[doc = "Main Oscillator Register"]
pub mod ckgr_mor;
#[doc = "Main Clock Frequency Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_mcfr](ckgr_mcfr) module"]
pub type CKGR_MCFR = crate::Reg<u32, _CKGR_MCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKGR_MCFR;
#[doc = "`read()` method returns [ckgr_mcfr::R](ckgr_mcfr::R) reader structure"]
impl crate::Readable for CKGR_MCFR {}
#[doc = "`write(|w| ..)` method takes [ckgr_mcfr::W](ckgr_mcfr::W) writer structure"]
impl crate::Writable for CKGR_MCFR {}
#[doc = "Main Clock Frequency Register"]
pub mod ckgr_mcfr;
#[doc = "PLLA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_pllar](ckgr_pllar) module"]
pub type CKGR_PLLAR = crate::Reg<u32, _CKGR_PLLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKGR_PLLAR;
#[doc = "`read()` method returns [ckgr_pllar::R](ckgr_pllar::R) reader structure"]
impl crate::Readable for CKGR_PLLAR {}
#[doc = "`write(|w| ..)` method takes [ckgr_pllar::W](ckgr_pllar::W) writer structure"]
impl crate::Writable for CKGR_PLLAR {}
#[doc = "PLLA Register"]
pub mod ckgr_pllar;
#[doc = "Master Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mckr](mckr) module"]
pub type MCKR = crate::Reg<u32, _MCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCKR;
#[doc = "`read()` method returns [mckr::R](mckr::R) reader structure"]
impl crate::Readable for MCKR {}
#[doc = "`write(|w| ..)` method takes [mckr::W](mckr::W) writer structure"]
impl crate::Writable for MCKR {}
#[doc = "Master Clock Register"]
pub mod mckr;
#[doc = "USB Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb](usb) module"]
pub type USB = crate::Reg<u32, _USB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB;
#[doc = "`read()` method returns [usb::R](usb::R) reader structure"]
impl crate::Readable for USB {}
#[doc = "`write(|w| ..)` method takes [usb::W](usb::W) writer structure"]
impl crate::Writable for USB {}
#[doc = "USB Clock Register"]
pub mod usb;
#[doc = "Programmable Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pck](pck) module"]
pub type PCK = crate::Reg<u32, _PCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCK;
#[doc = "`read()` method returns [pck::R](pck::R) reader structure"]
impl crate::Readable for PCK {}
#[doc = "`write(|w| ..)` method takes [pck::W](pck::W) writer structure"]
impl crate::Writable for PCK {}
#[doc = "Programmable Clock Register"]
pub mod pck;
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
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Fast Startup Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsmr](fsmr) module"]
pub type FSMR = crate::Reg<u32, _FSMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSMR;
#[doc = "`read()` method returns [fsmr::R](fsmr::R) reader structure"]
impl crate::Readable for FSMR {}
#[doc = "`write(|w| ..)` method takes [fsmr::W](fsmr::W) writer structure"]
impl crate::Writable for FSMR {}
#[doc = "Fast Startup Mode Register"]
pub mod fsmr;
#[doc = "Fast Startup Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fspr](fspr) module"]
pub type FSPR = crate::Reg<u32, _FSPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSPR;
#[doc = "`read()` method returns [fspr::R](fspr::R) reader structure"]
impl crate::Readable for FSPR {}
#[doc = "`write(|w| ..)` method takes [fspr::W](fspr::W) writer structure"]
impl crate::Writable for FSPR {}
#[doc = "Fast Startup Polarity Register"]
pub mod fspr;
#[doc = "Fault Output Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [focr](focr) module"]
pub type FOCR = crate::Reg<u32, _FOCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FOCR;
#[doc = "`write(|w| ..)` method takes [focr::W](focr::W) writer structure"]
impl crate::Writable for FOCR {}
#[doc = "Fault Output Clear Register"]
pub mod focr;
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
#[doc = "Peripheral Clock Enable Register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcer1](pcer1) module"]
pub type PCER1 = crate::Reg<u32, _PCER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCER1;
#[doc = "`write(|w| ..)` method takes [pcer1::W](pcer1::W) writer structure"]
impl crate::Writable for PCER1 {}
#[doc = "Peripheral Clock Enable Register 1"]
pub mod pcer1;
#[doc = "Peripheral Clock Disable Register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcdr1](pcdr1) module"]
pub type PCDR1 = crate::Reg<u32, _PCDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCDR1;
#[doc = "`write(|w| ..)` method takes [pcdr1::W](pcdr1::W) writer structure"]
impl crate::Writable for PCDR1 {}
#[doc = "Peripheral Clock Disable Register 1"]
pub mod pcdr1;
#[doc = "Peripheral Clock Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsr1](pcsr1) module"]
pub type PCSR1 = crate::Reg<u32, _PCSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCSR1;
#[doc = "`read()` method returns [pcsr1::R](pcsr1::R) reader structure"]
impl crate::Readable for PCSR1 {}
#[doc = "Peripheral Clock Status Register 1"]
pub mod pcsr1;
#[doc = "Peripheral Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](pcr) module"]
pub type PCR = crate::Reg<u32, _PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR;
#[doc = "`read()` method returns [pcr::R](pcr::R) reader structure"]
impl crate::Readable for PCR {}
#[doc = "`write(|w| ..)` method takes [pcr::W](pcr::W) writer structure"]
impl crate::Writable for PCR {}
#[doc = "Peripheral Control Register"]
pub mod pcr;
#[doc = "Oscillator Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocr](ocr) module"]
pub type OCR = crate::Reg<u32, _OCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCR;
#[doc = "`read()` method returns [ocr::R](ocr::R) reader structure"]
impl crate::Readable for OCR {}
#[doc = "`write(|w| ..)` method takes [ocr::W](ocr::W) writer structure"]
impl crate::Writable for OCR {}
#[doc = "Oscillator Calibration Register"]
pub mod ocr;
#[doc = "SleepWalking Enable Register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slpwk_er0](slpwk_er0) module"]
pub type SLPWK_ER0 = crate::Reg<u32, _SLPWK_ER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLPWK_ER0;
#[doc = "`write(|w| ..)` method takes [slpwk_er0::W](slpwk_er0::W) writer structure"]
impl crate::Writable for SLPWK_ER0 {}
#[doc = "SleepWalking Enable Register 0"]
pub mod slpwk_er0;
#[doc = "SleepWalking Disable Register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slpwk_dr0](slpwk_dr0) module"]
pub type SLPWK_DR0 = crate::Reg<u32, _SLPWK_DR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLPWK_DR0;
#[doc = "`write(|w| ..)` method takes [slpwk_dr0::W](slpwk_dr0::W) writer structure"]
impl crate::Writable for SLPWK_DR0 {}
#[doc = "SleepWalking Disable Register 0"]
pub mod slpwk_dr0;
#[doc = "SleepWalking Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slpwk_sr0](slpwk_sr0) module"]
pub type SLPWK_SR0 = crate::Reg<u32, _SLPWK_SR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLPWK_SR0;
#[doc = "`read()` method returns [slpwk_sr0::R](slpwk_sr0::R) reader structure"]
impl crate::Readable for SLPWK_SR0 {}
#[doc = "SleepWalking Status Register 0"]
pub mod slpwk_sr0;
#[doc = "SleepWalking Activity Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slpwk_asr0](slpwk_asr0) module"]
pub type SLPWK_ASR0 = crate::Reg<u32, _SLPWK_ASR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLPWK_ASR0;
#[doc = "`read()` method returns [slpwk_asr0::R](slpwk_asr0::R) reader structure"]
impl crate::Readable for SLPWK_ASR0 {}
#[doc = "SleepWalking Activity Status Register 0"]
pub mod slpwk_asr0;
#[doc = "PLL Maximum Multiplier Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmr](pmmr) module"]
pub type PMMR = crate::Reg<u32, _PMMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMMR;
#[doc = "`read()` method returns [pmmr::R](pmmr::R) reader structure"]
impl crate::Readable for PMMR {}
#[doc = "`write(|w| ..)` method takes [pmmr::W](pmmr::W) writer structure"]
impl crate::Writable for PMMR {}
#[doc = "PLL Maximum Multiplier Value Register"]
pub mod pmmr;
#[doc = "SleepWalking Enable Register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slpwk_er1](slpwk_er1) module"]
pub type SLPWK_ER1 = crate::Reg<u32, _SLPWK_ER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLPWK_ER1;
#[doc = "`write(|w| ..)` method takes [slpwk_er1::W](slpwk_er1::W) writer structure"]
impl crate::Writable for SLPWK_ER1 {}
#[doc = "SleepWalking Enable Register 1"]
pub mod slpwk_er1;
#[doc = "SleepWalking Disable Register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slpwk_dr1](slpwk_dr1) module"]
pub type SLPWK_DR1 = crate::Reg<u32, _SLPWK_DR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLPWK_DR1;
#[doc = "`write(|w| ..)` method takes [slpwk_dr1::W](slpwk_dr1::W) writer structure"]
impl crate::Writable for SLPWK_DR1 {}
#[doc = "SleepWalking Disable Register 1"]
pub mod slpwk_dr1;
#[doc = "SleepWalking Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slpwk_sr1](slpwk_sr1) module"]
pub type SLPWK_SR1 = crate::Reg<u32, _SLPWK_SR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLPWK_SR1;
#[doc = "`read()` method returns [slpwk_sr1::R](slpwk_sr1::R) reader structure"]
impl crate::Readable for SLPWK_SR1 {}
#[doc = "SleepWalking Status Register 1"]
pub mod slpwk_sr1;
#[doc = "SleepWalking Activity Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slpwk_asr1](slpwk_asr1) module"]
pub type SLPWK_ASR1 = crate::Reg<u32, _SLPWK_ASR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLPWK_ASR1;
#[doc = "`read()` method returns [slpwk_asr1::R](slpwk_asr1::R) reader structure"]
impl crate::Readable for SLPWK_ASR1 {}
#[doc = "SleepWalking Activity Status Register 1"]
pub mod slpwk_asr1;
#[doc = "SleepWalking Activity In Progress Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slpwk_aipr](slpwk_aipr) module"]
pub type SLPWK_AIPR = crate::Reg<u32, _SLPWK_AIPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLPWK_AIPR;
#[doc = "`read()` method returns [slpwk_aipr::R](slpwk_aipr::R) reader structure"]
impl crate::Readable for SLPWK_AIPR {}
#[doc = "SleepWalking Activity In Progress Register"]
pub mod slpwk_aipr;
