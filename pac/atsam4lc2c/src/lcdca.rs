#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x08 - Timing Register"]
    pub tim: TIM,
    #[doc = "0x0c - Status Register"]
    pub sr: SR,
    #[doc = "0x10 - Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x14 - Data Register Low 0"]
    pub drl0: DRL0,
    #[doc = "0x18 - Data Register High 0"]
    pub drh0: DRH0,
    #[doc = "0x1c - Data Register Low 1"]
    pub drl1: DRL1,
    #[doc = "0x20 - Data Register High 1"]
    pub drh1: DRH1,
    #[doc = "0x24 - Data Register Low 2"]
    pub drl2: DRL2,
    #[doc = "0x28 - Data Register High 2"]
    pub drh2: DRH2,
    #[doc = "0x2c - Data Register Low 3"]
    pub drl3: DRL3,
    #[doc = "0x30 - Data Register High 3"]
    pub drh3: DRH3,
    #[doc = "0x34 - Indirect Access Data Register"]
    pub iadr: IADR,
    #[doc = "0x38 - Blink Configuration Register"]
    pub bcfg: BCFG,
    #[doc = "0x3c - Circular Shift Register Configuration"]
    pub csrcfg: CSRCFG,
    #[doc = "0x40 - Character Mapping Configuration Register"]
    pub cmcfg: CMCFG,
    #[doc = "0x44 - Character Mapping Data Register"]
    pub cmdr: CMDR,
    #[doc = "0x48 - Automated Character Mapping Configuration Register"]
    pub acmcfg: ACMCFG,
    #[doc = "0x4c - Automated Character Mapping Data Register"]
    pub acmdr: ACMDR,
    #[doc = "0x50 - Automated Bit Mapping Configuration Register"]
    pub abmcfg: ABMCFG,
    #[doc = "0x54 - Automated Bit Mapping Data Register"]
    pub abmdr: ABMDR,
    #[doc = "0x58 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x5c - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x60 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x64 - Version Register"]
    pub version: VERSION,
}
#[doc = "ABMCFG (rw) register accessor: an alias for `Reg<ABMCFG_SPEC>`"]
pub type ABMCFG = crate::Reg<abmcfg::ABMCFG_SPEC>;
#[doc = "Automated Bit Mapping Configuration Register"]
pub mod abmcfg;
#[doc = "ABMDR (w) register accessor: an alias for `Reg<ABMDR_SPEC>`"]
pub type ABMDR = crate::Reg<abmdr::ABMDR_SPEC>;
#[doc = "Automated Bit Mapping Data Register"]
pub mod abmdr;
#[doc = "ACMCFG (rw) register accessor: an alias for `Reg<ACMCFG_SPEC>`"]
pub type ACMCFG = crate::Reg<acmcfg::ACMCFG_SPEC>;
#[doc = "Automated Character Mapping Configuration Register"]
pub mod acmcfg;
#[doc = "ACMDR (w) register accessor: an alias for `Reg<ACMDR_SPEC>`"]
pub type ACMDR = crate::Reg<acmdr::ACMDR_SPEC>;
#[doc = "Automated Character Mapping Data Register"]
pub mod acmdr;
#[doc = "BCFG (rw) register accessor: an alias for `Reg<BCFG_SPEC>`"]
pub type BCFG = crate::Reg<bcfg::BCFG_SPEC>;
#[doc = "Blink Configuration Register"]
pub mod bcfg;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "CMCFG (rw) register accessor: an alias for `Reg<CMCFG_SPEC>`"]
pub type CMCFG = crate::Reg<cmcfg::CMCFG_SPEC>;
#[doc = "Character Mapping Configuration Register"]
pub mod cmcfg;
#[doc = "CMDR (w) register accessor: an alias for `Reg<CMDR_SPEC>`"]
pub type CMDR = crate::Reg<cmdr::CMDR_SPEC>;
#[doc = "Character Mapping Data Register"]
pub mod cmdr;
#[doc = "CR (w) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "CSRCFG (rw) register accessor: an alias for `Reg<CSRCFG_SPEC>`"]
pub type CSRCFG = crate::Reg<csrcfg::CSRCFG_SPEC>;
#[doc = "Circular Shift Register Configuration"]
pub mod csrcfg;
#[doc = "DRH0 (rw) register accessor: an alias for `Reg<DRH0_SPEC>`"]
pub type DRH0 = crate::Reg<drh0::DRH0_SPEC>;
#[doc = "Data Register High 0"]
pub mod drh0;
#[doc = "DRH1 (rw) register accessor: an alias for `Reg<DRH1_SPEC>`"]
pub type DRH1 = crate::Reg<drh1::DRH1_SPEC>;
#[doc = "Data Register High 1"]
pub mod drh1;
#[doc = "DRH2 (rw) register accessor: an alias for `Reg<DRH2_SPEC>`"]
pub type DRH2 = crate::Reg<drh2::DRH2_SPEC>;
#[doc = "Data Register High 2"]
pub mod drh2;
#[doc = "DRH3 (rw) register accessor: an alias for `Reg<DRH3_SPEC>`"]
pub type DRH3 = crate::Reg<drh3::DRH3_SPEC>;
#[doc = "Data Register High 3"]
pub mod drh3;
#[doc = "DRL0 (rw) register accessor: an alias for `Reg<DRL0_SPEC>`"]
pub type DRL0 = crate::Reg<drl0::DRL0_SPEC>;
#[doc = "Data Register Low 0"]
pub mod drl0;
#[doc = "DRL1 (rw) register accessor: an alias for `Reg<DRL1_SPEC>`"]
pub type DRL1 = crate::Reg<drl1::DRL1_SPEC>;
#[doc = "Data Register Low 1"]
pub mod drl1;
#[doc = "DRL2 (rw) register accessor: an alias for `Reg<DRL2_SPEC>`"]
pub type DRL2 = crate::Reg<drl2::DRL2_SPEC>;
#[doc = "Data Register Low 2"]
pub mod drl2;
#[doc = "DRL3 (rw) register accessor: an alias for `Reg<DRL3_SPEC>`"]
pub type DRL3 = crate::Reg<drl3::DRL3_SPEC>;
#[doc = "Data Register Low 3"]
pub mod drl3;
#[doc = "IADR (w) register accessor: an alias for `Reg<IADR_SPEC>`"]
pub type IADR = crate::Reg<iadr::IADR_SPEC>;
#[doc = "Indirect Access Data Register"]
pub mod iadr;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Status Clear Register"]
pub mod scr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "TIM (rw) register accessor: an alias for `Reg<TIM_SPEC>`"]
pub type TIM = crate::Reg<tim::TIM_SPEC>;
#[doc = "Timing Register"]
pub mod tim;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
