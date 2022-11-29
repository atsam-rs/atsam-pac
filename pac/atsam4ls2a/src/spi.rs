#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Receive Data Register"]
    pub rdr: RDR,
    #[doc = "0x0c - Transmit Data Register"]
    pub tdr: TDR,
    #[doc = "0x10 - Status Register"]
    pub sr: SR,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x18 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub imr: IMR,
    _reserved8: [u8; 0x10],
    #[doc = "0x30..0x40 - Chip Select Register"]
    pub csr: [CSR; 4],
    _reserved9: [u8; 0xa4],
    #[doc = "0xe4 - Write Protection control Register"]
    pub wpcr: WPCR,
    #[doc = "0xe8 - Write Protection status Register"]
    pub wpsr: WPSR,
    _reserved11: [u8; 0x0c],
    #[doc = "0xf8 - Features Register"]
    pub features: FEATURES,
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
#[doc = "CR (w) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Chip Select Register"]
pub mod csr;
#[doc = "FEATURES (r) register accessor: an alias for `Reg<FEATURES_SPEC>`"]
pub type FEATURES = crate::Reg<features::FEATURES_SPEC>;
#[doc = "Features Register"]
pub mod features;
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
#[doc = "MR (rw) register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "RDR (r) register accessor: an alias for `Reg<RDR_SPEC>`"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "Receive Data Register"]
pub mod rdr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "TDR (w) register accessor: an alias for `Reg<TDR_SPEC>`"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod tdr;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
#[doc = "WPCR (rw) register accessor: an alias for `Reg<WPCR_SPEC>`"]
pub type WPCR = crate::Reg<wpcr::WPCR_SPEC>;
#[doc = "Write Protection control Register"]
pub mod wpcr;
#[doc = "WPSR (r) register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection status Register"]
pub mod wpsr;
