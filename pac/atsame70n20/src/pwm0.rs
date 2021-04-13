#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Clock Register"]
    pub clk: CLK,
    #[doc = "0x04 - PWM Enable Register"]
    pub ena: ENA,
    #[doc = "0x08 - PWM Disable Register"]
    pub dis: DIS,
    #[doc = "0x0c - PWM Status Register"]
    pub sr: SR,
    #[doc = "0x10 - PWM Interrupt Enable Register 1"]
    pub ier1: IER1,
    #[doc = "0x14 - PWM Interrupt Disable Register 1"]
    pub idr1: IDR1,
    #[doc = "0x18 - PWM Interrupt Mask Register 1"]
    pub imr1: IMR1,
    #[doc = "0x1c - PWM Interrupt Status Register 1"]
    pub isr1: ISR1,
    #[doc = "0x20 - PWM Sync Channels Mode Register"]
    pub scm: SCM,
    #[doc = "0x24 - PWM DMA Register"]
    pub dmar: DMAR,
    #[doc = "0x28 - PWM Sync Channels Update Control Register"]
    pub scuc: SCUC,
    #[doc = "0x2c - PWM Sync Channels Update Period Register"]
    pub scup: SCUP,
    #[doc = "0x30 - PWM Sync Channels Update Period Update Register"]
    pub scupupd: SCUPUPD,
    #[doc = "0x34 - PWM Interrupt Enable Register 2"]
    pub ier2: IER2,
    #[doc = "0x38 - PWM Interrupt Disable Register 2"]
    pub idr2: IDR2,
    #[doc = "0x3c - PWM Interrupt Mask Register 2"]
    pub imr2: IMR2,
    #[doc = "0x40 - PWM Interrupt Status Register 2"]
    pub isr2: ISR2,
    #[doc = "0x44 - PWM Output Override Value Register"]
    pub oov: OOV,
    #[doc = "0x48 - PWM Output Selection Register"]
    pub os: OS,
    #[doc = "0x4c - PWM Output Selection Set Register"]
    pub oss: OSS,
    #[doc = "0x50 - PWM Output Selection Clear Register"]
    pub osc: OSC,
    #[doc = "0x54 - PWM Output Selection Set Update Register"]
    pub ossupd: OSSUPD,
    #[doc = "0x58 - PWM Output Selection Clear Update Register"]
    pub oscupd: OSCUPD,
    #[doc = "0x5c - PWM Fault Mode Register"]
    pub fmr: FMR,
    #[doc = "0x60 - PWM Fault Status Register"]
    pub fsr: FSR,
    #[doc = "0x64 - PWM Fault Clear Register"]
    pub fcr: FCR,
    #[doc = "0x68 - PWM Fault Protection Value Register 1"]
    pub fpv1: FPV1,
    #[doc = "0x6c - PWM Fault Protection Enable Register"]
    pub fpe: FPE,
    _reserved28: [u8; 12usize],
    #[doc = "0x7c - PWM Event Line 0 Mode Register 0"]
    pub elmr: [ELMR; 2],
    _reserved29: [u8; 28usize],
    #[doc = "0xa0 - PWM Spread Spectrum Register"]
    pub sspr: SSPR,
    #[doc = "0xa4 - PWM Spread Spectrum Update Register"]
    pub sspup: SSPUP,
    _reserved31: [u8; 8usize],
    #[doc = "0xb0 - PWM Stepper Motor Mode Register"]
    pub smmr: SMMR,
    _reserved32: [u8; 12usize],
    #[doc = "0xc0 - PWM Fault Protection Value 2 Register"]
    pub fpv2: FPV2,
    _reserved33: [u8; 32usize],
    #[doc = "0xe4 - PWM Write Protection Control Register"]
    pub wpcr: WPCR,
    #[doc = "0xe8 - PWM Write Protection Status Register"]
    pub wpsr: WPSR,
    _reserved35: [u8; 68usize],
    #[doc = "0x130 - PWM Comparison 0 Value Register"]
    pub pwm_cmp: [PWM_CMP; 8],
    _reserved36: [u8; 80usize],
    #[doc = "0x200 - PWM Channel Mode Register (ch_num = 0)"]
    pub pwm_ch_num: [PWM_CH_NUM; 4],
    _reserved37: [u8; 384usize],
    #[doc = "0x400 - PWM Channel Mode Update Register (ch_num = 0)"]
    pub cmupd0: CMUPD0,
    _reserved38: [u8; 28usize],
    #[doc = "0x420 - PWM Channel Mode Update Register (ch_num = 1)"]
    pub cmupd1: CMUPD1,
    _reserved39: [u8; 8usize],
    #[doc = "0x42c - PWM External Trigger Register (trg_num = 1)"]
    pub etrg1: ETRG1,
    #[doc = "0x430 - PWM Leading-Edge Blanking Register (trg_num = 1)"]
    pub lebr1: LEBR1,
    _reserved41: [u8; 12usize],
    #[doc = "0x440 - PWM Channel Mode Update Register (ch_num = 2)"]
    pub cmupd2: CMUPD2,
    _reserved42: [u8; 8usize],
    #[doc = "0x44c - PWM External Trigger Register (trg_num = 2)"]
    pub etrg2: ETRG2,
    #[doc = "0x450 - PWM Leading-Edge Blanking Register (trg_num = 2)"]
    pub lebr2: LEBR2,
    _reserved44: [u8; 12usize],
    #[doc = "0x460 - PWM Channel Mode Update Register (ch_num = 3)"]
    pub cmupd3: CMUPD3,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PWM_CMP {
    #[doc = "0x00 - PWM Comparison 0 Value Register"]
    pub cmpv: self::pwm_cmp::CMPV,
    #[doc = "0x04 - PWM Comparison 0 Value Update Register"]
    pub cmpvupd: self::pwm_cmp::CMPVUPD,
    #[doc = "0x08 - PWM Comparison 0 Mode Register"]
    pub cmpm: self::pwm_cmp::CMPM,
    #[doc = "0x0c - PWM Comparison 0 Mode Update Register"]
    pub cmpmupd: self::pwm_cmp::CMPMUPD,
}
#[doc = r"Register block"]
#[doc = "PWM Comparison 0 Value Register"]
pub mod pwm_cmp;
#[doc = r"Register block"]
#[repr(C)]
pub struct PWM_CH_NUM {
    #[doc = "0x00 - PWM Channel Mode Register (ch_num = 0)"]
    pub cmr: self::pwm_ch_num::CMR,
    #[doc = "0x04 - PWM Channel Duty Cycle Register (ch_num = 0)"]
    pub cdty: self::pwm_ch_num::CDTY,
    #[doc = "0x08 - PWM Channel Duty Cycle Update Register (ch_num = 0)"]
    pub cdtyupd: self::pwm_ch_num::CDTYUPD,
    #[doc = "0x0c - PWM Channel Period Register (ch_num = 0)"]
    pub cprd: self::pwm_ch_num::CPRD,
    #[doc = "0x10 - PWM Channel Period Update Register (ch_num = 0)"]
    pub cprdupd: self::pwm_ch_num::CPRDUPD,
    #[doc = "0x14 - PWM Channel Counter Register (ch_num = 0)"]
    pub ccnt: self::pwm_ch_num::CCNT,
    #[doc = "0x18 - PWM Channel Dead Time Register (ch_num = 0)"]
    pub dt: self::pwm_ch_num::DT,
    #[doc = "0x1c - PWM Channel Dead Time Update Register (ch_num = 0)"]
    pub dtupd: self::pwm_ch_num::DTUPD,
}
#[doc = r"Register block"]
#[doc = "PWM Channel Mode Register (ch_num = 0)"]
pub mod pwm_ch_num;
#[doc = "PWM Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk](clk) module"]
pub type CLK = crate::Reg<u32, _CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK;
#[doc = "`read()` method returns [clk::R](clk::R) reader structure"]
impl crate::Readable for CLK {}
#[doc = "`write(|w| ..)` method takes [clk::W](clk::W) writer structure"]
impl crate::Writable for CLK {}
#[doc = "PWM Clock Register"]
pub mod clk;
#[doc = "PWM Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ena](ena) module"]
pub type ENA = crate::Reg<u32, _ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENA;
#[doc = "`write(|w| ..)` method takes [ena::W](ena::W) writer structure"]
impl crate::Writable for ENA {}
#[doc = "PWM Enable Register"]
pub mod ena;
#[doc = "PWM Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dis](dis) module"]
pub type DIS = crate::Reg<u32, _DIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIS;
#[doc = "`write(|w| ..)` method takes [dis::W](dis::W) writer structure"]
impl crate::Writable for DIS {}
#[doc = "PWM Disable Register"]
pub mod dis;
#[doc = "PWM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "PWM Status Register"]
pub mod sr;
#[doc = "PWM Interrupt Enable Register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier1](ier1) module"]
pub type IER1 = crate::Reg<u32, _IER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER1;
#[doc = "`write(|w| ..)` method takes [ier1::W](ier1::W) writer structure"]
impl crate::Writable for IER1 {}
#[doc = "PWM Interrupt Enable Register 1"]
pub mod ier1;
#[doc = "PWM Interrupt Disable Register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr1](idr1) module"]
pub type IDR1 = crate::Reg<u32, _IDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR1;
#[doc = "`write(|w| ..)` method takes [idr1::W](idr1::W) writer structure"]
impl crate::Writable for IDR1 {}
#[doc = "PWM Interrupt Disable Register 1"]
pub mod idr1;
#[doc = "PWM Interrupt Mask Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1](imr1) module"]
pub type IMR1 = crate::Reg<u32, _IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR1;
#[doc = "`read()` method returns [imr1::R](imr1::R) reader structure"]
impl crate::Readable for IMR1 {}
#[doc = "PWM Interrupt Mask Register 1"]
pub mod imr1;
#[doc = "PWM Interrupt Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr1](isr1) module"]
pub type ISR1 = crate::Reg<u32, _ISR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR1;
#[doc = "`read()` method returns [isr1::R](isr1::R) reader structure"]
impl crate::Readable for ISR1 {}
#[doc = "PWM Interrupt Status Register 1"]
pub mod isr1;
#[doc = "PWM Sync Channels Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scm](scm) module"]
pub type SCM = crate::Reg<u32, _SCM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCM;
#[doc = "`read()` method returns [scm::R](scm::R) reader structure"]
impl crate::Readable for SCM {}
#[doc = "`write(|w| ..)` method takes [scm::W](scm::W) writer structure"]
impl crate::Writable for SCM {}
#[doc = "PWM Sync Channels Mode Register"]
pub mod scm;
#[doc = "PWM DMA Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmar](dmar) module"]
pub type DMAR = crate::Reg<u32, _DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAR;
#[doc = "`write(|w| ..)` method takes [dmar::W](dmar::W) writer structure"]
impl crate::Writable for DMAR {}
#[doc = "PWM DMA Register"]
pub mod dmar;
#[doc = "PWM Sync Channels Update Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scuc](scuc) module"]
pub type SCUC = crate::Reg<u32, _SCUC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCUC;
#[doc = "`read()` method returns [scuc::R](scuc::R) reader structure"]
impl crate::Readable for SCUC {}
#[doc = "`write(|w| ..)` method takes [scuc::W](scuc::W) writer structure"]
impl crate::Writable for SCUC {}
#[doc = "PWM Sync Channels Update Control Register"]
pub mod scuc;
#[doc = "PWM Sync Channels Update Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scup](scup) module"]
pub type SCUP = crate::Reg<u32, _SCUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCUP;
#[doc = "`read()` method returns [scup::R](scup::R) reader structure"]
impl crate::Readable for SCUP {}
#[doc = "`write(|w| ..)` method takes [scup::W](scup::W) writer structure"]
impl crate::Writable for SCUP {}
#[doc = "PWM Sync Channels Update Period Register"]
pub mod scup;
#[doc = "PWM Sync Channels Update Period Update Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scupupd](scupupd) module"]
pub type SCUPUPD = crate::Reg<u32, _SCUPUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCUPUPD;
#[doc = "`write(|w| ..)` method takes [scupupd::W](scupupd::W) writer structure"]
impl crate::Writable for SCUPUPD {}
#[doc = "PWM Sync Channels Update Period Update Register"]
pub mod scupupd;
#[doc = "PWM Interrupt Enable Register 2\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier2](ier2) module"]
pub type IER2 = crate::Reg<u32, _IER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER2;
#[doc = "`write(|w| ..)` method takes [ier2::W](ier2::W) writer structure"]
impl crate::Writable for IER2 {}
#[doc = "PWM Interrupt Enable Register 2"]
pub mod ier2;
#[doc = "PWM Interrupt Disable Register 2\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr2](idr2) module"]
pub type IDR2 = crate::Reg<u32, _IDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR2;
#[doc = "`write(|w| ..)` method takes [idr2::W](idr2::W) writer structure"]
impl crate::Writable for IDR2 {}
#[doc = "PWM Interrupt Disable Register 2"]
pub mod idr2;
#[doc = "PWM Interrupt Mask Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr2](imr2) module"]
pub type IMR2 = crate::Reg<u32, _IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR2;
#[doc = "`read()` method returns [imr2::R](imr2::R) reader structure"]
impl crate::Readable for IMR2 {}
#[doc = "PWM Interrupt Mask Register 2"]
pub mod imr2;
#[doc = "PWM Interrupt Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr2](isr2) module"]
pub type ISR2 = crate::Reg<u32, _ISR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR2;
#[doc = "`read()` method returns [isr2::R](isr2::R) reader structure"]
impl crate::Readable for ISR2 {}
#[doc = "PWM Interrupt Status Register 2"]
pub mod isr2;
#[doc = "PWM Output Override Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oov](oov) module"]
pub type OOV = crate::Reg<u32, _OOV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OOV;
#[doc = "`read()` method returns [oov::R](oov::R) reader structure"]
impl crate::Readable for OOV {}
#[doc = "`write(|w| ..)` method takes [oov::W](oov::W) writer structure"]
impl crate::Writable for OOV {}
#[doc = "PWM Output Override Value Register"]
pub mod oov;
#[doc = "PWM Output Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [os](os) module"]
pub type OS = crate::Reg<u32, _OS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OS;
#[doc = "`read()` method returns [os::R](os::R) reader structure"]
impl crate::Readable for OS {}
#[doc = "`write(|w| ..)` method takes [os::W](os::W) writer structure"]
impl crate::Writable for OS {}
#[doc = "PWM Output Selection Register"]
pub mod os;
#[doc = "PWM Output Selection Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oss](oss) module"]
pub type OSS = crate::Reg<u32, _OSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSS;
#[doc = "`write(|w| ..)` method takes [oss::W](oss::W) writer structure"]
impl crate::Writable for OSS {}
#[doc = "PWM Output Selection Set Register"]
pub mod oss;
#[doc = "PWM Output Selection Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc](osc) module"]
pub type OSC = crate::Reg<u32, _OSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC;
#[doc = "`write(|w| ..)` method takes [osc::W](osc::W) writer structure"]
impl crate::Writable for OSC {}
#[doc = "PWM Output Selection Clear Register"]
pub mod osc;
#[doc = "PWM Output Selection Set Update Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ossupd](ossupd) module"]
pub type OSSUPD = crate::Reg<u32, _OSSUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSSUPD;
#[doc = "`write(|w| ..)` method takes [ossupd::W](ossupd::W) writer structure"]
impl crate::Writable for OSSUPD {}
#[doc = "PWM Output Selection Set Update Register"]
pub mod ossupd;
#[doc = "PWM Output Selection Clear Update Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscupd](oscupd) module"]
pub type OSCUPD = crate::Reg<u32, _OSCUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCUPD;
#[doc = "`write(|w| ..)` method takes [oscupd::W](oscupd::W) writer structure"]
impl crate::Writable for OSCUPD {}
#[doc = "PWM Output Selection Clear Update Register"]
pub mod oscupd;
#[doc = "PWM Fault Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmr](fmr) module"]
pub type FMR = crate::Reg<u32, _FMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMR;
#[doc = "`read()` method returns [fmr::R](fmr::R) reader structure"]
impl crate::Readable for FMR {}
#[doc = "`write(|w| ..)` method takes [fmr::W](fmr::W) writer structure"]
impl crate::Writable for FMR {}
#[doc = "PWM Fault Mode Register"]
pub mod fmr;
#[doc = "PWM Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsr](fsr) module"]
pub type FSR = crate::Reg<u32, _FSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSR;
#[doc = "`read()` method returns [fsr::R](fsr::R) reader structure"]
impl crate::Readable for FSR {}
#[doc = "PWM Fault Status Register"]
pub mod fsr;
#[doc = "PWM Fault Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](fcr) module"]
pub type FCR = crate::Reg<u32, _FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR;
#[doc = "`write(|w| ..)` method takes [fcr::W](fcr::W) writer structure"]
impl crate::Writable for FCR {}
#[doc = "PWM Fault Clear Register"]
pub mod fcr;
#[doc = "PWM Fault Protection Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpv1](fpv1) module"]
pub type FPV1 = crate::Reg<u32, _FPV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPV1;
#[doc = "`read()` method returns [fpv1::R](fpv1::R) reader structure"]
impl crate::Readable for FPV1 {}
#[doc = "`write(|w| ..)` method takes [fpv1::W](fpv1::W) writer structure"]
impl crate::Writable for FPV1 {}
#[doc = "PWM Fault Protection Value Register 1"]
pub mod fpv1;
#[doc = "PWM Fault Protection Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpe](fpe) module"]
pub type FPE = crate::Reg<u32, _FPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPE;
#[doc = "`read()` method returns [fpe::R](fpe::R) reader structure"]
impl crate::Readable for FPE {}
#[doc = "`write(|w| ..)` method takes [fpe::W](fpe::W) writer structure"]
impl crate::Writable for FPE {}
#[doc = "PWM Fault Protection Enable Register"]
pub mod fpe;
#[doc = "PWM Event Line 0 Mode Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elmr](elmr) module"]
pub type ELMR = crate::Reg<u32, _ELMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ELMR;
#[doc = "`read()` method returns [elmr::R](elmr::R) reader structure"]
impl crate::Readable for ELMR {}
#[doc = "`write(|w| ..)` method takes [elmr::W](elmr::W) writer structure"]
impl crate::Writable for ELMR {}
#[doc = "PWM Event Line 0 Mode Register 0"]
pub mod elmr;
#[doc = "PWM Spread Spectrum Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspr](sspr) module"]
pub type SSPR = crate::Reg<u32, _SSPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPR;
#[doc = "`read()` method returns [sspr::R](sspr::R) reader structure"]
impl crate::Readable for SSPR {}
#[doc = "`write(|w| ..)` method takes [sspr::W](sspr::W) writer structure"]
impl crate::Writable for SSPR {}
#[doc = "PWM Spread Spectrum Register"]
pub mod sspr;
#[doc = "PWM Spread Spectrum Update Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspup](sspup) module"]
pub type SSPUP = crate::Reg<u32, _SSPUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPUP;
#[doc = "`write(|w| ..)` method takes [sspup::W](sspup::W) writer structure"]
impl crate::Writable for SSPUP {}
#[doc = "PWM Spread Spectrum Update Register"]
pub mod sspup;
#[doc = "PWM Stepper Motor Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smmr](smmr) module"]
pub type SMMR = crate::Reg<u32, _SMMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMMR;
#[doc = "`read()` method returns [smmr::R](smmr::R) reader structure"]
impl crate::Readable for SMMR {}
#[doc = "`write(|w| ..)` method takes [smmr::W](smmr::W) writer structure"]
impl crate::Writable for SMMR {}
#[doc = "PWM Stepper Motor Mode Register"]
pub mod smmr;
#[doc = "PWM Fault Protection Value 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpv2](fpv2) module"]
pub type FPV2 = crate::Reg<u32, _FPV2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPV2;
#[doc = "`read()` method returns [fpv2::R](fpv2::R) reader structure"]
impl crate::Readable for FPV2 {}
#[doc = "`write(|w| ..)` method takes [fpv2::W](fpv2::W) writer structure"]
impl crate::Writable for FPV2 {}
#[doc = "PWM Fault Protection Value 2 Register"]
pub mod fpv2;
#[doc = "PWM Write Protection Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr](wpcr) module"]
pub type WPCR = crate::Reg<u32, _WPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPCR;
#[doc = "`write(|w| ..)` method takes [wpcr::W](wpcr::W) writer structure"]
impl crate::Writable for WPCR {}
#[doc = "PWM Write Protection Control Register"]
pub mod wpcr;
#[doc = "PWM Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](wpsr) module"]
pub type WPSR = crate::Reg<u32, _WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSR;
#[doc = "`read()` method returns [wpsr::R](wpsr::R) reader structure"]
impl crate::Readable for WPSR {}
#[doc = "PWM Write Protection Status Register"]
pub mod wpsr;
#[doc = "PWM Channel Mode Update Register (ch_num = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmupd0](cmupd0) module"]
pub type CMUPD0 = crate::Reg<u32, _CMUPD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMUPD0;
#[doc = "`write(|w| ..)` method takes [cmupd0::W](cmupd0::W) writer structure"]
impl crate::Writable for CMUPD0 {}
#[doc = "PWM Channel Mode Update Register (ch_num = 0)"]
pub mod cmupd0;
#[doc = "PWM Channel Mode Update Register (ch_num = 1)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmupd1](cmupd1) module"]
pub type CMUPD1 = crate::Reg<u32, _CMUPD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMUPD1;
#[doc = "`write(|w| ..)` method takes [cmupd1::W](cmupd1::W) writer structure"]
impl crate::Writable for CMUPD1 {}
#[doc = "PWM Channel Mode Update Register (ch_num = 1)"]
pub mod cmupd1;
#[doc = "PWM External Trigger Register (trg_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etrg1](etrg1) module"]
pub type ETRG1 = crate::Reg<u32, _ETRG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETRG1;
#[doc = "`read()` method returns [etrg1::R](etrg1::R) reader structure"]
impl crate::Readable for ETRG1 {}
#[doc = "`write(|w| ..)` method takes [etrg1::W](etrg1::W) writer structure"]
impl crate::Writable for ETRG1 {}
#[doc = "PWM External Trigger Register (trg_num = 1)"]
pub mod etrg1;
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lebr1](lebr1) module"]
pub type LEBR1 = crate::Reg<u32, _LEBR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEBR1;
#[doc = "`read()` method returns [lebr1::R](lebr1::R) reader structure"]
impl crate::Readable for LEBR1 {}
#[doc = "`write(|w| ..)` method takes [lebr1::W](lebr1::W) writer structure"]
impl crate::Writable for LEBR1 {}
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 1)"]
pub mod lebr1;
#[doc = "PWM Channel Mode Update Register (ch_num = 2)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmupd2](cmupd2) module"]
pub type CMUPD2 = crate::Reg<u32, _CMUPD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMUPD2;
#[doc = "`write(|w| ..)` method takes [cmupd2::W](cmupd2::W) writer structure"]
impl crate::Writable for CMUPD2 {}
#[doc = "PWM Channel Mode Update Register (ch_num = 2)"]
pub mod cmupd2;
#[doc = "PWM External Trigger Register (trg_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etrg2](etrg2) module"]
pub type ETRG2 = crate::Reg<u32, _ETRG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETRG2;
#[doc = "`read()` method returns [etrg2::R](etrg2::R) reader structure"]
impl crate::Readable for ETRG2 {}
#[doc = "`write(|w| ..)` method takes [etrg2::W](etrg2::W) writer structure"]
impl crate::Writable for ETRG2 {}
#[doc = "PWM External Trigger Register (trg_num = 2)"]
pub mod etrg2;
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lebr2](lebr2) module"]
pub type LEBR2 = crate::Reg<u32, _LEBR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEBR2;
#[doc = "`read()` method returns [lebr2::R](lebr2::R) reader structure"]
impl crate::Readable for LEBR2 {}
#[doc = "`write(|w| ..)` method takes [lebr2::W](lebr2::W) writer structure"]
impl crate::Writable for LEBR2 {}
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 2)"]
pub mod lebr2;
#[doc = "PWM Channel Mode Update Register (ch_num = 3)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmupd3](cmupd3) module"]
pub type CMUPD3 = crate::Reg<u32, _CMUPD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMUPD3;
#[doc = "`write(|w| ..)` method takes [cmupd3::W](cmupd3::W) writer structure"]
impl crate::Writable for CMUPD3 {}
#[doc = "PWM Channel Mode Update Register (ch_num = 3)"]
pub mod cmupd3;
