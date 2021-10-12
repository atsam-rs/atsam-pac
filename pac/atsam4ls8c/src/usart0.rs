#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_lin_mode_cr_lin: [u8; 0x04],
    _reserved_1_usart_mode_mr: [u8; 0x04],
    _reserved_2_lin_mode_ier_lin: [u8; 0x04],
    _reserved_3_lin_mode_idr_lin: [u8; 0x04],
    _reserved_4_lin_mode_imr_lin: [u8; 0x04],
    _reserved_5_lin_mode_csr_lin: [u8; 0x04],
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
    _reserved11: [u8; 0x14],
    #[doc = "0x40 - FI DI Ratio Register"]
    pub fidi: crate::Reg<fidi::FIDI_SPEC>,
    #[doc = "0x44 - Number of Errors Register"]
    pub ner: crate::Reg<ner::NER_SPEC>,
    _reserved13: [u8; 0x04],
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
    _reserved18: [u8; 0x84],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
    _reserved20: [u8; 0x10],
    #[doc = "0xfc - Version Register"]
    pub version: crate::Reg<version::VERSION_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn usart_mode_cr_usart(
        &self,
    ) -> &crate::Reg<usart_mode_cr_usart::USART_MODE_CR_USART_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<usart_mode_cr_usart::USART_MODE_CR_USART_SPEC>)
        }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn spi_master_mode_cr_spi(
        &self,
    ) -> &crate::Reg<spi_master_mode_cr_spi::SPI_MASTER_MODE_CR_SPI_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<spi_master_mode_cr_spi::SPI_MASTER_MODE_CR_SPI_SPEC>)
        }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn lin_mode_cr_lin(&self) -> &crate::Reg<lin_mode_cr_lin::LIN_MODE_CR_LIN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<lin_mode_cr_lin::LIN_MODE_CR_LIN_SPEC>)
        }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn usart_mode_mr(&self) -> &crate::Reg<usart_mode_mr::USART_MODE_MR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<usart_mode_mr::USART_MODE_MR_SPEC>)
        }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn spi_mode_mr_spi(&self) -> &crate::Reg<spi_mode_mr_spi::SPI_MODE_MR_SPI_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<spi_mode_mr_spi::SPI_MODE_MR_SPI_SPEC>)
        }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn usart_mode_ier_usart(
        &self,
    ) -> &crate::Reg<usart_mode_ier_usart::USART_MODE_IER_USART_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<usart_mode_ier_usart::USART_MODE_IER_USART_SPEC>)
        }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn spi_slave_mode_ier_spi(
        &self,
    ) -> &crate::Reg<spi_slave_mode_ier_spi::SPI_SLAVE_MODE_IER_SPI_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<spi_slave_mode_ier_spi::SPI_SLAVE_MODE_IER_SPI_SPEC>)
        }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn lin_mode_ier_lin(&self) -> &crate::Reg<lin_mode_ier_lin::LIN_MODE_IER_LIN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<lin_mode_ier_lin::LIN_MODE_IER_LIN_SPEC>)
        }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn usart_mode_idr_usart(
        &self,
    ) -> &crate::Reg<usart_mode_idr_usart::USART_MODE_IDR_USART_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<usart_mode_idr_usart::USART_MODE_IDR_USART_SPEC>)
        }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn spi_slave_mode_idr_spi(
        &self,
    ) -> &crate::Reg<spi_slave_mode_idr_spi::SPI_SLAVE_MODE_IDR_SPI_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<spi_slave_mode_idr_spi::SPI_SLAVE_MODE_IDR_SPI_SPEC>)
        }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn lin_mode_idr_lin(&self) -> &crate::Reg<lin_mode_idr_lin::LIN_MODE_IDR_LIN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<lin_mode_idr_lin::LIN_MODE_IDR_LIN_SPEC>)
        }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn usart_mode_imr_usart(
        &self,
    ) -> &crate::Reg<usart_mode_imr_usart::USART_MODE_IMR_USART_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<usart_mode_imr_usart::USART_MODE_IMR_USART_SPEC>)
        }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn spi_slave_mode_mr_usart(
        &self,
    ) -> &crate::Reg<spi_slave_mode_mr_usart::SPI_SLAVE_MODE_MR_USART_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<spi_slave_mode_mr_usart::SPI_SLAVE_MODE_MR_USART_SPEC>)
        }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn lin_mode_imr_lin(&self) -> &crate::Reg<lin_mode_imr_lin::LIN_MODE_IMR_LIN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<lin_mode_imr_lin::LIN_MODE_IMR_LIN_SPEC>)
        }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn usart_mode_csr_usart(
        &self,
    ) -> &crate::Reg<usart_mode_csr_usart::USART_MODE_CSR_USART_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<usart_mode_csr_usart::USART_MODE_CSR_USART_SPEC>)
        }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn spi_slave_mode_csr_spi(
        &self,
    ) -> &crate::Reg<spi_slave_mode_csr_spi::SPI_SLAVE_MODE_CSR_SPI_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<spi_slave_mode_csr_spi::SPI_SLAVE_MODE_CSR_SPI_SPEC>)
        }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn lin_mode_csr_lin(&self) -> &crate::Reg<lin_mode_csr_lin::LIN_MODE_CSR_LIN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<lin_mode_csr_lin::LIN_MODE_CSR_LIN_SPEC>)
        }
    }
}
#[doc = "BRGR register accessor: an alias for `Reg<BRGR_SPEC>`"]
pub type BRGR = crate::Reg<brgr::BRGR_SPEC>;
#[doc = "Baud Rate Generator Register"]
pub mod brgr;
#[doc = "LIN_MODE_CR_LIN register accessor: an alias for `Reg<LIN_MODE_CR_LIN_SPEC>`"]
pub type LIN_MODE_CR_LIN = crate::Reg<lin_mode_cr_lin::LIN_MODE_CR_LIN_SPEC>;
#[doc = "Control Register"]
pub mod lin_mode_cr_lin;
#[doc = "SPI_MASTER_MODE_CR_SPI register accessor: an alias for `Reg<SPI_MASTER_MODE_CR_SPI_SPEC>`"]
pub type SPI_MASTER_MODE_CR_SPI = crate::Reg<spi_master_mode_cr_spi::SPI_MASTER_MODE_CR_SPI_SPEC>;
#[doc = "Control Register"]
pub mod spi_master_mode_cr_spi;
#[doc = "USART_MODE_CR_USART register accessor: an alias for `Reg<USART_MODE_CR_USART_SPEC>`"]
pub type USART_MODE_CR_USART = crate::Reg<usart_mode_cr_usart::USART_MODE_CR_USART_SPEC>;
#[doc = "Control Register"]
pub mod usart_mode_cr_usart;
#[doc = "LIN_MODE_CSR_LIN register accessor: an alias for `Reg<LIN_MODE_CSR_LIN_SPEC>`"]
pub type LIN_MODE_CSR_LIN = crate::Reg<lin_mode_csr_lin::LIN_MODE_CSR_LIN_SPEC>;
#[doc = "Channel Status Register"]
pub mod lin_mode_csr_lin;
#[doc = "SPI_SLAVE_MODE_CSR_SPI register accessor: an alias for `Reg<SPI_SLAVE_MODE_CSR_SPI_SPEC>`"]
pub type SPI_SLAVE_MODE_CSR_SPI = crate::Reg<spi_slave_mode_csr_spi::SPI_SLAVE_MODE_CSR_SPI_SPEC>;
#[doc = "Channel Status Register"]
pub mod spi_slave_mode_csr_spi;
#[doc = "USART_MODE_CSR_USART register accessor: an alias for `Reg<USART_MODE_CSR_USART_SPEC>`"]
pub type USART_MODE_CSR_USART = crate::Reg<usart_mode_csr_usart::USART_MODE_CSR_USART_SPEC>;
#[doc = "Channel Status Register"]
pub mod usart_mode_csr_usart;
#[doc = "FIDI register accessor: an alias for `Reg<FIDI_SPEC>`"]
pub type FIDI = crate::Reg<fidi::FIDI_SPEC>;
#[doc = "FI DI Ratio Register"]
pub mod fidi;
#[doc = "LIN_MODE_IDR_LIN register accessor: an alias for `Reg<LIN_MODE_IDR_LIN_SPEC>`"]
pub type LIN_MODE_IDR_LIN = crate::Reg<lin_mode_idr_lin::LIN_MODE_IDR_LIN_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod lin_mode_idr_lin;
#[doc = "SPI_SLAVE_MODE_IDR_SPI register accessor: an alias for `Reg<SPI_SLAVE_MODE_IDR_SPI_SPEC>`"]
pub type SPI_SLAVE_MODE_IDR_SPI = crate::Reg<spi_slave_mode_idr_spi::SPI_SLAVE_MODE_IDR_SPI_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod spi_slave_mode_idr_spi;
#[doc = "USART_MODE_IDR_USART register accessor: an alias for `Reg<USART_MODE_IDR_USART_SPEC>`"]
pub type USART_MODE_IDR_USART = crate::Reg<usart_mode_idr_usart::USART_MODE_IDR_USART_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod usart_mode_idr_usart;
#[doc = "LIN_MODE_IER_LIN register accessor: an alias for `Reg<LIN_MODE_IER_LIN_SPEC>`"]
pub type LIN_MODE_IER_LIN = crate::Reg<lin_mode_ier_lin::LIN_MODE_IER_LIN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod lin_mode_ier_lin;
#[doc = "SPI_SLAVE_MODE_IER_SPI register accessor: an alias for `Reg<SPI_SLAVE_MODE_IER_SPI_SPEC>`"]
pub type SPI_SLAVE_MODE_IER_SPI = crate::Reg<spi_slave_mode_ier_spi::SPI_SLAVE_MODE_IER_SPI_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod spi_slave_mode_ier_spi;
#[doc = "USART_MODE_IER_USART register accessor: an alias for `Reg<USART_MODE_IER_USART_SPEC>`"]
pub type USART_MODE_IER_USART = crate::Reg<usart_mode_ier_usart::USART_MODE_IER_USART_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod usart_mode_ier_usart;
#[doc = "IFR register accessor: an alias for `Reg<IFR_SPEC>`"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "IrDA Filter Register"]
pub mod ifr;
#[doc = "LIN_MODE_IMR_LIN register accessor: an alias for `Reg<LIN_MODE_IMR_LIN_SPEC>`"]
pub type LIN_MODE_IMR_LIN = crate::Reg<lin_mode_imr_lin::LIN_MODE_IMR_LIN_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod lin_mode_imr_lin;
#[doc = "SPI_SLAVE_MODE_MR_USART register accessor: an alias for `Reg<SPI_SLAVE_MODE_MR_USART_SPEC>`"]
pub type SPI_SLAVE_MODE_MR_USART =
    crate::Reg<spi_slave_mode_mr_usart::SPI_SLAVE_MODE_MR_USART_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod spi_slave_mode_mr_usart;
#[doc = "USART_MODE_IMR_USART register accessor: an alias for `Reg<USART_MODE_IMR_USART_SPEC>`"]
pub type USART_MODE_IMR_USART = crate::Reg<usart_mode_imr_usart::USART_MODE_IMR_USART_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod usart_mode_imr_usart;
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
#[doc = "SPI_MODE_MR_SPI register accessor: an alias for `Reg<SPI_MODE_MR_SPI_SPEC>`"]
pub type SPI_MODE_MR_SPI = crate::Reg<spi_mode_mr_spi::SPI_MODE_MR_SPI_SPEC>;
#[doc = "Mode Register"]
pub mod spi_mode_mr_spi;
#[doc = "USART_MODE_MR register accessor: an alias for `Reg<USART_MODE_MR_SPEC>`"]
pub type USART_MODE_MR = crate::Reg<usart_mode_mr::USART_MODE_MR_SPEC>;
#[doc = "Mode Register"]
pub mod usart_mode_mr;
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
