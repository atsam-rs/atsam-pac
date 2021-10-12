#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Clock Enable Register"]
    pub pmc_scer: crate::Reg<pmc_scer::PMC_SCER_SPEC>,
    #[doc = "0x04 - System Clock Disable Register"]
    pub pmc_scdr: crate::Reg<pmc_scdr::PMC_SCDR_SPEC>,
    #[doc = "0x08 - System Clock Status Register"]
    pub pmc_scsr: crate::Reg<pmc_scsr::PMC_SCSR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Peripheral Clock Enable Register 0"]
    pub pmc_pcer0: crate::Reg<pmc_pcer0::PMC_PCER0_SPEC>,
    #[doc = "0x14 - Peripheral Clock Disable Register 0"]
    pub pmc_pcdr0: crate::Reg<pmc_pcdr0::PMC_PCDR0_SPEC>,
    #[doc = "0x18 - Peripheral Clock Status Register 0"]
    pub pmc_pcsr0: crate::Reg<pmc_pcsr0::PMC_PCSR0_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Main Oscillator Register"]
    pub ckgr_mor: crate::Reg<ckgr_mor::CKGR_MOR_SPEC>,
    #[doc = "0x24 - Main Clock Frequency Register"]
    pub ckgr_mcfr: crate::Reg<ckgr_mcfr::CKGR_MCFR_SPEC>,
    #[doc = "0x28 - PLLA Register"]
    pub ckgr_pllar: crate::Reg<ckgr_pllar::CKGR_PLLAR_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x30 - Master Clock Register"]
    pub pmc_mckr: crate::Reg<pmc_mckr::PMC_MCKR_SPEC>,
    _reserved10: [u8; 0x0c],
    #[doc = "0x40..0x4c - Programmable Clock 0 Register"]
    pub pmc_pck: [crate::Reg<pmc_pck::PMC_PCK_SPEC>; 3],
    _reserved11: [u8; 0x14],
    #[doc = "0x60 - Interrupt Enable Register"]
    pub pmc_ier: crate::Reg<pmc_ier::PMC_IER_SPEC>,
    #[doc = "0x64 - Interrupt Disable Register"]
    pub pmc_idr: crate::Reg<pmc_idr::PMC_IDR_SPEC>,
    #[doc = "0x68 - Status Register"]
    pub pmc_sr: crate::Reg<pmc_sr::PMC_SR_SPEC>,
    #[doc = "0x6c - Interrupt Mask Register"]
    pub pmc_imr: crate::Reg<pmc_imr::PMC_IMR_SPEC>,
    #[doc = "0x70 - Fast Start-up Mode Register"]
    pub pmc_fsmr: crate::Reg<pmc_fsmr::PMC_FSMR_SPEC>,
    #[doc = "0x74 - Fast Start-up Polarity Register"]
    pub pmc_fspr: crate::Reg<pmc_fspr::PMC_FSPR_SPEC>,
    #[doc = "0x78 - Fault Output Clear Register"]
    pub pmc_focr: crate::Reg<pmc_focr::PMC_FOCR_SPEC>,
    _reserved18: [u8; 0x68],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub pmc_wpmr: crate::Reg<pmc_wpmr::PMC_WPMR_SPEC>,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub pmc_wpsr: crate::Reg<pmc_wpsr::PMC_WPSR_SPEC>,
    _reserved20: [u8; 0x24],
    #[doc = "0x110 - Oscillator Calibration Register"]
    pub pmc_ocr: crate::Reg<pmc_ocr::PMC_OCR_SPEC>,
    _reserved21: [u8; 0x1c],
    #[doc = "0x130 - PLL Maximum Multiplier Value Register"]
    pub pmc_pmmr: crate::Reg<pmc_pmmr::PMC_PMMR_SPEC>,
}
#[doc = "PMC_SCER register accessor: an alias for `Reg<PMC_SCER_SPEC>`"]
pub type PMC_SCER = crate::Reg<pmc_scer::PMC_SCER_SPEC>;
#[doc = "System Clock Enable Register"]
pub mod pmc_scer;
#[doc = "PMC_SCDR register accessor: an alias for `Reg<PMC_SCDR_SPEC>`"]
pub type PMC_SCDR = crate::Reg<pmc_scdr::PMC_SCDR_SPEC>;
#[doc = "System Clock Disable Register"]
pub mod pmc_scdr;
#[doc = "PMC_SCSR register accessor: an alias for `Reg<PMC_SCSR_SPEC>`"]
pub type PMC_SCSR = crate::Reg<pmc_scsr::PMC_SCSR_SPEC>;
#[doc = "System Clock Status Register"]
pub mod pmc_scsr;
#[doc = "PMC_PCER0 register accessor: an alias for `Reg<PMC_PCER0_SPEC>`"]
pub type PMC_PCER0 = crate::Reg<pmc_pcer0::PMC_PCER0_SPEC>;
#[doc = "Peripheral Clock Enable Register 0"]
pub mod pmc_pcer0;
#[doc = "PMC_PCDR0 register accessor: an alias for `Reg<PMC_PCDR0_SPEC>`"]
pub type PMC_PCDR0 = crate::Reg<pmc_pcdr0::PMC_PCDR0_SPEC>;
#[doc = "Peripheral Clock Disable Register 0"]
pub mod pmc_pcdr0;
#[doc = "PMC_PCSR0 register accessor: an alias for `Reg<PMC_PCSR0_SPEC>`"]
pub type PMC_PCSR0 = crate::Reg<pmc_pcsr0::PMC_PCSR0_SPEC>;
#[doc = "Peripheral Clock Status Register 0"]
pub mod pmc_pcsr0;
#[doc = "CKGR_MOR register accessor: an alias for `Reg<CKGR_MOR_SPEC>`"]
pub type CKGR_MOR = crate::Reg<ckgr_mor::CKGR_MOR_SPEC>;
#[doc = "Main Oscillator Register"]
pub mod ckgr_mor;
#[doc = "CKGR_MCFR register accessor: an alias for `Reg<CKGR_MCFR_SPEC>`"]
pub type CKGR_MCFR = crate::Reg<ckgr_mcfr::CKGR_MCFR_SPEC>;
#[doc = "Main Clock Frequency Register"]
pub mod ckgr_mcfr;
#[doc = "CKGR_PLLAR register accessor: an alias for `Reg<CKGR_PLLAR_SPEC>`"]
pub type CKGR_PLLAR = crate::Reg<ckgr_pllar::CKGR_PLLAR_SPEC>;
#[doc = "PLLA Register"]
pub mod ckgr_pllar;
#[doc = "PMC_MCKR register accessor: an alias for `Reg<PMC_MCKR_SPEC>`"]
pub type PMC_MCKR = crate::Reg<pmc_mckr::PMC_MCKR_SPEC>;
#[doc = "Master Clock Register"]
pub mod pmc_mckr;
#[doc = "PMC_PCK register accessor: an alias for `Reg<PMC_PCK_SPEC>`"]
pub type PMC_PCK = crate::Reg<pmc_pck::PMC_PCK_SPEC>;
#[doc = "Programmable Clock 0 Register"]
pub mod pmc_pck;
#[doc = "PMC_IER register accessor: an alias for `Reg<PMC_IER_SPEC>`"]
pub type PMC_IER = crate::Reg<pmc_ier::PMC_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod pmc_ier;
#[doc = "PMC_IDR register accessor: an alias for `Reg<PMC_IDR_SPEC>`"]
pub type PMC_IDR = crate::Reg<pmc_idr::PMC_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod pmc_idr;
#[doc = "PMC_SR register accessor: an alias for `Reg<PMC_SR_SPEC>`"]
pub type PMC_SR = crate::Reg<pmc_sr::PMC_SR_SPEC>;
#[doc = "Status Register"]
pub mod pmc_sr;
#[doc = "PMC_IMR register accessor: an alias for `Reg<PMC_IMR_SPEC>`"]
pub type PMC_IMR = crate::Reg<pmc_imr::PMC_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod pmc_imr;
#[doc = "PMC_FSMR register accessor: an alias for `Reg<PMC_FSMR_SPEC>`"]
pub type PMC_FSMR = crate::Reg<pmc_fsmr::PMC_FSMR_SPEC>;
#[doc = "Fast Start-up Mode Register"]
pub mod pmc_fsmr;
#[doc = "PMC_FSPR register accessor: an alias for `Reg<PMC_FSPR_SPEC>`"]
pub type PMC_FSPR = crate::Reg<pmc_fspr::PMC_FSPR_SPEC>;
#[doc = "Fast Start-up Polarity Register"]
pub mod pmc_fspr;
#[doc = "PMC_FOCR register accessor: an alias for `Reg<PMC_FOCR_SPEC>`"]
pub type PMC_FOCR = crate::Reg<pmc_focr::PMC_FOCR_SPEC>;
#[doc = "Fault Output Clear Register"]
pub mod pmc_focr;
#[doc = "PMC_WPMR register accessor: an alias for `Reg<PMC_WPMR_SPEC>`"]
pub type PMC_WPMR = crate::Reg<pmc_wpmr::PMC_WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod pmc_wpmr;
#[doc = "PMC_WPSR register accessor: an alias for `Reg<PMC_WPSR_SPEC>`"]
pub type PMC_WPSR = crate::Reg<pmc_wpsr::PMC_WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod pmc_wpsr;
#[doc = "PMC_OCR register accessor: an alias for `Reg<PMC_OCR_SPEC>`"]
pub type PMC_OCR = crate::Reg<pmc_ocr::PMC_OCR_SPEC>;
#[doc = "Oscillator Calibration Register"]
pub mod pmc_ocr;
#[doc = "PMC_PMMR register accessor: an alias for `Reg<PMC_PMMR_SPEC>`"]
pub type PMC_PMMR = crate::Reg<pmc_pmmr::PMC_PMMR_SPEC>;
#[doc = "PLL Maximum Multiplier Value Register"]
pub mod pmc_pmmr;
