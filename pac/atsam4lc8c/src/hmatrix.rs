#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Configuration Register"]
    pub mcfg: [MCFG; 16],
    #[doc = "0x40 - Slave Configuration Register"]
    pub scfg: [SCFG; 16],
    #[doc = "0x80 - Priority Register A for Slave"]
    pub pras0: PRAS,
    #[doc = "0x84 - Priority Register B for Slave"]
    pub prbs0: PRBS,
    #[doc = "0x88 - Priority Register A for Slave"]
    pub pras1: PRAS,
    #[doc = "0x8c - Priority Register B for Slave"]
    pub prbs1: PRBS,
    #[doc = "0x90 - Priority Register A for Slave"]
    pub pras2: PRAS,
    #[doc = "0x94 - Priority Register B for Slave"]
    pub prbs2: PRBS,
    #[doc = "0x98 - Priority Register A for Slave"]
    pub pras3: PRAS,
    #[doc = "0x9c - Priority Register B for Slave"]
    pub prbs3: PRBS,
    #[doc = "0xa0 - Priority Register A for Slave"]
    pub pras4: PRAS,
    #[doc = "0xa4 - Priority Register B for Slave"]
    pub prbs4: PRBS,
    #[doc = "0xa8 - Priority Register A for Slave"]
    pub pras5: PRAS,
    #[doc = "0xac - Priority Register B for Slave"]
    pub prbs5: PRBS,
    #[doc = "0xb0 - Priority Register A for Slave"]
    pub pras6: PRAS,
    #[doc = "0xb4 - Priority Register B for Slave"]
    pub prbs6: PRBS,
    #[doc = "0xb8 - Priority Register A for Slave"]
    pub pras7: PRAS,
    #[doc = "0xbc - Priority Register B for Slave"]
    pub prbs7: PRBS,
    #[doc = "0xc0 - Priority Register A for Slave"]
    pub pras8: PRAS,
    #[doc = "0xc4 - Priority Register B for Slave"]
    pub prbs8: PRBS,
    #[doc = "0xc8 - Priority Register A for Slave"]
    pub pras9: PRAS,
    #[doc = "0xcc - Priority Register B for Slave"]
    pub prbs9: PRBS,
    #[doc = "0xd0 - Priority Register A for Slave"]
    pub pras10: PRAS,
    #[doc = "0xd4 - Priority Register B for Slave"]
    pub prbs10: PRBS,
    #[doc = "0xd8 - Priority Register A for Slave"]
    pub pras11: PRAS,
    #[doc = "0xdc - Priority Register B for Slave"]
    pub prbs11: PRBS,
    #[doc = "0xe0 - Priority Register A for Slave"]
    pub pras12: PRAS,
    #[doc = "0xe4 - Priority Register B for Slave"]
    pub prbs12: PRBS,
    #[doc = "0xe8 - Priority Register A for Slave"]
    pub pras13: PRAS,
    #[doc = "0xec - Priority Register B for Slave"]
    pub prbs13: PRBS,
    #[doc = "0xf0 - Priority Register A for Slave"]
    pub pras14: PRAS,
    #[doc = "0xf4 - Priority Register B for Slave"]
    pub prbs14: PRBS,
    #[doc = "0xf8 - Priority Register A for Slave"]
    pub pras15: PRAS,
    #[doc = "0xfc - Priority Register B for Slave"]
    pub prbs15: PRBS,
    #[doc = "0x100 - Master Remap Control Register"]
    pub mrcr: MRCR,
    _reserved35: [u8; 12usize],
    #[doc = "0x110 - Special Function Register"]
    pub sfr: [SFR; 16],
}
#[doc = "Master Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfg](mcfg) module"]
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
#[doc = "Priority Register A for Slave\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras](pras) module"]
pub type PRAS = crate::Reg<u32, _PRAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS;
#[doc = "`read()` method returns [pras::R](pras::R) reader structure"]
impl crate::Readable for PRAS {}
#[doc = "`write(|w| ..)` method takes [pras::W](pras::W) writer structure"]
impl crate::Writable for PRAS {}
#[doc = "Priority Register A for Slave"]
pub mod pras;
#[doc = "Priority Register B for Slave\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs](prbs) module"]
pub type PRBS = crate::Reg<u32, _PRBS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS;
#[doc = "`read()` method returns [prbs::R](prbs::R) reader structure"]
impl crate::Readable for PRBS {}
#[doc = "`write(|w| ..)` method takes [prbs::W](prbs::W) writer structure"]
impl crate::Writable for PRBS {}
#[doc = "Priority Register B for Slave"]
pub mod prbs;
#[doc = "Slave Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfg](scfg) module"]
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
#[doc = "Special Function Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfr](sfr) module"]
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
