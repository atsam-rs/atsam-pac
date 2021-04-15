#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Counter Control Register"]
    pub cntcr: CNTCR,
    #[doc = "0x08 - Sensor Idle Level"]
    pub idle: IDLE,
    #[doc = "0x0c - Sensor Relative Level"]
    pub level: LEVEL,
    #[doc = "0x10 - Sensor Raw Value"]
    pub raw: RAW,
    #[doc = "0x14 - Filter Timing Register"]
    pub timing: TIMING,
    #[doc = "0x18 - Threshold Register"]
    pub thresh: THRESH,
    #[doc = "0x1c - Pin Selection Register"]
    pub pinsel: PINSEL,
    #[doc = "0x20 - Direct Memory Access Register"]
    pub dma: DMA,
    #[doc = "0x24 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x2c - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x30 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x34 - Status Clear Register"]
    pub scr: SCR,
    _reserved14: [u8; 8usize],
    #[doc = "0x40 - In-Touch Status Register"]
    pub intch: [INTCH; 1],
    _reserved15: [u8; 12usize],
    #[doc = "0x50 - In-Touch Status Clear Register"]
    pub intchclr: [INTCHCLR; 1],
    _reserved16: [u8; 12usize],
    #[doc = "0x60 - Out-of-Touch Status Register"]
    pub outtch: [OUTTCH; 1],
    _reserved17: [u8; 12usize],
    #[doc = "0x70 - Out-of-Touch Status Clear Register"]
    pub outtchclr: [OUTTCHCLR; 1],
    _reserved18: [u8; 132usize],
    #[doc = "0xf8 - Parameter Register"]
    pub parameter: PARAMETER,
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
#[doc = "Counter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntcr](cntcr) module"]
pub type CNTCR = crate::Reg<u32, _CNTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTCR;
#[doc = "`read()` method returns [cntcr::R](cntcr::R) reader structure"]
impl crate::Readable for CNTCR {}
#[doc = "`write(|w| ..)` method takes [cntcr::W](cntcr::W) writer structure"]
impl crate::Writable for CNTCR {}
#[doc = "Counter Control Register"]
pub mod cntcr;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Direct Memory Access Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma](dma) module"]
pub type DMA = crate::Reg<u32, _DMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA;
#[doc = "`read()` method returns [dma::R](dma::R) reader structure"]
impl crate::Readable for DMA {}
#[doc = "`write(|w| ..)` method takes [dma::W](dma::W) writer structure"]
impl crate::Writable for DMA {}
#[doc = "Direct Memory Access Register"]
pub mod dma;
#[doc = "Sensor Idle Level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idle](idle) module"]
pub type IDLE = crate::Reg<u32, _IDLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDLE;
#[doc = "`read()` method returns [idle::R](idle::R) reader structure"]
impl crate::Readable for IDLE {}
#[doc = "`write(|w| ..)` method takes [idle::W](idle::W) writer structure"]
impl crate::Writable for IDLE {}
#[doc = "Sensor Idle Level"]
pub mod idle;
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
#[doc = "In-Touch Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intchclr](intchclr) module"]
pub type INTCHCLR = crate::Reg<u32, _INTCHCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCHCLR;
#[doc = "`write(|w| ..)` method takes [intchclr::W](intchclr::W) writer structure"]
impl crate::Writable for INTCHCLR {}
#[doc = "In-Touch Status Clear Register"]
pub mod intchclr;
#[doc = "In-Touch Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intch](intch) module"]
pub type INTCH = crate::Reg<u32, _INTCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCH;
#[doc = "`read()` method returns [intch::R](intch::R) reader structure"]
impl crate::Readable for INTCH {}
#[doc = "In-Touch Status Register"]
pub mod intch;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Sensor Relative Level\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [level](level) module"]
pub type LEVEL = crate::Reg<u32, _LEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEVEL;
#[doc = "`read()` method returns [level::R](level::R) reader structure"]
impl crate::Readable for LEVEL {}
#[doc = "Sensor Relative Level"]
pub mod level;
#[doc = "Out-of-Touch Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outtchclr](outtchclr) module"]
pub type OUTTCHCLR = crate::Reg<u32, _OUTTCHCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTTCHCLR;
#[doc = "`write(|w| ..)` method takes [outtchclr::W](outtchclr::W) writer structure"]
impl crate::Writable for OUTTCHCLR {}
#[doc = "Out-of-Touch Status Clear Register"]
pub mod outtchclr;
#[doc = "Out-of-Touch Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outtch](outtch) module"]
pub type OUTTCH = crate::Reg<u32, _OUTTCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTTCH;
#[doc = "`read()` method returns [outtch::R](outtch::R) reader structure"]
impl crate::Readable for OUTTCH {}
#[doc = "Out-of-Touch Status Register"]
pub mod outtch;
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [parameter](parameter) module"]
pub type PARAMETER = crate::Reg<u32, _PARAMETER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PARAMETER;
#[doc = "`read()` method returns [parameter::R](parameter::R) reader structure"]
impl crate::Readable for PARAMETER {}
#[doc = "Parameter Register"]
pub mod parameter;
#[doc = "Pin Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinsel](pinsel) module"]
pub type PINSEL = crate::Reg<u32, _PINSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINSEL;
#[doc = "`read()` method returns [pinsel::R](pinsel::R) reader structure"]
impl crate::Readable for PINSEL {}
#[doc = "`write(|w| ..)` method takes [pinsel::W](pinsel::W) writer structure"]
impl crate::Writable for PINSEL {}
#[doc = "Pin Selection Register"]
pub mod pinsel;
#[doc = "Sensor Raw Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [raw](raw) module"]
pub type RAW = crate::Reg<u32, _RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAW;
#[doc = "`read()` method returns [raw::R](raw::R) reader structure"]
impl crate::Readable for RAW {}
#[doc = "Sensor Raw Value"]
pub mod raw;
#[doc = "Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "Status Clear Register"]
pub mod scr;
#[doc = "Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thresh](thresh) module"]
pub type THRESH = crate::Reg<u32, _THRESH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THRESH;
#[doc = "`read()` method returns [thresh::R](thresh::R) reader structure"]
impl crate::Readable for THRESH {}
#[doc = "`write(|w| ..)` method takes [thresh::W](thresh::W) writer structure"]
impl crate::Writable for THRESH {}
#[doc = "Threshold Register"]
pub mod thresh;
#[doc = "Filter Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timing](timing) module"]
pub type TIMING = crate::Reg<u32, _TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMING;
#[doc = "`read()` method returns [timing::R](timing::R) reader structure"]
impl crate::Readable for TIMING {}
#[doc = "`write(|w| ..)` method takes [timing::W](timing::W) writer structure"]
impl crate::Writable for TIMING {}
#[doc = "Filter Timing Register"]
pub mod timing;
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "Version Register"]
pub mod version;
