#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Control Register Channel"]
    pub ccr0: CCR,
    _reserved_1_cmr: [u8; 4usize],
    #[doc = "0x08 - Stepper Motor Mode Register"]
    pub smmr0: SMMR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Counter Value Channel"]
    pub cv0: CV,
    #[doc = "0x14 - Register A Channel"]
    pub ra0: RA,
    #[doc = "0x18 - Register B Channel"]
    pub rb0: RB,
    #[doc = "0x1c - Register C Channel"]
    pub rc0: RC,
    #[doc = "0x20 - Status Register Channel"]
    pub sr0: SR,
    #[doc = "0x24 - Interrupt Enable Register Channel"]
    pub ier0: IER,
    #[doc = "0x28 - Interrupt Disable Register Channel"]
    pub idr0: IDR,
    #[doc = "0x2c - Interrupt Mask Register Channel"]
    pub imr0: IMR,
    _reserved11: [u8; 16usize],
    #[doc = "0x40 - Channel Control Register Channel"]
    pub ccr1: CCR,
    _reserved_12_cmr: [u8; 4usize],
    #[doc = "0x48 - Stepper Motor Mode Register"]
    pub smmr1: SMMR,
    _reserved14: [u8; 4usize],
    #[doc = "0x50 - Counter Value Channel"]
    pub cv1: CV,
    #[doc = "0x54 - Register A Channel"]
    pub ra1: RA,
    #[doc = "0x58 - Register B Channel"]
    pub rb1: RB,
    #[doc = "0x5c - Register C Channel"]
    pub rc1: RC,
    #[doc = "0x60 - Status Register Channel"]
    pub sr1: SR,
    #[doc = "0x64 - Interrupt Enable Register Channel"]
    pub ier1: IER,
    #[doc = "0x68 - Interrupt Disable Register Channel"]
    pub idr1: IDR,
    #[doc = "0x6c - Interrupt Mask Register Channel"]
    pub imr1: IMR,
    _reserved22: [u8; 16usize],
    #[doc = "0x80 - Channel Control Register Channel"]
    pub ccr2: CCR,
    _reserved_23_cmr: [u8; 4usize],
    #[doc = "0x88 - Stepper Motor Mode Register"]
    pub smmr2: SMMR,
    _reserved25: [u8; 4usize],
    #[doc = "0x90 - Counter Value Channel"]
    pub cv2: CV,
    #[doc = "0x94 - Register A Channel"]
    pub ra2: RA,
    #[doc = "0x98 - Register B Channel"]
    pub rb2: RB,
    #[doc = "0x9c - Register C Channel"]
    pub rc2: RC,
    #[doc = "0xa0 - Status Register Channel"]
    pub sr2: SR,
    #[doc = "0xa4 - Interrupt Enable Register Channel"]
    pub ier2: IER,
    #[doc = "0xa8 - Interrupt Disable Register Channel"]
    pub idr2: IDR,
    #[doc = "0xac - Interrupt Mask Register Channel"]
    pub imr2: IMR,
    _reserved33: [u8; 16usize],
    #[doc = "0xc0 - TC Block Control Register"]
    pub bcr: BCR,
    #[doc = "0xc4 - TC Block Mode Register"]
    pub bmr: BMR,
    _reserved35: [u8; 28usize],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    _reserved36: [u8; 16usize],
    #[doc = "0xf8 - Features Register"]
    pub features: FEATURES,
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
impl RegisterBlock {
    #[doc = "0x04 - Channel Mode Register Channel"]
    #[inline(always)]
    pub fn cmr0_alt(&self) -> &CMR_ALT {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const CMR_ALT) }
    }
    #[doc = "0x04 - Channel Mode Register Channel"]
    #[inline(always)]
    pub fn cmr0_alt_mut(&self) -> &mut CMR_ALT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut CMR_ALT) }
    }
    #[doc = "0x04 - Channel Mode Register Channel"]
    #[inline(always)]
    pub fn cmr0(&self) -> &CMR {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const CMR) }
    }
    #[doc = "0x04 - Channel Mode Register Channel"]
    #[inline(always)]
    pub fn cmr0_mut(&self) -> &mut CMR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut CMR) }
    }
    #[doc = "0x44 - Channel Mode Register Channel"]
    #[inline(always)]
    pub fn cmr1_alt(&self) -> &CMR_ALT {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const CMR_ALT) }
    }
    #[doc = "0x44 - Channel Mode Register Channel"]
    #[inline(always)]
    pub fn cmr1_alt_mut(&self) -> &mut CMR_ALT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut CMR_ALT) }
    }
    #[doc = "0x44 - Channel Mode Register Channel"]
    #[inline(always)]
    pub fn cmr1(&self) -> &CMR {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const CMR) }
    }
    #[doc = "0x44 - Channel Mode Register Channel"]
    #[inline(always)]
    pub fn cmr1_mut(&self) -> &mut CMR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut CMR) }
    }
    #[doc = "0x84 - Channel Mode Register Channel"]
    #[inline(always)]
    pub fn cmr2_alt(&self) -> &CMR_ALT {
        unsafe { &*(((self as *const Self) as *const u8).add(132usize) as *const CMR_ALT) }
    }
    #[doc = "0x84 - Channel Mode Register Channel"]
    #[inline(always)]
    pub fn cmr2_alt_mut(&self) -> &mut CMR_ALT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(132usize) as *mut CMR_ALT) }
    }
    #[doc = "0x84 - Channel Mode Register Channel"]
    #[inline(always)]
    pub fn cmr2(&self) -> &CMR {
        unsafe { &*(((self as *const Self) as *const u8).add(132usize) as *const CMR) }
    }
    #[doc = "0x84 - Channel Mode Register Channel"]
    #[inline(always)]
    pub fn cmr2_mut(&self) -> &mut CMR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(132usize) as *mut CMR) }
    }
}
#[doc = "TC Block Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr](bcr) module"]
pub type BCR = crate::Reg<u32, _BCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR;
#[doc = "`write(|w| ..)` method takes [bcr::W](bcr::W) writer structure"]
impl crate::Writable for BCR {}
#[doc = "TC Block Control Register"]
pub mod bcr;
#[doc = "TC Block Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmr](bmr) module"]
pub type BMR = crate::Reg<u32, _BMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMR;
#[doc = "`read()` method returns [bmr::R](bmr::R) reader structure"]
impl crate::Readable for BMR {}
#[doc = "`write(|w| ..)` method takes [bmr::W](bmr::W) writer structure"]
impl crate::Writable for BMR {}
#[doc = "TC Block Mode Register"]
pub mod bmr;
#[doc = "Channel Control Register Channel\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "Channel Control Register Channel"]
pub mod ccr;
#[doc = "Channel Mode Register Channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr](cmr) module"]
pub type CMR = crate::Reg<u32, _CMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR;
#[doc = "`read()` method returns [cmr::R](cmr::R) reader structure"]
impl crate::Readable for CMR {}
#[doc = "`write(|w| ..)` method takes [cmr::W](cmr::W) writer structure"]
impl crate::Writable for CMR {}
#[doc = "Channel Mode Register Channel"]
pub mod cmr;
#[doc = "Channel Mode Register Channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr_alt](cmr_alt) module"]
pub type CMR_ALT = crate::Reg<u32, _CMR_ALT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR_ALT;
#[doc = "`read()` method returns [cmr_alt::R](cmr_alt::R) reader structure"]
impl crate::Readable for CMR_ALT {}
#[doc = "`write(|w| ..)` method takes [cmr_alt::W](cmr_alt::W) writer structure"]
impl crate::Writable for CMR_ALT {}
#[doc = "Channel Mode Register Channel"]
pub mod cmr_alt;
#[doc = "Counter Value Channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cv](cv) module"]
pub type CV = crate::Reg<u32, _CV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CV;
#[doc = "`read()` method returns [cv::R](cv::R) reader structure"]
impl crate::Readable for CV {}
#[doc = "Counter Value Channel"]
pub mod cv;
#[doc = "Features Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [features](features) module"]
pub type FEATURES = crate::Reg<u32, _FEATURES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FEATURES;
#[doc = "`read()` method returns [features::R](features::R) reader structure"]
impl crate::Readable for FEATURES {}
#[doc = "Features Register"]
pub mod features;
#[doc = "Interrupt Disable Register Channel\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "Interrupt Disable Register Channel"]
pub mod idr;
#[doc = "Interrupt Enable Register Channel\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register Channel"]
pub mod ier;
#[doc = "Interrupt Mask Register Channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "Interrupt Mask Register Channel"]
pub mod imr;
#[doc = "Register A Channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ra](ra) module"]
pub type RA = crate::Reg<u32, _RA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RA;
#[doc = "`read()` method returns [ra::R](ra::R) reader structure"]
impl crate::Readable for RA {}
#[doc = "`write(|w| ..)` method takes [ra::W](ra::W) writer structure"]
impl crate::Writable for RA {}
#[doc = "Register A Channel"]
pub mod ra;
#[doc = "Register B Channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rb](rb) module"]
pub type RB = crate::Reg<u32, _RB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RB;
#[doc = "`read()` method returns [rb::R](rb::R) reader structure"]
impl crate::Readable for RB {}
#[doc = "`write(|w| ..)` method takes [rb::W](rb::W) writer structure"]
impl crate::Writable for RB {}
#[doc = "Register B Channel"]
pub mod rb;
#[doc = "Register C Channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc](rc) module"]
pub type RC = crate::Reg<u32, _RC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RC;
#[doc = "`read()` method returns [rc::R](rc::R) reader structure"]
impl crate::Readable for RC {}
#[doc = "`write(|w| ..)` method takes [rc::W](rc::W) writer structure"]
impl crate::Writable for RC {}
#[doc = "Register C Channel"]
pub mod rc;
#[doc = "Stepper Motor Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smmr](smmr) module"]
pub type SMMR = crate::Reg<u32, _SMMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMMR;
#[doc = "`read()` method returns [smmr::R](smmr::R) reader structure"]
impl crate::Readable for SMMR {}
#[doc = "`write(|w| ..)` method takes [smmr::W](smmr::W) writer structure"]
impl crate::Writable for SMMR {}
#[doc = "Stepper Motor Mode Register"]
pub mod smmr;
#[doc = "Status Register Channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register Channel"]
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
