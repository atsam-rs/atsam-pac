#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Clock Mode Register"]
    pub cmr: CMR,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Receive Clock Mode Register"]
    pub rcmr: RCMR,
    #[doc = "0x14 - Receive Frame Mode Register"]
    pub rfmr: RFMR,
    #[doc = "0x18 - Transmit Clock Mode Register"]
    pub tcmr: TCMR,
    #[doc = "0x1c - Transmit Frame Mode Register"]
    pub tfmr: TFMR,
    #[doc = "0x20 - Receive Holding Register"]
    pub rhr: RHR,
    #[doc = "0x24 - Transmit Holding Register"]
    pub thr: THR,
    _reserved8: [u8; 8usize],
    #[doc = "0x30 - Receive Sync. Holding Register"]
    pub rshr: RSHR,
    #[doc = "0x34 - Transmit Sync. Holding Register"]
    pub tshr: TSHR,
    #[doc = "0x38 - Receive Compare 0 Register"]
    pub rc0r: RC0R,
    #[doc = "0x3c - Receive Compare 1 Register"]
    pub rc1r: RC1R,
    #[doc = "0x40 - Status Register"]
    pub sr: SR,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x48 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x4c - Interrupt Mask Register"]
    pub imr: IMR,
    _reserved16: [u8; 148usize],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: WPSR,
    _reserved18: [u8; 20usize],
    #[doc = "0x100 - Receive Pointer Register"]
    pub rpr: RPR,
    #[doc = "0x104 - Receive Counter Register"]
    pub rcr: RCR,
    #[doc = "0x108 - Transmit Pointer Register"]
    pub tpr: TPR,
    #[doc = "0x10c - Transmit Counter Register"]
    pub tcr: TCR,
    #[doc = "0x110 - Receive Next Pointer Register"]
    pub rnpr: RNPR,
    #[doc = "0x114 - Receive Next Counter Register"]
    pub rncr: RNCR,
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
#[doc = "Clock Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr](cmr) module"]
pub type CMR = crate::Reg<u32, _CMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR;
#[doc = "`read()` method returns [cmr::R](cmr::R) reader structure"]
impl crate::Readable for CMR {}
#[doc = "`write(|w| ..)` method takes [cmr::W](cmr::W) writer structure"]
impl crate::Writable for CMR {}
#[doc = "Clock Mode Register"]
pub mod cmr;
#[doc = "Receive Clock Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcmr](rcmr) module"]
pub type RCMR = crate::Reg<u32, _RCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCMR;
#[doc = "`read()` method returns [rcmr::R](rcmr::R) reader structure"]
impl crate::Readable for RCMR {}
#[doc = "`write(|w| ..)` method takes [rcmr::W](rcmr::W) writer structure"]
impl crate::Writable for RCMR {}
#[doc = "Receive Clock Mode Register"]
pub mod rcmr;
#[doc = "Receive Frame Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfmr](rfmr) module"]
pub type RFMR = crate::Reg<u32, _RFMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFMR;
#[doc = "`read()` method returns [rfmr::R](rfmr::R) reader structure"]
impl crate::Readable for RFMR {}
#[doc = "`write(|w| ..)` method takes [rfmr::W](rfmr::W) writer structure"]
impl crate::Writable for RFMR {}
#[doc = "Receive Frame Mode Register"]
pub mod rfmr;
#[doc = "Transmit Clock Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcmr](tcmr) module"]
pub type TCMR = crate::Reg<u32, _TCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCMR;
#[doc = "`read()` method returns [tcmr::R](tcmr::R) reader structure"]
impl crate::Readable for TCMR {}
#[doc = "`write(|w| ..)` method takes [tcmr::W](tcmr::W) writer structure"]
impl crate::Writable for TCMR {}
#[doc = "Transmit Clock Mode Register"]
pub mod tcmr;
#[doc = "Transmit Frame Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfmr](tfmr) module"]
pub type TFMR = crate::Reg<u32, _TFMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFMR;
#[doc = "`read()` method returns [tfmr::R](tfmr::R) reader structure"]
impl crate::Readable for TFMR {}
#[doc = "`write(|w| ..)` method takes [tfmr::W](tfmr::W) writer structure"]
impl crate::Writable for TFMR {}
#[doc = "Transmit Frame Mode Register"]
pub mod tfmr;
#[doc = "Receive Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rhr](rhr) module"]
pub type RHR = crate::Reg<u32, _RHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RHR;
#[doc = "`read()` method returns [rhr::R](rhr::R) reader structure"]
impl crate::Readable for RHR {}
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "Transmit Holding Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr](thr) module"]
pub type THR = crate::Reg<u32, _THR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THR;
#[doc = "`write(|w| ..)` method takes [thr::W](thr::W) writer structure"]
impl crate::Writable for THR {}
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "Receive Sync. Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rshr](rshr) module"]
pub type RSHR = crate::Reg<u32, _RSHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSHR;
#[doc = "`read()` method returns [rshr::R](rshr::R) reader structure"]
impl crate::Readable for RSHR {}
#[doc = "Receive Sync. Holding Register"]
pub mod rshr;
#[doc = "Transmit Sync. Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tshr](tshr) module"]
pub type TSHR = crate::Reg<u32, _TSHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSHR;
#[doc = "`read()` method returns [tshr::R](tshr::R) reader structure"]
impl crate::Readable for TSHR {}
#[doc = "`write(|w| ..)` method takes [tshr::W](tshr::W) writer structure"]
impl crate::Writable for TSHR {}
#[doc = "Transmit Sync. Holding Register"]
pub mod tshr;
#[doc = "Receive Compare 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc0r](rc0r) module"]
pub type RC0R = crate::Reg<u32, _RC0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RC0R;
#[doc = "`read()` method returns [rc0r::R](rc0r::R) reader structure"]
impl crate::Readable for RC0R {}
#[doc = "`write(|w| ..)` method takes [rc0r::W](rc0r::W) writer structure"]
impl crate::Writable for RC0R {}
#[doc = "Receive Compare 0 Register"]
pub mod rc0r;
#[doc = "Receive Compare 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc1r](rc1r) module"]
pub type RC1R = crate::Reg<u32, _RC1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RC1R;
#[doc = "`read()` method returns [rc1r::R](rc1r::R) reader structure"]
impl crate::Readable for RC1R {}
#[doc = "`write(|w| ..)` method takes [rc1r::W](rc1r::W) writer structure"]
impl crate::Writable for RC1R {}
#[doc = "Receive Compare 1 Register"]
pub mod rc1r;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register"]
pub mod sr;
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
#[doc = "Receive Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpr](rpr) module"]
pub type RPR = crate::Reg<u32, _RPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPR;
#[doc = "`read()` method returns [rpr::R](rpr::R) reader structure"]
impl crate::Readable for RPR {}
#[doc = "`write(|w| ..)` method takes [rpr::W](rpr::W) writer structure"]
impl crate::Writable for RPR {}
#[doc = "Receive Pointer Register"]
pub mod rpr;
#[doc = "Receive Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr](rcr) module"]
pub type RCR = crate::Reg<u32, _RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCR;
#[doc = "`read()` method returns [rcr::R](rcr::R) reader structure"]
impl crate::Readable for RCR {}
#[doc = "`write(|w| ..)` method takes [rcr::W](rcr::W) writer structure"]
impl crate::Writable for RCR {}
#[doc = "Receive Counter Register"]
pub mod rcr;
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
#[doc = "Receive Next Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rnpr](rnpr) module"]
pub type RNPR = crate::Reg<u32, _RNPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNPR;
#[doc = "`read()` method returns [rnpr::R](rnpr::R) reader structure"]
impl crate::Readable for RNPR {}
#[doc = "`write(|w| ..)` method takes [rnpr::W](rnpr::W) writer structure"]
impl crate::Writable for RNPR {}
#[doc = "Receive Next Pointer Register"]
pub mod rnpr;
#[doc = "Receive Next Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rncr](rncr) module"]
pub type RNCR = crate::Reg<u32, _RNCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNCR;
#[doc = "`read()` method returns [rncr::R](rncr::R) reader structure"]
impl crate::Readable for RNCR {}
#[doc = "`write(|w| ..)` method takes [rncr::W](rncr::W) writer structure"]
impl crate::Writable for RNCR {}
#[doc = "Receive Next Counter Register"]
pub mod rncr;
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
