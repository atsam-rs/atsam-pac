#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Configuration Register"]
    pub matrix_mcfg: [MATRIX_MCFG; 3],
    _reserved1: [u8; 52usize],
    #[doc = "0x40 - Slave Configuration Register"]
    pub matrix_scfg: [MATRIX_SCFG; 4],
    _reserved2: [u8; 48usize],
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
    _reserved6: [u8; 120usize],
    #[doc = "0x114 - System I/O Configuration register"]
    pub ccfg_sysio: CCFG_SYSIO,
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
