#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Frame Number Register"]
    pub frm_num: FRM_NUM,
    #[doc = "0x04 - Global State Register"]
    pub glb_stat: GLB_STAT,
    #[doc = "0x08 - Function Address Register"]
    pub faddr: FADDR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x1c - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x20 - Interrupt Clear Register"]
    pub icr: ICR,
    _reserved8: [u8; 4usize],
    #[doc = "0x28 - Reset Endpoint Register"]
    pub rst_ep: RST_EP,
    _reserved9: [u8; 4usize],
    _reserved_9_csr: [u8; 32usize],
    #[doc = "0x50 - Endpoint FIFO Data Register"]
    pub fdr: [FDR; 8],
    _reserved11: [u8; 4usize],
    #[doc = "0x74 - Transceiver Control Register"]
    pub txvc: TXVC,
}
impl RegisterBlock {
    #[doc = "0x30 - Endpoint Control and Status Register"]
    #[inline(always)]
    pub fn csr0_isoendpt(&self) -> &CSR0_ISOENDPT {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const CSR0_ISOENDPT) }
    }
    #[doc = "0x30 - Endpoint Control and Status Register"]
    #[inline(always)]
    pub fn csr0_isoendpt_mut(&self) -> &mut CSR0_ISOENDPT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(48usize) as *mut CSR0_ISOENDPT) }
    }
    #[doc = "0x30 - Endpoint Control and Status Register"]
    #[inline(always)]
    pub fn csr(&self) -> &[CSR; 8] {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const [CSR; 8]) }
    }
    #[doc = "0x30 - Endpoint Control and Status Register"]
    #[inline(always)]
    pub fn csr_mut(&self) -> &mut [CSR; 8] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(48usize) as *mut [CSR; 8]) }
    }
}
#[doc = "Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frm_num](frm_num) module"]
pub type FRM_NUM = crate::Reg<u32, _FRM_NUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRM_NUM;
#[doc = "`read()` method returns [frm_num::R](frm_num::R) reader structure"]
impl crate::Readable for FRM_NUM {}
#[doc = "Frame Number Register"]
pub mod frm_num;
#[doc = "Global State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [glb_stat](glb_stat) module"]
pub type GLB_STAT = crate::Reg<u32, _GLB_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLB_STAT;
#[doc = "`read()` method returns [glb_stat::R](glb_stat::R) reader structure"]
impl crate::Readable for GLB_STAT {}
#[doc = "`write(|w| ..)` method takes [glb_stat::W](glb_stat::W) writer structure"]
impl crate::Writable for GLB_STAT {}
#[doc = "Global State Register"]
pub mod glb_stat;
#[doc = "Function Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faddr](faddr) module"]
pub type FADDR = crate::Reg<u32, _FADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FADDR;
#[doc = "`read()` method returns [faddr::R](faddr::R) reader structure"]
impl crate::Readable for FADDR {}
#[doc = "`write(|w| ..)` method takes [faddr::W](faddr::W) writer structure"]
impl crate::Writable for FADDR {}
#[doc = "Function Address Register"]
pub mod faddr;
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
#[doc = "Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "Reset Endpoint Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_ep](rst_ep) module"]
pub type RST_EP = crate::Reg<u32, _RST_EP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RST_EP;
#[doc = "`read()` method returns [rst_ep::R](rst_ep::R) reader structure"]
impl crate::Readable for RST_EP {}
#[doc = "`write(|w| ..)` method takes [rst_ep::W](rst_ep::W) writer structure"]
impl crate::Writable for RST_EP {}
#[doc = "Reset Endpoint Register"]
pub mod rst_ep;
#[doc = "Endpoint Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "Endpoint Control and Status Register"]
pub mod csr;
#[doc = "Endpoint Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr0_isoendpt](csr0_isoendpt) module"]
pub type CSR0_ISOENDPT = crate::Reg<u32, _CSR0_ISOENDPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR0_ISOENDPT;
#[doc = "`read()` method returns [csr0_isoendpt::R](csr0_isoendpt::R) reader structure"]
impl crate::Readable for CSR0_ISOENDPT {}
#[doc = "`write(|w| ..)` method takes [csr0_isoendpt::W](csr0_isoendpt::W) writer structure"]
impl crate::Writable for CSR0_ISOENDPT {}
#[doc = "Endpoint Control and Status Register"]
pub mod csr0_isoendpt;
#[doc = "Endpoint FIFO Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdr](fdr) module"]
pub type FDR = crate::Reg<u32, _FDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDR;
#[doc = "`read()` method returns [fdr::R](fdr::R) reader structure"]
impl crate::Readable for FDR {}
#[doc = "`write(|w| ..)` method takes [fdr::W](fdr::W) writer structure"]
impl crate::Writable for FDR {}
#[doc = "Endpoint FIFO Data Register"]
pub mod fdr;
#[doc = "Transceiver Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txvc](txvc) module"]
pub type TXVC = crate::Reg<u32, _TXVC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXVC;
#[doc = "`read()` method returns [txvc::R](txvc::R) reader structure"]
impl crate::Readable for TXVC {}
#[doc = "`write(|w| ..)` method takes [txvc::W](txvc::W) writer structure"]
impl crate::Writable for TXVC {}
#[doc = "Transceiver Control Register"]
pub mod txvc;
