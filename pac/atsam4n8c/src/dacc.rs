#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Conversion Data Register"]
    pub cdr: CDR,
    #[doc = "0x0c - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x10 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x14 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x18 - Interrupt Status Register"]
    pub isr: ISR,
    _reserved7: [u8; 200usize],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: WPSR,
    _reserved9: [u8; 28usize],
    #[doc = "0x108 - Transmit Pointer Register"]
    pub tpr: TPR,
    #[doc = "0x10c - Transmit Counter Register"]
    pub tcr: TCR,
    _reserved11: [u8; 8usize],
    #[doc = "0x118 - Transmit Next Pointer Register"]
    pub tnpr: TNPR,
    #[doc = "0x11c - Transmit Next Counter Register"]
    pub tncr: TNCR,
    #[doc = "0x120 - Transfer Control Register"]
    pub ptcr: PTCR,
    #[doc = "0x124 - Transfer Status Register"]
    pub ptsr: PTSR,
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](mr) module"]
pub type MR = crate::Reg<u32, _MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR;
#[doc = "`read()` method returns [mr::R](mr::R) reader structure"]
impl crate::Readable for MR {}
#[doc = "`write(|w| ..)` method takes [mr::W](mr::W) writer structure"]
impl crate::Writable for MR {}
#[doc = "Mode Register"]
pub mod mr;
#[doc = "Conversion Data Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdr](cdr) module"]
pub type CDR = crate::Reg<u32, _CDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDR;
#[doc = "`write(|w| ..)` method takes [cdr::W](cdr::W) writer structure"]
impl crate::Writable for CDR {}
#[doc = "Conversion Data Register"]
pub mod cdr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
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
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Write Protect Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpmr](wpmr) module"]
pub type WPMR = crate::Reg<u32, _WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPMR;
#[doc = "`read()` method returns [wpmr::R](wpmr::R) reader structure"]
impl crate::Readable for WPMR {}
#[doc = "`write(|w| ..)` method takes [wpmr::W](wpmr::W) writer structure"]
impl crate::Writable for WPMR {}
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "Write Protect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](wpsr) module"]
pub type WPSR = crate::Reg<u32, _WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSR;
#[doc = "`read()` method returns [wpsr::R](wpsr::R) reader structure"]
impl crate::Readable for WPSR {}
#[doc = "Write Protect Status Register"]
pub mod wpsr;
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
