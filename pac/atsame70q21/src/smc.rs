#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SMC Setup Register (CS_number = 0)"]
    pub smc_cs_number: [SMC_CS_NUMBER; 4],
    _reserved1: [u8; 64usize],
    #[doc = "0x80 - SMC OCMS MODE Register"]
    pub ocms: OCMS,
    #[doc = "0x84 - SMC OCMS KEY1 Register"]
    pub key1: KEY1,
    #[doc = "0x88 - SMC OCMS KEY2 Register"]
    pub key2: KEY2,
    _reserved4: [u8; 88usize],
    #[doc = "0xe4 - SMC Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - SMC Write Protection Status Register"]
    pub wpsr: WPSR,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SMC_CS_NUMBER {
    #[doc = "0x00 - SMC Setup Register (CS_number = 0)"]
    pub setup: self::smc_cs_number::SETUP,
    #[doc = "0x04 - SMC Pulse Register (CS_number = 0)"]
    pub pulse: self::smc_cs_number::PULSE,
    #[doc = "0x08 - SMC Cycle Register (CS_number = 0)"]
    pub cycle: self::smc_cs_number::CYCLE,
    #[doc = "0x0c - SMC MODE Register (CS_number = 0)"]
    pub mode: self::smc_cs_number::MODE,
}
#[doc = r"Register block"]
#[doc = "SMC Setup Register (CS_number = 0)"]
pub mod smc_cs_number;
#[doc = "SMC OCMS MODE Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocms](ocms) module"]
pub type OCMS = crate::Reg<u32, _OCMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCMS;
#[doc = "`read()` method returns [ocms::R](ocms::R) reader structure"]
impl crate::Readable for OCMS {}
#[doc = "`write(|w| ..)` method takes [ocms::W](ocms::W) writer structure"]
impl crate::Writable for OCMS {}
#[doc = "SMC OCMS MODE Register"]
pub mod ocms;
#[doc = "SMC OCMS KEY1 Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key1](key1) module"]
pub type KEY1 = crate::Reg<u32, _KEY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY1;
#[doc = "`write(|w| ..)` method takes [key1::W](key1::W) writer structure"]
impl crate::Writable for KEY1 {}
#[doc = "SMC OCMS KEY1 Register"]
pub mod key1;
#[doc = "SMC OCMS KEY2 Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key2](key2) module"]
pub type KEY2 = crate::Reg<u32, _KEY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY2;
#[doc = "`write(|w| ..)` method takes [key2::W](key2::W) writer structure"]
impl crate::Writable for KEY2 {}
#[doc = "SMC OCMS KEY2 Register"]
pub mod key2;
#[doc = "SMC Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpmr](wpmr) module"]
pub type WPMR = crate::Reg<u32, _WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPMR;
#[doc = "`read()` method returns [wpmr::R](wpmr::R) reader structure"]
impl crate::Readable for WPMR {}
#[doc = "`write(|w| ..)` method takes [wpmr::W](wpmr::W) writer structure"]
impl crate::Writable for WPMR {}
#[doc = "SMC Write Protection Mode Register"]
pub mod wpmr;
#[doc = "SMC Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](wpsr) module"]
pub type WPSR = crate::Reg<u32, _WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSR;
#[doc = "`read()` method returns [wpsr::R](wpsr::R) reader structure"]
impl crate::Readable for WPSR {}
#[doc = "SMC Write Protection Status Register"]
pub mod wpsr;
