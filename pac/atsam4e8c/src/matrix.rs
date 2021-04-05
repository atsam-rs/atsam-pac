#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Configuration Register"]
    pub mcfg: [MCFG; 16],
    #[doc = "0x40 - Slave Configuration Register"]
    pub scfg: [SCFG; 16],
    #[doc = "0x80 - Priority Register A for Slave 0"]
    pub pras0: PRAS0,
    #[doc = "0x84 - Priority Register B for Slave 0"]
    pub prbs0: PRBS0,
    #[doc = "0x88 - Priority Register A for Slave 1"]
    pub pras1: PRAS1,
    #[doc = "0x8c - Priority Register B for Slave 1"]
    pub prbs1: PRBS1,
    #[doc = "0x90 - Priority Register A for Slave 2"]
    pub pras2: PRAS2,
    #[doc = "0x94 - Priority Register B for Slave 2"]
    pub prbs2: PRBS2,
    #[doc = "0x98 - Priority Register A for Slave 3"]
    pub pras3: PRAS3,
    #[doc = "0x9c - Priority Register B for Slave 3"]
    pub prbs3: PRBS3,
    #[doc = "0xa0 - Priority Register A for Slave 4"]
    pub pras4: PRAS4,
    #[doc = "0xa4 - Priority Register B for Slave 4"]
    pub prbs4: PRBS4,
    #[doc = "0xa8 - Priority Register A for Slave 5"]
    pub pras5: PRAS5,
    #[doc = "0xac - Priority Register B for Slave 5"]
    pub prbs5: PRBS5,
    #[doc = "0xb0 - Priority Register A for Slave 6"]
    pub pras6: PRAS6,
    #[doc = "0xb4 - Priority Register B for Slave 6"]
    pub prbs6: PRBS6,
    #[doc = "0xb8 - Priority Register A for Slave 7"]
    pub pras7: PRAS7,
    #[doc = "0xbc - Priority Register B for Slave 7"]
    pub prbs7: PRBS7,
    #[doc = "0xc0 - Priority Register A for Slave 8"]
    pub pras8: PRAS8,
    #[doc = "0xc4 - Priority Register B for Slave 8"]
    pub prbs8: PRBS8,
    #[doc = "0xc8 - Priority Register A for Slave 9"]
    pub pras9: PRAS9,
    #[doc = "0xcc - Priority Register B for Slave 9"]
    pub prbs9: PRBS9,
    #[doc = "0xd0 - Priority Register A for Slave 10"]
    pub pras10: PRAS10,
    #[doc = "0xd4 - Priority Register B for Slave 10"]
    pub prbs10: PRBS10,
    #[doc = "0xd8 - Priority Register A for Slave 11"]
    pub pras11: PRAS11,
    #[doc = "0xdc - Priority Register B for Slave 11"]
    pub prbs11: PRBS11,
    #[doc = "0xe0 - Priority Register A for Slave 12"]
    pub pras12: PRAS12,
    #[doc = "0xe4 - Priority Register B for Slave 12"]
    pub prbs12: PRBS12,
    #[doc = "0xe8 - Priority Register A for Slave 13"]
    pub pras13: PRAS13,
    #[doc = "0xec - Priority Register B for Slave 13"]
    pub prbs13: PRBS13,
    #[doc = "0xf0 - Priority Register A for Slave 14"]
    pub pras14: PRAS14,
    #[doc = "0xf4 - Priority Register B for Slave 14"]
    pub prbs14: PRBS14,
    #[doc = "0xf8 - Priority Register A for Slave 15"]
    pub pras15: PRAS15,
    #[doc = "0xfc - Priority Register B for Slave 15"]
    pub prbs15: PRBS15,
    #[doc = "0x100 - Master Remap Control Register"]
    pub mrcr: MRCR,
    _reserved35: [u8; 12usize],
    #[doc = "0x110 - Special Function Register"]
    pub sfr: [SFR; 16],
    _reserved36: [u8; 148usize],
    #[doc = "0x1e4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0x1e8 - Write Protect Status Register"]
    pub wpsr: WPSR,
}
#[doc = "Master Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfg](mcfg) module"]
pub type MCFG = crate::Reg<u32, _MCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCFG;
#[doc = "`read()` method returns [mcfg::R](mcfg::R) reader structure"]
impl crate::Readable for MCFG {}
#[doc = "`write(|w| ..)` method takes [mcfg::W](mcfg::W) writer structure"]
impl crate::Writable for MCFG {}
#[doc = "Master Configuration Register"]
pub mod mcfg;
#[doc = "Slave Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfg](scfg) module"]
pub type SCFG = crate::Reg<u32, _SCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCFG;
#[doc = "`read()` method returns [scfg::R](scfg::R) reader structure"]
impl crate::Readable for SCFG {}
#[doc = "`write(|w| ..)` method takes [scfg::W](scfg::W) writer structure"]
impl crate::Writable for SCFG {}
#[doc = "Slave Configuration Register"]
pub mod scfg;
#[doc = "Priority Register A for Slave 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras0](pras0) module"]
pub type PRAS0 = crate::Reg<u32, _PRAS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS0;
#[doc = "`read()` method returns [pras0::R](pras0::R) reader structure"]
impl crate::Readable for PRAS0 {}
#[doc = "`write(|w| ..)` method takes [pras0::W](pras0::W) writer structure"]
impl crate::Writable for PRAS0 {}
#[doc = "Priority Register A for Slave 0"]
pub mod pras0;
#[doc = "Priority Register B for Slave 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs0](prbs0) module"]
pub type PRBS0 = crate::Reg<u32, _PRBS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS0;
#[doc = "`read()` method returns [prbs0::R](prbs0::R) reader structure"]
impl crate::Readable for PRBS0 {}
#[doc = "`write(|w| ..)` method takes [prbs0::W](prbs0::W) writer structure"]
impl crate::Writable for PRBS0 {}
#[doc = "Priority Register B for Slave 0"]
pub mod prbs0;
#[doc = "Priority Register A for Slave 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras1](pras1) module"]
pub type PRAS1 = crate::Reg<u32, _PRAS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS1;
#[doc = "`read()` method returns [pras1::R](pras1::R) reader structure"]
impl crate::Readable for PRAS1 {}
#[doc = "`write(|w| ..)` method takes [pras1::W](pras1::W) writer structure"]
impl crate::Writable for PRAS1 {}
#[doc = "Priority Register A for Slave 1"]
pub mod pras1;
#[doc = "Priority Register B for Slave 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs1](prbs1) module"]
pub type PRBS1 = crate::Reg<u32, _PRBS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS1;
#[doc = "`read()` method returns [prbs1::R](prbs1::R) reader structure"]
impl crate::Readable for PRBS1 {}
#[doc = "`write(|w| ..)` method takes [prbs1::W](prbs1::W) writer structure"]
impl crate::Writable for PRBS1 {}
#[doc = "Priority Register B for Slave 1"]
pub mod prbs1;
#[doc = "Priority Register A for Slave 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras2](pras2) module"]
pub type PRAS2 = crate::Reg<u32, _PRAS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS2;
#[doc = "`read()` method returns [pras2::R](pras2::R) reader structure"]
impl crate::Readable for PRAS2 {}
#[doc = "`write(|w| ..)` method takes [pras2::W](pras2::W) writer structure"]
impl crate::Writable for PRAS2 {}
#[doc = "Priority Register A for Slave 2"]
pub mod pras2;
#[doc = "Priority Register B for Slave 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs2](prbs2) module"]
pub type PRBS2 = crate::Reg<u32, _PRBS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS2;
#[doc = "`read()` method returns [prbs2::R](prbs2::R) reader structure"]
impl crate::Readable for PRBS2 {}
#[doc = "`write(|w| ..)` method takes [prbs2::W](prbs2::W) writer structure"]
impl crate::Writable for PRBS2 {}
#[doc = "Priority Register B for Slave 2"]
pub mod prbs2;
#[doc = "Priority Register A for Slave 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras3](pras3) module"]
pub type PRAS3 = crate::Reg<u32, _PRAS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS3;
#[doc = "`read()` method returns [pras3::R](pras3::R) reader structure"]
impl crate::Readable for PRAS3 {}
#[doc = "`write(|w| ..)` method takes [pras3::W](pras3::W) writer structure"]
impl crate::Writable for PRAS3 {}
#[doc = "Priority Register A for Slave 3"]
pub mod pras3;
#[doc = "Priority Register B for Slave 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs3](prbs3) module"]
pub type PRBS3 = crate::Reg<u32, _PRBS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS3;
#[doc = "`read()` method returns [prbs3::R](prbs3::R) reader structure"]
impl crate::Readable for PRBS3 {}
#[doc = "`write(|w| ..)` method takes [prbs3::W](prbs3::W) writer structure"]
impl crate::Writable for PRBS3 {}
#[doc = "Priority Register B for Slave 3"]
pub mod prbs3;
#[doc = "Priority Register A for Slave 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras4](pras4) module"]
pub type PRAS4 = crate::Reg<u32, _PRAS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS4;
#[doc = "`read()` method returns [pras4::R](pras4::R) reader structure"]
impl crate::Readable for PRAS4 {}
#[doc = "`write(|w| ..)` method takes [pras4::W](pras4::W) writer structure"]
impl crate::Writable for PRAS4 {}
#[doc = "Priority Register A for Slave 4"]
pub mod pras4;
#[doc = "Priority Register B for Slave 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs4](prbs4) module"]
pub type PRBS4 = crate::Reg<u32, _PRBS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS4;
#[doc = "`read()` method returns [prbs4::R](prbs4::R) reader structure"]
impl crate::Readable for PRBS4 {}
#[doc = "`write(|w| ..)` method takes [prbs4::W](prbs4::W) writer structure"]
impl crate::Writable for PRBS4 {}
#[doc = "Priority Register B for Slave 4"]
pub mod prbs4;
#[doc = "Priority Register A for Slave 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras5](pras5) module"]
pub type PRAS5 = crate::Reg<u32, _PRAS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS5;
#[doc = "`read()` method returns [pras5::R](pras5::R) reader structure"]
impl crate::Readable for PRAS5 {}
#[doc = "`write(|w| ..)` method takes [pras5::W](pras5::W) writer structure"]
impl crate::Writable for PRAS5 {}
#[doc = "Priority Register A for Slave 5"]
pub mod pras5;
#[doc = "Priority Register B for Slave 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs5](prbs5) module"]
pub type PRBS5 = crate::Reg<u32, _PRBS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS5;
#[doc = "`read()` method returns [prbs5::R](prbs5::R) reader structure"]
impl crate::Readable for PRBS5 {}
#[doc = "`write(|w| ..)` method takes [prbs5::W](prbs5::W) writer structure"]
impl crate::Writable for PRBS5 {}
#[doc = "Priority Register B for Slave 5"]
pub mod prbs5;
#[doc = "Priority Register A for Slave 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras6](pras6) module"]
pub type PRAS6 = crate::Reg<u32, _PRAS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS6;
#[doc = "`read()` method returns [pras6::R](pras6::R) reader structure"]
impl crate::Readable for PRAS6 {}
#[doc = "`write(|w| ..)` method takes [pras6::W](pras6::W) writer structure"]
impl crate::Writable for PRAS6 {}
#[doc = "Priority Register A for Slave 6"]
pub mod pras6;
#[doc = "Priority Register B for Slave 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs6](prbs6) module"]
pub type PRBS6 = crate::Reg<u32, _PRBS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS6;
#[doc = "`read()` method returns [prbs6::R](prbs6::R) reader structure"]
impl crate::Readable for PRBS6 {}
#[doc = "`write(|w| ..)` method takes [prbs6::W](prbs6::W) writer structure"]
impl crate::Writable for PRBS6 {}
#[doc = "Priority Register B for Slave 6"]
pub mod prbs6;
#[doc = "Priority Register A for Slave 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras7](pras7) module"]
pub type PRAS7 = crate::Reg<u32, _PRAS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS7;
#[doc = "`read()` method returns [pras7::R](pras7::R) reader structure"]
impl crate::Readable for PRAS7 {}
#[doc = "`write(|w| ..)` method takes [pras7::W](pras7::W) writer structure"]
impl crate::Writable for PRAS7 {}
#[doc = "Priority Register A for Slave 7"]
pub mod pras7;
#[doc = "Priority Register B for Slave 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs7](prbs7) module"]
pub type PRBS7 = crate::Reg<u32, _PRBS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS7;
#[doc = "`read()` method returns [prbs7::R](prbs7::R) reader structure"]
impl crate::Readable for PRBS7 {}
#[doc = "`write(|w| ..)` method takes [prbs7::W](prbs7::W) writer structure"]
impl crate::Writable for PRBS7 {}
#[doc = "Priority Register B for Slave 7"]
pub mod prbs7;
#[doc = "Priority Register A for Slave 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras8](pras8) module"]
pub type PRAS8 = crate::Reg<u32, _PRAS8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS8;
#[doc = "`read()` method returns [pras8::R](pras8::R) reader structure"]
impl crate::Readable for PRAS8 {}
#[doc = "`write(|w| ..)` method takes [pras8::W](pras8::W) writer structure"]
impl crate::Writable for PRAS8 {}
#[doc = "Priority Register A for Slave 8"]
pub mod pras8;
#[doc = "Priority Register B for Slave 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs8](prbs8) module"]
pub type PRBS8 = crate::Reg<u32, _PRBS8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS8;
#[doc = "`read()` method returns [prbs8::R](prbs8::R) reader structure"]
impl crate::Readable for PRBS8 {}
#[doc = "`write(|w| ..)` method takes [prbs8::W](prbs8::W) writer structure"]
impl crate::Writable for PRBS8 {}
#[doc = "Priority Register B for Slave 8"]
pub mod prbs8;
#[doc = "Priority Register A for Slave 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras9](pras9) module"]
pub type PRAS9 = crate::Reg<u32, _PRAS9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS9;
#[doc = "`read()` method returns [pras9::R](pras9::R) reader structure"]
impl crate::Readable for PRAS9 {}
#[doc = "`write(|w| ..)` method takes [pras9::W](pras9::W) writer structure"]
impl crate::Writable for PRAS9 {}
#[doc = "Priority Register A for Slave 9"]
pub mod pras9;
#[doc = "Priority Register B for Slave 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs9](prbs9) module"]
pub type PRBS9 = crate::Reg<u32, _PRBS9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS9;
#[doc = "`read()` method returns [prbs9::R](prbs9::R) reader structure"]
impl crate::Readable for PRBS9 {}
#[doc = "`write(|w| ..)` method takes [prbs9::W](prbs9::W) writer structure"]
impl crate::Writable for PRBS9 {}
#[doc = "Priority Register B for Slave 9"]
pub mod prbs9;
#[doc = "Priority Register A for Slave 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras10](pras10) module"]
pub type PRAS10 = crate::Reg<u32, _PRAS10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS10;
#[doc = "`read()` method returns [pras10::R](pras10::R) reader structure"]
impl crate::Readable for PRAS10 {}
#[doc = "`write(|w| ..)` method takes [pras10::W](pras10::W) writer structure"]
impl crate::Writable for PRAS10 {}
#[doc = "Priority Register A for Slave 10"]
pub mod pras10;
#[doc = "Priority Register B for Slave 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs10](prbs10) module"]
pub type PRBS10 = crate::Reg<u32, _PRBS10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS10;
#[doc = "`read()` method returns [prbs10::R](prbs10::R) reader structure"]
impl crate::Readable for PRBS10 {}
#[doc = "`write(|w| ..)` method takes [prbs10::W](prbs10::W) writer structure"]
impl crate::Writable for PRBS10 {}
#[doc = "Priority Register B for Slave 10"]
pub mod prbs10;
#[doc = "Priority Register A for Slave 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras11](pras11) module"]
pub type PRAS11 = crate::Reg<u32, _PRAS11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS11;
#[doc = "`read()` method returns [pras11::R](pras11::R) reader structure"]
impl crate::Readable for PRAS11 {}
#[doc = "`write(|w| ..)` method takes [pras11::W](pras11::W) writer structure"]
impl crate::Writable for PRAS11 {}
#[doc = "Priority Register A for Slave 11"]
pub mod pras11;
#[doc = "Priority Register B for Slave 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs11](prbs11) module"]
pub type PRBS11 = crate::Reg<u32, _PRBS11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS11;
#[doc = "`read()` method returns [prbs11::R](prbs11::R) reader structure"]
impl crate::Readable for PRBS11 {}
#[doc = "`write(|w| ..)` method takes [prbs11::W](prbs11::W) writer structure"]
impl crate::Writable for PRBS11 {}
#[doc = "Priority Register B for Slave 11"]
pub mod prbs11;
#[doc = "Priority Register A for Slave 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras12](pras12) module"]
pub type PRAS12 = crate::Reg<u32, _PRAS12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS12;
#[doc = "`read()` method returns [pras12::R](pras12::R) reader structure"]
impl crate::Readable for PRAS12 {}
#[doc = "`write(|w| ..)` method takes [pras12::W](pras12::W) writer structure"]
impl crate::Writable for PRAS12 {}
#[doc = "Priority Register A for Slave 12"]
pub mod pras12;
#[doc = "Priority Register B for Slave 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs12](prbs12) module"]
pub type PRBS12 = crate::Reg<u32, _PRBS12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS12;
#[doc = "`read()` method returns [prbs12::R](prbs12::R) reader structure"]
impl crate::Readable for PRBS12 {}
#[doc = "`write(|w| ..)` method takes [prbs12::W](prbs12::W) writer structure"]
impl crate::Writable for PRBS12 {}
#[doc = "Priority Register B for Slave 12"]
pub mod prbs12;
#[doc = "Priority Register A for Slave 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras13](pras13) module"]
pub type PRAS13 = crate::Reg<u32, _PRAS13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS13;
#[doc = "`read()` method returns [pras13::R](pras13::R) reader structure"]
impl crate::Readable for PRAS13 {}
#[doc = "`write(|w| ..)` method takes [pras13::W](pras13::W) writer structure"]
impl crate::Writable for PRAS13 {}
#[doc = "Priority Register A for Slave 13"]
pub mod pras13;
#[doc = "Priority Register B for Slave 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs13](prbs13) module"]
pub type PRBS13 = crate::Reg<u32, _PRBS13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS13;
#[doc = "`read()` method returns [prbs13::R](prbs13::R) reader structure"]
impl crate::Readable for PRBS13 {}
#[doc = "`write(|w| ..)` method takes [prbs13::W](prbs13::W) writer structure"]
impl crate::Writable for PRBS13 {}
#[doc = "Priority Register B for Slave 13"]
pub mod prbs13;
#[doc = "Priority Register A for Slave 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras14](pras14) module"]
pub type PRAS14 = crate::Reg<u32, _PRAS14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS14;
#[doc = "`read()` method returns [pras14::R](pras14::R) reader structure"]
impl crate::Readable for PRAS14 {}
#[doc = "`write(|w| ..)` method takes [pras14::W](pras14::W) writer structure"]
impl crate::Writable for PRAS14 {}
#[doc = "Priority Register A for Slave 14"]
pub mod pras14;
#[doc = "Priority Register B for Slave 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs14](prbs14) module"]
pub type PRBS14 = crate::Reg<u32, _PRBS14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS14;
#[doc = "`read()` method returns [prbs14::R](prbs14::R) reader structure"]
impl crate::Readable for PRBS14 {}
#[doc = "`write(|w| ..)` method takes [prbs14::W](prbs14::W) writer structure"]
impl crate::Writable for PRBS14 {}
#[doc = "Priority Register B for Slave 14"]
pub mod prbs14;
#[doc = "Priority Register A for Slave 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras15](pras15) module"]
pub type PRAS15 = crate::Reg<u32, _PRAS15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS15;
#[doc = "`read()` method returns [pras15::R](pras15::R) reader structure"]
impl crate::Readable for PRAS15 {}
#[doc = "`write(|w| ..)` method takes [pras15::W](pras15::W) writer structure"]
impl crate::Writable for PRAS15 {}
#[doc = "Priority Register A for Slave 15"]
pub mod pras15;
#[doc = "Priority Register B for Slave 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs15](prbs15) module"]
pub type PRBS15 = crate::Reg<u32, _PRBS15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS15;
#[doc = "`read()` method returns [prbs15::R](prbs15::R) reader structure"]
impl crate::Readable for PRBS15 {}
#[doc = "`write(|w| ..)` method takes [prbs15::W](prbs15::W) writer structure"]
impl crate::Writable for PRBS15 {}
#[doc = "Priority Register B for Slave 15"]
pub mod prbs15;
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
#[doc = "Special Function Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfr](sfr) module"]
pub type SFR = crate::Reg<u32, _SFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFR;
#[doc = "`read()` method returns [sfr::R](sfr::R) reader structure"]
impl crate::Readable for SFR {}
#[doc = "`write(|w| ..)` method takes [sfr::W](sfr::W) writer structure"]
impl crate::Writable for SFR {}
#[doc = "Special Function Register"]
pub mod sfr;
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
