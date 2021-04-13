#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x1c - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x20 - Key Word Register"]
    pub keywr: [KEYWR; 8],
    #[doc = "0x40 - Input Data Register"]
    pub idatar: [IDATAR; 4],
    #[doc = "0x50 - Output Data Register"]
    pub odatar: [ODATAR; 4],
    #[doc = "0x60 - Initialization Vector Register"]
    pub ivr: [IVR; 4],
    #[doc = "0x70 - Additional Authenticated Data Length Register"]
    pub aadlenr: AADLENR,
    #[doc = "0x74 - Plaintext/Ciphertext Length Register"]
    pub clenr: CLENR,
    #[doc = "0x78 - GCM Intermediate Hash Word Register"]
    pub ghashr: [GHASHR; 4],
    #[doc = "0x88 - GCM Authentication Tag Word Register"]
    pub tagr: [TAGR; 4],
    #[doc = "0x98 - GCM Encryption Counter Value Register"]
    pub ctrr: CTRR,
    #[doc = "0x9c - GCM H Word Register"]
    pub gcmhr: [GCMHR; 4],
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
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
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
#[doc = "Key Word Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keywr](keywr) module"]
pub type KEYWR = crate::Reg<u32, _KEYWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYWR;
#[doc = "`write(|w| ..)` method takes [keywr::W](keywr::W) writer structure"]
impl crate::Writable for KEYWR {}
#[doc = "Key Word Register"]
pub mod keywr;
#[doc = "Input Data Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idatar](idatar) module"]
pub type IDATAR = crate::Reg<u32, _IDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDATAR;
#[doc = "`write(|w| ..)` method takes [idatar::W](idatar::W) writer structure"]
impl crate::Writable for IDATAR {}
#[doc = "Input Data Register"]
pub mod idatar;
#[doc = "Output Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odatar](odatar) module"]
pub type ODATAR = crate::Reg<u32, _ODATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODATAR;
#[doc = "`read()` method returns [odatar::R](odatar::R) reader structure"]
impl crate::Readable for ODATAR {}
#[doc = "Output Data Register"]
pub mod odatar;
#[doc = "Initialization Vector Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivr](ivr) module"]
pub type IVR = crate::Reg<u32, _IVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IVR;
#[doc = "`write(|w| ..)` method takes [ivr::W](ivr::W) writer structure"]
impl crate::Writable for IVR {}
#[doc = "Initialization Vector Register"]
pub mod ivr;
#[doc = "Additional Authenticated Data Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aadlenr](aadlenr) module"]
pub type AADLENR = crate::Reg<u32, _AADLENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AADLENR;
#[doc = "`read()` method returns [aadlenr::R](aadlenr::R) reader structure"]
impl crate::Readable for AADLENR {}
#[doc = "`write(|w| ..)` method takes [aadlenr::W](aadlenr::W) writer structure"]
impl crate::Writable for AADLENR {}
#[doc = "Additional Authenticated Data Length Register"]
pub mod aadlenr;
#[doc = "Plaintext/Ciphertext Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clenr](clenr) module"]
pub type CLENR = crate::Reg<u32, _CLENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLENR;
#[doc = "`read()` method returns [clenr::R](clenr::R) reader structure"]
impl crate::Readable for CLENR {}
#[doc = "`write(|w| ..)` method takes [clenr::W](clenr::W) writer structure"]
impl crate::Writable for CLENR {}
#[doc = "Plaintext/Ciphertext Length Register"]
pub mod clenr;
#[doc = "GCM Intermediate Hash Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ghashr](ghashr) module"]
pub type GHASHR = crate::Reg<u32, _GHASHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GHASHR;
#[doc = "`read()` method returns [ghashr::R](ghashr::R) reader structure"]
impl crate::Readable for GHASHR {}
#[doc = "`write(|w| ..)` method takes [ghashr::W](ghashr::W) writer structure"]
impl crate::Writable for GHASHR {}
#[doc = "GCM Intermediate Hash Word Register"]
pub mod ghashr;
#[doc = "GCM Authentication Tag Word Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tagr](tagr) module"]
pub type TAGR = crate::Reg<u32, _TAGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAGR;
#[doc = "`read()` method returns [tagr::R](tagr::R) reader structure"]
impl crate::Readable for TAGR {}
#[doc = "GCM Authentication Tag Word Register"]
pub mod tagr;
#[doc = "GCM Encryption Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrr](ctrr) module"]
pub type CTRR = crate::Reg<u32, _CTRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRR;
#[doc = "`read()` method returns [ctrr::R](ctrr::R) reader structure"]
impl crate::Readable for CTRR {}
#[doc = "GCM Encryption Counter Value Register"]
pub mod ctrr;
#[doc = "GCM H Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcmhr](gcmhr) module"]
pub type GCMHR = crate::Reg<u32, _GCMHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCMHR;
#[doc = "`read()` method returns [gcmhr::R](gcmhr::R) reader structure"]
impl crate::Readable for GCMHR {}
#[doc = "`write(|w| ..)` method takes [gcmhr::W](gcmhr::W) writer structure"]
impl crate::Writable for GCMHR {}
#[doc = "GCM H Word Register"]
pub mod gcmhr;
