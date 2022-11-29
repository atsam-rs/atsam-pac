#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Control Register Channel"]
    pub ccr0: CCR,
    _reserved_1_capture_cmr0: [u8; 0x04],
    #[doc = "0x08 - Stepper Motor Mode Register"]
    pub smmr0: SMMR,
    _reserved3: [u8; 0x04],
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
    _reserved11: [u8; 0x10],
    #[doc = "0x40 - Channel Control Register Channel"]
    pub ccr1: CCR,
    _reserved_12_capture_cmr1: [u8; 0x04],
    #[doc = "0x48 - Stepper Motor Mode Register"]
    pub smmr1: SMMR,
    _reserved14: [u8; 0x04],
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
    _reserved22: [u8; 0x10],
    #[doc = "0x80 - Channel Control Register Channel"]
    pub ccr2: CCR,
    _reserved_23_capture_cmr2: [u8; 0x04],
    #[doc = "0x88 - Stepper Motor Mode Register"]
    pub smmr2: SMMR,
    _reserved25: [u8; 0x04],
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
    _reserved33: [u8; 0x10],
    #[doc = "0xc0 - TC Block Control Register"]
    pub bcr: BCR,
    #[doc = "0xc4 - TC Block Mode Register"]
    pub bmr: BMR,
    _reserved35: [u8; 0x1c],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    _reserved36: [u8; 0x10],
    #[doc = "0xf8 - Features Register"]
    pub features: FEATURES,
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
impl RegisterBlock {
    #[doc = "0x04 - Channel Mode Register Channel"]
    #[inline(always)]
    pub const fn waveform_cmr0_alt(&self) -> &WAVEFORM_CMR_ALT {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Channel Mode Register Channel"]
    #[inline(always)]
    pub const fn capture_cmr0(&self) -> &CAPTURE_CMR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x44 - Channel Mode Register Channel"]
    #[inline(always)]
    pub const fn waveform_cmr1_alt(&self) -> &WAVEFORM_CMR_ALT {
        unsafe { &*(self as *const Self).cast::<u8>().add(68usize).cast() }
    }
    #[doc = "0x44 - Channel Mode Register Channel"]
    #[inline(always)]
    pub const fn capture_cmr1(&self) -> &CAPTURE_CMR {
        unsafe { &*(self as *const Self).cast::<u8>().add(68usize).cast() }
    }
    #[doc = "0x84 - Channel Mode Register Channel"]
    #[inline(always)]
    pub const fn waveform_cmr2_alt(&self) -> &WAVEFORM_CMR_ALT {
        unsafe { &*(self as *const Self).cast::<u8>().add(132usize).cast() }
    }
    #[doc = "0x84 - Channel Mode Register Channel"]
    #[inline(always)]
    pub const fn capture_cmr2(&self) -> &CAPTURE_CMR {
        unsafe { &*(self as *const Self).cast::<u8>().add(132usize).cast() }
    }
}
#[doc = "BCR (w) register accessor: an alias for `Reg<BCR_SPEC>`"]
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
#[doc = "TC Block Control Register"]
pub mod bcr;
#[doc = "BMR (rw) register accessor: an alias for `Reg<BMR_SPEC>`"]
pub type BMR = crate::Reg<bmr::BMR_SPEC>;
#[doc = "TC Block Mode Register"]
pub mod bmr;
#[doc = "CCR (w) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Channel Control Register Channel"]
pub mod ccr;
#[doc = "CAPTURE_CMR (rw) register accessor: an alias for `Reg<CAPTURE_CMR_SPEC>`"]
pub type CAPTURE_CMR = crate::Reg<capture_cmr::CAPTURE_CMR_SPEC>;
#[doc = "Channel Mode Register Channel"]
pub mod capture_cmr;
#[doc = "WAVEFORM_CMR_ALT (rw) register accessor: an alias for `Reg<WAVEFORM_CMR_ALT_SPEC>`"]
pub type WAVEFORM_CMR_ALT = crate::Reg<waveform_cmr_alt::WAVEFORM_CMR_ALT_SPEC>;
#[doc = "Channel Mode Register Channel"]
pub mod waveform_cmr_alt;
#[doc = "CV (r) register accessor: an alias for `Reg<CV_SPEC>`"]
pub type CV = crate::Reg<cv::CV_SPEC>;
#[doc = "Counter Value Channel"]
pub mod cv;
#[doc = "FEATURES (r) register accessor: an alias for `Reg<FEATURES_SPEC>`"]
pub type FEATURES = crate::Reg<features::FEATURES_SPEC>;
#[doc = "Features Register"]
pub mod features;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register Channel"]
pub mod idr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register Channel"]
pub mod ier;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register Channel"]
pub mod imr;
#[doc = "RA (rw) register accessor: an alias for `Reg<RA_SPEC>`"]
pub type RA = crate::Reg<ra::RA_SPEC>;
#[doc = "Register A Channel"]
pub mod ra;
#[doc = "RB (rw) register accessor: an alias for `Reg<RB_SPEC>`"]
pub type RB = crate::Reg<rb::RB_SPEC>;
#[doc = "Register B Channel"]
pub mod rb;
#[doc = "RC (rw) register accessor: an alias for `Reg<RC_SPEC>`"]
pub type RC = crate::Reg<rc::RC_SPEC>;
#[doc = "Register C Channel"]
pub mod rc;
#[doc = "SMMR (rw) register accessor: an alias for `Reg<SMMR_SPEC>`"]
pub type SMMR = crate::Reg<smmr::SMMR_SPEC>;
#[doc = "Stepper Motor Mode Register"]
pub mod smmr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register Channel"]
pub mod sr;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
#[doc = "WPMR (rw) register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
