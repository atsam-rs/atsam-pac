#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Control Register (channel = 0)"]
    pub ccr0: CCR0,
    _reserved_1_cmr0: [u8; 0x04],
    #[doc = "0x08 - Stepper Motor Mode Register (channel = 0)"]
    pub smmr0: SMMR0,
    #[doc = "0x0c - Register AB (channel = 0)"]
    pub rab0: RAB0,
    #[doc = "0x10 - Counter Value (channel = 0)"]
    pub cv0: CV0,
    #[doc = "0x14 - Register A (channel = 0)"]
    pub ra0: RA0,
    #[doc = "0x18 - Register B (channel = 0)"]
    pub rb0: RB0,
    #[doc = "0x1c - Register C (channel = 0)"]
    pub rc0: RC0,
    #[doc = "0x20 - Status Register (channel = 0)"]
    pub sr0: SR0,
    #[doc = "0x24 - Interrupt Enable Register (channel = 0)"]
    pub ier0: IER0,
    #[doc = "0x28 - Interrupt Disable Register (channel = 0)"]
    pub idr0: IDR0,
    #[doc = "0x2c - Interrupt Mask Register (channel = 0)"]
    pub imr0: IMR0,
    #[doc = "0x30 - Extended Mode Register (channel = 0)"]
    pub emr0: EMR0,
    _reserved13: [u8; 0x0c],
    #[doc = "0x40 - Channel Control Register (channel = 1)"]
    pub ccr1: CCR1,
    _reserved_14_cmr1: [u8; 0x04],
    #[doc = "0x48 - Stepper Motor Mode Register (channel = 1)"]
    pub smmr1: SMMR1,
    #[doc = "0x4c - Register AB (channel = 1)"]
    pub rab1: RAB1,
    #[doc = "0x50 - Counter Value (channel = 1)"]
    pub cv1: CV1,
    #[doc = "0x54 - Register A (channel = 1)"]
    pub ra1: RA1,
    #[doc = "0x58 - Register B (channel = 1)"]
    pub rb1: RB1,
    #[doc = "0x5c - Register C (channel = 1)"]
    pub rc1: RC1,
    #[doc = "0x60 - Status Register (channel = 1)"]
    pub sr1: SR1,
    #[doc = "0x64 - Interrupt Enable Register (channel = 1)"]
    pub ier1: IER1,
    #[doc = "0x68 - Interrupt Disable Register (channel = 1)"]
    pub idr1: IDR1,
    #[doc = "0x6c - Interrupt Mask Register (channel = 1)"]
    pub imr1: IMR1,
    #[doc = "0x70 - Extended Mode Register (channel = 1)"]
    pub emr1: EMR1,
    _reserved26: [u8; 0x0c],
    #[doc = "0x80 - Channel Control Register (channel = 2)"]
    pub ccr2: CCR2,
    _reserved_27_cmr2: [u8; 0x04],
    #[doc = "0x88 - Stepper Motor Mode Register (channel = 2)"]
    pub smmr2: SMMR2,
    #[doc = "0x8c - Register AB (channel = 2)"]
    pub rab2: RAB2,
    #[doc = "0x90 - Counter Value (channel = 2)"]
    pub cv2: CV2,
    #[doc = "0x94 - Register A (channel = 2)"]
    pub ra2: RA2,
    #[doc = "0x98 - Register B (channel = 2)"]
    pub rb2: RB2,
    #[doc = "0x9c - Register C (channel = 2)"]
    pub rc2: RC2,
    #[doc = "0xa0 - Status Register (channel = 2)"]
    pub sr2: SR2,
    #[doc = "0xa4 - Interrupt Enable Register (channel = 2)"]
    pub ier2: IER2,
    #[doc = "0xa8 - Interrupt Disable Register (channel = 2)"]
    pub idr2: IDR2,
    #[doc = "0xac - Interrupt Mask Register (channel = 2)"]
    pub imr2: IMR2,
    #[doc = "0xb0 - Extended Mode Register (channel = 2)"]
    pub emr2: EMR2,
    _reserved39: [u8; 0x0c],
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
    _reserved46: [u8; 0x08],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    _reserved47: [u8; 0x18],
    #[doc = "0x100 - Receive Pointer Register (pdc = 0)"]
    pub rpr0: RPR0,
    #[doc = "0x104 - Receive Counter Register (pdc = 0)"]
    pub rcr0: RCR0,
    _reserved49: [u8; 0x08],
    #[doc = "0x110 - Receive Next Pointer Register (pdc = 0)"]
    pub rnpr0: RNPR0,
    #[doc = "0x114 - Receive Next Counter Register (pdc = 0)"]
    pub rncr0: RNCR0,
    _reserved51: [u8; 0x08],
    #[doc = "0x120 - Transfer Control Register (pdc = 0)"]
    pub ptcr0: PTCR0,
    #[doc = "0x124 - Transfer Status Register (pdc = 0)"]
    pub ptsr0: PTSR0,
    _reserved53: [u8; 0x18],
    #[doc = "0x140 - Receive Pointer Register (pdc = 1)"]
    pub rpr1: RPR1,
    #[doc = "0x144 - Receive Counter Register (pdc = 1)"]
    pub rcr1: RCR1,
    _reserved55: [u8; 0x08],
    #[doc = "0x150 - Receive Next Pointer Register (pdc = 1)"]
    pub rnpr1: RNPR1,
    #[doc = "0x154 - Receive Next Counter Register (pdc = 1)"]
    pub rncr1: RNCR1,
    _reserved57: [u8; 0x08],
    #[doc = "0x160 - Transfer Control Register (pdc = 1)"]
    pub ptcr1: PTCR1,
    #[doc = "0x164 - Transfer Status Register (pdc = 1)"]
    pub ptsr1: PTSR1,
    _reserved59: [u8; 0x18],
    #[doc = "0x180 - Receive Pointer Register (pdc = 2)"]
    pub rpr2: RPR2,
    #[doc = "0x184 - Receive Counter Register (pdc = 2)"]
    pub rcr2: RCR2,
    _reserved61: [u8; 0x08],
    #[doc = "0x190 - Receive Next Pointer Register (pdc = 2)"]
    pub rnpr2: RNPR2,
    #[doc = "0x194 - Receive Next Counter Register (pdc = 2)"]
    pub rncr2: RNCR2,
    _reserved63: [u8; 0x08],
    #[doc = "0x1a0 - Transfer Control Register (pdc = 2)"]
    pub ptcr2: PTCR2,
    #[doc = "0x1a4 - Transfer Status Register (pdc = 2)"]
    pub ptsr2: PTSR2,
}
impl RegisterBlock {
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn wave_eq_1_cmr0_wave_eq_1(&self) -> &WAVE_EQ_1_CMR0_WAVE_EQ_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn cmr0(&self) -> &CMR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x44 - Channel Mode Register (channel = 1)"]
    #[inline(always)]
    pub const fn wave_eq_1_cmr1_wave_eq_1(&self) -> &WAVE_EQ_1_CMR1_WAVE_EQ_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(68usize).cast() }
    }
    #[doc = "0x44 - Channel Mode Register (channel = 1)"]
    #[inline(always)]
    pub const fn cmr1(&self) -> &CMR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(68usize).cast() }
    }
    #[doc = "0x84 - Channel Mode Register (channel = 2)"]
    #[inline(always)]
    pub const fn wave_eq_1_cmr2_wave_eq_1(&self) -> &WAVE_EQ_1_CMR2_WAVE_EQ_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(132usize).cast() }
    }
    #[doc = "0x84 - Channel Mode Register (channel = 2)"]
    #[inline(always)]
    pub const fn cmr2(&self) -> &CMR2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(132usize).cast() }
    }
}
#[doc = "CCR0 (w) register accessor: an alias for `Reg<CCR0_SPEC>`"]
pub type CCR0 = crate::Reg<ccr0::CCR0_SPEC>;
#[doc = "Channel Control Register (channel = 0)"]
pub mod ccr0;
#[doc = "CMR0 (rw) register accessor: an alias for `Reg<CMR0_SPEC>`"]
pub type CMR0 = crate::Reg<cmr0::CMR0_SPEC>;
#[doc = "Channel Mode Register (channel = 0)"]
pub mod cmr0;
#[doc = "WAVE_EQ_1_CMR0_WAVE_EQ_1 (rw) register accessor: an alias for `Reg<WAVE_EQ_1_CMR0_WAVE_EQ_1_SPEC>`"]
pub type WAVE_EQ_1_CMR0_WAVE_EQ_1 =
    crate::Reg<wave_eq_1_cmr0_wave_eq_1::WAVE_EQ_1_CMR0_WAVE_EQ_1_SPEC>;
#[doc = "Channel Mode Register (channel = 0)"]
pub mod wave_eq_1_cmr0_wave_eq_1;
#[doc = "SMMR0 (rw) register accessor: an alias for `Reg<SMMR0_SPEC>`"]
pub type SMMR0 = crate::Reg<smmr0::SMMR0_SPEC>;
#[doc = "Stepper Motor Mode Register (channel = 0)"]
pub mod smmr0;
#[doc = "RAB0 (r) register accessor: an alias for `Reg<RAB0_SPEC>`"]
pub type RAB0 = crate::Reg<rab0::RAB0_SPEC>;
#[doc = "Register AB (channel = 0)"]
pub mod rab0;
#[doc = "CV0 (r) register accessor: an alias for `Reg<CV0_SPEC>`"]
pub type CV0 = crate::Reg<cv0::CV0_SPEC>;
#[doc = "Counter Value (channel = 0)"]
pub mod cv0;
#[doc = "RA0 (rw) register accessor: an alias for `Reg<RA0_SPEC>`"]
pub type RA0 = crate::Reg<ra0::RA0_SPEC>;
#[doc = "Register A (channel = 0)"]
pub mod ra0;
#[doc = "RB0 (rw) register accessor: an alias for `Reg<RB0_SPEC>`"]
pub type RB0 = crate::Reg<rb0::RB0_SPEC>;
#[doc = "Register B (channel = 0)"]
pub mod rb0;
#[doc = "RC0 (rw) register accessor: an alias for `Reg<RC0_SPEC>`"]
pub type RC0 = crate::Reg<rc0::RC0_SPEC>;
#[doc = "Register C (channel = 0)"]
pub mod rc0;
#[doc = "SR0 (r) register accessor: an alias for `Reg<SR0_SPEC>`"]
pub type SR0 = crate::Reg<sr0::SR0_SPEC>;
#[doc = "Status Register (channel = 0)"]
pub mod sr0;
#[doc = "IER0 (w) register accessor: an alias for `Reg<IER0_SPEC>`"]
pub type IER0 = crate::Reg<ier0::IER0_SPEC>;
#[doc = "Interrupt Enable Register (channel = 0)"]
pub mod ier0;
#[doc = "IDR0 (w) register accessor: an alias for `Reg<IDR0_SPEC>`"]
pub type IDR0 = crate::Reg<idr0::IDR0_SPEC>;
#[doc = "Interrupt Disable Register (channel = 0)"]
pub mod idr0;
#[doc = "IMR0 (r) register accessor: an alias for `Reg<IMR0_SPEC>`"]
pub type IMR0 = crate::Reg<imr0::IMR0_SPEC>;
#[doc = "Interrupt Mask Register (channel = 0)"]
pub mod imr0;
#[doc = "EMR0 (rw) register accessor: an alias for `Reg<EMR0_SPEC>`"]
pub type EMR0 = crate::Reg<emr0::EMR0_SPEC>;
#[doc = "Extended Mode Register (channel = 0)"]
pub mod emr0;
#[doc = "CCR1 (w) register accessor: an alias for `Reg<CCR1_SPEC>`"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "Channel Control Register (channel = 1)"]
pub mod ccr1;
#[doc = "CMR1 (rw) register accessor: an alias for `Reg<CMR1_SPEC>`"]
pub type CMR1 = crate::Reg<cmr1::CMR1_SPEC>;
#[doc = "Channel Mode Register (channel = 1)"]
pub mod cmr1;
#[doc = "WAVE_EQ_1_CMR1_WAVE_EQ_1 (rw) register accessor: an alias for `Reg<WAVE_EQ_1_CMR1_WAVE_EQ_1_SPEC>`"]
pub type WAVE_EQ_1_CMR1_WAVE_EQ_1 =
    crate::Reg<wave_eq_1_cmr1_wave_eq_1::WAVE_EQ_1_CMR1_WAVE_EQ_1_SPEC>;
#[doc = "Channel Mode Register (channel = 1)"]
pub mod wave_eq_1_cmr1_wave_eq_1;
#[doc = "SMMR1 (rw) register accessor: an alias for `Reg<SMMR1_SPEC>`"]
pub type SMMR1 = crate::Reg<smmr1::SMMR1_SPEC>;
#[doc = "Stepper Motor Mode Register (channel = 1)"]
pub mod smmr1;
#[doc = "RAB1 (r) register accessor: an alias for `Reg<RAB1_SPEC>`"]
pub type RAB1 = crate::Reg<rab1::RAB1_SPEC>;
#[doc = "Register AB (channel = 1)"]
pub mod rab1;
#[doc = "CV1 (r) register accessor: an alias for `Reg<CV1_SPEC>`"]
pub type CV1 = crate::Reg<cv1::CV1_SPEC>;
#[doc = "Counter Value (channel = 1)"]
pub mod cv1;
#[doc = "RA1 (rw) register accessor: an alias for `Reg<RA1_SPEC>`"]
pub type RA1 = crate::Reg<ra1::RA1_SPEC>;
#[doc = "Register A (channel = 1)"]
pub mod ra1;
#[doc = "RB1 (rw) register accessor: an alias for `Reg<RB1_SPEC>`"]
pub type RB1 = crate::Reg<rb1::RB1_SPEC>;
#[doc = "Register B (channel = 1)"]
pub mod rb1;
#[doc = "RC1 (rw) register accessor: an alias for `Reg<RC1_SPEC>`"]
pub type RC1 = crate::Reg<rc1::RC1_SPEC>;
#[doc = "Register C (channel = 1)"]
pub mod rc1;
#[doc = "SR1 (r) register accessor: an alias for `Reg<SR1_SPEC>`"]
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
#[doc = "Status Register (channel = 1)"]
pub mod sr1;
#[doc = "IER1 (w) register accessor: an alias for `Reg<IER1_SPEC>`"]
pub type IER1 = crate::Reg<ier1::IER1_SPEC>;
#[doc = "Interrupt Enable Register (channel = 1)"]
pub mod ier1;
#[doc = "IDR1 (w) register accessor: an alias for `Reg<IDR1_SPEC>`"]
pub type IDR1 = crate::Reg<idr1::IDR1_SPEC>;
#[doc = "Interrupt Disable Register (channel = 1)"]
pub mod idr1;
#[doc = "IMR1 (r) register accessor: an alias for `Reg<IMR1_SPEC>`"]
pub type IMR1 = crate::Reg<imr1::IMR1_SPEC>;
#[doc = "Interrupt Mask Register (channel = 1)"]
pub mod imr1;
#[doc = "EMR1 (rw) register accessor: an alias for `Reg<EMR1_SPEC>`"]
pub type EMR1 = crate::Reg<emr1::EMR1_SPEC>;
#[doc = "Extended Mode Register (channel = 1)"]
pub mod emr1;
#[doc = "CCR2 (w) register accessor: an alias for `Reg<CCR2_SPEC>`"]
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
#[doc = "Channel Control Register (channel = 2)"]
pub mod ccr2;
#[doc = "CMR2 (rw) register accessor: an alias for `Reg<CMR2_SPEC>`"]
pub type CMR2 = crate::Reg<cmr2::CMR2_SPEC>;
#[doc = "Channel Mode Register (channel = 2)"]
pub mod cmr2;
#[doc = "WAVE_EQ_1_CMR2_WAVE_EQ_1 (rw) register accessor: an alias for `Reg<WAVE_EQ_1_CMR2_WAVE_EQ_1_SPEC>`"]
pub type WAVE_EQ_1_CMR2_WAVE_EQ_1 =
    crate::Reg<wave_eq_1_cmr2_wave_eq_1::WAVE_EQ_1_CMR2_WAVE_EQ_1_SPEC>;
#[doc = "Channel Mode Register (channel = 2)"]
pub mod wave_eq_1_cmr2_wave_eq_1;
#[doc = "SMMR2 (rw) register accessor: an alias for `Reg<SMMR2_SPEC>`"]
pub type SMMR2 = crate::Reg<smmr2::SMMR2_SPEC>;
#[doc = "Stepper Motor Mode Register (channel = 2)"]
pub mod smmr2;
#[doc = "RAB2 (r) register accessor: an alias for `Reg<RAB2_SPEC>`"]
pub type RAB2 = crate::Reg<rab2::RAB2_SPEC>;
#[doc = "Register AB (channel = 2)"]
pub mod rab2;
#[doc = "CV2 (r) register accessor: an alias for `Reg<CV2_SPEC>`"]
pub type CV2 = crate::Reg<cv2::CV2_SPEC>;
#[doc = "Counter Value (channel = 2)"]
pub mod cv2;
#[doc = "RA2 (rw) register accessor: an alias for `Reg<RA2_SPEC>`"]
pub type RA2 = crate::Reg<ra2::RA2_SPEC>;
#[doc = "Register A (channel = 2)"]
pub mod ra2;
#[doc = "RB2 (rw) register accessor: an alias for `Reg<RB2_SPEC>`"]
pub type RB2 = crate::Reg<rb2::RB2_SPEC>;
#[doc = "Register B (channel = 2)"]
pub mod rb2;
#[doc = "RC2 (rw) register accessor: an alias for `Reg<RC2_SPEC>`"]
pub type RC2 = crate::Reg<rc2::RC2_SPEC>;
#[doc = "Register C (channel = 2)"]
pub mod rc2;
#[doc = "SR2 (r) register accessor: an alias for `Reg<SR2_SPEC>`"]
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
#[doc = "Status Register (channel = 2)"]
pub mod sr2;
#[doc = "IER2 (w) register accessor: an alias for `Reg<IER2_SPEC>`"]
pub type IER2 = crate::Reg<ier2::IER2_SPEC>;
#[doc = "Interrupt Enable Register (channel = 2)"]
pub mod ier2;
#[doc = "IDR2 (w) register accessor: an alias for `Reg<IDR2_SPEC>`"]
pub type IDR2 = crate::Reg<idr2::IDR2_SPEC>;
#[doc = "Interrupt Disable Register (channel = 2)"]
pub mod idr2;
#[doc = "IMR2 (r) register accessor: an alias for `Reg<IMR2_SPEC>`"]
pub type IMR2 = crate::Reg<imr2::IMR2_SPEC>;
#[doc = "Interrupt Mask Register (channel = 2)"]
pub mod imr2;
#[doc = "EMR2 (rw) register accessor: an alias for `Reg<EMR2_SPEC>`"]
pub type EMR2 = crate::Reg<emr2::EMR2_SPEC>;
#[doc = "Extended Mode Register (channel = 2)"]
pub mod emr2;
#[doc = "BCR (w) register accessor: an alias for `Reg<BCR_SPEC>`"]
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
#[doc = "Block Control Register"]
pub mod bcr;
#[doc = "BMR (rw) register accessor: an alias for `Reg<BMR_SPEC>`"]
pub type BMR = crate::Reg<bmr::BMR_SPEC>;
#[doc = "Block Mode Register"]
pub mod bmr;
#[doc = "QIER (w) register accessor: an alias for `Reg<QIER_SPEC>`"]
pub type QIER = crate::Reg<qier::QIER_SPEC>;
#[doc = "QDEC Interrupt Enable Register"]
pub mod qier;
#[doc = "QIDR (w) register accessor: an alias for `Reg<QIDR_SPEC>`"]
pub type QIDR = crate::Reg<qidr::QIDR_SPEC>;
#[doc = "QDEC Interrupt Disable Register"]
pub mod qidr;
#[doc = "QIMR (r) register accessor: an alias for `Reg<QIMR_SPEC>`"]
pub type QIMR = crate::Reg<qimr::QIMR_SPEC>;
#[doc = "QDEC Interrupt Mask Register"]
pub mod qimr;
#[doc = "QISR (r) register accessor: an alias for `Reg<QISR_SPEC>`"]
pub type QISR = crate::Reg<qisr::QISR_SPEC>;
#[doc = "QDEC Interrupt Status Register"]
pub mod qisr;
#[doc = "FMR (rw) register accessor: an alias for `Reg<FMR_SPEC>`"]
pub type FMR = crate::Reg<fmr::FMR_SPEC>;
#[doc = "Fault Mode Register"]
pub mod fmr;
#[doc = "WPMR (rw) register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "RPR0 (rw) register accessor: an alias for `Reg<RPR0_SPEC>`"]
pub type RPR0 = crate::Reg<rpr0::RPR0_SPEC>;
#[doc = "Receive Pointer Register (pdc = 0)"]
pub mod rpr0;
#[doc = "RCR0 (rw) register accessor: an alias for `Reg<RCR0_SPEC>`"]
pub type RCR0 = crate::Reg<rcr0::RCR0_SPEC>;
#[doc = "Receive Counter Register (pdc = 0)"]
pub mod rcr0;
#[doc = "RNPR0 (rw) register accessor: an alias for `Reg<RNPR0_SPEC>`"]
pub type RNPR0 = crate::Reg<rnpr0::RNPR0_SPEC>;
#[doc = "Receive Next Pointer Register (pdc = 0)"]
pub mod rnpr0;
#[doc = "RNCR0 (rw) register accessor: an alias for `Reg<RNCR0_SPEC>`"]
pub type RNCR0 = crate::Reg<rncr0::RNCR0_SPEC>;
#[doc = "Receive Next Counter Register (pdc = 0)"]
pub mod rncr0;
#[doc = "PTCR0 (w) register accessor: an alias for `Reg<PTCR0_SPEC>`"]
pub type PTCR0 = crate::Reg<ptcr0::PTCR0_SPEC>;
#[doc = "Transfer Control Register (pdc = 0)"]
pub mod ptcr0;
#[doc = "PTSR0 (r) register accessor: an alias for `Reg<PTSR0_SPEC>`"]
pub type PTSR0 = crate::Reg<ptsr0::PTSR0_SPEC>;
#[doc = "Transfer Status Register (pdc = 0)"]
pub mod ptsr0;
#[doc = "RPR1 (rw) register accessor: an alias for `Reg<RPR1_SPEC>`"]
pub type RPR1 = crate::Reg<rpr1::RPR1_SPEC>;
#[doc = "Receive Pointer Register (pdc = 1)"]
pub mod rpr1;
#[doc = "RCR1 (rw) register accessor: an alias for `Reg<RCR1_SPEC>`"]
pub type RCR1 = crate::Reg<rcr1::RCR1_SPEC>;
#[doc = "Receive Counter Register (pdc = 1)"]
pub mod rcr1;
#[doc = "RNPR1 (rw) register accessor: an alias for `Reg<RNPR1_SPEC>`"]
pub type RNPR1 = crate::Reg<rnpr1::RNPR1_SPEC>;
#[doc = "Receive Next Pointer Register (pdc = 1)"]
pub mod rnpr1;
#[doc = "RNCR1 (rw) register accessor: an alias for `Reg<RNCR1_SPEC>`"]
pub type RNCR1 = crate::Reg<rncr1::RNCR1_SPEC>;
#[doc = "Receive Next Counter Register (pdc = 1)"]
pub mod rncr1;
#[doc = "PTCR1 (w) register accessor: an alias for `Reg<PTCR1_SPEC>`"]
pub type PTCR1 = crate::Reg<ptcr1::PTCR1_SPEC>;
#[doc = "Transfer Control Register (pdc = 1)"]
pub mod ptcr1;
#[doc = "PTSR1 (r) register accessor: an alias for `Reg<PTSR1_SPEC>`"]
pub type PTSR1 = crate::Reg<ptsr1::PTSR1_SPEC>;
#[doc = "Transfer Status Register (pdc = 1)"]
pub mod ptsr1;
#[doc = "RPR2 (rw) register accessor: an alias for `Reg<RPR2_SPEC>`"]
pub type RPR2 = crate::Reg<rpr2::RPR2_SPEC>;
#[doc = "Receive Pointer Register (pdc = 2)"]
pub mod rpr2;
#[doc = "RCR2 (rw) register accessor: an alias for `Reg<RCR2_SPEC>`"]
pub type RCR2 = crate::Reg<rcr2::RCR2_SPEC>;
#[doc = "Receive Counter Register (pdc = 2)"]
pub mod rcr2;
#[doc = "RNPR2 (rw) register accessor: an alias for `Reg<RNPR2_SPEC>`"]
pub type RNPR2 = crate::Reg<rnpr2::RNPR2_SPEC>;
#[doc = "Receive Next Pointer Register (pdc = 2)"]
pub mod rnpr2;
#[doc = "RNCR2 (rw) register accessor: an alias for `Reg<RNCR2_SPEC>`"]
pub type RNCR2 = crate::Reg<rncr2::RNCR2_SPEC>;
#[doc = "Receive Next Counter Register (pdc = 2)"]
pub mod rncr2;
#[doc = "PTCR2 (w) register accessor: an alias for `Reg<PTCR2_SPEC>`"]
pub type PTCR2 = crate::Reg<ptcr2::PTCR2_SPEC>;
#[doc = "Transfer Control Register (pdc = 2)"]
pub mod ptcr2;
#[doc = "PTSR2 (r) register accessor: an alias for `Reg<PTSR2_SPEC>`"]
pub type PTSR2 = crate::Reg<ptsr2::PTSR2_SPEC>;
#[doc = "Transfer Status Register (pdc = 2)"]
pub mod ptsr2;
