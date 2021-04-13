#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Configuration Register 0"]
    pub mcfg: [MCFG; 12],
    _reserved1: [u8; 16usize],
    #[doc = "0x40 - Slave Configuration Register 0"]
    pub scfg: [SCFG; 9],
    _reserved2: [u8; 28usize],
    #[doc = "0x80 - Priority Register A for Slave 0"]
    pub matrix_pr: [MATRIX_PR; 9],
    _reserved3: [u8; 56usize],
    #[doc = "0x100 - Master Remap Control Register"]
    pub mrcr: MRCR,
    _reserved4: [u8; 12usize],
    #[doc = "0x110 - CAN0 Configuration Register"]
    pub ccfg_can0: CCFG_CAN0,
    #[doc = "0x114 - System I/O and CAN1 Configuration Register"]
    pub ccfg_sysio: CCFG_SYSIO,
    _reserved6: [u8; 12usize],
    #[doc = "0x124 - SMC NAND Flash Chip Select Configuration Register"]
    pub ccfg_smcnfcs: CCFG_SMCNFCS,
    _reserved7: [u8; 188usize],
    #[doc = "0x1e4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0x1e8 - Write Protection Status Register"]
    pub wpsr: WPSR,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MATRIX_PR {
    #[doc = "0x00 - Priority Register A for Slave 0"]
    pub pras: self::matrix_pr::PRAS,
    #[doc = "0x04 - Priority Register B for Slave 0"]
    pub prbs: self::matrix_pr::PRBS,
}
#[doc = r"Register block"]
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pr;
#[doc = "Master Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfg](mcfg) module"]
pub type MCFG = crate::Reg<u32, _MCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCFG;
#[doc = "`read()` method returns [mcfg::R](mcfg::R) reader structure"]
impl crate::Readable for MCFG {}
#[doc = "`write(|w| ..)` method takes [mcfg::W](mcfg::W) writer structure"]
impl crate::Writable for MCFG {}
#[doc = "Master Configuration Register 0"]
pub mod mcfg;
#[doc = "Slave Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfg](scfg) module"]
pub type SCFG = crate::Reg<u32, _SCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCFG;
#[doc = "`read()` method returns [scfg::R](scfg::R) reader structure"]
impl crate::Readable for SCFG {}
#[doc = "`write(|w| ..)` method takes [scfg::W](scfg::W) writer structure"]
impl crate::Writable for SCFG {}
#[doc = "Slave Configuration Register 0"]
pub mod scfg;
#[doc = "Master Remap Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrcr](mrcr) module"]
pub type MRCR = crate::Reg<u32, _MRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRCR;
#[doc = "`read()` method returns [mrcr::R](mrcr::R) reader structure"]
impl crate::Readable for MRCR {}
#[doc = "`write(|w| ..)` method takes [mrcr::W](mrcr::W) writer structure"]
impl crate::Writable for MRCR {}
#[doc = "Master Remap Control Register"]
pub mod mrcr;
#[doc = "CAN0 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_can0](ccfg_can0) module"]
pub type CCFG_CAN0 = crate::Reg<u32, _CCFG_CAN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_CAN0;
#[doc = "`read()` method returns [ccfg_can0::R](ccfg_can0::R) reader structure"]
impl crate::Readable for CCFG_CAN0 {}
#[doc = "`write(|w| ..)` method takes [ccfg_can0::W](ccfg_can0::W) writer structure"]
impl crate::Writable for CCFG_CAN0 {}
#[doc = "CAN0 Configuration Register"]
pub mod ccfg_can0;
#[doc = "System I/O and CAN1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_sysio](ccfg_sysio) module"]
pub type CCFG_SYSIO = crate::Reg<u32, _CCFG_SYSIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_SYSIO;
#[doc = "`read()` method returns [ccfg_sysio::R](ccfg_sysio::R) reader structure"]
impl crate::Readable for CCFG_SYSIO {}
#[doc = "`write(|w| ..)` method takes [ccfg_sysio::W](ccfg_sysio::W) writer structure"]
impl crate::Writable for CCFG_SYSIO {}
#[doc = "System I/O and CAN1 Configuration Register"]
pub mod ccfg_sysio;
#[doc = "SMC NAND Flash Chip Select Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_smcnfcs](ccfg_smcnfcs) module"]
pub type CCFG_SMCNFCS = crate::Reg<u32, _CCFG_SMCNFCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_SMCNFCS;
#[doc = "`read()` method returns [ccfg_smcnfcs::R](ccfg_smcnfcs::R) reader structure"]
impl crate::Readable for CCFG_SMCNFCS {}
#[doc = "`write(|w| ..)` method takes [ccfg_smcnfcs::W](ccfg_smcnfcs::W) writer structure"]
impl crate::Writable for CCFG_SMCNFCS {}
#[doc = "SMC NAND Flash Chip Select Configuration Register"]
pub mod ccfg_smcnfcs;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpmr](wpmr) module"]
pub type WPMR = crate::Reg<u32, _WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPMR;
#[doc = "`read()` method returns [wpmr::R](wpmr::R) reader structure"]
impl crate::Readable for WPMR {}
#[doc = "`write(|w| ..)` method takes [wpmr::W](wpmr::W) writer structure"]
impl crate::Writable for WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](wpsr) module"]
pub type WPSR = crate::Reg<u32, _WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSR;
#[doc = "`read()` method returns [wpsr::R](wpsr::R) reader structure"]
impl crate::Readable for WPSR {}
#[doc = "Write Protection Status Register"]
pub mod wpsr;
