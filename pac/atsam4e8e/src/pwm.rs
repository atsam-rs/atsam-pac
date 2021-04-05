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
    _reserved9: [u8; 4usize],
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
    _reserved27: [u8; 12usize],
    #[doc = "0x7c - PWM Event Line 0 Mode Register"]
    pub elmr: [ELMR; 2],
    _reserved28: [u8; 28usize],
    #[doc = "0xa0 - PWM Spread Spectrum Register"]
    pub sspr: SSPR,
    #[doc = "0xa4 - PWM Spread Spectrum Update Register"]
    pub sspup: SSPUP,
    _reserved30: [u8; 8usize],
    #[doc = "0xb0 - PWM Stepper Motor Mode Register"]
    pub smmr: SMMR,
    _reserved31: [u8; 12usize],
    #[doc = "0xc0 - PWM Fault Protection Value 2 Register"]
    pub fpv2: FPV2,
    _reserved32: [u8; 32usize],
    #[doc = "0xe4 - PWM Write Protect Control Register"]
    pub wpcr: WPCR,
    #[doc = "0xe8 - PWM Write Protect Status Register"]
    pub wpsr: WPSR,
    _reserved34: [u8; 28usize],
    #[doc = "0x108 - Transmit Pointer Register"]
    pub tpr: TPR,
    #[doc = "0x10c - Transmit Counter Register"]
    pub tcr: TCR,
    _reserved36: [u8; 8usize],
    #[doc = "0x118 - Transmit Next Pointer Register"]
    pub tnpr: TNPR,
    #[doc = "0x11c - Transmit Next Counter Register"]
    pub tncr: TNCR,
    #[doc = "0x120 - Transfer Control Register"]
    pub ptcr: PTCR,
    #[doc = "0x124 - Transfer Status Register"]
    pub ptsr: PTSR,
    _reserved40: [u8; 8usize],
    #[doc = "0x130 - PWM Comparison 0 Value Register"]
    pub cmpv0: CMPV0,
    #[doc = "0x134 - PWM Comparison 0 Value Update Register"]
    pub cmpvupd0: CMPVUPD0,
    #[doc = "0x138 - PWM Comparison 0 Mode Register"]
    pub cmpm0: CMPM0,
    #[doc = "0x13c - PWM Comparison 0 Mode Update Register"]
    pub cmpmupd0: CMPMUPD0,
    #[doc = "0x140 - PWM Comparison 1 Value Register"]
    pub cmpv1: CMPV1,
    #[doc = "0x144 - PWM Comparison 1 Value Update Register"]
    pub cmpvupd1: CMPVUPD1,
    #[doc = "0x148 - PWM Comparison 1 Mode Register"]
    pub cmpm1: CMPM1,
    #[doc = "0x14c - PWM Comparison 1 Mode Update Register"]
    pub cmpmupd1: CMPMUPD1,
    #[doc = "0x150 - PWM Comparison 2 Value Register"]
    pub cmpv2: CMPV2,
    #[doc = "0x154 - PWM Comparison 2 Value Update Register"]
    pub cmpvupd2: CMPVUPD2,
    #[doc = "0x158 - PWM Comparison 2 Mode Register"]
    pub cmpm2: CMPM2,
    #[doc = "0x15c - PWM Comparison 2 Mode Update Register"]
    pub cmpmupd2: CMPMUPD2,
    #[doc = "0x160 - PWM Comparison 3 Value Register"]
    pub cmpv3: CMPV3,
    #[doc = "0x164 - PWM Comparison 3 Value Update Register"]
    pub cmpvupd3: CMPVUPD3,
    #[doc = "0x168 - PWM Comparison 3 Mode Register"]
    pub cmpm3: CMPM3,
    #[doc = "0x16c - PWM Comparison 3 Mode Update Register"]
    pub cmpmupd3: CMPMUPD3,
    #[doc = "0x170 - PWM Comparison 4 Value Register"]
    pub cmpv4: CMPV4,
    #[doc = "0x174 - PWM Comparison 4 Value Update Register"]
    pub cmpvupd4: CMPVUPD4,
    #[doc = "0x178 - PWM Comparison 4 Mode Register"]
    pub cmpm4: CMPM4,
    #[doc = "0x17c - PWM Comparison 4 Mode Update Register"]
    pub cmpmupd4: CMPMUPD4,
    #[doc = "0x180 - PWM Comparison 5 Value Register"]
    pub cmpv5: CMPV5,
    #[doc = "0x184 - PWM Comparison 5 Value Update Register"]
    pub cmpvupd5: CMPVUPD5,
    #[doc = "0x188 - PWM Comparison 5 Mode Register"]
    pub cmpm5: CMPM5,
    #[doc = "0x18c - PWM Comparison 5 Mode Update Register"]
    pub cmpmupd5: CMPMUPD5,
    #[doc = "0x190 - PWM Comparison 6 Value Register"]
    pub cmpv6: CMPV6,
    #[doc = "0x194 - PWM Comparison 6 Value Update Register"]
    pub cmpvupd6: CMPVUPD6,
    #[doc = "0x198 - PWM Comparison 6 Mode Register"]
    pub cmpm6: CMPM6,
    #[doc = "0x19c - PWM Comparison 6 Mode Update Register"]
    pub cmpmupd6: CMPMUPD6,
    #[doc = "0x1a0 - PWM Comparison 7 Value Register"]
    pub cmpv7: CMPV7,
    #[doc = "0x1a4 - PWM Comparison 7 Value Update Register"]
    pub cmpvupd7: CMPVUPD7,
    #[doc = "0x1a8 - PWM Comparison 7 Mode Register"]
    pub cmpm7: CMPM7,
    #[doc = "0x1ac - PWM Comparison 7 Mode Update Register"]
    pub cmpmupd7: CMPMUPD7,
    _reserved72: [u8; 80usize],
    #[doc = "0x200 - PWM Channel Mode Register (ch_num = 0)"]
    pub cmr0: CMR0,
    #[doc = "0x204 - PWM Channel Duty Cycle Register (ch_num = 0)"]
    pub cdty0: CDTY0,
    #[doc = "0x208 - PWM Channel Duty Cycle Update Register (ch_num = 0)"]
    pub cdtyupd0: CDTYUPD0,
    #[doc = "0x20c - PWM Channel Period Register (ch_num = 0)"]
    pub cprd0: CPRD0,
    #[doc = "0x210 - PWM Channel Period Update Register (ch_num = 0)"]
    pub cprdupd0: CPRDUPD0,
    #[doc = "0x214 - PWM Channel Counter Register (ch_num = 0)"]
    pub ccnt0: CCNT0,
    #[doc = "0x218 - PWM Channel Dead Time Register (ch_num = 0)"]
    pub dt0: DT0,
    #[doc = "0x21c - PWM Channel Dead Time Update Register (ch_num = 0)"]
    pub dtupd0: DTUPD0,
    #[doc = "0x220 - PWM Channel Mode Register (ch_num = 1)"]
    pub cmr1: CMR1,
    #[doc = "0x224 - PWM Channel Duty Cycle Register (ch_num = 1)"]
    pub cdty1: CDTY1,
    #[doc = "0x228 - PWM Channel Duty Cycle Update Register (ch_num = 1)"]
    pub cdtyupd1: CDTYUPD1,
    #[doc = "0x22c - PWM Channel Period Register (ch_num = 1)"]
    pub cprd1: CPRD1,
    #[doc = "0x230 - PWM Channel Period Update Register (ch_num = 1)"]
    pub cprdupd1: CPRDUPD1,
    #[doc = "0x234 - PWM Channel Counter Register (ch_num = 1)"]
    pub ccnt1: CCNT1,
    #[doc = "0x238 - PWM Channel Dead Time Register (ch_num = 1)"]
    pub dt1: DT1,
    #[doc = "0x23c - PWM Channel Dead Time Update Register (ch_num = 1)"]
    pub dtupd1: DTUPD1,
    #[doc = "0x240 - PWM Channel Mode Register (ch_num = 2)"]
    pub cmr2: CMR2,
    #[doc = "0x244 - PWM Channel Duty Cycle Register (ch_num = 2)"]
    pub cdty2: CDTY2,
    #[doc = "0x248 - PWM Channel Duty Cycle Update Register (ch_num = 2)"]
    pub cdtyupd2: CDTYUPD2,
    #[doc = "0x24c - PWM Channel Period Register (ch_num = 2)"]
    pub cprd2: CPRD2,
    #[doc = "0x250 - PWM Channel Period Update Register (ch_num = 2)"]
    pub cprdupd2: CPRDUPD2,
    #[doc = "0x254 - PWM Channel Counter Register (ch_num = 2)"]
    pub ccnt2: CCNT2,
    #[doc = "0x258 - PWM Channel Dead Time Register (ch_num = 2)"]
    pub dt2: DT2,
    #[doc = "0x25c - PWM Channel Dead Time Update Register (ch_num = 2)"]
    pub dtupd2: DTUPD2,
    #[doc = "0x260 - PWM Channel Mode Register (ch_num = 3)"]
    pub cmr3: CMR3,
    #[doc = "0x264 - PWM Channel Duty Cycle Register (ch_num = 3)"]
    pub cdty3: CDTY3,
    #[doc = "0x268 - PWM Channel Duty Cycle Update Register (ch_num = 3)"]
    pub cdtyupd3: CDTYUPD3,
    #[doc = "0x26c - PWM Channel Period Register (ch_num = 3)"]
    pub cprd3: CPRD3,
    #[doc = "0x270 - PWM Channel Period Update Register (ch_num = 3)"]
    pub cprdupd3: CPRDUPD3,
    #[doc = "0x274 - PWM Channel Counter Register (ch_num = 3)"]
    pub ccnt3: CCNT3,
    #[doc = "0x278 - PWM Channel Dead Time Register (ch_num = 3)"]
    pub dt3: DT3,
    #[doc = "0x27c - PWM Channel Dead Time Update Register (ch_num = 3)"]
    pub dtupd3: DTUPD3,
    _reserved104: [u8; 384usize],
    #[doc = "0x400 - PWM Channel Mode Update Register (ch_num = 0)"]
    pub cmupd0: CMUPD0,
    #[doc = "0x404 - PWM Channel Additional Edge Register (ch_num = 0)"]
    pub cae0: CAE0,
    #[doc = "0x408 - PWM Channel Additional Edge Update Register (ch_num = 0)"]
    pub caeupd0: CAEUPD0,
    _reserved107: [u8; 20usize],
    #[doc = "0x420 - PWM Channel Mode Update Register (ch_num = 1)"]
    pub cmupd1: CMUPD1,
    #[doc = "0x424 - PWM Channel Additional Edge Register (ch_num = 1)"]
    pub cae1: CAE1,
    #[doc = "0x428 - PWM Channel Additional Edge Update Register (ch_num = 1)"]
    pub caeupd1: CAEUPD1,
    _reserved110: [u8; 20usize],
    #[doc = "0x440 - PWM Channel Mode Update Register (ch_num = 2)"]
    pub cmupd2: CMUPD2,
    #[doc = "0x444 - PWM Channel Additional Edge Register (ch_num = 2)"]
    pub cae2: CAE2,
    #[doc = "0x448 - PWM Channel Additional Edge Update Register (ch_num = 2)"]
    pub caeupd2: CAEUPD2,
    _reserved113: [u8; 20usize],
    #[doc = "0x460 - PWM Channel Mode Update Register (ch_num = 3)"]
    pub cmupd3: CMUPD3,
    #[doc = "0x464 - PWM Channel Additional Edge Register (ch_num = 3)"]
    pub cae3: CAE3,
    #[doc = "0x468 - PWM Channel Additional Edge Update Register (ch_num = 3)"]
    pub caeupd3: CAEUPD3,
}
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
#[doc = "PWM Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ena](ena) module"]
pub type ENA = crate::Reg<u32, _ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENA;
#[doc = "`write(|w| ..)` method takes [ena::W](ena::W) writer structure"]
impl crate::Writable for ENA {}
#[doc = "PWM Enable Register"]
pub mod ena;
#[doc = "PWM Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dis](dis) module"]
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
#[doc = "PWM Interrupt Enable Register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier1](ier1) module"]
pub type IER1 = crate::Reg<u32, _IER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER1;
#[doc = "`write(|w| ..)` method takes [ier1::W](ier1::W) writer structure"]
impl crate::Writable for IER1 {}
#[doc = "PWM Interrupt Enable Register 1"]
pub mod ier1;
#[doc = "PWM Interrupt Disable Register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr1](idr1) module"]
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
#[doc = "PWM Interrupt Enable Register 2\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier2](ier2) module"]
pub type IER2 = crate::Reg<u32, _IER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER2;
#[doc = "`write(|w| ..)` method takes [ier2::W](ier2::W) writer structure"]
impl crate::Writable for IER2 {}
#[doc = "PWM Interrupt Enable Register 2"]
pub mod ier2;
#[doc = "PWM Interrupt Disable Register 2\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr2](idr2) module"]
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
#[doc = "PWM Output Selection Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oss](oss) module"]
pub type OSS = crate::Reg<u32, _OSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSS;
#[doc = "`write(|w| ..)` method takes [oss::W](oss::W) writer structure"]
impl crate::Writable for OSS {}
#[doc = "PWM Output Selection Set Register"]
pub mod oss;
#[doc = "PWM Output Selection Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc](osc) module"]
pub type OSC = crate::Reg<u32, _OSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC;
#[doc = "`write(|w| ..)` method takes [osc::W](osc::W) writer structure"]
impl crate::Writable for OSC {}
#[doc = "PWM Output Selection Clear Register"]
pub mod osc;
#[doc = "PWM Output Selection Set Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ossupd](ossupd) module"]
pub type OSSUPD = crate::Reg<u32, _OSSUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSSUPD;
#[doc = "`write(|w| ..)` method takes [ossupd::W](ossupd::W) writer structure"]
impl crate::Writable for OSSUPD {}
#[doc = "PWM Output Selection Set Update Register"]
pub mod ossupd;
#[doc = "PWM Output Selection Clear Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscupd](oscupd) module"]
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
#[doc = "PWM Fault Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](fcr) module"]
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
#[doc = "PWM Event Line 0 Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elmr](elmr) module"]
pub type ELMR = crate::Reg<u32, _ELMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ELMR;
#[doc = "`read()` method returns [elmr::R](elmr::R) reader structure"]
impl crate::Readable for ELMR {}
#[doc = "`write(|w| ..)` method takes [elmr::W](elmr::W) writer structure"]
impl crate::Writable for ELMR {}
#[doc = "PWM Event Line 0 Mode Register"]
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
#[doc = "PWM Spread Spectrum Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspup](sspup) module"]
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
#[doc = "PWM Write Protect Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr](wpcr) module"]
pub type WPCR = crate::Reg<u32, _WPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPCR;
#[doc = "`write(|w| ..)` method takes [wpcr::W](wpcr::W) writer structure"]
impl crate::Writable for WPCR {}
#[doc = "PWM Write Protect Control Register"]
pub mod wpcr;
#[doc = "PWM Write Protect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](wpsr) module"]
pub type WPSR = crate::Reg<u32, _WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSR;
#[doc = "`read()` method returns [wpsr::R](wpsr::R) reader structure"]
impl crate::Readable for WPSR {}
#[doc = "PWM Write Protect Status Register"]
pub mod wpsr;
#[doc = "PWM Comparison 0 Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpv0](cmpv0) module"]
pub type CMPV0 = crate::Reg<u32, _CMPV0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPV0;
#[doc = "`read()` method returns [cmpv0::R](cmpv0::R) reader structure"]
impl crate::Readable for CMPV0 {}
#[doc = "`write(|w| ..)` method takes [cmpv0::W](cmpv0::W) writer structure"]
impl crate::Writable for CMPV0 {}
#[doc = "PWM Comparison 0 Value Register"]
pub mod cmpv0;
#[doc = "PWM Comparison 0 Value Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpvupd0](cmpvupd0) module"]
pub type CMPVUPD0 = crate::Reg<u32, _CMPVUPD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPVUPD0;
#[doc = "`write(|w| ..)` method takes [cmpvupd0::W](cmpvupd0::W) writer structure"]
impl crate::Writable for CMPVUPD0 {}
#[doc = "PWM Comparison 0 Value Update Register"]
pub mod cmpvupd0;
#[doc = "PWM Comparison 0 Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpm0](cmpm0) module"]
pub type CMPM0 = crate::Reg<u32, _CMPM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPM0;
#[doc = "`read()` method returns [cmpm0::R](cmpm0::R) reader structure"]
impl crate::Readable for CMPM0 {}
#[doc = "`write(|w| ..)` method takes [cmpm0::W](cmpm0::W) writer structure"]
impl crate::Writable for CMPM0 {}
#[doc = "PWM Comparison 0 Mode Register"]
pub mod cmpm0;
#[doc = "PWM Comparison 0 Mode Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpmupd0](cmpmupd0) module"]
pub type CMPMUPD0 = crate::Reg<u32, _CMPMUPD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPMUPD0;
#[doc = "`write(|w| ..)` method takes [cmpmupd0::W](cmpmupd0::W) writer structure"]
impl crate::Writable for CMPMUPD0 {}
#[doc = "PWM Comparison 0 Mode Update Register"]
pub mod cmpmupd0;
#[doc = "PWM Comparison 1 Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpv1](cmpv1) module"]
pub type CMPV1 = crate::Reg<u32, _CMPV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPV1;
#[doc = "`read()` method returns [cmpv1::R](cmpv1::R) reader structure"]
impl crate::Readable for CMPV1 {}
#[doc = "`write(|w| ..)` method takes [cmpv1::W](cmpv1::W) writer structure"]
impl crate::Writable for CMPV1 {}
#[doc = "PWM Comparison 1 Value Register"]
pub mod cmpv1;
#[doc = "PWM Comparison 1 Value Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpvupd1](cmpvupd1) module"]
pub type CMPVUPD1 = crate::Reg<u32, _CMPVUPD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPVUPD1;
#[doc = "`write(|w| ..)` method takes [cmpvupd1::W](cmpvupd1::W) writer structure"]
impl crate::Writable for CMPVUPD1 {}
#[doc = "PWM Comparison 1 Value Update Register"]
pub mod cmpvupd1;
#[doc = "PWM Comparison 1 Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpm1](cmpm1) module"]
pub type CMPM1 = crate::Reg<u32, _CMPM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPM1;
#[doc = "`read()` method returns [cmpm1::R](cmpm1::R) reader structure"]
impl crate::Readable for CMPM1 {}
#[doc = "`write(|w| ..)` method takes [cmpm1::W](cmpm1::W) writer structure"]
impl crate::Writable for CMPM1 {}
#[doc = "PWM Comparison 1 Mode Register"]
pub mod cmpm1;
#[doc = "PWM Comparison 1 Mode Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpmupd1](cmpmupd1) module"]
pub type CMPMUPD1 = crate::Reg<u32, _CMPMUPD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPMUPD1;
#[doc = "`write(|w| ..)` method takes [cmpmupd1::W](cmpmupd1::W) writer structure"]
impl crate::Writable for CMPMUPD1 {}
#[doc = "PWM Comparison 1 Mode Update Register"]
pub mod cmpmupd1;
#[doc = "PWM Comparison 2 Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpv2](cmpv2) module"]
pub type CMPV2 = crate::Reg<u32, _CMPV2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPV2;
#[doc = "`read()` method returns [cmpv2::R](cmpv2::R) reader structure"]
impl crate::Readable for CMPV2 {}
#[doc = "`write(|w| ..)` method takes [cmpv2::W](cmpv2::W) writer structure"]
impl crate::Writable for CMPV2 {}
#[doc = "PWM Comparison 2 Value Register"]
pub mod cmpv2;
#[doc = "PWM Comparison 2 Value Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpvupd2](cmpvupd2) module"]
pub type CMPVUPD2 = crate::Reg<u32, _CMPVUPD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPVUPD2;
#[doc = "`write(|w| ..)` method takes [cmpvupd2::W](cmpvupd2::W) writer structure"]
impl crate::Writable for CMPVUPD2 {}
#[doc = "PWM Comparison 2 Value Update Register"]
pub mod cmpvupd2;
#[doc = "PWM Comparison 2 Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpm2](cmpm2) module"]
pub type CMPM2 = crate::Reg<u32, _CMPM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPM2;
#[doc = "`read()` method returns [cmpm2::R](cmpm2::R) reader structure"]
impl crate::Readable for CMPM2 {}
#[doc = "`write(|w| ..)` method takes [cmpm2::W](cmpm2::W) writer structure"]
impl crate::Writable for CMPM2 {}
#[doc = "PWM Comparison 2 Mode Register"]
pub mod cmpm2;
#[doc = "PWM Comparison 2 Mode Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpmupd2](cmpmupd2) module"]
pub type CMPMUPD2 = crate::Reg<u32, _CMPMUPD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPMUPD2;
#[doc = "`write(|w| ..)` method takes [cmpmupd2::W](cmpmupd2::W) writer structure"]
impl crate::Writable for CMPMUPD2 {}
#[doc = "PWM Comparison 2 Mode Update Register"]
pub mod cmpmupd2;
#[doc = "PWM Comparison 3 Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpv3](cmpv3) module"]
pub type CMPV3 = crate::Reg<u32, _CMPV3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPV3;
#[doc = "`read()` method returns [cmpv3::R](cmpv3::R) reader structure"]
impl crate::Readable for CMPV3 {}
#[doc = "`write(|w| ..)` method takes [cmpv3::W](cmpv3::W) writer structure"]
impl crate::Writable for CMPV3 {}
#[doc = "PWM Comparison 3 Value Register"]
pub mod cmpv3;
#[doc = "PWM Comparison 3 Value Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpvupd3](cmpvupd3) module"]
pub type CMPVUPD3 = crate::Reg<u32, _CMPVUPD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPVUPD3;
#[doc = "`write(|w| ..)` method takes [cmpvupd3::W](cmpvupd3::W) writer structure"]
impl crate::Writable for CMPVUPD3 {}
#[doc = "PWM Comparison 3 Value Update Register"]
pub mod cmpvupd3;
#[doc = "PWM Comparison 3 Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpm3](cmpm3) module"]
pub type CMPM3 = crate::Reg<u32, _CMPM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPM3;
#[doc = "`read()` method returns [cmpm3::R](cmpm3::R) reader structure"]
impl crate::Readable for CMPM3 {}
#[doc = "`write(|w| ..)` method takes [cmpm3::W](cmpm3::W) writer structure"]
impl crate::Writable for CMPM3 {}
#[doc = "PWM Comparison 3 Mode Register"]
pub mod cmpm3;
#[doc = "PWM Comparison 3 Mode Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpmupd3](cmpmupd3) module"]
pub type CMPMUPD3 = crate::Reg<u32, _CMPMUPD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPMUPD3;
#[doc = "`write(|w| ..)` method takes [cmpmupd3::W](cmpmupd3::W) writer structure"]
impl crate::Writable for CMPMUPD3 {}
#[doc = "PWM Comparison 3 Mode Update Register"]
pub mod cmpmupd3;
#[doc = "PWM Comparison 4 Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpv4](cmpv4) module"]
pub type CMPV4 = crate::Reg<u32, _CMPV4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPV4;
#[doc = "`read()` method returns [cmpv4::R](cmpv4::R) reader structure"]
impl crate::Readable for CMPV4 {}
#[doc = "`write(|w| ..)` method takes [cmpv4::W](cmpv4::W) writer structure"]
impl crate::Writable for CMPV4 {}
#[doc = "PWM Comparison 4 Value Register"]
pub mod cmpv4;
#[doc = "PWM Comparison 4 Value Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpvupd4](cmpvupd4) module"]
pub type CMPVUPD4 = crate::Reg<u32, _CMPVUPD4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPVUPD4;
#[doc = "`write(|w| ..)` method takes [cmpvupd4::W](cmpvupd4::W) writer structure"]
impl crate::Writable for CMPVUPD4 {}
#[doc = "PWM Comparison 4 Value Update Register"]
pub mod cmpvupd4;
#[doc = "PWM Comparison 4 Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpm4](cmpm4) module"]
pub type CMPM4 = crate::Reg<u32, _CMPM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPM4;
#[doc = "`read()` method returns [cmpm4::R](cmpm4::R) reader structure"]
impl crate::Readable for CMPM4 {}
#[doc = "`write(|w| ..)` method takes [cmpm4::W](cmpm4::W) writer structure"]
impl crate::Writable for CMPM4 {}
#[doc = "PWM Comparison 4 Mode Register"]
pub mod cmpm4;
#[doc = "PWM Comparison 4 Mode Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpmupd4](cmpmupd4) module"]
pub type CMPMUPD4 = crate::Reg<u32, _CMPMUPD4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPMUPD4;
#[doc = "`write(|w| ..)` method takes [cmpmupd4::W](cmpmupd4::W) writer structure"]
impl crate::Writable for CMPMUPD4 {}
#[doc = "PWM Comparison 4 Mode Update Register"]
pub mod cmpmupd4;
#[doc = "PWM Comparison 5 Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpv5](cmpv5) module"]
pub type CMPV5 = crate::Reg<u32, _CMPV5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPV5;
#[doc = "`read()` method returns [cmpv5::R](cmpv5::R) reader structure"]
impl crate::Readable for CMPV5 {}
#[doc = "`write(|w| ..)` method takes [cmpv5::W](cmpv5::W) writer structure"]
impl crate::Writable for CMPV5 {}
#[doc = "PWM Comparison 5 Value Register"]
pub mod cmpv5;
#[doc = "PWM Comparison 5 Value Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpvupd5](cmpvupd5) module"]
pub type CMPVUPD5 = crate::Reg<u32, _CMPVUPD5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPVUPD5;
#[doc = "`write(|w| ..)` method takes [cmpvupd5::W](cmpvupd5::W) writer structure"]
impl crate::Writable for CMPVUPD5 {}
#[doc = "PWM Comparison 5 Value Update Register"]
pub mod cmpvupd5;
#[doc = "PWM Comparison 5 Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpm5](cmpm5) module"]
pub type CMPM5 = crate::Reg<u32, _CMPM5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPM5;
#[doc = "`read()` method returns [cmpm5::R](cmpm5::R) reader structure"]
impl crate::Readable for CMPM5 {}
#[doc = "`write(|w| ..)` method takes [cmpm5::W](cmpm5::W) writer structure"]
impl crate::Writable for CMPM5 {}
#[doc = "PWM Comparison 5 Mode Register"]
pub mod cmpm5;
#[doc = "PWM Comparison 5 Mode Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpmupd5](cmpmupd5) module"]
pub type CMPMUPD5 = crate::Reg<u32, _CMPMUPD5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPMUPD5;
#[doc = "`write(|w| ..)` method takes [cmpmupd5::W](cmpmupd5::W) writer structure"]
impl crate::Writable for CMPMUPD5 {}
#[doc = "PWM Comparison 5 Mode Update Register"]
pub mod cmpmupd5;
#[doc = "PWM Comparison 6 Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpv6](cmpv6) module"]
pub type CMPV6 = crate::Reg<u32, _CMPV6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPV6;
#[doc = "`read()` method returns [cmpv6::R](cmpv6::R) reader structure"]
impl crate::Readable for CMPV6 {}
#[doc = "`write(|w| ..)` method takes [cmpv6::W](cmpv6::W) writer structure"]
impl crate::Writable for CMPV6 {}
#[doc = "PWM Comparison 6 Value Register"]
pub mod cmpv6;
#[doc = "PWM Comparison 6 Value Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpvupd6](cmpvupd6) module"]
pub type CMPVUPD6 = crate::Reg<u32, _CMPVUPD6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPVUPD6;
#[doc = "`write(|w| ..)` method takes [cmpvupd6::W](cmpvupd6::W) writer structure"]
impl crate::Writable for CMPVUPD6 {}
#[doc = "PWM Comparison 6 Value Update Register"]
pub mod cmpvupd6;
#[doc = "PWM Comparison 6 Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpm6](cmpm6) module"]
pub type CMPM6 = crate::Reg<u32, _CMPM6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPM6;
#[doc = "`read()` method returns [cmpm6::R](cmpm6::R) reader structure"]
impl crate::Readable for CMPM6 {}
#[doc = "`write(|w| ..)` method takes [cmpm6::W](cmpm6::W) writer structure"]
impl crate::Writable for CMPM6 {}
#[doc = "PWM Comparison 6 Mode Register"]
pub mod cmpm6;
#[doc = "PWM Comparison 6 Mode Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpmupd6](cmpmupd6) module"]
pub type CMPMUPD6 = crate::Reg<u32, _CMPMUPD6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPMUPD6;
#[doc = "`write(|w| ..)` method takes [cmpmupd6::W](cmpmupd6::W) writer structure"]
impl crate::Writable for CMPMUPD6 {}
#[doc = "PWM Comparison 6 Mode Update Register"]
pub mod cmpmupd6;
#[doc = "PWM Comparison 7 Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpv7](cmpv7) module"]
pub type CMPV7 = crate::Reg<u32, _CMPV7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPV7;
#[doc = "`read()` method returns [cmpv7::R](cmpv7::R) reader structure"]
impl crate::Readable for CMPV7 {}
#[doc = "`write(|w| ..)` method takes [cmpv7::W](cmpv7::W) writer structure"]
impl crate::Writable for CMPV7 {}
#[doc = "PWM Comparison 7 Value Register"]
pub mod cmpv7;
#[doc = "PWM Comparison 7 Value Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpvupd7](cmpvupd7) module"]
pub type CMPVUPD7 = crate::Reg<u32, _CMPVUPD7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPVUPD7;
#[doc = "`write(|w| ..)` method takes [cmpvupd7::W](cmpvupd7::W) writer structure"]
impl crate::Writable for CMPVUPD7 {}
#[doc = "PWM Comparison 7 Value Update Register"]
pub mod cmpvupd7;
#[doc = "PWM Comparison 7 Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpm7](cmpm7) module"]
pub type CMPM7 = crate::Reg<u32, _CMPM7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPM7;
#[doc = "`read()` method returns [cmpm7::R](cmpm7::R) reader structure"]
impl crate::Readable for CMPM7 {}
#[doc = "`write(|w| ..)` method takes [cmpm7::W](cmpm7::W) writer structure"]
impl crate::Writable for CMPM7 {}
#[doc = "PWM Comparison 7 Mode Register"]
pub mod cmpm7;
#[doc = "PWM Comparison 7 Mode Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpmupd7](cmpmupd7) module"]
pub type CMPMUPD7 = crate::Reg<u32, _CMPMUPD7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPMUPD7;
#[doc = "`write(|w| ..)` method takes [cmpmupd7::W](cmpmupd7::W) writer structure"]
impl crate::Writable for CMPMUPD7 {}
#[doc = "PWM Comparison 7 Mode Update Register"]
pub mod cmpmupd7;
#[doc = "PWM Channel Mode Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr0](cmr0) module"]
pub type CMR0 = crate::Reg<u32, _CMR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR0;
#[doc = "`read()` method returns [cmr0::R](cmr0::R) reader structure"]
impl crate::Readable for CMR0 {}
#[doc = "`write(|w| ..)` method takes [cmr0::W](cmr0::W) writer structure"]
impl crate::Writable for CMR0 {}
#[doc = "PWM Channel Mode Register (ch_num = 0)"]
pub mod cmr0;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdty0](cdty0) module"]
pub type CDTY0 = crate::Reg<u32, _CDTY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDTY0;
#[doc = "`read()` method returns [cdty0::R](cdty0::R) reader structure"]
impl crate::Readable for CDTY0 {}
#[doc = "`write(|w| ..)` method takes [cdty0::W](cdty0::W) writer structure"]
impl crate::Writable for CDTY0 {}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 0)"]
pub mod cdty0;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdtyupd0](cdtyupd0) module"]
pub type CDTYUPD0 = crate::Reg<u32, _CDTYUPD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDTYUPD0;
#[doc = "`write(|w| ..)` method takes [cdtyupd0::W](cdtyupd0::W) writer structure"]
impl crate::Writable for CDTYUPD0 {}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 0)"]
pub mod cdtyupd0;
#[doc = "PWM Channel Period Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprd0](cprd0) module"]
pub type CPRD0 = crate::Reg<u32, _CPRD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPRD0;
#[doc = "`read()` method returns [cprd0::R](cprd0::R) reader structure"]
impl crate::Readable for CPRD0 {}
#[doc = "`write(|w| ..)` method takes [cprd0::W](cprd0::W) writer structure"]
impl crate::Writable for CPRD0 {}
#[doc = "PWM Channel Period Register (ch_num = 0)"]
pub mod cprd0;
#[doc = "PWM Channel Period Update Register (ch_num = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprdupd0](cprdupd0) module"]
pub type CPRDUPD0 = crate::Reg<u32, _CPRDUPD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPRDUPD0;
#[doc = "`write(|w| ..)` method takes [cprdupd0::W](cprdupd0::W) writer structure"]
impl crate::Writable for CPRDUPD0 {}
#[doc = "PWM Channel Period Update Register (ch_num = 0)"]
pub mod cprdupd0;
#[doc = "PWM Channel Counter Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccnt0](ccnt0) module"]
pub type CCNT0 = crate::Reg<u32, _CCNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCNT0;
#[doc = "`read()` method returns [ccnt0::R](ccnt0::R) reader structure"]
impl crate::Readable for CCNT0 {}
#[doc = "PWM Channel Counter Register (ch_num = 0)"]
pub mod ccnt0;
#[doc = "PWM Channel Dead Time Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt0](dt0) module"]
pub type DT0 = crate::Reg<u32, _DT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DT0;
#[doc = "`read()` method returns [dt0::R](dt0::R) reader structure"]
impl crate::Readable for DT0 {}
#[doc = "`write(|w| ..)` method takes [dt0::W](dt0::W) writer structure"]
impl crate::Writable for DT0 {}
#[doc = "PWM Channel Dead Time Register (ch_num = 0)"]
pub mod dt0;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtupd0](dtupd0) module"]
pub type DTUPD0 = crate::Reg<u32, _DTUPD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTUPD0;
#[doc = "`write(|w| ..)` method takes [dtupd0::W](dtupd0::W) writer structure"]
impl crate::Writable for DTUPD0 {}
#[doc = "PWM Channel Dead Time Update Register (ch_num = 0)"]
pub mod dtupd0;
#[doc = "PWM Channel Mode Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr1](cmr1) module"]
pub type CMR1 = crate::Reg<u32, _CMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR1;
#[doc = "`read()` method returns [cmr1::R](cmr1::R) reader structure"]
impl crate::Readable for CMR1 {}
#[doc = "`write(|w| ..)` method takes [cmr1::W](cmr1::W) writer structure"]
impl crate::Writable for CMR1 {}
#[doc = "PWM Channel Mode Register (ch_num = 1)"]
pub mod cmr1;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdty1](cdty1) module"]
pub type CDTY1 = crate::Reg<u32, _CDTY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDTY1;
#[doc = "`read()` method returns [cdty1::R](cdty1::R) reader structure"]
impl crate::Readable for CDTY1 {}
#[doc = "`write(|w| ..)` method takes [cdty1::W](cdty1::W) writer structure"]
impl crate::Writable for CDTY1 {}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 1)"]
pub mod cdty1;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdtyupd1](cdtyupd1) module"]
pub type CDTYUPD1 = crate::Reg<u32, _CDTYUPD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDTYUPD1;
#[doc = "`write(|w| ..)` method takes [cdtyupd1::W](cdtyupd1::W) writer structure"]
impl crate::Writable for CDTYUPD1 {}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 1)"]
pub mod cdtyupd1;
#[doc = "PWM Channel Period Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprd1](cprd1) module"]
pub type CPRD1 = crate::Reg<u32, _CPRD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPRD1;
#[doc = "`read()` method returns [cprd1::R](cprd1::R) reader structure"]
impl crate::Readable for CPRD1 {}
#[doc = "`write(|w| ..)` method takes [cprd1::W](cprd1::W) writer structure"]
impl crate::Writable for CPRD1 {}
#[doc = "PWM Channel Period Register (ch_num = 1)"]
pub mod cprd1;
#[doc = "PWM Channel Period Update Register (ch_num = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprdupd1](cprdupd1) module"]
pub type CPRDUPD1 = crate::Reg<u32, _CPRDUPD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPRDUPD1;
#[doc = "`write(|w| ..)` method takes [cprdupd1::W](cprdupd1::W) writer structure"]
impl crate::Writable for CPRDUPD1 {}
#[doc = "PWM Channel Period Update Register (ch_num = 1)"]
pub mod cprdupd1;
#[doc = "PWM Channel Counter Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccnt1](ccnt1) module"]
pub type CCNT1 = crate::Reg<u32, _CCNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCNT1;
#[doc = "`read()` method returns [ccnt1::R](ccnt1::R) reader structure"]
impl crate::Readable for CCNT1 {}
#[doc = "PWM Channel Counter Register (ch_num = 1)"]
pub mod ccnt1;
#[doc = "PWM Channel Dead Time Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt1](dt1) module"]
pub type DT1 = crate::Reg<u32, _DT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DT1;
#[doc = "`read()` method returns [dt1::R](dt1::R) reader structure"]
impl crate::Readable for DT1 {}
#[doc = "`write(|w| ..)` method takes [dt1::W](dt1::W) writer structure"]
impl crate::Writable for DT1 {}
#[doc = "PWM Channel Dead Time Register (ch_num = 1)"]
pub mod dt1;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtupd1](dtupd1) module"]
pub type DTUPD1 = crate::Reg<u32, _DTUPD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTUPD1;
#[doc = "`write(|w| ..)` method takes [dtupd1::W](dtupd1::W) writer structure"]
impl crate::Writable for DTUPD1 {}
#[doc = "PWM Channel Dead Time Update Register (ch_num = 1)"]
pub mod dtupd1;
#[doc = "PWM Channel Mode Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr2](cmr2) module"]
pub type CMR2 = crate::Reg<u32, _CMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR2;
#[doc = "`read()` method returns [cmr2::R](cmr2::R) reader structure"]
impl crate::Readable for CMR2 {}
#[doc = "`write(|w| ..)` method takes [cmr2::W](cmr2::W) writer structure"]
impl crate::Writable for CMR2 {}
#[doc = "PWM Channel Mode Register (ch_num = 2)"]
pub mod cmr2;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdty2](cdty2) module"]
pub type CDTY2 = crate::Reg<u32, _CDTY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDTY2;
#[doc = "`read()` method returns [cdty2::R](cdty2::R) reader structure"]
impl crate::Readable for CDTY2 {}
#[doc = "`write(|w| ..)` method takes [cdty2::W](cdty2::W) writer structure"]
impl crate::Writable for CDTY2 {}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 2)"]
pub mod cdty2;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdtyupd2](cdtyupd2) module"]
pub type CDTYUPD2 = crate::Reg<u32, _CDTYUPD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDTYUPD2;
#[doc = "`write(|w| ..)` method takes [cdtyupd2::W](cdtyupd2::W) writer structure"]
impl crate::Writable for CDTYUPD2 {}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 2)"]
pub mod cdtyupd2;
#[doc = "PWM Channel Period Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprd2](cprd2) module"]
pub type CPRD2 = crate::Reg<u32, _CPRD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPRD2;
#[doc = "`read()` method returns [cprd2::R](cprd2::R) reader structure"]
impl crate::Readable for CPRD2 {}
#[doc = "`write(|w| ..)` method takes [cprd2::W](cprd2::W) writer structure"]
impl crate::Writable for CPRD2 {}
#[doc = "PWM Channel Period Register (ch_num = 2)"]
pub mod cprd2;
#[doc = "PWM Channel Period Update Register (ch_num = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprdupd2](cprdupd2) module"]
pub type CPRDUPD2 = crate::Reg<u32, _CPRDUPD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPRDUPD2;
#[doc = "`write(|w| ..)` method takes [cprdupd2::W](cprdupd2::W) writer structure"]
impl crate::Writable for CPRDUPD2 {}
#[doc = "PWM Channel Period Update Register (ch_num = 2)"]
pub mod cprdupd2;
#[doc = "PWM Channel Counter Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccnt2](ccnt2) module"]
pub type CCNT2 = crate::Reg<u32, _CCNT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCNT2;
#[doc = "`read()` method returns [ccnt2::R](ccnt2::R) reader structure"]
impl crate::Readable for CCNT2 {}
#[doc = "PWM Channel Counter Register (ch_num = 2)"]
pub mod ccnt2;
#[doc = "PWM Channel Dead Time Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt2](dt2) module"]
pub type DT2 = crate::Reg<u32, _DT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DT2;
#[doc = "`read()` method returns [dt2::R](dt2::R) reader structure"]
impl crate::Readable for DT2 {}
#[doc = "`write(|w| ..)` method takes [dt2::W](dt2::W) writer structure"]
impl crate::Writable for DT2 {}
#[doc = "PWM Channel Dead Time Register (ch_num = 2)"]
pub mod dt2;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtupd2](dtupd2) module"]
pub type DTUPD2 = crate::Reg<u32, _DTUPD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTUPD2;
#[doc = "`write(|w| ..)` method takes [dtupd2::W](dtupd2::W) writer structure"]
impl crate::Writable for DTUPD2 {}
#[doc = "PWM Channel Dead Time Update Register (ch_num = 2)"]
pub mod dtupd2;
#[doc = "PWM Channel Mode Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr3](cmr3) module"]
pub type CMR3 = crate::Reg<u32, _CMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR3;
#[doc = "`read()` method returns [cmr3::R](cmr3::R) reader structure"]
impl crate::Readable for CMR3 {}
#[doc = "`write(|w| ..)` method takes [cmr3::W](cmr3::W) writer structure"]
impl crate::Writable for CMR3 {}
#[doc = "PWM Channel Mode Register (ch_num = 3)"]
pub mod cmr3;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdty3](cdty3) module"]
pub type CDTY3 = crate::Reg<u32, _CDTY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDTY3;
#[doc = "`read()` method returns [cdty3::R](cdty3::R) reader structure"]
impl crate::Readable for CDTY3 {}
#[doc = "`write(|w| ..)` method takes [cdty3::W](cdty3::W) writer structure"]
impl crate::Writable for CDTY3 {}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 3)"]
pub mod cdty3;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 3)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdtyupd3](cdtyupd3) module"]
pub type CDTYUPD3 = crate::Reg<u32, _CDTYUPD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDTYUPD3;
#[doc = "`write(|w| ..)` method takes [cdtyupd3::W](cdtyupd3::W) writer structure"]
impl crate::Writable for CDTYUPD3 {}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 3)"]
pub mod cdtyupd3;
#[doc = "PWM Channel Period Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprd3](cprd3) module"]
pub type CPRD3 = crate::Reg<u32, _CPRD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPRD3;
#[doc = "`read()` method returns [cprd3::R](cprd3::R) reader structure"]
impl crate::Readable for CPRD3 {}
#[doc = "`write(|w| ..)` method takes [cprd3::W](cprd3::W) writer structure"]
impl crate::Writable for CPRD3 {}
#[doc = "PWM Channel Period Register (ch_num = 3)"]
pub mod cprd3;
#[doc = "PWM Channel Period Update Register (ch_num = 3)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprdupd3](cprdupd3) module"]
pub type CPRDUPD3 = crate::Reg<u32, _CPRDUPD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPRDUPD3;
#[doc = "`write(|w| ..)` method takes [cprdupd3::W](cprdupd3::W) writer structure"]
impl crate::Writable for CPRDUPD3 {}
#[doc = "PWM Channel Period Update Register (ch_num = 3)"]
pub mod cprdupd3;
#[doc = "PWM Channel Counter Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccnt3](ccnt3) module"]
pub type CCNT3 = crate::Reg<u32, _CCNT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCNT3;
#[doc = "`read()` method returns [ccnt3::R](ccnt3::R) reader structure"]
impl crate::Readable for CCNT3 {}
#[doc = "PWM Channel Counter Register (ch_num = 3)"]
pub mod ccnt3;
#[doc = "PWM Channel Dead Time Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt3](dt3) module"]
pub type DT3 = crate::Reg<u32, _DT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DT3;
#[doc = "`read()` method returns [dt3::R](dt3::R) reader structure"]
impl crate::Readable for DT3 {}
#[doc = "`write(|w| ..)` method takes [dt3::W](dt3::W) writer structure"]
impl crate::Writable for DT3 {}
#[doc = "PWM Channel Dead Time Register (ch_num = 3)"]
pub mod dt3;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 3)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtupd3](dtupd3) module"]
pub type DTUPD3 = crate::Reg<u32, _DTUPD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTUPD3;
#[doc = "`write(|w| ..)` method takes [dtupd3::W](dtupd3::W) writer structure"]
impl crate::Writable for DTUPD3 {}
#[doc = "PWM Channel Dead Time Update Register (ch_num = 3)"]
pub mod dtupd3;
#[doc = "PWM Channel Mode Update Register (ch_num = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmupd0](cmupd0) module"]
pub type CMUPD0 = crate::Reg<u32, _CMUPD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMUPD0;
#[doc = "`write(|w| ..)` method takes [cmupd0::W](cmupd0::W) writer structure"]
impl crate::Writable for CMUPD0 {}
#[doc = "PWM Channel Mode Update Register (ch_num = 0)"]
pub mod cmupd0;
#[doc = "PWM Channel Additional Edge Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cae0](cae0) module"]
pub type CAE0 = crate::Reg<u32, _CAE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAE0;
#[doc = "`read()` method returns [cae0::R](cae0::R) reader structure"]
impl crate::Readable for CAE0 {}
#[doc = "`write(|w| ..)` method takes [cae0::W](cae0::W) writer structure"]
impl crate::Writable for CAE0 {}
#[doc = "PWM Channel Additional Edge Register (ch_num = 0)"]
pub mod cae0;
#[doc = "PWM Channel Additional Edge Update Register (ch_num = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caeupd0](caeupd0) module"]
pub type CAEUPD0 = crate::Reg<u32, _CAEUPD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAEUPD0;
#[doc = "`write(|w| ..)` method takes [caeupd0::W](caeupd0::W) writer structure"]
impl crate::Writable for CAEUPD0 {}
#[doc = "PWM Channel Additional Edge Update Register (ch_num = 0)"]
pub mod caeupd0;
#[doc = "PWM Channel Mode Update Register (ch_num = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmupd1](cmupd1) module"]
pub type CMUPD1 = crate::Reg<u32, _CMUPD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMUPD1;
#[doc = "`write(|w| ..)` method takes [cmupd1::W](cmupd1::W) writer structure"]
impl crate::Writable for CMUPD1 {}
#[doc = "PWM Channel Mode Update Register (ch_num = 1)"]
pub mod cmupd1;
#[doc = "PWM Channel Additional Edge Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cae1](cae1) module"]
pub type CAE1 = crate::Reg<u32, _CAE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAE1;
#[doc = "`read()` method returns [cae1::R](cae1::R) reader structure"]
impl crate::Readable for CAE1 {}
#[doc = "`write(|w| ..)` method takes [cae1::W](cae1::W) writer structure"]
impl crate::Writable for CAE1 {}
#[doc = "PWM Channel Additional Edge Register (ch_num = 1)"]
pub mod cae1;
#[doc = "PWM Channel Additional Edge Update Register (ch_num = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caeupd1](caeupd1) module"]
pub type CAEUPD1 = crate::Reg<u32, _CAEUPD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAEUPD1;
#[doc = "`write(|w| ..)` method takes [caeupd1::W](caeupd1::W) writer structure"]
impl crate::Writable for CAEUPD1 {}
#[doc = "PWM Channel Additional Edge Update Register (ch_num = 1)"]
pub mod caeupd1;
#[doc = "PWM Channel Mode Update Register (ch_num = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmupd2](cmupd2) module"]
pub type CMUPD2 = crate::Reg<u32, _CMUPD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMUPD2;
#[doc = "`write(|w| ..)` method takes [cmupd2::W](cmupd2::W) writer structure"]
impl crate::Writable for CMUPD2 {}
#[doc = "PWM Channel Mode Update Register (ch_num = 2)"]
pub mod cmupd2;
#[doc = "PWM Channel Additional Edge Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cae2](cae2) module"]
pub type CAE2 = crate::Reg<u32, _CAE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAE2;
#[doc = "`read()` method returns [cae2::R](cae2::R) reader structure"]
impl crate::Readable for CAE2 {}
#[doc = "`write(|w| ..)` method takes [cae2::W](cae2::W) writer structure"]
impl crate::Writable for CAE2 {}
#[doc = "PWM Channel Additional Edge Register (ch_num = 2)"]
pub mod cae2;
#[doc = "PWM Channel Additional Edge Update Register (ch_num = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caeupd2](caeupd2) module"]
pub type CAEUPD2 = crate::Reg<u32, _CAEUPD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAEUPD2;
#[doc = "`write(|w| ..)` method takes [caeupd2::W](caeupd2::W) writer structure"]
impl crate::Writable for CAEUPD2 {}
#[doc = "PWM Channel Additional Edge Update Register (ch_num = 2)"]
pub mod caeupd2;
#[doc = "PWM Channel Mode Update Register (ch_num = 3)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmupd3](cmupd3) module"]
pub type CMUPD3 = crate::Reg<u32, _CMUPD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMUPD3;
#[doc = "`write(|w| ..)` method takes [cmupd3::W](cmupd3::W) writer structure"]
impl crate::Writable for CMUPD3 {}
#[doc = "PWM Channel Mode Update Register (ch_num = 3)"]
pub mod cmupd3;
#[doc = "PWM Channel Additional Edge Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cae3](cae3) module"]
pub type CAE3 = crate::Reg<u32, _CAE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAE3;
#[doc = "`read()` method returns [cae3::R](cae3::R) reader structure"]
impl crate::Readable for CAE3 {}
#[doc = "`write(|w| ..)` method takes [cae3::W](cae3::W) writer structure"]
impl crate::Writable for CAE3 {}
#[doc = "PWM Channel Additional Edge Register (ch_num = 3)"]
pub mod cae3;
#[doc = "PWM Channel Additional Edge Update Register (ch_num = 3)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caeupd3](caeupd3) module"]
pub type CAEUPD3 = crate::Reg<u32, _CAEUPD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAEUPD3;
#[doc = "`write(|w| ..)` method takes [caeupd3::W](caeupd3::W) writer structure"]
impl crate::Writable for CAEUPD3 {}
#[doc = "PWM Channel Additional Edge Update Register (ch_num = 3)"]
pub mod caeupd3;
#[doc = "Transmit Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpr](tpr) module"]
pub type TPR = crate::Reg<u32, _TPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPR;
#[doc = "`read()` method returns [tpr::R](tpr::R) reader structure"]
impl crate::Readable for TPR {}
#[doc = "`write(|w| ..)` method takes [tpr::W](tpr::W) writer structure"]
impl crate::Writable for TPR {}
#[doc = "Transmit Pointer Register"]
pub mod tpr;
#[doc = "Transmit Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](tcr) module"]
pub type TCR = crate::Reg<u32, _TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR;
#[doc = "`read()` method returns [tcr::R](tcr::R) reader structure"]
impl crate::Readable for TCR {}
#[doc = "`write(|w| ..)` method takes [tcr::W](tcr::W) writer structure"]
impl crate::Writable for TCR {}
#[doc = "Transmit Counter Register"]
pub mod tcr;
#[doc = "Transmit Next Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tnpr](tnpr) module"]
pub type TNPR = crate::Reg<u32, _TNPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TNPR;
#[doc = "`read()` method returns [tnpr::R](tnpr::R) reader structure"]
impl crate::Readable for TNPR {}
#[doc = "`write(|w| ..)` method takes [tnpr::W](tnpr::W) writer structure"]
impl crate::Writable for TNPR {}
#[doc = "Transmit Next Pointer Register"]
pub mod tnpr;
#[doc = "Transmit Next Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tncr](tncr) module"]
pub type TNCR = crate::Reg<u32, _TNCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TNCR;
#[doc = "`read()` method returns [tncr::R](tncr::R) reader structure"]
impl crate::Readable for TNCR {}
#[doc = "`write(|w| ..)` method takes [tncr::W](tncr::W) writer structure"]
impl crate::Writable for TNCR {}
#[doc = "Transmit Next Counter Register"]
pub mod tncr;
#[doc = "Transfer Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptcr](ptcr) module"]
pub type PTCR = crate::Reg<u32, _PTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTCR;
#[doc = "`write(|w| ..)` method takes [ptcr::W](ptcr::W) writer structure"]
impl crate::Writable for PTCR {}
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "Transfer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr](ptsr) module"]
pub type PTSR = crate::Reg<u32, _PTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTSR;
#[doc = "`read()` method returns [ptsr::R](ptsr::R) reader structure"]
impl crate::Readable for PTSR {}
#[doc = "Transfer Status Register"]
pub mod ptsr;
