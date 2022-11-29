#[doc = r"Register block"]
#[repr(C)]
pub struct SMC_CS_NUMBER {
    #[doc = "0x00 - SMC Setup Register (CS_number = 0)"]
    pub setup: SETUP,
    #[doc = "0x04 - SMC Pulse Register (CS_number = 0)"]
    pub pulse: PULSE,
    #[doc = "0x08 - SMC Cycle Register (CS_number = 0)"]
    pub cycle: CYCLE,
    #[doc = "0x0c - SMC MODE Register (CS_number = 0)"]
    pub mode: MODE,
}
#[doc = "SETUP (rw) register accessor: an alias for `Reg<SETUP_SPEC>`"]
pub type SETUP = crate::Reg<setup::SETUP_SPEC>;
#[doc = "SMC Setup Register (CS_number = 0)"]
pub mod setup;
#[doc = "PULSE (rw) register accessor: an alias for `Reg<PULSE_SPEC>`"]
pub type PULSE = crate::Reg<pulse::PULSE_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 0)"]
pub mod pulse;
#[doc = "CYCLE (rw) register accessor: an alias for `Reg<CYCLE_SPEC>`"]
pub type CYCLE = crate::Reg<cycle::CYCLE_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 0)"]
pub mod cycle;
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "SMC MODE Register (CS_number = 0)"]
pub mod mode;
