#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - OHCI Interrupt Configuration Register"]
    pub ohciicr: OHCIICR,
    _reserved1: [u8; 28usize],
    #[doc = "0x30 - UTMI Clock Trimming Register"]
    pub cktrim: CKTRIM,
}
#[doc = "OHCI Interrupt Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ohciicr](ohciicr) module"]
pub type OHCIICR = crate::Reg<u32, _OHCIICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OHCIICR;
#[doc = "`read()` method returns [ohciicr::R](ohciicr::R) reader structure"]
impl crate::Readable for OHCIICR {}
#[doc = "`write(|w| ..)` method takes [ohciicr::W](ohciicr::W) writer structure"]
impl crate::Writable for OHCIICR {}
#[doc = "OHCI Interrupt Configuration Register"]
pub mod ohciicr;
#[doc = "UTMI Clock Trimming Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cktrim](cktrim) module"]
pub type CKTRIM = crate::Reg<u32, _CKTRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKTRIM;
#[doc = "`read()` method returns [cktrim::R](cktrim::R) reader structure"]
impl crate::Readable for CKTRIM {}
#[doc = "`write(|w| ..)` method takes [cktrim::W](cktrim::W) writer structure"]
impl crate::Writable for CKTRIM {}
#[doc = "UTMI Clock Trimming Register"]
pub mod cktrim;
