#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - Master Configuration Register"]
    pub mcfg: [MCFG; 16],
    #[doc = "0x40..0x80 - Slave Configuration Register"]
    pub scfg: [SCFG; 16],
    #[doc = "0x80 - Priority Register A for Slave"]
    pub pras0: PRAS,
    #[doc = "0x84 - Priority Register B for Slave"]
    pub prbs0: PRBS,
    #[doc = "0x88 - Priority Register A for Slave"]
    pub pras1: PRAS,
    #[doc = "0x8c - Priority Register B for Slave"]
    pub prbs1: PRBS,
    #[doc = "0x90 - Priority Register A for Slave"]
    pub pras2: PRAS,
    #[doc = "0x94 - Priority Register B for Slave"]
    pub prbs2: PRBS,
    #[doc = "0x98 - Priority Register A for Slave"]
    pub pras3: PRAS,
    #[doc = "0x9c - Priority Register B for Slave"]
    pub prbs3: PRBS,
    #[doc = "0xa0 - Priority Register A for Slave"]
    pub pras4: PRAS,
    #[doc = "0xa4 - Priority Register B for Slave"]
    pub prbs4: PRBS,
    #[doc = "0xa8 - Priority Register A for Slave"]
    pub pras5: PRAS,
    #[doc = "0xac - Priority Register B for Slave"]
    pub prbs5: PRBS,
    #[doc = "0xb0 - Priority Register A for Slave"]
    pub pras6: PRAS,
    #[doc = "0xb4 - Priority Register B for Slave"]
    pub prbs6: PRBS,
    #[doc = "0xb8 - Priority Register A for Slave"]
    pub pras7: PRAS,
    #[doc = "0xbc - Priority Register B for Slave"]
    pub prbs7: PRBS,
    #[doc = "0xc0 - Priority Register A for Slave"]
    pub pras8: PRAS,
    #[doc = "0xc4 - Priority Register B for Slave"]
    pub prbs8: PRBS,
    #[doc = "0xc8 - Priority Register A for Slave"]
    pub pras9: PRAS,
    #[doc = "0xcc - Priority Register B for Slave"]
    pub prbs9: PRBS,
    #[doc = "0xd0 - Priority Register A for Slave"]
    pub pras10: PRAS,
    #[doc = "0xd4 - Priority Register B for Slave"]
    pub prbs10: PRBS,
    #[doc = "0xd8 - Priority Register A for Slave"]
    pub pras11: PRAS,
    #[doc = "0xdc - Priority Register B for Slave"]
    pub prbs11: PRBS,
    #[doc = "0xe0 - Priority Register A for Slave"]
    pub pras12: PRAS,
    #[doc = "0xe4 - Priority Register B for Slave"]
    pub prbs12: PRBS,
    #[doc = "0xe8 - Priority Register A for Slave"]
    pub pras13: PRAS,
    #[doc = "0xec - Priority Register B for Slave"]
    pub prbs13: PRBS,
    #[doc = "0xf0 - Priority Register A for Slave"]
    pub pras14: PRAS,
    #[doc = "0xf4 - Priority Register B for Slave"]
    pub prbs14: PRBS,
    #[doc = "0xf8 - Priority Register A for Slave"]
    pub pras15: PRAS,
    #[doc = "0xfc - Priority Register B for Slave"]
    pub prbs15: PRBS,
    #[doc = "0x100 - Master Remap Control Register"]
    pub mrcr: MRCR,
    _reserved35: [u8; 0x0c],
    #[doc = "0x110..0x150 - Special Function Register"]
    pub sfr: [SFR; 16],
}
#[doc = "MCFG (rw) register accessor: an alias for `Reg<MCFG_SPEC>`"]
pub type MCFG = crate::Reg<mcfg::MCFG_SPEC>;
#[doc = "Master Configuration Register"]
pub mod mcfg;
#[doc = "MRCR (rw) register accessor: an alias for `Reg<MRCR_SPEC>`"]
pub type MRCR = crate::Reg<mrcr::MRCR_SPEC>;
#[doc = "Master Remap Control Register"]
pub mod mrcr;
#[doc = "PRAS (rw) register accessor: an alias for `Reg<PRAS_SPEC>`"]
pub type PRAS = crate::Reg<pras::PRAS_SPEC>;
#[doc = "Priority Register A for Slave"]
pub mod pras;
#[doc = "PRBS (rw) register accessor: an alias for `Reg<PRBS_SPEC>`"]
pub type PRBS = crate::Reg<prbs::PRBS_SPEC>;
#[doc = "Priority Register B for Slave"]
pub mod prbs;
#[doc = "SCFG (rw) register accessor: an alias for `Reg<SCFG_SPEC>`"]
pub type SCFG = crate::Reg<scfg::SCFG_SPEC>;
#[doc = "Slave Configuration Register"]
pub mod scfg;
#[doc = "SFR (rw) register accessor: an alias for `Reg<SFR_SPEC>`"]
pub type SFR = crate::Reg<sfr::SFR_SPEC>;
#[doc = "Special Function Register"]
pub mod sfr;
