#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Mode Register"]
    pub mr: MR,
    #[doc = "0x04 - PWM Enable Register"]
    pub ena: ENA,
    #[doc = "0x08 - PWM Disable Register"]
    pub dis: DIS,
    #[doc = "0x0c - PWM Status Register"]
    pub sr: SR,
    #[doc = "0x10 - PWM Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x14 - PWM Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x18 - PWM Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x1c - PWM Interrupt Status Register"]
    pub isr: ISR,
    _reserved8: [u8; 480usize],
    #[doc = "0x200 - PWM Channel Mode Register (ch_num = 0)"]
    pub cmr0: CMR0,
    #[doc = "0x204 - PWM Channel Duty Cycle Register (ch_num = 0)"]
    pub cdty0: CDTY0,
    #[doc = "0x208 - PWM Channel Period Register (ch_num = 0)"]
    pub cprd0: CPRD0,
    #[doc = "0x20c - PWM Channel Counter Register (ch_num = 0)"]
    pub ccnt0: CCNT0,
    #[doc = "0x210 - PWM Channel Update Register (ch_num = 0)"]
    pub cupd0: CUPD0,
    _reserved13: [u8; 12usize],
    #[doc = "0x220 - PWM Channel Mode Register (ch_num = 1)"]
    pub cmr1: CMR1,
    #[doc = "0x224 - PWM Channel Duty Cycle Register (ch_num = 1)"]
    pub cdty1: CDTY1,
    #[doc = "0x228 - PWM Channel Period Register (ch_num = 1)"]
    pub cprd1: CPRD1,
    #[doc = "0x22c - PWM Channel Counter Register (ch_num = 1)"]
    pub ccnt1: CCNT1,
    #[doc = "0x230 - PWM Channel Update Register (ch_num = 1)"]
    pub cupd1: CUPD1,
    _reserved18: [u8; 12usize],
    #[doc = "0x240 - PWM Channel Mode Register (ch_num = 2)"]
    pub cmr2: CMR2,
    #[doc = "0x244 - PWM Channel Duty Cycle Register (ch_num = 2)"]
    pub cdty2: CDTY2,
    #[doc = "0x248 - PWM Channel Period Register (ch_num = 2)"]
    pub cprd2: CPRD2,
    #[doc = "0x24c - PWM Channel Counter Register (ch_num = 2)"]
    pub ccnt2: CCNT2,
    #[doc = "0x250 - PWM Channel Update Register (ch_num = 2)"]
    pub cupd2: CUPD2,
    _reserved23: [u8; 12usize],
    #[doc = "0x260 - PWM Channel Mode Register (ch_num = 3)"]
    pub cmr3: CMR3,
    #[doc = "0x264 - PWM Channel Duty Cycle Register (ch_num = 3)"]
    pub cdty3: CDTY3,
    #[doc = "0x268 - PWM Channel Period Register (ch_num = 3)"]
    pub cprd3: CPRD3,
    #[doc = "0x26c - PWM Channel Counter Register (ch_num = 3)"]
    pub ccnt3: CCNT3,
    #[doc = "0x270 - PWM Channel Update Register (ch_num = 3)"]
    pub cupd3: CUPD3,
}
#[doc = "PWM Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](mr) module"]
pub type MR = crate::Reg<u32, _MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR;
#[doc = "`read()` method returns [mr::R](mr::R) reader structure"]
impl crate::Readable for MR {}
#[doc = "`write(|w| ..)` method takes [mr::W](mr::W) writer structure"]
impl crate::Writable for MR {}
#[doc = "PWM Mode Register"]
pub mod mr;
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
#[doc = "PWM Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "PWM Interrupt Enable Register"]
pub mod ier;
#[doc = "PWM Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "PWM Interrupt Disable Register"]
pub mod idr;
#[doc = "PWM Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "PWM Interrupt Mask Register"]
pub mod imr;
#[doc = "PWM Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "PWM Interrupt Status Register"]
pub mod isr;
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
#[doc = "PWM Channel Counter Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccnt0](ccnt0) module"]
pub type CCNT0 = crate::Reg<u32, _CCNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCNT0;
#[doc = "`read()` method returns [ccnt0::R](ccnt0::R) reader structure"]
impl crate::Readable for CCNT0 {}
#[doc = "PWM Channel Counter Register (ch_num = 0)"]
pub mod ccnt0;
#[doc = "PWM Channel Update Register (ch_num = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cupd0](cupd0) module"]
pub type CUPD0 = crate::Reg<u32, _CUPD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUPD0;
#[doc = "`write(|w| ..)` method takes [cupd0::W](cupd0::W) writer structure"]
impl crate::Writable for CUPD0 {}
#[doc = "PWM Channel Update Register (ch_num = 0)"]
pub mod cupd0;
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
#[doc = "PWM Channel Counter Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccnt1](ccnt1) module"]
pub type CCNT1 = crate::Reg<u32, _CCNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCNT1;
#[doc = "`read()` method returns [ccnt1::R](ccnt1::R) reader structure"]
impl crate::Readable for CCNT1 {}
#[doc = "PWM Channel Counter Register (ch_num = 1)"]
pub mod ccnt1;
#[doc = "PWM Channel Update Register (ch_num = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cupd1](cupd1) module"]
pub type CUPD1 = crate::Reg<u32, _CUPD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUPD1;
#[doc = "`write(|w| ..)` method takes [cupd1::W](cupd1::W) writer structure"]
impl crate::Writable for CUPD1 {}
#[doc = "PWM Channel Update Register (ch_num = 1)"]
pub mod cupd1;
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
#[doc = "PWM Channel Counter Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccnt2](ccnt2) module"]
pub type CCNT2 = crate::Reg<u32, _CCNT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCNT2;
#[doc = "`read()` method returns [ccnt2::R](ccnt2::R) reader structure"]
impl crate::Readable for CCNT2 {}
#[doc = "PWM Channel Counter Register (ch_num = 2)"]
pub mod ccnt2;
#[doc = "PWM Channel Update Register (ch_num = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cupd2](cupd2) module"]
pub type CUPD2 = crate::Reg<u32, _CUPD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUPD2;
#[doc = "`write(|w| ..)` method takes [cupd2::W](cupd2::W) writer structure"]
impl crate::Writable for CUPD2 {}
#[doc = "PWM Channel Update Register (ch_num = 2)"]
pub mod cupd2;
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
#[doc = "PWM Channel Counter Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccnt3](ccnt3) module"]
pub type CCNT3 = crate::Reg<u32, _CCNT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCNT3;
#[doc = "`read()` method returns [ccnt3::R](ccnt3::R) reader structure"]
impl crate::Readable for CCNT3 {}
#[doc = "PWM Channel Counter Register (ch_num = 3)"]
pub mod ccnt3;
#[doc = "PWM Channel Update Register (ch_num = 3)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cupd3](cupd3) module"]
pub type CUPD3 = crate::Reg<u32, _CUPD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUPD3;
#[doc = "`write(|w| ..)` method takes [cupd3::W](cupd3::W) writer structure"]
impl crate::Writable for CUPD3 {}
#[doc = "PWM Channel Update Register (ch_num = 3)"]
pub mod cupd3;
