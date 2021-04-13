#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ISI Configuration 1 Register"]
    pub cfg1: CFG1,
    #[doc = "0x04 - ISI Configuration 2 Register"]
    pub cfg2: CFG2,
    #[doc = "0x08 - ISI Preview Size Register"]
    pub psize: PSIZE,
    #[doc = "0x0c - ISI Preview Decimation Factor Register"]
    pub pdecf: PDECF,
    #[doc = "0x10 - ISI Color Space Conversion YCrCb To RGB Set 0 Register"]
    pub y2r_set0: Y2R_SET0,
    #[doc = "0x14 - ISI Color Space Conversion YCrCb To RGB Set 1 Register"]
    pub y2r_set1: Y2R_SET1,
    #[doc = "0x18 - ISI Color Space Conversion RGB To YCrCb Set 0 Register"]
    pub r2y_set0: R2Y_SET0,
    #[doc = "0x1c - ISI Color Space Conversion RGB To YCrCb Set 1 Register"]
    pub r2y_set1: R2Y_SET1,
    #[doc = "0x20 - ISI Color Space Conversion RGB To YCrCb Set 2 Register"]
    pub r2y_set2: R2Y_SET2,
    #[doc = "0x24 - ISI Control Register"]
    pub cr: CR,
    #[doc = "0x28 - ISI Status Register"]
    pub sr: SR,
    #[doc = "0x2c - ISI Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x30 - ISI Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x34 - ISI Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x38 - DMA Channel Enable Register"]
    pub dma_cher: DMA_CHER,
    #[doc = "0x3c - DMA Channel Disable Register"]
    pub dma_chdr: DMA_CHDR,
    #[doc = "0x40 - DMA Channel Status Register"]
    pub dma_chsr: DMA_CHSR,
    #[doc = "0x44 - DMA Preview Base Address Register"]
    pub dma_p_addr: DMA_P_ADDR,
    #[doc = "0x48 - DMA Preview Control Register"]
    pub dma_p_ctrl: DMA_P_CTRL,
    #[doc = "0x4c - DMA Preview Descriptor Address Register"]
    pub dma_p_dscr: DMA_P_DSCR,
    #[doc = "0x50 - DMA Codec Base Address Register"]
    pub dma_c_addr: DMA_C_ADDR,
    #[doc = "0x54 - DMA Codec Control Register"]
    pub dma_c_ctrl: DMA_C_CTRL,
    #[doc = "0x58 - DMA Codec Descriptor Address Register"]
    pub dma_c_dscr: DMA_C_DSCR,
    _reserved23: [u8; 136usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: WPSR,
}
#[doc = "ISI Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](cfg1) module"]
pub type CFG1 = crate::Reg<u32, _CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG1;
#[doc = "`read()` method returns [cfg1::R](cfg1::R) reader structure"]
impl crate::Readable for CFG1 {}
#[doc = "`write(|w| ..)` method takes [cfg1::W](cfg1::W) writer structure"]
impl crate::Writable for CFG1 {}
#[doc = "ISI Configuration 1 Register"]
pub mod cfg1;
#[doc = "ISI Configuration 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](cfg2) module"]
pub type CFG2 = crate::Reg<u32, _CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG2;
#[doc = "`read()` method returns [cfg2::R](cfg2::R) reader structure"]
impl crate::Readable for CFG2 {}
#[doc = "`write(|w| ..)` method takes [cfg2::W](cfg2::W) writer structure"]
impl crate::Writable for CFG2 {}
#[doc = "ISI Configuration 2 Register"]
pub mod cfg2;
#[doc = "ISI Preview Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psize](psize) module"]
pub type PSIZE = crate::Reg<u32, _PSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSIZE;
#[doc = "`read()` method returns [psize::R](psize::R) reader structure"]
impl crate::Readable for PSIZE {}
#[doc = "`write(|w| ..)` method takes [psize::W](psize::W) writer structure"]
impl crate::Writable for PSIZE {}
#[doc = "ISI Preview Size Register"]
pub mod psize;
#[doc = "ISI Preview Decimation Factor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdecf](pdecf) module"]
pub type PDECF = crate::Reg<u32, _PDECF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDECF;
#[doc = "`read()` method returns [pdecf::R](pdecf::R) reader structure"]
impl crate::Readable for PDECF {}
#[doc = "`write(|w| ..)` method takes [pdecf::W](pdecf::W) writer structure"]
impl crate::Writable for PDECF {}
#[doc = "ISI Preview Decimation Factor Register"]
pub mod pdecf;
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [y2r_set0](y2r_set0) module"]
pub type Y2R_SET0 = crate::Reg<u32, _Y2R_SET0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _Y2R_SET0;
#[doc = "`read()` method returns [y2r_set0::R](y2r_set0::R) reader structure"]
impl crate::Readable for Y2R_SET0 {}
#[doc = "`write(|w| ..)` method takes [y2r_set0::W](y2r_set0::W) writer structure"]
impl crate::Writable for Y2R_SET0 {}
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 0 Register"]
pub mod y2r_set0;
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [y2r_set1](y2r_set1) module"]
pub type Y2R_SET1 = crate::Reg<u32, _Y2R_SET1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _Y2R_SET1;
#[doc = "`read()` method returns [y2r_set1::R](y2r_set1::R) reader structure"]
impl crate::Readable for Y2R_SET1 {}
#[doc = "`write(|w| ..)` method takes [y2r_set1::W](y2r_set1::W) writer structure"]
impl crate::Writable for Y2R_SET1 {}
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 1 Register"]
pub mod y2r_set1;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2y_set0](r2y_set0) module"]
pub type R2Y_SET0 = crate::Reg<u32, _R2Y_SET0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R2Y_SET0;
#[doc = "`read()` method returns [r2y_set0::R](r2y_set0::R) reader structure"]
impl crate::Readable for R2Y_SET0 {}
#[doc = "`write(|w| ..)` method takes [r2y_set0::W](r2y_set0::W) writer structure"]
impl crate::Writable for R2Y_SET0 {}
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 0 Register"]
pub mod r2y_set0;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2y_set1](r2y_set1) module"]
pub type R2Y_SET1 = crate::Reg<u32, _R2Y_SET1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R2Y_SET1;
#[doc = "`read()` method returns [r2y_set1::R](r2y_set1::R) reader structure"]
impl crate::Readable for R2Y_SET1 {}
#[doc = "`write(|w| ..)` method takes [r2y_set1::W](r2y_set1::W) writer structure"]
impl crate::Writable for R2Y_SET1 {}
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 1 Register"]
pub mod r2y_set1;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2y_set2](r2y_set2) module"]
pub type R2Y_SET2 = crate::Reg<u32, _R2Y_SET2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R2Y_SET2;
#[doc = "`read()` method returns [r2y_set2::R](r2y_set2::R) reader structure"]
impl crate::Readable for R2Y_SET2 {}
#[doc = "`write(|w| ..)` method takes [r2y_set2::W](r2y_set2::W) writer structure"]
impl crate::Writable for R2Y_SET2 {}
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 2 Register"]
pub mod r2y_set2;
#[doc = "ISI Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "ISI Control Register"]
pub mod cr;
#[doc = "ISI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "ISI Status Register"]
pub mod sr;
#[doc = "ISI Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "ISI Interrupt Enable Register"]
pub mod ier;
#[doc = "ISI Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "ISI Interrupt Disable Register"]
pub mod idr;
#[doc = "ISI Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "ISI Interrupt Mask Register"]
pub mod imr;
#[doc = "DMA Channel Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_cher](dma_cher) module"]
pub type DMA_CHER = crate::Reg<u32, _DMA_CHER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHER;
#[doc = "`write(|w| ..)` method takes [dma_cher::W](dma_cher::W) writer structure"]
impl crate::Writable for DMA_CHER {}
#[doc = "DMA Channel Enable Register"]
pub mod dma_cher;
#[doc = "DMA Channel Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chdr](dma_chdr) module"]
pub type DMA_CHDR = crate::Reg<u32, _DMA_CHDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHDR;
#[doc = "`write(|w| ..)` method takes [dma_chdr::W](dma_chdr::W) writer structure"]
impl crate::Writable for DMA_CHDR {}
#[doc = "DMA Channel Disable Register"]
pub mod dma_chdr;
#[doc = "DMA Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chsr](dma_chsr) module"]
pub type DMA_CHSR = crate::Reg<u32, _DMA_CHSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHSR;
#[doc = "`read()` method returns [dma_chsr::R](dma_chsr::R) reader structure"]
impl crate::Readable for DMA_CHSR {}
#[doc = "DMA Channel Status Register"]
pub mod dma_chsr;
#[doc = "DMA Preview Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_p_addr](dma_p_addr) module"]
pub type DMA_P_ADDR = crate::Reg<u32, _DMA_P_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_P_ADDR;
#[doc = "`read()` method returns [dma_p_addr::R](dma_p_addr::R) reader structure"]
impl crate::Readable for DMA_P_ADDR {}
#[doc = "`write(|w| ..)` method takes [dma_p_addr::W](dma_p_addr::W) writer structure"]
impl crate::Writable for DMA_P_ADDR {}
#[doc = "DMA Preview Base Address Register"]
pub mod dma_p_addr;
#[doc = "DMA Preview Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_p_ctrl](dma_p_ctrl) module"]
pub type DMA_P_CTRL = crate::Reg<u32, _DMA_P_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_P_CTRL;
#[doc = "`read()` method returns [dma_p_ctrl::R](dma_p_ctrl::R) reader structure"]
impl crate::Readable for DMA_P_CTRL {}
#[doc = "`write(|w| ..)` method takes [dma_p_ctrl::W](dma_p_ctrl::W) writer structure"]
impl crate::Writable for DMA_P_CTRL {}
#[doc = "DMA Preview Control Register"]
pub mod dma_p_ctrl;
#[doc = "DMA Preview Descriptor Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_p_dscr](dma_p_dscr) module"]
pub type DMA_P_DSCR = crate::Reg<u32, _DMA_P_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_P_DSCR;
#[doc = "`read()` method returns [dma_p_dscr::R](dma_p_dscr::R) reader structure"]
impl crate::Readable for DMA_P_DSCR {}
#[doc = "`write(|w| ..)` method takes [dma_p_dscr::W](dma_p_dscr::W) writer structure"]
impl crate::Writable for DMA_P_DSCR {}
#[doc = "DMA Preview Descriptor Address Register"]
pub mod dma_p_dscr;
#[doc = "DMA Codec Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c_addr](dma_c_addr) module"]
pub type DMA_C_ADDR = crate::Reg<u32, _DMA_C_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C_ADDR;
#[doc = "`read()` method returns [dma_c_addr::R](dma_c_addr::R) reader structure"]
impl crate::Readable for DMA_C_ADDR {}
#[doc = "`write(|w| ..)` method takes [dma_c_addr::W](dma_c_addr::W) writer structure"]
impl crate::Writable for DMA_C_ADDR {}
#[doc = "DMA Codec Base Address Register"]
pub mod dma_c_addr;
#[doc = "DMA Codec Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c_ctrl](dma_c_ctrl) module"]
pub type DMA_C_CTRL = crate::Reg<u32, _DMA_C_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C_CTRL;
#[doc = "`read()` method returns [dma_c_ctrl::R](dma_c_ctrl::R) reader structure"]
impl crate::Readable for DMA_C_CTRL {}
#[doc = "`write(|w| ..)` method takes [dma_c_ctrl::W](dma_c_ctrl::W) writer structure"]
impl crate::Writable for DMA_C_CTRL {}
#[doc = "DMA Codec Control Register"]
pub mod dma_c_ctrl;
#[doc = "DMA Codec Descriptor Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c_dscr](dma_c_dscr) module"]
pub type DMA_C_DSCR = crate::Reg<u32, _DMA_C_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C_DSCR;
#[doc = "`read()` method returns [dma_c_dscr::R](dma_c_dscr::R) reader structure"]
impl crate::Readable for DMA_C_DSCR {}
#[doc = "`write(|w| ..)` method takes [dma_c_dscr::W](dma_c_dscr::W) writer structure"]
impl crate::Writable for DMA_C_DSCR {}
#[doc = "DMA Codec Descriptor Address Register"]
pub mod dma_c_dscr;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpmr](wpmr) module"]
pub type WPMR = crate::Reg<u32, _WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPMR;
#[doc = "`read()` method returns [wpmr::R](wpmr::R) reader structure"]
impl crate::Readable for WPMR {}
#[doc = "`write(|w| ..)` method takes [wpmr::W](wpmr::W) writer structure"]
impl crate::Writable for WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](wpsr) module"]
pub type WPSR = crate::Reg<u32, _WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSR;
#[doc = "`read()` method returns [wpsr::R](wpsr::R) reader structure"]
impl crate::Readable for WPSR {}
#[doc = "Write Protection Status Register"]
pub mod wpsr;
