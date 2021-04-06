#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRCCU Descriptor Base Register"]
    pub dscr: DSCR,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - CRCCU DMA Enable Register"]
    pub dma_en: DMA_EN,
    #[doc = "0x0c - CRCCU DMA Disable Register"]
    pub dma_dis: DMA_DIS,
    #[doc = "0x10 - CRCCU DMA Status Register"]
    pub dma_sr: DMA_SR,
    #[doc = "0x14 - CRCCU DMA Interrupt Enable Register"]
    pub dma_ier: DMA_IER,
    #[doc = "0x18 - CRCCU DMA Interrupt Disable Register"]
    pub dma_idr: DMA_IDR,
    #[doc = "0x1c - CRCCU DMA Interrupt Mask Register"]
    pub dma_imr: DMA_IMR,
    #[doc = "0x20 - CRCCU DMA Interrupt Status Register"]
    pub dma_isr: DMA_ISR,
    _reserved8: [u8; 16usize],
    #[doc = "0x34 - CRCCU Control Register"]
    pub cr: CR,
    #[doc = "0x38 - CRCCU Mode Register"]
    pub mr: MR,
    #[doc = "0x3c - CRCCU Status Register"]
    pub sr: SR,
    #[doc = "0x40 - CRCCU Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x44 - CRCCU Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x48 - CRCCU Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x4c - CRCCU Interrupt Status Register"]
    pub isr: ISR,
}
#[doc = "CRCCU Descriptor Base Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dscr](dscr) module"]
pub type DSCR = crate::Reg<u32, _DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSCR;
#[doc = "`read()` method returns [dscr::R](dscr::R) reader structure"]
impl crate::Readable for DSCR {}
#[doc = "`write(|w| ..)` method takes [dscr::W](dscr::W) writer structure"]
impl crate::Writable for DSCR {}
#[doc = "CRCCU Descriptor Base Register"]
pub mod dscr;
#[doc = "CRCCU DMA Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_en](dma_en) module"]
pub type DMA_EN = crate::Reg<u32, _DMA_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_EN;
#[doc = "`write(|w| ..)` method takes [dma_en::W](dma_en::W) writer structure"]
impl crate::Writable for DMA_EN {}
#[doc = "CRCCU DMA Enable Register"]
pub mod dma_en;
#[doc = "CRCCU DMA Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_dis](dma_dis) module"]
pub type DMA_DIS = crate::Reg<u32, _DMA_DIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_DIS;
#[doc = "`write(|w| ..)` method takes [dma_dis::W](dma_dis::W) writer structure"]
impl crate::Writable for DMA_DIS {}
#[doc = "CRCCU DMA Disable Register"]
pub mod dma_dis;
#[doc = "CRCCU DMA Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_sr](dma_sr) module"]
pub type DMA_SR = crate::Reg<u32, _DMA_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_SR;
#[doc = "`read()` method returns [dma_sr::R](dma_sr::R) reader structure"]
impl crate::Readable for DMA_SR {}
#[doc = "CRCCU DMA Status Register"]
pub mod dma_sr;
#[doc = "CRCCU DMA Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_ier](dma_ier) module"]
pub type DMA_IER = crate::Reg<u32, _DMA_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IER;
#[doc = "`write(|w| ..)` method takes [dma_ier::W](dma_ier::W) writer structure"]
impl crate::Writable for DMA_IER {}
#[doc = "CRCCU DMA Interrupt Enable Register"]
pub mod dma_ier;
#[doc = "CRCCU DMA Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_idr](dma_idr) module"]
pub type DMA_IDR = crate::Reg<u32, _DMA_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IDR;
#[doc = "`write(|w| ..)` method takes [dma_idr::W](dma_idr::W) writer structure"]
impl crate::Writable for DMA_IDR {}
#[doc = "CRCCU DMA Interrupt Disable Register"]
pub mod dma_idr;
#[doc = "CRCCU DMA Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_imr](dma_imr) module"]
pub type DMA_IMR = crate::Reg<u32, _DMA_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IMR;
#[doc = "`read()` method returns [dma_imr::R](dma_imr::R) reader structure"]
impl crate::Readable for DMA_IMR {}
#[doc = "CRCCU DMA Interrupt Mask Register"]
pub mod dma_imr;
#[doc = "CRCCU DMA Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_isr](dma_isr) module"]
pub type DMA_ISR = crate::Reg<u32, _DMA_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_ISR;
#[doc = "`read()` method returns [dma_isr::R](dma_isr::R) reader structure"]
impl crate::Readable for DMA_ISR {}
#[doc = "CRCCU DMA Interrupt Status Register"]
pub mod dma_isr;
#[doc = "CRCCU Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "CRCCU Control Register"]
pub mod cr;
#[doc = "CRCCU Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](mr) module"]
pub type MR = crate::Reg<u32, _MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR;
#[doc = "`read()` method returns [mr::R](mr::R) reader structure"]
impl crate::Readable for MR {}
#[doc = "`write(|w| ..)` method takes [mr::W](mr::W) writer structure"]
impl crate::Writable for MR {}
#[doc = "CRCCU Mode Register"]
pub mod mr;
#[doc = "CRCCU Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "CRCCU Status Register"]
pub mod sr;
#[doc = "CRCCU Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "CRCCU Interrupt Enable Register"]
pub mod ier;
#[doc = "CRCCU Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "CRCCU Interrupt Disable Register"]
pub mod idr;
#[doc = "CRCCU Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "CRCCU Interrupt Mask Register"]
pub mod imr;
#[doc = "CRCCU Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "CRCCU Interrupt Status Register"]
pub mod isr;
