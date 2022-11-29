#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x50 - General Purpose Backup Register"]
    pub gpbr: [GPBR; 20],
}
#[doc = "GPBR (rw) register accessor: an alias for `Reg<GPBR_SPEC>`"]
pub type GPBR = crate::Reg<gpbr::GPBR_SPEC>;
#[doc = "General Purpose Backup Register"]
pub mod gpbr;
