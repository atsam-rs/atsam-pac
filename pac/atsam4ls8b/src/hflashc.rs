#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Controller Control Register"]
    pub fcr: FCR,
    #[doc = "0x04 - Flash Controller Command Register"]
    pub fcmd: FCMD,
    #[doc = "0x08 - Flash Controller Status Register"]
    pub fsr: FSR,
    #[doc = "0x0c - Flash Controller Parameter Register"]
    pub fpr: FPR,
    #[doc = "0x10 - Flash Controller Version Register"]
    pub version: VERSION,
    #[doc = "0x14 - Flash Controller General Purpose Fuse Register High"]
    pub fgpfrhi: FGPFRHI,
    #[doc = "0x18 - Flash Controller General Purpose Fuse Register Low"]
    pub fgpfrlo: FGPFRLO,
}
#[doc = "Flash Controller Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcmd](fcmd) module"]
pub type FCMD = crate::Reg<u32, _FCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCMD;
#[doc = "`read()` method returns [fcmd::R](fcmd::R) reader structure"]
impl crate::Readable for FCMD {}
#[doc = "`write(|w| ..)` method takes [fcmd::W](fcmd::W) writer structure"]
impl crate::Writable for FCMD {}
#[doc = "Flash Controller Command Register"]
pub mod fcmd;
#[doc = "Flash Controller Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](fcr) module"]
pub type FCR = crate::Reg<u32, _FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR;
#[doc = "`read()` method returns [fcr::R](fcr::R) reader structure"]
impl crate::Readable for FCR {}
#[doc = "`write(|w| ..)` method takes [fcr::W](fcr::W) writer structure"]
impl crate::Writable for FCR {}
#[doc = "Flash Controller Control Register"]
pub mod fcr;
#[doc = "Flash Controller General Purpose Fuse Register High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgpfrhi](fgpfrhi) module"]
pub type FGPFRHI = crate::Reg<u32, _FGPFRHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FGPFRHI;
#[doc = "`read()` method returns [fgpfrhi::R](fgpfrhi::R) reader structure"]
impl crate::Readable for FGPFRHI {}
#[doc = "Flash Controller General Purpose Fuse Register High"]
pub mod fgpfrhi;
#[doc = "Flash Controller General Purpose Fuse Register Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgpfrlo](fgpfrlo) module"]
pub type FGPFRLO = crate::Reg<u32, _FGPFRLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FGPFRLO;
#[doc = "`read()` method returns [fgpfrlo::R](fgpfrlo::R) reader structure"]
impl crate::Readable for FGPFRLO {}
#[doc = "Flash Controller General Purpose Fuse Register Low"]
pub mod fgpfrlo;
#[doc = "Flash Controller Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpr](fpr) module"]
pub type FPR = crate::Reg<u32, _FPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPR;
#[doc = "`read()` method returns [fpr::R](fpr::R) reader structure"]
impl crate::Readable for FPR {}
#[doc = "Flash Controller Parameter Register"]
pub mod fpr;
#[doc = "Flash Controller Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsr](fsr) module"]
pub type FSR = crate::Reg<u32, _FSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSR;
#[doc = "`read()` method returns [fsr::R](fsr::R) reader structure"]
impl crate::Readable for FSR {}
#[doc = "Flash Controller Status Register"]
pub mod fsr;
#[doc = "Flash Controller Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "Flash Controller Version Register"]
pub mod version;
