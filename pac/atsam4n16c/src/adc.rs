#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Channel Sequence Register"]
    pub seqr: [SEQR; 2],
    #[doc = "0x10 - Channel Enable Register"]
    pub cher: CHER,
    #[doc = "0x14 - Channel Disable Register"]
    pub chdr: CHDR,
    #[doc = "0x18 - Channel Status Register"]
    pub chsr: CHSR,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - Last Converted Data Register"]
    pub lcdr: LCDR,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x30 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x34 - Temperature Sensor Mode Register"]
    pub tempmr: TEMPMR,
    #[doc = "0x38 - Temperature Compare Window Register"]
    pub tempcwr: TEMPCWR,
    #[doc = "0x3c - Overrun Status Register"]
    pub over: OVER,
    #[doc = "0x40 - Extended Mode Register"]
    pub emr: EMR,
    #[doc = "0x44 - Compare Window Register"]
    pub cwr: CWR,
    _reserved16: [u8; 8usize],
    #[doc = "0x50 - Channel Data Register"]
    pub cdr: [CDR; 17],
    #[doc = "0x94 - Analog Control Register"]
    pub acr: ACR,
    _reserved18: [u8; 76usize],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: WPSR,
    _reserved20: [u8; 20usize],
    #[doc = "0x100 - Receive Pointer Register"]
    pub rpr: RPR,
    #[doc = "0x104 - Receive Counter Register"]
    pub rcr: RCR,
    _reserved22: [u8; 8usize],
    #[doc = "0x110 - Receive Next Pointer Register"]
    pub rnpr: RNPR,
    #[doc = "0x114 - Receive Next Counter Register"]
    pub rncr: RNCR,
    _reserved24: [u8; 8usize],
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
#[doc = "Channel Sequence Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqr](seqr) module"]
pub type SEQR = crate::Reg<u32, _SEQR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQR;
#[doc = "`read()` method returns [seqr::R](seqr::R) reader structure"]
impl crate::Readable for SEQR {}
#[doc = "`write(|w| ..)` method takes [seqr::W](seqr::W) writer structure"]
impl crate::Writable for SEQR {}
#[doc = "Channel Sequence Register"]
pub mod seqr;
#[doc = "Channel Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cher](cher) module"]
pub type CHER = crate::Reg<u32, _CHER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHER;
#[doc = "`write(|w| ..)` method takes [cher::W](cher::W) writer structure"]
impl crate::Writable for CHER {}
#[doc = "Channel Enable Register"]
pub mod cher;
#[doc = "Channel Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdr](chdr) module"]
pub type CHDR = crate::Reg<u32, _CHDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDR;
#[doc = "`write(|w| ..)` method takes [chdr::W](chdr::W) writer structure"]
impl crate::Writable for CHDR {}
#[doc = "Channel Disable Register"]
pub mod chdr;
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chsr](chsr) module"]
pub type CHSR = crate::Reg<u32, _CHSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHSR;
#[doc = "`read()` method returns [chsr::R](chsr::R) reader structure"]
impl crate::Readable for CHSR {}
#[doc = "Channel Status Register"]
pub mod chsr;
#[doc = "Last Converted Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdr](lcdr) module"]
pub type LCDR = crate::Reg<u32, _LCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDR;
#[doc = "`read()` method returns [lcdr::R](lcdr::R) reader structure"]
impl crate::Readable for LCDR {}
#[doc = "Last Converted Data Register"]
pub mod lcdr;
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
#[doc = "Temperature Sensor Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempmr](tempmr) module"]
pub type TEMPMR = crate::Reg<u32, _TEMPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPMR;
#[doc = "`read()` method returns [tempmr::R](tempmr::R) reader structure"]
impl crate::Readable for TEMPMR {}
#[doc = "`write(|w| ..)` method takes [tempmr::W](tempmr::W) writer structure"]
impl crate::Writable for TEMPMR {}
#[doc = "Temperature Sensor Mode Register"]
pub mod tempmr;
#[doc = "Temperature Compare Window Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempcwr](tempcwr) module"]
pub type TEMPCWR = crate::Reg<u32, _TEMPCWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPCWR;
#[doc = "`read()` method returns [tempcwr::R](tempcwr::R) reader structure"]
impl crate::Readable for TEMPCWR {}
#[doc = "`write(|w| ..)` method takes [tempcwr::W](tempcwr::W) writer structure"]
impl crate::Writable for TEMPCWR {}
#[doc = "Temperature Compare Window Register"]
pub mod tempcwr;
#[doc = "Overrun Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [over](over) module"]
pub type OVER = crate::Reg<u32, _OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OVER;
#[doc = "`read()` method returns [over::R](over::R) reader structure"]
impl crate::Readable for OVER {}
#[doc = "Overrun Status Register"]
pub mod over;
#[doc = "Extended Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr](emr) module"]
pub type EMR = crate::Reg<u32, _EMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMR;
#[doc = "`read()` method returns [emr::R](emr::R) reader structure"]
impl crate::Readable for EMR {}
#[doc = "`write(|w| ..)` method takes [emr::W](emr::W) writer structure"]
impl crate::Writable for EMR {}
#[doc = "Extended Mode Register"]
pub mod emr;
#[doc = "Compare Window Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwr](cwr) module"]
pub type CWR = crate::Reg<u32, _CWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CWR;
#[doc = "`read()` method returns [cwr::R](cwr::R) reader structure"]
impl crate::Readable for CWR {}
#[doc = "`write(|w| ..)` method takes [cwr::W](cwr::W) writer structure"]
impl crate::Writable for CWR {}
#[doc = "Compare Window Register"]
pub mod cwr;
#[doc = "Channel Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdr](cdr) module"]
pub type CDR = crate::Reg<u32, _CDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDR;
#[doc = "`read()` method returns [cdr::R](cdr::R) reader structure"]
impl crate::Readable for CDR {}
#[doc = "Channel Data Register"]
pub mod cdr;
#[doc = "Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](acr) module"]
pub type ACR = crate::Reg<u32, _ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACR;
#[doc = "`read()` method returns [acr::R](acr::R) reader structure"]
impl crate::Readable for ACR {}
#[doc = "`write(|w| ..)` method takes [acr::W](acr::W) writer structure"]
impl crate::Writable for ACR {}
#[doc = "Analog Control Register"]
pub mod acr;
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
