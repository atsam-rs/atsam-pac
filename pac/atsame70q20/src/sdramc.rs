#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDRAMC Mode Register"]
    pub mr: MR,
    #[doc = "0x04 - SDRAMC Refresh Timer Register"]
    pub tr: TR,
    #[doc = "0x08 - SDRAMC Configuration Register"]
    pub cr: CR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - SDRAMC Low Power Register"]
    pub lpr: LPR,
    #[doc = "0x14 - SDRAMC Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x18 - SDRAMC Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x1c - SDRAMC Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x20 - SDRAMC Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x24 - SDRAMC Memory Device Register"]
    pub mdr: MDR,
    #[doc = "0x28 - SDRAMC Configuration Register 1"]
    pub cfr1: CFR1,
    #[doc = "0x2c - SDRAMC OCMS Register"]
    pub ocms: OCMS,
    #[doc = "0x30 - SDRAMC OCMS KEY1 Register"]
    pub ocms_key1: OCMS_KEY1,
    #[doc = "0x34 - SDRAMC OCMS KEY2 Register"]
    pub ocms_key2: OCMS_KEY2,
}
#[doc = "SDRAMC Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](mr) module"]
pub type MR = crate::Reg<u32, _MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR;
#[doc = "`read()` method returns [mr::R](mr::R) reader structure"]
impl crate::Readable for MR {}
#[doc = "`write(|w| ..)` method takes [mr::W](mr::W) writer structure"]
impl crate::Writable for MR {}
#[doc = "SDRAMC Mode Register"]
pub mod mr;
#[doc = "SDRAMC Refresh Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr](tr) module"]
pub type TR = crate::Reg<u32, _TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR;
#[doc = "`read()` method returns [tr::R](tr::R) reader structure"]
impl crate::Readable for TR {}
#[doc = "`write(|w| ..)` method takes [tr::W](tr::W) writer structure"]
impl crate::Writable for TR {}
#[doc = "SDRAMC Refresh Timer Register"]
pub mod tr;
#[doc = "SDRAMC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "SDRAMC Configuration Register"]
pub mod cr;
#[doc = "SDRAMC Low Power Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpr](lpr) module"]
pub type LPR = crate::Reg<u32, _LPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPR;
#[doc = "`read()` method returns [lpr::R](lpr::R) reader structure"]
impl crate::Readable for LPR {}
#[doc = "`write(|w| ..)` method takes [lpr::W](lpr::W) writer structure"]
impl crate::Writable for LPR {}
#[doc = "SDRAMC Low Power Register"]
pub mod lpr;
#[doc = "SDRAMC Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "SDRAMC Interrupt Enable Register"]
pub mod ier;
#[doc = "SDRAMC Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "SDRAMC Interrupt Disable Register"]
pub mod idr;
#[doc = "SDRAMC Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "SDRAMC Interrupt Mask Register"]
pub mod imr;
#[doc = "SDRAMC Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "SDRAMC Interrupt Status Register"]
pub mod isr;
#[doc = "SDRAMC Memory Device Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdr](mdr) module"]
pub type MDR = crate::Reg<u32, _MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDR;
#[doc = "`read()` method returns [mdr::R](mdr::R) reader structure"]
impl crate::Readable for MDR {}
#[doc = "`write(|w| ..)` method takes [mdr::W](mdr::W) writer structure"]
impl crate::Writable for MDR {}
#[doc = "SDRAMC Memory Device Register"]
pub mod mdr;
#[doc = "SDRAMC Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfr1](cfr1) module"]
pub type CFR1 = crate::Reg<u32, _CFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFR1;
#[doc = "`read()` method returns [cfr1::R](cfr1::R) reader structure"]
impl crate::Readable for CFR1 {}
#[doc = "`write(|w| ..)` method takes [cfr1::W](cfr1::W) writer structure"]
impl crate::Writable for CFR1 {}
#[doc = "SDRAMC Configuration Register 1"]
pub mod cfr1;
#[doc = "SDRAMC OCMS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocms](ocms) module"]
pub type OCMS = crate::Reg<u32, _OCMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCMS;
#[doc = "`read()` method returns [ocms::R](ocms::R) reader structure"]
impl crate::Readable for OCMS {}
#[doc = "`write(|w| ..)` method takes [ocms::W](ocms::W) writer structure"]
impl crate::Writable for OCMS {}
#[doc = "SDRAMC OCMS Register"]
pub mod ocms;
#[doc = "SDRAMC OCMS KEY1 Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocms_key1](ocms_key1) module"]
pub type OCMS_KEY1 = crate::Reg<u32, _OCMS_KEY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCMS_KEY1;
#[doc = "`write(|w| ..)` method takes [ocms_key1::W](ocms_key1::W) writer structure"]
impl crate::Writable for OCMS_KEY1 {}
#[doc = "SDRAMC OCMS KEY1 Register"]
pub mod ocms_key1;
#[doc = "SDRAMC OCMS KEY2 Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocms_key2](ocms_key2) module"]
pub type OCMS_KEY2 = crate::Reg<u32, _OCMS_KEY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCMS_KEY2;
#[doc = "`write(|w| ..)` method takes [ocms_key2::W](ocms_key2::W) writer structure"]
impl crate::Writable for OCMS_KEY2 {}
#[doc = "SDRAMC OCMS KEY2 Register"]
pub mod ocms_key2;
