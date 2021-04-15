#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_cr: [u8; 4usize],
    _reserved_1_mr: [u8; 4usize],
    _reserved_2_ier: [u8; 4usize],
    _reserved_3_idr: [u8; 4usize],
    _reserved_4_imr: [u8; 4usize],
    _reserved_5_csr: [u8; 4usize],
    #[doc = "0x18 - Receiver Holding Register"]
    pub rhr: crate::Reg<rhr::RHR_SPEC>,
    #[doc = "0x1c - Transmitter Holding Register"]
    pub thr: crate::Reg<thr::THR_SPEC>,
    #[doc = "0x20 - Baud Rate Generator Register"]
    pub brgr: crate::Reg<brgr::BRGR_SPEC>,
    #[doc = "0x24 - Receiver Time-out Register"]
    pub rtor: crate::Reg<rtor::RTOR_SPEC>,
    #[doc = "0x28 - Transmitter Timeguard Register"]
    pub ttgr: crate::Reg<ttgr::TTGR_SPEC>,
    _reserved11: [u8; 20usize],
    #[doc = "0x40 - FI DI Ratio Register"]
    pub fidi: crate::Reg<fidi::FIDI_SPEC>,
    #[doc = "0x44 - Number of Errors Register"]
    pub ner: crate::Reg<ner::NER_SPEC>,
    _reserved13: [u8; 4usize],
    #[doc = "0x4c - IrDA Filter Register"]
    pub ifr: crate::Reg<ifr::IFR_SPEC>,
    #[doc = "0x50 - Manchester Configuration Register"]
    pub man: crate::Reg<man::MAN_SPEC>,
    #[doc = "0x54 - LIN Mode Register"]
    pub linmr: crate::Reg<linmr::LINMR_SPEC>,
    #[doc = "0x58 - LIN Identifier Register"]
    pub linir: crate::Reg<linir::LINIR_SPEC>,
    #[doc = "0x5c - LIN Baud Rate Register"]
    pub linbrr: crate::Reg<linbrr::LINBRR_SPEC>,
    _reserved18: [u8; 132usize],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
    _reserved20: [u8; 16usize],
    #[doc = "0xfc - Version Register"]
    pub version: crate::Reg<version::VERSION_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn cr(&self) -> &crate::Reg<cr::CR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize) as *const crate::Reg<cr::CR_SPEC>)
        }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn mr(&self) -> &crate::Reg<mr::MR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize) as *const crate::Reg<mr::MR_SPEC>)
        }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier(&self) -> &crate::Reg<ier::IER_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize) as *const crate::Reg<ier::IER_SPEC>)
        }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr(&self) -> &crate::Reg<idr::IDR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<idr::IDR_SPEC>)
        }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr(&self) -> &crate::Reg<imr::IMR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<imr::IMR_SPEC>)
        }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr(&self) -> &crate::Reg<csr::CSR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<csr::CSR_SPEC>)
        }
    }
}
#[doc = "BRGR register accessor: an alias for `Reg<BRGR_SPEC>`"]
pub type BRGR = crate::Reg<brgr::BRGR_SPEC>;
#[doc = "Baud Rate Generator Register"]
pub mod brgr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Channel Status Register"]
pub mod csr;
#[doc = "FIDI register accessor: an alias for `Reg<FIDI_SPEC>`"]
pub type FIDI = crate::Reg<fidi::FIDI_SPEC>;
#[doc = "FI DI Ratio Register"]
pub mod fidi;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IFR register accessor: an alias for `Reg<IFR_SPEC>`"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "IrDA Filter Register"]
pub mod ifr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "LINBRR register accessor: an alias for `Reg<LINBRR_SPEC>`"]
pub type LINBRR = crate::Reg<linbrr::LINBRR_SPEC>;
#[doc = "LIN Baud Rate Register"]
pub mod linbrr;
#[doc = "LINIR register accessor: an alias for `Reg<LINIR_SPEC>`"]
pub type LINIR = crate::Reg<linir::LINIR_SPEC>;
#[doc = "LIN Identifier Register"]
pub mod linir;
#[doc = "LINMR register accessor: an alias for `Reg<LINMR_SPEC>`"]
pub type LINMR = crate::Reg<linmr::LINMR_SPEC>;
#[doc = "LIN Mode Register"]
pub mod linmr;
#[doc = "MAN register accessor: an alias for `Reg<MAN_SPEC>`"]
pub type MAN = crate::Reg<man::MAN_SPEC>;
#[doc = "Manchester Configuration Register"]
pub mod man;
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "NER register accessor: an alias for `Reg<NER_SPEC>`"]
pub type NER = crate::Reg<ner::NER_SPEC>;
#[doc = "Number of Errors Register"]
pub mod ner;
#[doc = "RHR register accessor: an alias for `Reg<RHR_SPEC>`"]
pub type RHR = crate::Reg<rhr::RHR_SPEC>;
#[doc = "Receiver Holding Register"]
pub mod rhr;
#[doc = "RTOR register accessor: an alias for `Reg<RTOR_SPEC>`"]
pub type RTOR = crate::Reg<rtor::RTOR_SPEC>;
#[doc = "Receiver Time-out Register"]
pub mod rtor;
#[doc = "THR register accessor: an alias for `Reg<THR_SPEC>`"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "Transmitter Holding Register"]
pub mod thr;
#[doc = "TTGR register accessor: an alias for `Reg<TTGR_SPEC>`"]
pub type TTGR = crate::Reg<ttgr::TTGR_SPEC>;
#[doc = "Transmitter Timeguard Register"]
pub mod ttgr;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
