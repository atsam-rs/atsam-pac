#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Customer Register"]
    pub cust: CUST,
    #[doc = "0x0c - Fast Bit Timing and Prescaler Register"]
    pub fbtp: FBTP,
    #[doc = "0x10 - Test Register"]
    pub test: TEST,
    #[doc = "0x14 - RAM Watchdog Register"]
    pub rwd: RWD,
    #[doc = "0x18 - CC Control Register"]
    pub cccr: CCCR,
    #[doc = "0x1c - Bit Timing and Prescaler Register"]
    pub btp: BTP,
    #[doc = "0x20 - Timestamp Counter Configuration Register"]
    pub tscc: TSCC,
    #[doc = "0x24 - Timestamp Counter Value Register"]
    pub tscv: TSCV,
    #[doc = "0x28 - Timeout Counter Configuration Register"]
    pub tocc: TOCC,
    #[doc = "0x2c - Timeout Counter Value Register"]
    pub tocv: TOCV,
    _reserved10: [u8; 16usize],
    #[doc = "0x40 - Error Counter Register"]
    pub ecr: ECR,
    #[doc = "0x44 - Protocol Status Register"]
    pub psr: PSR,
    _reserved12: [u8; 8usize],
    #[doc = "0x50 - Interrupt Register"]
    pub ir: IR,
    #[doc = "0x54 - Interrupt Enable Register"]
    pub ie: IE,
    #[doc = "0x58 - Interrupt Line Select Register"]
    pub ils: ILS,
    #[doc = "0x5c - Interrupt Line Enable Register"]
    pub ile: ILE,
    _reserved16: [u8; 32usize],
    #[doc = "0x80 - Global Filter Configuration Register"]
    pub gfc: GFC,
    #[doc = "0x84 - Standard ID Filter Configuration Register"]
    pub sidfc: SIDFC,
    #[doc = "0x88 - Extended ID Filter Configuration Register"]
    pub xidfc: XIDFC,
    _reserved19: [u8; 4usize],
    #[doc = "0x90 - Extended ID AND Mask Register"]
    pub xidam: XIDAM,
    #[doc = "0x94 - High Priority Message Status Register"]
    pub hpms: HPMS,
    #[doc = "0x98 - New Data 1 Register"]
    pub ndat1: NDAT1,
    #[doc = "0x9c - New Data 2 Register"]
    pub ndat2: NDAT2,
    #[doc = "0xa0 - Receive FIFO 0 Configuration Register"]
    pub rxf0c: RXF0C,
    #[doc = "0xa4 - Receive FIFO 0 Status Register"]
    pub rxf0s: RXF0S,
    #[doc = "0xa8 - Receive FIFO 0 Acknowledge Register"]
    pub rxf0a: RXF0A,
    #[doc = "0xac - Receive Rx Buffer Configuration Register"]
    pub rxbc: RXBC,
    #[doc = "0xb0 - Receive FIFO 1 Configuration Register"]
    pub rxf1c: RXF1C,
    #[doc = "0xb4 - Receive FIFO 1 Status Register"]
    pub rxf1s: RXF1S,
    #[doc = "0xb8 - Receive FIFO 1 Acknowledge Register"]
    pub rxf1a: RXF1A,
    #[doc = "0xbc - Receive Buffer / FIFO Element Size Configuration Register"]
    pub rxesc: RXESC,
    #[doc = "0xc0 - Transmit Buffer Configuration Register"]
    pub txbc: TXBC,
    #[doc = "0xc4 - Transmit FIFO/Queue Status Register"]
    pub txfqs: TXFQS,
    #[doc = "0xc8 - Transmit Buffer Element Size Configuration Register"]
    pub txesc: TXESC,
    #[doc = "0xcc - Transmit Buffer Request Pending Register"]
    pub txbrp: TXBRP,
    #[doc = "0xd0 - Transmit Buffer Add Request Register"]
    pub txbar: TXBAR,
    #[doc = "0xd4 - Transmit Buffer Cancellation Request Register"]
    pub txbcr: TXBCR,
    #[doc = "0xd8 - Transmit Buffer Transmission Occurred Register"]
    pub txbto: TXBTO,
    #[doc = "0xdc - Transmit Buffer Cancellation Finished Register"]
    pub txbcf: TXBCF,
    #[doc = "0xe0 - Transmit Buffer Transmission Interrupt Enable Register"]
    pub txbtie: TXBTIE,
    #[doc = "0xe4 - Transmit Buffer Cancellation Finished Interrupt Enable Register"]
    pub txbcie: TXBCIE,
    _reserved41: [u8; 8usize],
    #[doc = "0xf0 - Transmit Event FIFO Configuration Register"]
    pub txefc: TXEFC,
    #[doc = "0xf4 - Transmit Event FIFO Status Register"]
    pub txefs: TXEFS,
    #[doc = "0xf8 - Transmit Event FIFO Acknowledge Register"]
    pub txefa: TXEFA,
}
#[doc = "Customer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cust](cust) module"]
pub type CUST = crate::Reg<u32, _CUST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUST;
#[doc = "`read()` method returns [cust::R](cust::R) reader structure"]
impl crate::Readable for CUST {}
#[doc = "`write(|w| ..)` method takes [cust::W](cust::W) writer structure"]
impl crate::Writable for CUST {}
#[doc = "Customer Register"]
pub mod cust;
#[doc = "Fast Bit Timing and Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbtp](fbtp) module"]
pub type FBTP = crate::Reg<u32, _FBTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FBTP;
#[doc = "`read()` method returns [fbtp::R](fbtp::R) reader structure"]
impl crate::Readable for FBTP {}
#[doc = "`write(|w| ..)` method takes [fbtp::W](fbtp::W) writer structure"]
impl crate::Writable for FBTP {}
#[doc = "Fast Bit Timing and Prescaler Register"]
pub mod fbtp;
#[doc = "Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](test) module"]
pub type TEST = crate::Reg<u32, _TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEST;
#[doc = "`read()` method returns [test::R](test::R) reader structure"]
impl crate::Readable for TEST {}
#[doc = "`write(|w| ..)` method takes [test::W](test::W) writer structure"]
impl crate::Writable for TEST {}
#[doc = "Test Register"]
pub mod test;
#[doc = "RAM Watchdog Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rwd](rwd) module"]
pub type RWD = crate::Reg<u32, _RWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RWD;
#[doc = "`read()` method returns [rwd::R](rwd::R) reader structure"]
impl crate::Readable for RWD {}
#[doc = "`write(|w| ..)` method takes [rwd::W](rwd::W) writer structure"]
impl crate::Writable for RWD {}
#[doc = "RAM Watchdog Register"]
pub mod rwd;
#[doc = "CC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cccr](cccr) module"]
pub type CCCR = crate::Reg<u32, _CCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCCR;
#[doc = "`read()` method returns [cccr::R](cccr::R) reader structure"]
impl crate::Readable for CCCR {}
#[doc = "`write(|w| ..)` method takes [cccr::W](cccr::W) writer structure"]
impl crate::Writable for CCCR {}
#[doc = "CC Control Register"]
pub mod cccr;
#[doc = "Bit Timing and Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btp](btp) module"]
pub type BTP = crate::Reg<u32, _BTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTP;
#[doc = "`read()` method returns [btp::R](btp::R) reader structure"]
impl crate::Readable for BTP {}
#[doc = "`write(|w| ..)` method takes [btp::W](btp::W) writer structure"]
impl crate::Writable for BTP {}
#[doc = "Bit Timing and Prescaler Register"]
pub mod btp;
#[doc = "Timestamp Counter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscc](tscc) module"]
pub type TSCC = crate::Reg<u32, _TSCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSCC;
#[doc = "`read()` method returns [tscc::R](tscc::R) reader structure"]
impl crate::Readable for TSCC {}
#[doc = "`write(|w| ..)` method takes [tscc::W](tscc::W) writer structure"]
impl crate::Writable for TSCC {}
#[doc = "Timestamp Counter Configuration Register"]
pub mod tscc;
#[doc = "Timestamp Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscv](tscv) module"]
pub type TSCV = crate::Reg<u32, _TSCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSCV;
#[doc = "`read()` method returns [tscv::R](tscv::R) reader structure"]
impl crate::Readable for TSCV {}
#[doc = "`write(|w| ..)` method takes [tscv::W](tscv::W) writer structure"]
impl crate::Writable for TSCV {}
#[doc = "Timestamp Counter Value Register"]
pub mod tscv;
#[doc = "Timeout Counter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocc](tocc) module"]
pub type TOCC = crate::Reg<u32, _TOCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOCC;
#[doc = "`read()` method returns [tocc::R](tocc::R) reader structure"]
impl crate::Readable for TOCC {}
#[doc = "`write(|w| ..)` method takes [tocc::W](tocc::W) writer structure"]
impl crate::Writable for TOCC {}
#[doc = "Timeout Counter Configuration Register"]
pub mod tocc;
#[doc = "Timeout Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocv](tocv) module"]
pub type TOCV = crate::Reg<u32, _TOCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOCV;
#[doc = "`read()` method returns [tocv::R](tocv::R) reader structure"]
impl crate::Readable for TOCV {}
#[doc = "`write(|w| ..)` method takes [tocv::W](tocv::W) writer structure"]
impl crate::Writable for TOCV {}
#[doc = "Timeout Counter Value Register"]
pub mod tocv;
#[doc = "Error Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](ecr) module"]
pub type ECR = crate::Reg<u32, _ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECR;
#[doc = "`read()` method returns [ecr::R](ecr::R) reader structure"]
impl crate::Readable for ECR {}
#[doc = "Error Counter Register"]
pub mod ecr;
#[doc = "Protocol Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](psr) module"]
pub type PSR = crate::Reg<u32, _PSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSR;
#[doc = "`read()` method returns [psr::R](psr::R) reader structure"]
impl crate::Readable for PSR {}
#[doc = "Protocol Status Register"]
pub mod psr;
#[doc = "Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](ir) module"]
pub type IR = crate::Reg<u32, _IR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IR;
#[doc = "`read()` method returns [ir::R](ir::R) reader structure"]
impl crate::Readable for IR {}
#[doc = "`write(|w| ..)` method takes [ir::W](ir::W) writer structure"]
impl crate::Writable for IR {}
#[doc = "Interrupt Register"]
pub mod ir;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](ie) module"]
pub type IE = crate::Reg<u32, _IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IE;
#[doc = "`read()` method returns [ie::R](ie::R) reader structure"]
impl crate::Readable for IE {}
#[doc = "`write(|w| ..)` method takes [ie::W](ie::W) writer structure"]
impl crate::Writable for IE {}
#[doc = "Interrupt Enable Register"]
pub mod ie;
#[doc = "Interrupt Line Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ils](ils) module"]
pub type ILS = crate::Reg<u32, _ILS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ILS;
#[doc = "`read()` method returns [ils::R](ils::R) reader structure"]
impl crate::Readable for ILS {}
#[doc = "`write(|w| ..)` method takes [ils::W](ils::W) writer structure"]
impl crate::Writable for ILS {}
#[doc = "Interrupt Line Select Register"]
pub mod ils;
#[doc = "Interrupt Line Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ile](ile) module"]
pub type ILE = crate::Reg<u32, _ILE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ILE;
#[doc = "`read()` method returns [ile::R](ile::R) reader structure"]
impl crate::Readable for ILE {}
#[doc = "`write(|w| ..)` method takes [ile::W](ile::W) writer structure"]
impl crate::Writable for ILE {}
#[doc = "Interrupt Line Enable Register"]
pub mod ile;
#[doc = "Global Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gfc](gfc) module"]
pub type GFC = crate::Reg<u32, _GFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GFC;
#[doc = "`read()` method returns [gfc::R](gfc::R) reader structure"]
impl crate::Readable for GFC {}
#[doc = "`write(|w| ..)` method takes [gfc::W](gfc::W) writer structure"]
impl crate::Writable for GFC {}
#[doc = "Global Filter Configuration Register"]
pub mod gfc;
#[doc = "Standard ID Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sidfc](sidfc) module"]
pub type SIDFC = crate::Reg<u32, _SIDFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIDFC;
#[doc = "`read()` method returns [sidfc::R](sidfc::R) reader structure"]
impl crate::Readable for SIDFC {}
#[doc = "`write(|w| ..)` method takes [sidfc::W](sidfc::W) writer structure"]
impl crate::Writable for SIDFC {}
#[doc = "Standard ID Filter Configuration Register"]
pub mod sidfc;
#[doc = "Extended ID Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xidfc](xidfc) module"]
pub type XIDFC = crate::Reg<u32, _XIDFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XIDFC;
#[doc = "`read()` method returns [xidfc::R](xidfc::R) reader structure"]
impl crate::Readable for XIDFC {}
#[doc = "`write(|w| ..)` method takes [xidfc::W](xidfc::W) writer structure"]
impl crate::Writable for XIDFC {}
#[doc = "Extended ID Filter Configuration Register"]
pub mod xidfc;
#[doc = "Extended ID AND Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xidam](xidam) module"]
pub type XIDAM = crate::Reg<u32, _XIDAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XIDAM;
#[doc = "`read()` method returns [xidam::R](xidam::R) reader structure"]
impl crate::Readable for XIDAM {}
#[doc = "`write(|w| ..)` method takes [xidam::W](xidam::W) writer structure"]
impl crate::Writable for XIDAM {}
#[doc = "Extended ID AND Mask Register"]
pub mod xidam;
#[doc = "High Priority Message Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpms](hpms) module"]
pub type HPMS = crate::Reg<u32, _HPMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPMS;
#[doc = "`read()` method returns [hpms::R](hpms::R) reader structure"]
impl crate::Readable for HPMS {}
#[doc = "High Priority Message Status Register"]
pub mod hpms;
#[doc = "New Data 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ndat1](ndat1) module"]
pub type NDAT1 = crate::Reg<u32, _NDAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NDAT1;
#[doc = "`read()` method returns [ndat1::R](ndat1::R) reader structure"]
impl crate::Readable for NDAT1 {}
#[doc = "`write(|w| ..)` method takes [ndat1::W](ndat1::W) writer structure"]
impl crate::Writable for NDAT1 {}
#[doc = "New Data 1 Register"]
pub mod ndat1;
#[doc = "New Data 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ndat2](ndat2) module"]
pub type NDAT2 = crate::Reg<u32, _NDAT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NDAT2;
#[doc = "`read()` method returns [ndat2::R](ndat2::R) reader structure"]
impl crate::Readable for NDAT2 {}
#[doc = "`write(|w| ..)` method takes [ndat2::W](ndat2::W) writer structure"]
impl crate::Writable for NDAT2 {}
#[doc = "New Data 2 Register"]
pub mod ndat2;
#[doc = "Receive FIFO 0 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0c](rxf0c) module"]
pub type RXF0C = crate::Reg<u32, _RXF0C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF0C;
#[doc = "`read()` method returns [rxf0c::R](rxf0c::R) reader structure"]
impl crate::Readable for RXF0C {}
#[doc = "`write(|w| ..)` method takes [rxf0c::W](rxf0c::W) writer structure"]
impl crate::Writable for RXF0C {}
#[doc = "Receive FIFO 0 Configuration Register"]
pub mod rxf0c;
#[doc = "Receive FIFO 0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0s](rxf0s) module"]
pub type RXF0S = crate::Reg<u32, _RXF0S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF0S;
#[doc = "`read()` method returns [rxf0s::R](rxf0s::R) reader structure"]
impl crate::Readable for RXF0S {}
#[doc = "Receive FIFO 0 Status Register"]
pub mod rxf0s;
#[doc = "Receive FIFO 0 Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0a](rxf0a) module"]
pub type RXF0A = crate::Reg<u32, _RXF0A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF0A;
#[doc = "`read()` method returns [rxf0a::R](rxf0a::R) reader structure"]
impl crate::Readable for RXF0A {}
#[doc = "`write(|w| ..)` method takes [rxf0a::W](rxf0a::W) writer structure"]
impl crate::Writable for RXF0A {}
#[doc = "Receive FIFO 0 Acknowledge Register"]
pub mod rxf0a;
#[doc = "Receive Rx Buffer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbc](rxbc) module"]
pub type RXBC = crate::Reg<u32, _RXBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBC;
#[doc = "`read()` method returns [rxbc::R](rxbc::R) reader structure"]
impl crate::Readable for RXBC {}
#[doc = "`write(|w| ..)` method takes [rxbc::W](rxbc::W) writer structure"]
impl crate::Writable for RXBC {}
#[doc = "Receive Rx Buffer Configuration Register"]
pub mod rxbc;
#[doc = "Receive FIFO 1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf1c](rxf1c) module"]
pub type RXF1C = crate::Reg<u32, _RXF1C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF1C;
#[doc = "`read()` method returns [rxf1c::R](rxf1c::R) reader structure"]
impl crate::Readable for RXF1C {}
#[doc = "`write(|w| ..)` method takes [rxf1c::W](rxf1c::W) writer structure"]
impl crate::Writable for RXF1C {}
#[doc = "Receive FIFO 1 Configuration Register"]
pub mod rxf1c;
#[doc = "Receive FIFO 1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf1s](rxf1s) module"]
pub type RXF1S = crate::Reg<u32, _RXF1S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF1S;
#[doc = "`read()` method returns [rxf1s::R](rxf1s::R) reader structure"]
impl crate::Readable for RXF1S {}
#[doc = "Receive FIFO 1 Status Register"]
pub mod rxf1s;
#[doc = "Receive FIFO 1 Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf1a](rxf1a) module"]
pub type RXF1A = crate::Reg<u32, _RXF1A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF1A;
#[doc = "`read()` method returns [rxf1a::R](rxf1a::R) reader structure"]
impl crate::Readable for RXF1A {}
#[doc = "`write(|w| ..)` method takes [rxf1a::W](rxf1a::W) writer structure"]
impl crate::Writable for RXF1A {}
#[doc = "Receive FIFO 1 Acknowledge Register"]
pub mod rxf1a;
#[doc = "Receive Buffer / FIFO Element Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxesc](rxesc) module"]
pub type RXESC = crate::Reg<u32, _RXESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXESC;
#[doc = "`read()` method returns [rxesc::R](rxesc::R) reader structure"]
impl crate::Readable for RXESC {}
#[doc = "`write(|w| ..)` method takes [rxesc::W](rxesc::W) writer structure"]
impl crate::Writable for RXESC {}
#[doc = "Receive Buffer / FIFO Element Size Configuration Register"]
pub mod rxesc;
#[doc = "Transmit Buffer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbc](txbc) module"]
pub type TXBC = crate::Reg<u32, _TXBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBC;
#[doc = "`read()` method returns [txbc::R](txbc::R) reader structure"]
impl crate::Readable for TXBC {}
#[doc = "`write(|w| ..)` method takes [txbc::W](txbc::W) writer structure"]
impl crate::Writable for TXBC {}
#[doc = "Transmit Buffer Configuration Register"]
pub mod txbc;
#[doc = "Transmit FIFO/Queue Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfqs](txfqs) module"]
pub type TXFQS = crate::Reg<u32, _TXFQS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFQS;
#[doc = "`read()` method returns [txfqs::R](txfqs::R) reader structure"]
impl crate::Readable for TXFQS {}
#[doc = "Transmit FIFO/Queue Status Register"]
pub mod txfqs;
#[doc = "Transmit Buffer Element Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txesc](txesc) module"]
pub type TXESC = crate::Reg<u32, _TXESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXESC;
#[doc = "`read()` method returns [txesc::R](txesc::R) reader structure"]
impl crate::Readable for TXESC {}
#[doc = "`write(|w| ..)` method takes [txesc::W](txesc::W) writer structure"]
impl crate::Writable for TXESC {}
#[doc = "Transmit Buffer Element Size Configuration Register"]
pub mod txesc;
#[doc = "Transmit Buffer Request Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbrp](txbrp) module"]
pub type TXBRP = crate::Reg<u32, _TXBRP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBRP;
#[doc = "`read()` method returns [txbrp::R](txbrp::R) reader structure"]
impl crate::Readable for TXBRP {}
#[doc = "Transmit Buffer Request Pending Register"]
pub mod txbrp;
#[doc = "Transmit Buffer Add Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbar](txbar) module"]
pub type TXBAR = crate::Reg<u32, _TXBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBAR;
#[doc = "`read()` method returns [txbar::R](txbar::R) reader structure"]
impl crate::Readable for TXBAR {}
#[doc = "`write(|w| ..)` method takes [txbar::W](txbar::W) writer structure"]
impl crate::Writable for TXBAR {}
#[doc = "Transmit Buffer Add Request Register"]
pub mod txbar;
#[doc = "Transmit Buffer Cancellation Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcr](txbcr) module"]
pub type TXBCR = crate::Reg<u32, _TXBCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBCR;
#[doc = "`read()` method returns [txbcr::R](txbcr::R) reader structure"]
impl crate::Readable for TXBCR {}
#[doc = "`write(|w| ..)` method takes [txbcr::W](txbcr::W) writer structure"]
impl crate::Writable for TXBCR {}
#[doc = "Transmit Buffer Cancellation Request Register"]
pub mod txbcr;
#[doc = "Transmit Buffer Transmission Occurred Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbto](txbto) module"]
pub type TXBTO = crate::Reg<u32, _TXBTO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBTO;
#[doc = "`read()` method returns [txbto::R](txbto::R) reader structure"]
impl crate::Readable for TXBTO {}
#[doc = "Transmit Buffer Transmission Occurred Register"]
pub mod txbto;
#[doc = "Transmit Buffer Cancellation Finished Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcf](txbcf) module"]
pub type TXBCF = crate::Reg<u32, _TXBCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBCF;
#[doc = "`read()` method returns [txbcf::R](txbcf::R) reader structure"]
impl crate::Readable for TXBCF {}
#[doc = "Transmit Buffer Cancellation Finished Register"]
pub mod txbcf;
#[doc = "Transmit Buffer Transmission Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbtie](txbtie) module"]
pub type TXBTIE = crate::Reg<u32, _TXBTIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBTIE;
#[doc = "`read()` method returns [txbtie::R](txbtie::R) reader structure"]
impl crate::Readable for TXBTIE {}
#[doc = "`write(|w| ..)` method takes [txbtie::W](txbtie::W) writer structure"]
impl crate::Writable for TXBTIE {}
#[doc = "Transmit Buffer Transmission Interrupt Enable Register"]
pub mod txbtie;
#[doc = "Transmit Buffer Cancellation Finished Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcie](txbcie) module"]
pub type TXBCIE = crate::Reg<u32, _TXBCIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBCIE;
#[doc = "`read()` method returns [txbcie::R](txbcie::R) reader structure"]
impl crate::Readable for TXBCIE {}
#[doc = "`write(|w| ..)` method takes [txbcie::W](txbcie::W) writer structure"]
impl crate::Writable for TXBCIE {}
#[doc = "Transmit Buffer Cancellation Finished Interrupt Enable Register"]
pub mod txbcie;
#[doc = "Transmit Event FIFO Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txefc](txefc) module"]
pub type TXEFC = crate::Reg<u32, _TXEFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXEFC;
#[doc = "`read()` method returns [txefc::R](txefc::R) reader structure"]
impl crate::Readable for TXEFC {}
#[doc = "`write(|w| ..)` method takes [txefc::W](txefc::W) writer structure"]
impl crate::Writable for TXEFC {}
#[doc = "Transmit Event FIFO Configuration Register"]
pub mod txefc;
#[doc = "Transmit Event FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txefs](txefs) module"]
pub type TXEFS = crate::Reg<u32, _TXEFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXEFS;
#[doc = "`read()` method returns [txefs::R](txefs::R) reader structure"]
impl crate::Readable for TXEFS {}
#[doc = "Transmit Event FIFO Status Register"]
pub mod txefs;
#[doc = "Transmit Event FIFO Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txefa](txefa) module"]
pub type TXEFA = crate::Reg<u32, _TXEFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXEFA;
#[doc = "`read()` method returns [txefa::R](txefa::R) reader structure"]
impl crate::Readable for TXEFA {}
#[doc = "`write(|w| ..)` method takes [txefa::W](txefa::W) writer structure"]
impl crate::Writable for TXEFA {}
#[doc = "Transmit Event FIFO Acknowledge Register"]
pub mod txefa;
