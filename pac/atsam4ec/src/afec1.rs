#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AFEC Control Register"]
    pub cr: CR,
    #[doc = "0x04 - AFEC Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - AFEC Extended Mode Register"]
    pub emr: EMR,
    #[doc = "0x0c - AFEC Channel Sequence 1 Register"]
    pub seq1r: SEQ1R,
    #[doc = "0x10 - AFEC Channel Sequence 2 Register"]
    pub seq2r: SEQ2R,
    #[doc = "0x14 - AFEC Channel Enable Register"]
    pub cher: CHER,
    #[doc = "0x18 - AFEC Channel Disable Register"]
    pub chdr: CHDR,
    #[doc = "0x1c - AFEC Channel Status Register"]
    pub chsr: CHSR,
    #[doc = "0x20 - AFEC Last Converted Data Register"]
    pub lcdr: LCDR,
    #[doc = "0x24 - AFEC Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x28 - AFEC Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x2c - AFEC Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x30 - AFEC Interrupt Status Register"]
    pub isr: ISR,
    _reserved13: [u8; 24usize],
    #[doc = "0x4c - AFEC Overrun Status Register"]
    pub over: OVER,
    #[doc = "0x50 - AFEC Compare Window Register"]
    pub cwr: CWR,
    #[doc = "0x54 - AFEC Channel Gain Register"]
    pub cgr: CGR,
    _reserved16: [u8; 4usize],
    #[doc = "0x5c - AFEC Channel Calibration DC Offset Register"]
    pub cdor: CDOR,
    #[doc = "0x60 - AFEC Channel Differential Register"]
    pub diffr: DIFFR,
    #[doc = "0x64 - AFEC Channel Register Selection"]
    pub cselr: CSELR,
    #[doc = "0x68 - AFEC Channel Data Register"]
    pub cdr: CDR,
    #[doc = "0x6c - AFEC Channel Offset Compensation Register"]
    pub cocr: COCR,
    #[doc = "0x70 - AFEC Temperature Sensor Mode Register"]
    pub tempmr: TEMPMR,
    #[doc = "0x74 - AFEC Temperature Compare Window Register"]
    pub tempcwr: TEMPCWR,
    _reserved23: [u8; 28usize],
    #[doc = "0x94 - AFEC Analog Control Register"]
    pub acr: ACR,
    _reserved24: [u8; 56usize],
    #[doc = "0xd0 - AFEC Correction Select Register"]
    pub cosr: COSR,
    #[doc = "0xd4 - AFEC Correction Values Register"]
    pub cvr: CVR,
    #[doc = "0xd8 - AFEC Channel Error Correction Register"]
    pub cecr: CECR,
    _reserved27: [u8; 8usize],
    #[doc = "0xe4 - AFEC Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - AFEC Write Protection Status Register"]
    pub wpsr: WPSR,
    _reserved29: [u8; 20usize],
    #[doc = "0x100 - Receive Pointer Register"]
    pub rpr: RPR,
    #[doc = "0x104 - Receive Counter Register"]
    pub rcr: RCR,
    _reserved31: [u8; 8usize],
    #[doc = "0x110 - Receive Next Pointer Register"]
    pub rnpr: RNPR,
    #[doc = "0x114 - Receive Next Counter Register"]
    pub rncr: RNCR,
    _reserved33: [u8; 8usize],
    #[doc = "0x120 - Transfer Control Register"]
    pub ptcr: PTCR,
    #[doc = "0x124 - Transfer Status Register"]
    pub ptsr: PTSR,
}
#[doc = "AFEC Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "AFEC Control Register"]
pub mod cr;
#[doc = "AFEC Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](mr) module"]
pub type MR = crate::Reg<u32, _MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR;
#[doc = "`read()` method returns [mr::R](mr::R) reader structure"]
impl crate::Readable for MR {}
#[doc = "`write(|w| ..)` method takes [mr::W](mr::W) writer structure"]
impl crate::Writable for MR {}
#[doc = "AFEC Mode Register"]
pub mod mr;
#[doc = "AFEC Extended Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr](emr) module"]
pub type EMR = crate::Reg<u32, _EMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMR;
#[doc = "`read()` method returns [emr::R](emr::R) reader structure"]
impl crate::Readable for EMR {}
#[doc = "`write(|w| ..)` method takes [emr::W](emr::W) writer structure"]
impl crate::Writable for EMR {}
#[doc = "AFEC Extended Mode Register"]
pub mod emr;
#[doc = "AFEC Channel Sequence 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq1r](seq1r) module"]
pub type SEQ1R = crate::Reg<u32, _SEQ1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ1R;
#[doc = "`read()` method returns [seq1r::R](seq1r::R) reader structure"]
impl crate::Readable for SEQ1R {}
#[doc = "`write(|w| ..)` method takes [seq1r::W](seq1r::W) writer structure"]
impl crate::Writable for SEQ1R {}
#[doc = "AFEC Channel Sequence 1 Register"]
pub mod seq1r;
#[doc = "AFEC Channel Sequence 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq2r](seq2r) module"]
pub type SEQ2R = crate::Reg<u32, _SEQ2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ2R;
#[doc = "`read()` method returns [seq2r::R](seq2r::R) reader structure"]
impl crate::Readable for SEQ2R {}
#[doc = "`write(|w| ..)` method takes [seq2r::W](seq2r::W) writer structure"]
impl crate::Writable for SEQ2R {}
#[doc = "AFEC Channel Sequence 2 Register"]
pub mod seq2r;
#[doc = "AFEC Channel Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cher](cher) module"]
pub type CHER = crate::Reg<u32, _CHER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHER;
#[doc = "`write(|w| ..)` method takes [cher::W](cher::W) writer structure"]
impl crate::Writable for CHER {}
#[doc = "AFEC Channel Enable Register"]
pub mod cher;
#[doc = "AFEC Channel Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdr](chdr) module"]
pub type CHDR = crate::Reg<u32, _CHDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDR;
#[doc = "`write(|w| ..)` method takes [chdr::W](chdr::W) writer structure"]
impl crate::Writable for CHDR {}
#[doc = "AFEC Channel Disable Register"]
pub mod chdr;
#[doc = "AFEC Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chsr](chsr) module"]
pub type CHSR = crate::Reg<u32, _CHSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHSR;
#[doc = "`read()` method returns [chsr::R](chsr::R) reader structure"]
impl crate::Readable for CHSR {}
#[doc = "AFEC Channel Status Register"]
pub mod chsr;
#[doc = "AFEC Last Converted Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdr](lcdr) module"]
pub type LCDR = crate::Reg<u32, _LCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDR;
#[doc = "`read()` method returns [lcdr::R](lcdr::R) reader structure"]
impl crate::Readable for LCDR {}
#[doc = "AFEC Last Converted Data Register"]
pub mod lcdr;
#[doc = "AFEC Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "AFEC Interrupt Enable Register"]
pub mod ier;
#[doc = "AFEC Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "AFEC Interrupt Disable Register"]
pub mod idr;
#[doc = "AFEC Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "AFEC Interrupt Mask Register"]
pub mod imr;
#[doc = "AFEC Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "AFEC Interrupt Status Register"]
pub mod isr;
#[doc = "AFEC Overrun Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [over](over) module"]
pub type OVER = crate::Reg<u32, _OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OVER;
#[doc = "`read()` method returns [over::R](over::R) reader structure"]
impl crate::Readable for OVER {}
#[doc = "AFEC Overrun Status Register"]
pub mod over;
#[doc = "AFEC Compare Window Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwr](cwr) module"]
pub type CWR = crate::Reg<u32, _CWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CWR;
#[doc = "`read()` method returns [cwr::R](cwr::R) reader structure"]
impl crate::Readable for CWR {}
#[doc = "`write(|w| ..)` method takes [cwr::W](cwr::W) writer structure"]
impl crate::Writable for CWR {}
#[doc = "AFEC Compare Window Register"]
pub mod cwr;
#[doc = "AFEC Channel Gain Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgr](cgr) module"]
pub type CGR = crate::Reg<u32, _CGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGR;
#[doc = "`read()` method returns [cgr::R](cgr::R) reader structure"]
impl crate::Readable for CGR {}
#[doc = "`write(|w| ..)` method takes [cgr::W](cgr::W) writer structure"]
impl crate::Writable for CGR {}
#[doc = "AFEC Channel Gain Register"]
pub mod cgr;
#[doc = "AFEC Channel Calibration DC Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdor](cdor) module"]
pub type CDOR = crate::Reg<u32, _CDOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDOR;
#[doc = "`read()` method returns [cdor::R](cdor::R) reader structure"]
impl crate::Readable for CDOR {}
#[doc = "`write(|w| ..)` method takes [cdor::W](cdor::W) writer structure"]
impl crate::Writable for CDOR {}
#[doc = "AFEC Channel Calibration DC Offset Register"]
pub mod cdor;
#[doc = "AFEC Channel Differential Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diffr](diffr) module"]
pub type DIFFR = crate::Reg<u32, _DIFFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIFFR;
#[doc = "`read()` method returns [diffr::R](diffr::R) reader structure"]
impl crate::Readable for DIFFR {}
#[doc = "`write(|w| ..)` method takes [diffr::W](diffr::W) writer structure"]
impl crate::Writable for DIFFR {}
#[doc = "AFEC Channel Differential Register"]
pub mod diffr;
#[doc = "AFEC Channel Register Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cselr](cselr) module"]
pub type CSELR = crate::Reg<u32, _CSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSELR;
#[doc = "`read()` method returns [cselr::R](cselr::R) reader structure"]
impl crate::Readable for CSELR {}
#[doc = "`write(|w| ..)` method takes [cselr::W](cselr::W) writer structure"]
impl crate::Writable for CSELR {}
#[doc = "AFEC Channel Register Selection"]
pub mod cselr;
#[doc = "AFEC Channel Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdr](cdr) module"]
pub type CDR = crate::Reg<u32, _CDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDR;
#[doc = "`read()` method returns [cdr::R](cdr::R) reader structure"]
impl crate::Readable for CDR {}
#[doc = "AFEC Channel Data Register"]
pub mod cdr;
#[doc = "AFEC Channel Offset Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cocr](cocr) module"]
pub type COCR = crate::Reg<u32, _COCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COCR;
#[doc = "`read()` method returns [cocr::R](cocr::R) reader structure"]
impl crate::Readable for COCR {}
#[doc = "`write(|w| ..)` method takes [cocr::W](cocr::W) writer structure"]
impl crate::Writable for COCR {}
#[doc = "AFEC Channel Offset Compensation Register"]
pub mod cocr;
#[doc = "AFEC Temperature Sensor Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempmr](tempmr) module"]
pub type TEMPMR = crate::Reg<u32, _TEMPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPMR;
#[doc = "`read()` method returns [tempmr::R](tempmr::R) reader structure"]
impl crate::Readable for TEMPMR {}
#[doc = "`write(|w| ..)` method takes [tempmr::W](tempmr::W) writer structure"]
impl crate::Writable for TEMPMR {}
#[doc = "AFEC Temperature Sensor Mode Register"]
pub mod tempmr;
#[doc = "AFEC Temperature Compare Window Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempcwr](tempcwr) module"]
pub type TEMPCWR = crate::Reg<u32, _TEMPCWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPCWR;
#[doc = "`read()` method returns [tempcwr::R](tempcwr::R) reader structure"]
impl crate::Readable for TEMPCWR {}
#[doc = "`write(|w| ..)` method takes [tempcwr::W](tempcwr::W) writer structure"]
impl crate::Writable for TEMPCWR {}
#[doc = "AFEC Temperature Compare Window Register"]
pub mod tempcwr;
#[doc = "AFEC Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](acr) module"]
pub type ACR = crate::Reg<u32, _ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACR;
#[doc = "`read()` method returns [acr::R](acr::R) reader structure"]
impl crate::Readable for ACR {}
#[doc = "`write(|w| ..)` method takes [acr::W](acr::W) writer structure"]
impl crate::Writable for ACR {}
#[doc = "AFEC Analog Control Register"]
pub mod acr;
#[doc = "AFEC Correction Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cosr](cosr) module"]
pub type COSR = crate::Reg<u32, _COSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COSR;
#[doc = "`read()` method returns [cosr::R](cosr::R) reader structure"]
impl crate::Readable for COSR {}
#[doc = "`write(|w| ..)` method takes [cosr::W](cosr::W) writer structure"]
impl crate::Writable for COSR {}
#[doc = "AFEC Correction Select Register"]
pub mod cosr;
#[doc = "AFEC Correction Values Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cvr](cvr) module"]
pub type CVR = crate::Reg<u32, _CVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CVR;
#[doc = "`read()` method returns [cvr::R](cvr::R) reader structure"]
impl crate::Readable for CVR {}
#[doc = "`write(|w| ..)` method takes [cvr::W](cvr::W) writer structure"]
impl crate::Writable for CVR {}
#[doc = "AFEC Correction Values Register"]
pub mod cvr;
#[doc = "AFEC Channel Error Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cecr](cecr) module"]
pub type CECR = crate::Reg<u32, _CECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CECR;
#[doc = "`read()` method returns [cecr::R](cecr::R) reader structure"]
impl crate::Readable for CECR {}
#[doc = "`write(|w| ..)` method takes [cecr::W](cecr::W) writer structure"]
impl crate::Writable for CECR {}
#[doc = "AFEC Channel Error Correction Register"]
pub mod cecr;
#[doc = "AFEC Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpmr](wpmr) module"]
pub type WPMR = crate::Reg<u32, _WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPMR;
#[doc = "`read()` method returns [wpmr::R](wpmr::R) reader structure"]
impl crate::Readable for WPMR {}
#[doc = "`write(|w| ..)` method takes [wpmr::W](wpmr::W) writer structure"]
impl crate::Writable for WPMR {}
#[doc = "AFEC Write Protection Mode Register"]
pub mod wpmr;
#[doc = "AFEC Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](wpsr) module"]
pub type WPSR = crate::Reg<u32, _WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSR;
#[doc = "`read()` method returns [wpsr::R](wpsr::R) reader structure"]
impl crate::Readable for WPSR {}
#[doc = "AFEC Write Protection Status Register"]
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
