#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x0c - Master Configuration Register"]
    pub matrix_mcfg: [MATRIX_MCFG; 3],
    _reserved1: [u8; 0x34],
    #[doc = "0x40..0x50 - Slave Configuration Register"]
    pub matrix_scfg: [MATRIX_SCFG; 4],
    _reserved2: [u8; 0x30],
    #[doc = "0x80 - Priority Register A for Slave 0"]
    pub matrix_pras0: MATRIX_PRAS0,
    _reserved3: [u8; 0x04],
    #[doc = "0x88 - Priority Register A for Slave 1"]
    pub matrix_pras1: MATRIX_PRAS1,
    _reserved4: [u8; 0x04],
    #[doc = "0x90 - Priority Register A for Slave 2"]
    pub matrix_pras2: MATRIX_PRAS2,
    _reserved5: [u8; 0x04],
    #[doc = "0x98 - Priority Register A for Slave 3"]
    pub matrix_pras3: MATRIX_PRAS3,
    _reserved6: [u8; 0x78],
    #[doc = "0x114 - System I/O Configuration register"]
    pub ccfg_sysio: CCFG_SYSIO,
}
#[doc = "MATRIX_MCFG (rw) register accessor: an alias for `Reg<MATRIX_MCFG_SPEC>`"]
pub type MATRIX_MCFG = crate::Reg<matrix_mcfg::MATRIX_MCFG_SPEC>;
#[doc = "Master Configuration Register"]
pub mod matrix_mcfg;
#[doc = "MATRIX_SCFG (rw) register accessor: an alias for `Reg<MATRIX_SCFG_SPEC>`"]
pub type MATRIX_SCFG = crate::Reg<matrix_scfg::MATRIX_SCFG_SPEC>;
#[doc = "Slave Configuration Register"]
pub mod matrix_scfg;
#[doc = "MATRIX_PRAS0 (rw) register accessor: an alias for `Reg<MATRIX_PRAS0_SPEC>`"]
pub type MATRIX_PRAS0 = crate::Reg<matrix_pras0::MATRIX_PRAS0_SPEC>;
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pras0;
#[doc = "MATRIX_PRAS1 (rw) register accessor: an alias for `Reg<MATRIX_PRAS1_SPEC>`"]
pub type MATRIX_PRAS1 = crate::Reg<matrix_pras1::MATRIX_PRAS1_SPEC>;
#[doc = "Priority Register A for Slave 1"]
pub mod matrix_pras1;
#[doc = "MATRIX_PRAS2 (rw) register accessor: an alias for `Reg<MATRIX_PRAS2_SPEC>`"]
pub type MATRIX_PRAS2 = crate::Reg<matrix_pras2::MATRIX_PRAS2_SPEC>;
#[doc = "Priority Register A for Slave 2"]
pub mod matrix_pras2;
#[doc = "MATRIX_PRAS3 (rw) register accessor: an alias for `Reg<MATRIX_PRAS3_SPEC>`"]
pub type MATRIX_PRAS3 = crate::Reg<matrix_pras3::MATRIX_PRAS3_SPEC>;
#[doc = "Priority Register A for Slave 3"]
pub mod matrix_pras3;
#[doc = "CCFG_SYSIO (rw) register accessor: an alias for `Reg<CCFG_SYSIO_SPEC>`"]
pub type CCFG_SYSIO = crate::Reg<ccfg_sysio::CCFG_SYSIO_SPEC>;
#[doc = "System I/O Configuration register"]
pub mod ccfg_sysio;
