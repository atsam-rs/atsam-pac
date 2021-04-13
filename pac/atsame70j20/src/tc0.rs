#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Control Register (channel = 0)"]
    pub tc_channel0: TC_CHANNEL,
    _reserved1: [u8; 12usize],
    #[doc = "0x40 - Channel Control Register (channel = 0)"]
    pub tc_channel1: TC_CHANNEL,
    _reserved2: [u8; 12usize],
    #[doc = "0x80 - Channel Control Register (channel = 0)"]
    pub tc_channel2: TC_CHANNEL,
    _reserved3: [u8; 12usize],
    #[doc = "0xc0 - Block Control Register"]
    pub bcr: BCR,
    #[doc = "0xc4 - Block Mode Register"]
    pub bmr: BMR,
    #[doc = "0xc8 - QDEC Interrupt Enable Register"]
    pub qier: QIER,
    #[doc = "0xcc - QDEC Interrupt Disable Register"]
    pub qidr: QIDR,
    #[doc = "0xd0 - QDEC Interrupt Mask Register"]
    pub qimr: QIMR,
    #[doc = "0xd4 - QDEC Interrupt Status Register"]
    pub qisr: QISR,
    #[doc = "0xd8 - Fault Mode Register"]
    pub fmr: FMR,
    _reserved10: [u8; 8usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct TC_CHANNEL {
    #[doc = "0x00 - Channel Control Register (channel = 0)"]
    pub ccr: self::tc_channel::CCR,
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    pub cmr: self::tc_channel::CMR,
    #[doc = "0x08 - Stepper Motor Mode Register (channel = 0)"]
    pub smmr: self::tc_channel::SMMR,
    #[doc = "0x0c - Register AB (channel = 0)"]
    pub rab: self::tc_channel::RAB,
    #[doc = "0x10 - Counter Value (channel = 0)"]
    pub cv: self::tc_channel::CV,
    #[doc = "0x14 - Register A (channel = 0)"]
    pub ra: self::tc_channel::RA,
    #[doc = "0x18 - Register B (channel = 0)"]
    pub rb: self::tc_channel::RB,
    #[doc = "0x1c - Register C (channel = 0)"]
    pub rc: self::tc_channel::RC,
    #[doc = "0x20 - Status Register (channel = 0)"]
    pub sr: self::tc_channel::SR,
    #[doc = "0x24 - Interrupt Enable Register (channel = 0)"]
    pub ier: self::tc_channel::IER,
    #[doc = "0x28 - Interrupt Disable Register (channel = 0)"]
    pub idr: self::tc_channel::IDR,
    #[doc = "0x2c - Interrupt Mask Register (channel = 0)"]
    pub imr: self::tc_channel::IMR,
    #[doc = "0x30 - Extended Mode Register (channel = 0)"]
    pub emr: self::tc_channel::EMR,
}
#[doc = r"Register block"]
#[doc = "Channel Control Register (channel = 0)"]
pub mod tc_channel;
#[doc = "Block Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr](bcr) module"]
pub type BCR = crate::Reg<u32, _BCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR;
#[doc = "`write(|w| ..)` method takes [bcr::W](bcr::W) writer structure"]
impl crate::Writable for BCR {}
#[doc = "Block Control Register"]
pub mod bcr;
#[doc = "Block Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmr](bmr) module"]
pub type BMR = crate::Reg<u32, _BMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMR;
#[doc = "`read()` method returns [bmr::R](bmr::R) reader structure"]
impl crate::Readable for BMR {}
#[doc = "`write(|w| ..)` method takes [bmr::W](bmr::W) writer structure"]
impl crate::Writable for BMR {}
#[doc = "Block Mode Register"]
pub mod bmr;
#[doc = "QDEC Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qier](qier) module"]
pub type QIER = crate::Reg<u32, _QIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QIER;
#[doc = "`write(|w| ..)` method takes [qier::W](qier::W) writer structure"]
impl crate::Writable for QIER {}
#[doc = "QDEC Interrupt Enable Register"]
pub mod qier;
#[doc = "QDEC Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qidr](qidr) module"]
pub type QIDR = crate::Reg<u32, _QIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QIDR;
#[doc = "`write(|w| ..)` method takes [qidr::W](qidr::W) writer structure"]
impl crate::Writable for QIDR {}
#[doc = "QDEC Interrupt Disable Register"]
pub mod qidr;
#[doc = "QDEC Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qimr](qimr) module"]
pub type QIMR = crate::Reg<u32, _QIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QIMR;
#[doc = "`read()` method returns [qimr::R](qimr::R) reader structure"]
impl crate::Readable for QIMR {}
#[doc = "QDEC Interrupt Mask Register"]
pub mod qimr;
#[doc = "QDEC Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qisr](qisr) module"]
pub type QISR = crate::Reg<u32, _QISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QISR;
#[doc = "`read()` method returns [qisr::R](qisr::R) reader structure"]
impl crate::Readable for QISR {}
#[doc = "QDEC Interrupt Status Register"]
pub mod qisr;
#[doc = "Fault Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmr](fmr) module"]
pub type FMR = crate::Reg<u32, _FMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMR;
#[doc = "`read()` method returns [fmr::R](fmr::R) reader structure"]
impl crate::Readable for FMR {}
#[doc = "`write(|w| ..)` method takes [fmr::W](fmr::W) writer structure"]
impl crate::Writable for FMR {}
#[doc = "Fault Mode Register"]
pub mod fmr;
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
