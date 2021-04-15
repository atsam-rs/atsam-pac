#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_cr: [u8; 4usize],
    _reserved_1_mr: [u8; 4usize],
    _reserved_2_ier: [u8; 4usize],
    _reserved_3_idr: [u8; 4usize],
    _reserved_4_imr_lin: [u8; 4usize],
    _reserved_5_csr: [u8; 4usize],
    #[doc = "0x18 - Receiver Holding Register"]
    pub rhr: RHR,
    #[doc = "0x1c - Transmitter Holding Register"]
    pub thr: THR,
    #[doc = "0x20 - Baud Rate Generator Register"]
    pub brgr: BRGR,
    #[doc = "0x24 - Receiver Time-out Register"]
    pub rtor: RTOR,
    #[doc = "0x28 - Transmitter Timeguard Register"]
    pub ttgr: TTGR,
    _reserved11: [u8; 20usize],
    #[doc = "0x40 - FI DI Ratio Register"]
    pub fidi: FIDI,
    #[doc = "0x44 - Number of Errors Register"]
    pub ner: NER,
    _reserved13: [u8; 4usize],
    #[doc = "0x4c - IrDA Filter Register"]
    pub ifr: IFR,
    #[doc = "0x50 - Manchester Configuration Register"]
    pub man: MAN,
    #[doc = "0x54 - LIN Mode Register"]
    pub linmr: LINMR,
    #[doc = "0x58 - LIN Identifier Register"]
    pub linir: LINIR,
    #[doc = "0x5c - LIN Baud Rate Register"]
    pub linbrr: LINBRR,
    _reserved18: [u8; 132usize],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: WPSR,
    _reserved20: [u8; 16usize],
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn cr_usart(&self) -> &CR_USART {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const CR_USART) }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn cr_usart_mut(&self) -> &mut CR_USART {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut CR_USART) }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn cr_spi(&self) -> &CR_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const CR_SPI) }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn cr_spi_mut(&self) -> &mut CR_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut CR_SPI) }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn cr_lin(&self) -> &CR_LIN {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const CR_LIN) }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn cr_lin_mut(&self) -> &mut CR_LIN {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut CR_LIN) }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn mr(&self) -> &MR {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const MR) }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn mr_mut(&self) -> &mut MR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut MR) }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn mr_spi(&self) -> &MR_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const MR_SPI) }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn mr_spi_mut(&self) -> &mut MR_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut MR_SPI) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_usart(&self) -> &IER_USART {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const IER_USART) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_usart_mut(&self) -> &mut IER_USART {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut IER_USART) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_spi(&self) -> &IER_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const IER_SPI) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_spi_mut(&self) -> &mut IER_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut IER_SPI) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_lin(&self) -> &IER_LIN {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const IER_LIN) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_lin_mut(&self) -> &mut IER_LIN {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut IER_LIN) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_usart(&self) -> &IDR_USART {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const IDR_USART) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_usart_mut(&self) -> &mut IDR_USART {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut IDR_USART) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_spi(&self) -> &IDR_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const IDR_SPI) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_spi_mut(&self) -> &mut IDR_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut IDR_SPI) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_lin(&self) -> &IDR_LIN {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const IDR_LIN) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_lin_mut(&self) -> &mut IDR_LIN {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut IDR_LIN) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr_usart(&self) -> &IMR_USART {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const IMR_USART) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr_usart_mut(&self) -> &mut IMR_USART {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut IMR_USART) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn mr_usart(&self) -> &MR_USART {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const MR_USART) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn mr_usart_mut(&self) -> &mut MR_USART {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut MR_USART) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr_lin(&self) -> &IMR_LIN {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const IMR_LIN) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr_lin_mut(&self) -> &mut IMR_LIN {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut IMR_LIN) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_usart(&self) -> &CSR_USART {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const CSR_USART) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_usart_mut(&self) -> &mut CSR_USART {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut CSR_USART) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_spi(&self) -> &CSR_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const CSR_SPI) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_spi_mut(&self) -> &mut CSR_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut CSR_SPI) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_lin(&self) -> &CSR_LIN {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const CSR_LIN) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_lin_mut(&self) -> &mut CSR_LIN {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut CSR_LIN) }
    }
}
#[doc = "Baud Rate Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brgr](brgr) module"]
pub type BRGR = crate::Reg<u32, _BRGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRGR;
#[doc = "`read()` method returns [brgr::R](brgr::R) reader structure"]
impl crate::Readable for BRGR {}
#[doc = "`write(|w| ..)` method takes [brgr::W](brgr::W) writer structure"]
impl crate::Writable for BRGR {}
#[doc = "Baud Rate Generator Register"]
pub mod brgr;
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr_lin](cr_lin) module"]
pub type CR_LIN = crate::Reg<u32, _CR_LIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR_LIN;
#[doc = "`write(|w| ..)` method takes [cr_lin::W](cr_lin::W) writer structure"]
impl crate::Writable for CR_LIN {}
#[doc = "Control Register"]
pub mod cr_lin;
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr_spi](cr_spi) module"]
pub type CR_SPI = crate::Reg<u32, _CR_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR_SPI;
#[doc = "`write(|w| ..)` method takes [cr_spi::W](cr_spi::W) writer structure"]
impl crate::Writable for CR_SPI {}
#[doc = "Control Register"]
pub mod cr_spi;
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr_usart](cr_usart) module"]
pub type CR_USART = crate::Reg<u32, _CR_USART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR_USART;
#[doc = "`write(|w| ..)` method takes [cr_usart::W](cr_usart::W) writer structure"]
impl crate::Writable for CR_USART {}
#[doc = "Control Register"]
pub mod cr_usart;
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr_lin](csr_lin) module"]
pub type CSR_LIN = crate::Reg<u32, _CSR_LIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR_LIN;
#[doc = "`read()` method returns [csr_lin::R](csr_lin::R) reader structure"]
impl crate::Readable for CSR_LIN {}
#[doc = "Channel Status Register"]
pub mod csr_lin;
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr_spi](csr_spi) module"]
pub type CSR_SPI = crate::Reg<u32, _CSR_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR_SPI;
#[doc = "`read()` method returns [csr_spi::R](csr_spi::R) reader structure"]
impl crate::Readable for CSR_SPI {}
#[doc = "Channel Status Register"]
pub mod csr_spi;
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr_usart](csr_usart) module"]
pub type CSR_USART = crate::Reg<u32, _CSR_USART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR_USART;
#[doc = "`read()` method returns [csr_usart::R](csr_usart::R) reader structure"]
impl crate::Readable for CSR_USART {}
#[doc = "Channel Status Register"]
pub mod csr_usart;
#[doc = "FI DI Ratio Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fidi](fidi) module"]
pub type FIDI = crate::Reg<u32, _FIDI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIDI;
#[doc = "`read()` method returns [fidi::R](fidi::R) reader structure"]
impl crate::Readable for FIDI {}
#[doc = "`write(|w| ..)` method takes [fidi::W](fidi::W) writer structure"]
impl crate::Writable for FIDI {}
#[doc = "FI DI Ratio Register"]
pub mod fidi;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr_lin](idr_lin) module"]
pub type IDR_LIN = crate::Reg<u32, _IDR_LIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR_LIN;
#[doc = "`write(|w| ..)` method takes [idr_lin::W](idr_lin::W) writer structure"]
impl crate::Writable for IDR_LIN {}
#[doc = "Interrupt Disable Register"]
pub mod idr_lin;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr_spi](idr_spi) module"]
pub type IDR_SPI = crate::Reg<u32, _IDR_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR_SPI;
#[doc = "`write(|w| ..)` method takes [idr_spi::W](idr_spi::W) writer structure"]
impl crate::Writable for IDR_SPI {}
#[doc = "Interrupt Disable Register"]
pub mod idr_spi;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr_usart](idr_usart) module"]
pub type IDR_USART = crate::Reg<u32, _IDR_USART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR_USART;
#[doc = "`write(|w| ..)` method takes [idr_usart::W](idr_usart::W) writer structure"]
impl crate::Writable for IDR_USART {}
#[doc = "Interrupt Disable Register"]
pub mod idr_usart;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier_lin](ier_lin) module"]
pub type IER_LIN = crate::Reg<u32, _IER_LIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER_LIN;
#[doc = "`write(|w| ..)` method takes [ier_lin::W](ier_lin::W) writer structure"]
impl crate::Writable for IER_LIN {}
#[doc = "Interrupt Enable Register"]
pub mod ier_lin;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier_spi](ier_spi) module"]
pub type IER_SPI = crate::Reg<u32, _IER_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER_SPI;
#[doc = "`write(|w| ..)` method takes [ier_spi::W](ier_spi::W) writer structure"]
impl crate::Writable for IER_SPI {}
#[doc = "Interrupt Enable Register"]
pub mod ier_spi;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier_usart](ier_usart) module"]
pub type IER_USART = crate::Reg<u32, _IER_USART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER_USART;
#[doc = "`write(|w| ..)` method takes [ier_usart::W](ier_usart::W) writer structure"]
impl crate::Writable for IER_USART {}
#[doc = "Interrupt Enable Register"]
pub mod ier_usart;
#[doc = "IrDA Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifr](ifr) module"]
pub type IFR = crate::Reg<u32, _IFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFR;
#[doc = "`read()` method returns [ifr::R](ifr::R) reader structure"]
impl crate::Readable for IFR {}
#[doc = "`write(|w| ..)` method takes [ifr::W](ifr::W) writer structure"]
impl crate::Writable for IFR {}
#[doc = "IrDA Filter Register"]
pub mod ifr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr_lin](imr_lin) module"]
pub type IMR_LIN = crate::Reg<u32, _IMR_LIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR_LIN;
#[doc = "`read()` method returns [imr_lin::R](imr_lin::R) reader structure"]
impl crate::Readable for IMR_LIN {}
#[doc = "Interrupt Mask Register"]
pub mod imr_lin;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr_usart](mr_usart) module"]
pub type MR_USART = crate::Reg<u32, _MR_USART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR_USART;
#[doc = "`read()` method returns [mr_usart::R](mr_usart::R) reader structure"]
impl crate::Readable for MR_USART {}
#[doc = "Interrupt Mask Register"]
pub mod mr_usart;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr_usart](imr_usart) module"]
pub type IMR_USART = crate::Reg<u32, _IMR_USART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR_USART;
#[doc = "`read()` method returns [imr_usart::R](imr_usart::R) reader structure"]
impl crate::Readable for IMR_USART {}
#[doc = "Interrupt Mask Register"]
pub mod imr_usart;
#[doc = "LIN Baud Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linbrr](linbrr) module"]
pub type LINBRR = crate::Reg<u32, _LINBRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LINBRR;
#[doc = "`read()` method returns [linbrr::R](linbrr::R) reader structure"]
impl crate::Readable for LINBRR {}
#[doc = "LIN Baud Rate Register"]
pub mod linbrr;
#[doc = "LIN Identifier Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linir](linir) module"]
pub type LINIR = crate::Reg<u32, _LINIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LINIR;
#[doc = "`read()` method returns [linir::R](linir::R) reader structure"]
impl crate::Readable for LINIR {}
#[doc = "`write(|w| ..)` method takes [linir::W](linir::W) writer structure"]
impl crate::Writable for LINIR {}
#[doc = "LIN Identifier Register"]
pub mod linir;
#[doc = "LIN Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linmr](linmr) module"]
pub type LINMR = crate::Reg<u32, _LINMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LINMR;
#[doc = "`read()` method returns [linmr::R](linmr::R) reader structure"]
impl crate::Readable for LINMR {}
#[doc = "`write(|w| ..)` method takes [linmr::W](linmr::W) writer structure"]
impl crate::Writable for LINMR {}
#[doc = "LIN Mode Register"]
pub mod linmr;
#[doc = "Manchester Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [man](man) module"]
pub type MAN = crate::Reg<u32, _MAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAN;
#[doc = "`read()` method returns [man::R](man::R) reader structure"]
impl crate::Readable for MAN {}
#[doc = "`write(|w| ..)` method takes [man::W](man::W) writer structure"]
impl crate::Writable for MAN {}
#[doc = "Manchester Configuration Register"]
pub mod man;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr_spi](mr_spi) module"]
pub type MR_SPI = crate::Reg<u32, _MR_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR_SPI;
#[doc = "`read()` method returns [mr_spi::R](mr_spi::R) reader structure"]
impl crate::Readable for MR_SPI {}
#[doc = "`write(|w| ..)` method takes [mr_spi::W](mr_spi::W) writer structure"]
impl crate::Writable for MR_SPI {}
#[doc = "Mode Register"]
pub mod mr_spi;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](mr) module"]
pub type MR = crate::Reg<u32, _MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR;
#[doc = "`read()` method returns [mr::R](mr::R) reader structure"]
impl crate::Readable for MR {}
#[doc = "`write(|w| ..)` method takes [mr::W](mr::W) writer structure"]
impl crate::Writable for MR {}
#[doc = "Mode Register"]
pub mod mr;
#[doc = "Number of Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ner](ner) module"]
pub type NER = crate::Reg<u32, _NER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NER;
#[doc = "`read()` method returns [ner::R](ner::R) reader structure"]
impl crate::Readable for NER {}
#[doc = "Number of Errors Register"]
pub mod ner;
#[doc = "Receiver Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rhr](rhr) module"]
pub type RHR = crate::Reg<u32, _RHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RHR;
#[doc = "`read()` method returns [rhr::R](rhr::R) reader structure"]
impl crate::Readable for RHR {}
#[doc = "Receiver Holding Register"]
pub mod rhr;
#[doc = "Receiver Time-out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtor](rtor) module"]
pub type RTOR = crate::Reg<u32, _RTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTOR;
#[doc = "`read()` method returns [rtor::R](rtor::R) reader structure"]
impl crate::Readable for RTOR {}
#[doc = "`write(|w| ..)` method takes [rtor::W](rtor::W) writer structure"]
impl crate::Writable for RTOR {}
#[doc = "Receiver Time-out Register"]
pub mod rtor;
#[doc = "Transmitter Holding Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr](thr) module"]
pub type THR = crate::Reg<u32, _THR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THR;
#[doc = "`write(|w| ..)` method takes [thr::W](thr::W) writer structure"]
impl crate::Writable for THR {}
#[doc = "Transmitter Holding Register"]
pub mod thr;
#[doc = "Transmitter Timeguard Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttgr](ttgr) module"]
pub type TTGR = crate::Reg<u32, _TTGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTGR;
#[doc = "`read()` method returns [ttgr::R](ttgr::R) reader structure"]
impl crate::Readable for TTGR {}
#[doc = "`write(|w| ..)` method takes [ttgr::W](ttgr::W) writer structure"]
impl crate::Writable for TTGR {}
#[doc = "Transmitter Timeguard Register"]
pub mod ttgr;
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "Version Register"]
pub mod version;
#[doc = "Write Protect Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpmr](wpmr) module"]
pub type WPMR = crate::Reg<u32, _WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPMR;
#[doc = "`read()` method returns [wpmr::R](wpmr::R) reader structure"]
impl crate::Readable for WPMR {}
#[doc = "`write(|w| ..)` method takes [wpmr::W](wpmr::W) writer structure"]
impl crate::Writable for WPMR {}
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "Write Protect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](wpsr) module"]
pub type WPSR = crate::Reg<u32, _WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSR;
#[doc = "`read()` method returns [wpsr::R](wpsr::R) reader structure"]
impl crate::Readable for WPSR {}
#[doc = "Write Protect Status Register"]
pub mod wpsr;
