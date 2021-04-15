#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Descriptor Base Register"]
    pub dscr: DSCR,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - DMA Enable Register"]
    pub dmaen: DMAEN,
    #[doc = "0x0c - DMA Disable Register"]
    pub dmadis: DMADIS,
    #[doc = "0x10 - DMA Status Register"]
    pub dmasr: DMASR,
    #[doc = "0x14 - DMA Interrupt Enable Register"]
    pub dmaier: DMAIER,
    #[doc = "0x18 - DMA Interrupt Disable Register"]
    pub dmaidr: DMAIDR,
    #[doc = "0x1c - DMA Interrupt Mask Register"]
    pub dmaimr: DMAIMR,
    #[doc = "0x20 - DMA Interrupt Status Register"]
    pub dmaisr: DMAISR,
    _reserved8: [u8; 16usize],
    #[doc = "0x34 - Control Register"]
    pub cr: CR,
    #[doc = "0x38 - Mode Register"]
    pub mr: MR,
    #[doc = "0x3c - Status Register"]
    pub sr: SR,
    #[doc = "0x40 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x44 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x48 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x4c - Interrupt Status Register"]
    pub isr: ISR,
    _reserved15: [u8; 172usize],
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "DMA Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmadis](dmadis) module"]
pub type DMADIS = crate::Reg<u32, _DMADIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMADIS;
#[doc = "`write(|w| ..)` method takes [dmadis::W](dmadis::W) writer structure"]
impl crate::Writable for DMADIS {}
#[doc = "DMA Disable Register"]
pub mod dmadis;
#[doc = "DMA Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaen](dmaen) module"]
pub type DMAEN = crate::Reg<u32, _DMAEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAEN;
#[doc = "`write(|w| ..)` method takes [dmaen::W](dmaen::W) writer structure"]
impl crate::Writable for DMAEN {}
#[doc = "DMA Enable Register"]
pub mod dmaen;
#[doc = "DMA Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaidr](dmaidr) module"]
pub type DMAIDR = crate::Reg<u32, _DMAIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAIDR;
#[doc = "`write(|w| ..)` method takes [dmaidr::W](dmaidr::W) writer structure"]
impl crate::Writable for DMAIDR {}
#[doc = "DMA Interrupt Disable Register"]
pub mod dmaidr;
#[doc = "DMA Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaier](dmaier) module"]
pub type DMAIER = crate::Reg<u32, _DMAIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAIER;
#[doc = "`write(|w| ..)` method takes [dmaier::W](dmaier::W) writer structure"]
impl crate::Writable for DMAIER {}
#[doc = "DMA Interrupt Enable Register"]
pub mod dmaier;
#[doc = "DMA Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaimr](dmaimr) module"]
pub type DMAIMR = crate::Reg<u32, _DMAIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAIMR;
#[doc = "`read()` method returns [dmaimr::R](dmaimr::R) reader structure"]
impl crate::Readable for DMAIMR {}
#[doc = "DMA Interrupt Mask Register"]
pub mod dmaimr;
#[doc = "DMA Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaisr](dmaisr) module"]
pub type DMAISR = crate::Reg<u32, _DMAISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAISR;
#[doc = "`read()` method returns [dmaisr::R](dmaisr::R) reader structure"]
impl crate::Readable for DMAISR {}
#[doc = "DMA Interrupt Status Register"]
pub mod dmaisr;
#[doc = "DMA Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasr](dmasr) module"]
pub type DMASR = crate::Reg<u32, _DMASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASR;
#[doc = "`read()` method returns [dmasr::R](dmasr::R) reader structure"]
impl crate::Readable for DMASR {}
#[doc = "DMA Status Register"]
pub mod dmasr;
#[doc = "Descriptor Base Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dscr](dscr) module"]
pub type DSCR = crate::Reg<u32, _DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSCR;
#[doc = "`read()` method returns [dscr::R](dscr::R) reader structure"]
impl crate::Readable for DSCR {}
#[doc = "`write(|w| ..)` method takes [dscr::W](dscr::W) writer structure"]
impl crate::Writable for DSCR {}
#[doc = "Descriptor Base Register"]
pub mod dscr;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
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
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "Version Register"]
pub mod version;
