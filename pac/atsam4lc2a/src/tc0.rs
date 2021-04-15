#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Control Register Channel"]
    pub ccr0: crate::Reg<ccr::CCR_SPEC>,
    _reserved_1_cmr: [u8; 4usize],
    #[doc = "0x08 - Stepper Motor Mode Register"]
    pub smmr0: crate::Reg<smmr::SMMR_SPEC>,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Counter Value Channel"]
    pub cv0: crate::Reg<cv::CV_SPEC>,
    #[doc = "0x14 - Register A Channel"]
    pub ra0: crate::Reg<ra::RA_SPEC>,
    #[doc = "0x18 - Register B Channel"]
    pub rb0: crate::Reg<rb::RB_SPEC>,
    #[doc = "0x1c - Register C Channel"]
    pub rc0: crate::Reg<rc::RC_SPEC>,
    #[doc = "0x20 - Status Register Channel"]
    pub sr0: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x24 - Interrupt Enable Register Channel"]
    pub ier0: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x28 - Interrupt Disable Register Channel"]
    pub idr0: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x2c - Interrupt Mask Register Channel"]
    pub imr0: crate::Reg<imr::IMR_SPEC>,
    _reserved11: [u8; 16usize],
    #[doc = "0x40 - Channel Control Register Channel"]
    pub ccr1: crate::Reg<ccr::CCR_SPEC>,
    _reserved_12_cmr: [u8; 4usize],
    #[doc = "0x48 - Stepper Motor Mode Register"]
    pub smmr1: crate::Reg<smmr::SMMR_SPEC>,
    _reserved14: [u8; 4usize],
    #[doc = "0x50 - Counter Value Channel"]
    pub cv1: crate::Reg<cv::CV_SPEC>,
    #[doc = "0x54 - Register A Channel"]
    pub ra1: crate::Reg<ra::RA_SPEC>,
    #[doc = "0x58 - Register B Channel"]
    pub rb1: crate::Reg<rb::RB_SPEC>,
    #[doc = "0x5c - Register C Channel"]
    pub rc1: crate::Reg<rc::RC_SPEC>,
    #[doc = "0x60 - Status Register Channel"]
    pub sr1: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x64 - Interrupt Enable Register Channel"]
    pub ier1: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x68 - Interrupt Disable Register Channel"]
    pub idr1: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x6c - Interrupt Mask Register Channel"]
    pub imr1: crate::Reg<imr::IMR_SPEC>,
    _reserved22: [u8; 16usize],
    #[doc = "0x80 - Channel Control Register Channel"]
    pub ccr2: crate::Reg<ccr::CCR_SPEC>,
    _reserved_23_cmr: [u8; 4usize],
    #[doc = "0x88 - Stepper Motor Mode Register"]
    pub smmr2: crate::Reg<smmr::SMMR_SPEC>,
    _reserved25: [u8; 4usize],
    #[doc = "0x90 - Counter Value Channel"]
    pub cv2: crate::Reg<cv::CV_SPEC>,
    #[doc = "0x94 - Register A Channel"]
    pub ra2: crate::Reg<ra::RA_SPEC>,
    #[doc = "0x98 - Register B Channel"]
    pub rb2: crate::Reg<rb::RB_SPEC>,
    #[doc = "0x9c - Register C Channel"]
    pub rc2: crate::Reg<rc::RC_SPEC>,
    #[doc = "0xa0 - Status Register Channel"]
    pub sr2: crate::Reg<sr::SR_SPEC>,
    #[doc = "0xa4 - Interrupt Enable Register Channel"]
    pub ier2: crate::Reg<ier::IER_SPEC>,
    #[doc = "0xa8 - Interrupt Disable Register Channel"]
    pub idr2: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0xac - Interrupt Mask Register Channel"]
    pub imr2: crate::Reg<imr::IMR_SPEC>,
    _reserved33: [u8; 16usize],
    #[doc = "0xc0 - TC Block Control Register"]
    pub bcr: crate::Reg<bcr::BCR_SPEC>,
    #[doc = "0xc4 - TC Block Mode Register"]
    pub bmr: crate::Reg<bmr::BMR_SPEC>,
    _reserved35: [u8; 28usize],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    _reserved36: [u8; 16usize],
    #[doc = "0xf8 - Features Register"]
    pub features: crate::Reg<features::FEATURES_SPEC>,
    #[doc = "0xfc - Version Register"]
    pub version: crate::Reg<version::VERSION_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x04 - Channel Mode Register Channel"]
    #[inline(always)]
    pub fn cmr0(&self) -> &crate::Reg<cmr::CMR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize) as *const crate::Reg<cmr::CMR_SPEC>)
        }
    }
    #[doc = "0x44 - Channel Mode Register Channel"]
    #[inline(always)]
    pub fn cmr1(&self) -> &crate::Reg<cmr::CMR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const crate::Reg<cmr::CMR_SPEC>)
        }
    }
    #[doc = "0x84 - Channel Mode Register Channel"]
    #[inline(always)]
    pub fn cmr2(&self) -> &crate::Reg<cmr::CMR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(132usize)
                as *const crate::Reg<cmr::CMR_SPEC>)
        }
    }
}
#[doc = "BCR register accessor: an alias for `Reg<BCR_SPEC>`"]
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
#[doc = "TC Block Control Register"]
pub mod bcr;
#[doc = "BMR register accessor: an alias for `Reg<BMR_SPEC>`"]
pub type BMR = crate::Reg<bmr::BMR_SPEC>;
#[doc = "TC Block Mode Register"]
pub mod bmr;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Channel Control Register Channel"]
pub mod ccr;
#[doc = "CMR register accessor: an alias for `Reg<CMR_SPEC>`"]
pub type CMR = crate::Reg<cmr::CMR_SPEC>;
#[doc = "Channel Mode Register Channel"]
pub mod cmr;
#[doc = "CV register accessor: an alias for `Reg<CV_SPEC>`"]
pub type CV = crate::Reg<cv::CV_SPEC>;
#[doc = "Counter Value Channel"]
pub mod cv;
#[doc = "FEATURES register accessor: an alias for `Reg<FEATURES_SPEC>`"]
pub type FEATURES = crate::Reg<features::FEATURES_SPEC>;
#[doc = "Features Register"]
pub mod features;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register Channel"]
pub mod idr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register Channel"]
pub mod ier;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register Channel"]
pub mod imr;
#[doc = "RA register accessor: an alias for `Reg<RA_SPEC>`"]
pub type RA = crate::Reg<ra::RA_SPEC>;
#[doc = "Register A Channel"]
pub mod ra;
#[doc = "RB register accessor: an alias for `Reg<RB_SPEC>`"]
pub type RB = crate::Reg<rb::RB_SPEC>;
#[doc = "Register B Channel"]
pub mod rb;
#[doc = "RC register accessor: an alias for `Reg<RC_SPEC>`"]
pub type RC = crate::Reg<rc::RC_SPEC>;
#[doc = "Register C Channel"]
pub mod rc;
#[doc = "SMMR register accessor: an alias for `Reg<SMMR_SPEC>`"]
pub type SMMR = crate::Reg<smmr::SMMR_SPEC>;
#[doc = "Stepper Motor Mode Register"]
pub mod smmr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register Channel"]
pub mod sr;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
