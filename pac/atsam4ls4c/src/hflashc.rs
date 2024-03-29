#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Controller Control Register"]
    pub fcr: FCR,
    #[doc = "0x04 - Flash Controller Command Register"]
    pub fcmd: FCMD,
    #[doc = "0x08 - Flash Controller Status Register"]
    pub fsr: FSR,
    #[doc = "0x0c - Flash Controller Parameter Register"]
    pub fpr: FPR,
    #[doc = "0x10 - Flash Controller Version Register"]
    pub version: VERSION,
    #[doc = "0x14 - Flash Controller General Purpose Fuse Register High"]
    pub fgpfrhi: FGPFRHI,
    #[doc = "0x18 - Flash Controller General Purpose Fuse Register Low"]
    pub fgpfrlo: FGPFRLO,
}
#[doc = "FCMD (rw) register accessor: an alias for `Reg<FCMD_SPEC>`"]
pub type FCMD = crate::Reg<fcmd::FCMD_SPEC>;
#[doc = "Flash Controller Command Register"]
pub mod fcmd;
#[doc = "FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "Flash Controller Control Register"]
pub mod fcr;
#[doc = "FGPFRHI (rw) register accessor: an alias for `Reg<FGPFRHI_SPEC>`"]
pub type FGPFRHI = crate::Reg<fgpfrhi::FGPFRHI_SPEC>;
#[doc = "Flash Controller General Purpose Fuse Register High"]
pub mod fgpfrhi;
#[doc = "FGPFRLO (rw) register accessor: an alias for `Reg<FGPFRLO_SPEC>`"]
pub type FGPFRLO = crate::Reg<fgpfrlo::FGPFRLO_SPEC>;
#[doc = "Flash Controller General Purpose Fuse Register Low"]
pub mod fgpfrlo;
#[doc = "FPR (r) register accessor: an alias for `Reg<FPR_SPEC>`"]
pub type FPR = crate::Reg<fpr::FPR_SPEC>;
#[doc = "Flash Controller Parameter Register"]
pub mod fpr;
#[doc = "FSR (rw) register accessor: an alias for `Reg<FSR_SPEC>`"]
pub type FSR = crate::Reg<fsr::FSR_SPEC>;
#[doc = "Flash Controller Status Register"]
pub mod fsr;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Flash Controller Version Register"]
pub mod version;
