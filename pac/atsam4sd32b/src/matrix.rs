#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Configuration Register"]
    pub matrix_mcfg: [MATRIX_MCFG; 4],
    _reserved1: [u8; 48usize],
    #[doc = "0x40 - Slave Configuration Register"]
    pub matrix_scfg: [MATRIX_SCFG; 5],
    _reserved2: [u8; 44usize],
    #[doc = "0x80 - Priority Register A for Slave 0"]
    pub matrix_pras0: MATRIX_PRAS0,
    _reserved3: [u8; 4usize],
    #[doc = "0x88 - Priority Register A for Slave 1"]
    pub matrix_pras1: MATRIX_PRAS1,
    _reserved4: [u8; 4usize],
    #[doc = "0x90 - Priority Register A for Slave 2"]
    pub matrix_pras2: MATRIX_PRAS2,
    _reserved5: [u8; 4usize],
    #[doc = "0x98 - Priority Register A for Slave 3"]
    pub matrix_pras3: MATRIX_PRAS3,
    _reserved6: [u8; 4usize],
    #[doc = "0xa0 - Priority Register A for Slave 4"]
    pub matrix_pras4: MATRIX_PRAS4,
    _reserved7: [u8; 112usize],
    #[doc = "0x114 - System I/O Configuration register"]
    pub ccfg_sysio: CCFG_SYSIO,
    _reserved8: [u8; 4usize],
    #[doc = "0x11c - SMC Chip Select NAND Flash Assignment Register"]
    pub ccfg_smcnfcs: CCFG_SMCNFCS,
    _reserved9: [u8; 196usize],
    #[doc = "0x1e4 - Write Protection Mode Register"]
    pub matrix_wpmr: MATRIX_WPMR,
    #[doc = "0x1e8 - Write Protection Status Register"]
    pub matrix_wpsr: MATRIX_WPSR,
}
#[doc = "Master Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_mcfg](matrix_mcfg) module"]
pub type MATRIX_MCFG = crate::Reg<u32, _MATRIX_MCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_MCFG;
#[doc = "`read()` method returns [matrix_mcfg::R](matrix_mcfg::R) reader structure"]
impl crate::Readable for MATRIX_MCFG {}
#[doc = "`write(|w| ..)` method takes [matrix_mcfg::W](matrix_mcfg::W) writer structure"]
impl crate::Writable for MATRIX_MCFG {}
#[doc = "Master Configuration Register"]
pub mod matrix_mcfg;
#[doc = "Slave Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_scfg](matrix_scfg) module"]
pub type MATRIX_SCFG = crate::Reg<u32, _MATRIX_SCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_SCFG;
#[doc = "`read()` method returns [matrix_scfg::R](matrix_scfg::R) reader structure"]
impl crate::Readable for MATRIX_SCFG {}
#[doc = "`write(|w| ..)` method takes [matrix_scfg::W](matrix_scfg::W) writer structure"]
impl crate::Writable for MATRIX_SCFG {}
#[doc = "Slave Configuration Register"]
pub mod matrix_scfg;
#[doc = "Priority Register A for Slave 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_pras0](matrix_pras0) module"]
pub type MATRIX_PRAS0 = crate::Reg<u32, _MATRIX_PRAS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRAS0;
#[doc = "`read()` method returns [matrix_pras0::R](matrix_pras0::R) reader structure"]
impl crate::Readable for MATRIX_PRAS0 {}
#[doc = "`write(|w| ..)` method takes [matrix_pras0::W](matrix_pras0::W) writer structure"]
impl crate::Writable for MATRIX_PRAS0 {}
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pras0;
#[doc = "Priority Register A for Slave 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_pras1](matrix_pras1) module"]
pub type MATRIX_PRAS1 = crate::Reg<u32, _MATRIX_PRAS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRAS1;
#[doc = "`read()` method returns [matrix_pras1::R](matrix_pras1::R) reader structure"]
impl crate::Readable for MATRIX_PRAS1 {}
#[doc = "`write(|w| ..)` method takes [matrix_pras1::W](matrix_pras1::W) writer structure"]
impl crate::Writable for MATRIX_PRAS1 {}
#[doc = "Priority Register A for Slave 1"]
pub mod matrix_pras1;
#[doc = "Priority Register A for Slave 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_pras2](matrix_pras2) module"]
pub type MATRIX_PRAS2 = crate::Reg<u32, _MATRIX_PRAS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRAS2;
#[doc = "`read()` method returns [matrix_pras2::R](matrix_pras2::R) reader structure"]
impl crate::Readable for MATRIX_PRAS2 {}
#[doc = "`write(|w| ..)` method takes [matrix_pras2::W](matrix_pras2::W) writer structure"]
impl crate::Writable for MATRIX_PRAS2 {}
#[doc = "Priority Register A for Slave 2"]
pub mod matrix_pras2;
#[doc = "Priority Register A for Slave 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_pras3](matrix_pras3) module"]
pub type MATRIX_PRAS3 = crate::Reg<u32, _MATRIX_PRAS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRAS3;
#[doc = "`read()` method returns [matrix_pras3::R](matrix_pras3::R) reader structure"]
impl crate::Readable for MATRIX_PRAS3 {}
#[doc = "`write(|w| ..)` method takes [matrix_pras3::W](matrix_pras3::W) writer structure"]
impl crate::Writable for MATRIX_PRAS3 {}
#[doc = "Priority Register A for Slave 3"]
pub mod matrix_pras3;
#[doc = "Priority Register A for Slave 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_pras4](matrix_pras4) module"]
pub type MATRIX_PRAS4 = crate::Reg<u32, _MATRIX_PRAS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRAS4;
#[doc = "`read()` method returns [matrix_pras4::R](matrix_pras4::R) reader structure"]
impl crate::Readable for MATRIX_PRAS4 {}
#[doc = "`write(|w| ..)` method takes [matrix_pras4::W](matrix_pras4::W) writer structure"]
impl crate::Writable for MATRIX_PRAS4 {}
#[doc = "Priority Register A for Slave 4"]
pub mod matrix_pras4;
#[doc = "System I/O Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_sysio](ccfg_sysio) module"]
pub type CCFG_SYSIO = crate::Reg<u32, _CCFG_SYSIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_SYSIO;
#[doc = "`read()` method returns [ccfg_sysio::R](ccfg_sysio::R) reader structure"]
impl crate::Readable for CCFG_SYSIO {}
#[doc = "`write(|w| ..)` method takes [ccfg_sysio::W](ccfg_sysio::W) writer structure"]
impl crate::Writable for CCFG_SYSIO {}
#[doc = "System I/O Configuration register"]
pub mod ccfg_sysio;
#[doc = "SMC Chip Select NAND Flash Assignment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_smcnfcs](ccfg_smcnfcs) module"]
pub type CCFG_SMCNFCS = crate::Reg<u32, _CCFG_SMCNFCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_SMCNFCS;
#[doc = "`read()` method returns [ccfg_smcnfcs::R](ccfg_smcnfcs::R) reader structure"]
impl crate::Readable for CCFG_SMCNFCS {}
#[doc = "`write(|w| ..)` method takes [ccfg_smcnfcs::W](ccfg_smcnfcs::W) writer structure"]
impl crate::Writable for CCFG_SMCNFCS {}
#[doc = "SMC Chip Select NAND Flash Assignment Register"]
pub mod ccfg_smcnfcs;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_wpmr](matrix_wpmr) module"]
pub type MATRIX_WPMR = crate::Reg<u32, _MATRIX_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_WPMR;
#[doc = "`read()` method returns [matrix_wpmr::R](matrix_wpmr::R) reader structure"]
impl crate::Readable for MATRIX_WPMR {}
#[doc = "`write(|w| ..)` method takes [matrix_wpmr::W](matrix_wpmr::W) writer structure"]
impl crate::Writable for MATRIX_WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod matrix_wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_wpsr](matrix_wpsr) module"]
pub type MATRIX_WPSR = crate::Reg<u32, _MATRIX_WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_WPSR;
#[doc = "`read()` method returns [matrix_wpsr::R](matrix_wpsr::R) reader structure"]
impl crate::Readable for MATRIX_WPSR {}
#[doc = "Write Protection Status Register"]
pub mod matrix_wpsr;
