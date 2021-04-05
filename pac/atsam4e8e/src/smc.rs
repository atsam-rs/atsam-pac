#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SMC Setup Register (CS_number = 0)"]
    pub setup0: SETUP0,
    #[doc = "0x04 - SMC Pulse Register (CS_number = 0)"]
    pub pulse0: PULSE0,
    #[doc = "0x08 - SMC Cycle Register (CS_number = 0)"]
    pub cycle0: CYCLE0,
    #[doc = "0x0c - SMC Mode Register (CS_number = 0)"]
    pub mode0: MODE0,
    #[doc = "0x10 - SMC Setup Register (CS_number = 1)"]
    pub setup1: SETUP1,
    #[doc = "0x14 - SMC Pulse Register (CS_number = 1)"]
    pub pulse1: PULSE1,
    #[doc = "0x18 - SMC Cycle Register (CS_number = 1)"]
    pub cycle1: CYCLE1,
    #[doc = "0x1c - SMC Mode Register (CS_number = 1)"]
    pub mode1: MODE1,
    #[doc = "0x20 - SMC Setup Register (CS_number = 2)"]
    pub setup2: SETUP2,
    #[doc = "0x24 - SMC Pulse Register (CS_number = 2)"]
    pub pulse2: PULSE2,
    #[doc = "0x28 - SMC Cycle Register (CS_number = 2)"]
    pub cycle2: CYCLE2,
    #[doc = "0x2c - SMC Mode Register (CS_number = 2)"]
    pub mode2: MODE2,
    #[doc = "0x30 - SMC Setup Register (CS_number = 3)"]
    pub setup3: SETUP3,
    #[doc = "0x34 - SMC Pulse Register (CS_number = 3)"]
    pub pulse3: PULSE3,
    #[doc = "0x38 - SMC Cycle Register (CS_number = 3)"]
    pub cycle3: CYCLE3,
    #[doc = "0x3c - SMC Mode Register (CS_number = 3)"]
    pub mode3: MODE3,
    _reserved16: [u8; 64usize],
    #[doc = "0x80 - SMC OCMS MODE Register"]
    pub ocms: OCMS,
    #[doc = "0x84 - SMC OCMS KEY1 Register"]
    pub key1: KEY1,
    #[doc = "0x88 - SMC OCMS KEY2 Register"]
    pub key2: KEY2,
    _reserved19: [u8; 88usize],
    #[doc = "0xe4 - SMC Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - SMC Write Protect Status Register"]
    pub wpsr: WPSR,
}
#[doc = "SMC Setup Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup0](setup0) module"]
pub type SETUP0 = crate::Reg<u32, _SETUP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP0;
#[doc = "`read()` method returns [setup0::R](setup0::R) reader structure"]
impl crate::Readable for SETUP0 {}
#[doc = "`write(|w| ..)` method takes [setup0::W](setup0::W) writer structure"]
impl crate::Writable for SETUP0 {}
#[doc = "SMC Setup Register (CS_number = 0)"]
pub mod setup0;
#[doc = "SMC Pulse Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulse0](pulse0) module"]
pub type PULSE0 = crate::Reg<u32, _PULSE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PULSE0;
#[doc = "`read()` method returns [pulse0::R](pulse0::R) reader structure"]
impl crate::Readable for PULSE0 {}
#[doc = "`write(|w| ..)` method takes [pulse0::W](pulse0::W) writer structure"]
impl crate::Writable for PULSE0 {}
#[doc = "SMC Pulse Register (CS_number = 0)"]
pub mod pulse0;
#[doc = "SMC Cycle Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle0](cycle0) module"]
pub type CYCLE0 = crate::Reg<u32, _CYCLE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CYCLE0;
#[doc = "`read()` method returns [cycle0::R](cycle0::R) reader structure"]
impl crate::Readable for CYCLE0 {}
#[doc = "`write(|w| ..)` method takes [cycle0::W](cycle0::W) writer structure"]
impl crate::Writable for CYCLE0 {}
#[doc = "SMC Cycle Register (CS_number = 0)"]
pub mod cycle0;
#[doc = "SMC Mode Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode0](mode0) module"]
pub type MODE0 = crate::Reg<u32, _MODE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE0;
#[doc = "`read()` method returns [mode0::R](mode0::R) reader structure"]
impl crate::Readable for MODE0 {}
#[doc = "`write(|w| ..)` method takes [mode0::W](mode0::W) writer structure"]
impl crate::Writable for MODE0 {}
#[doc = "SMC Mode Register (CS_number = 0)"]
pub mod mode0;
#[doc = "SMC Setup Register (CS_number = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup1](setup1) module"]
pub type SETUP1 = crate::Reg<u32, _SETUP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP1;
#[doc = "`read()` method returns [setup1::R](setup1::R) reader structure"]
impl crate::Readable for SETUP1 {}
#[doc = "`write(|w| ..)` method takes [setup1::W](setup1::W) writer structure"]
impl crate::Writable for SETUP1 {}
#[doc = "SMC Setup Register (CS_number = 1)"]
pub mod setup1;
#[doc = "SMC Pulse Register (CS_number = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulse1](pulse1) module"]
pub type PULSE1 = crate::Reg<u32, _PULSE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PULSE1;
#[doc = "`read()` method returns [pulse1::R](pulse1::R) reader structure"]
impl crate::Readable for PULSE1 {}
#[doc = "`write(|w| ..)` method takes [pulse1::W](pulse1::W) writer structure"]
impl crate::Writable for PULSE1 {}
#[doc = "SMC Pulse Register (CS_number = 1)"]
pub mod pulse1;
#[doc = "SMC Cycle Register (CS_number = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle1](cycle1) module"]
pub type CYCLE1 = crate::Reg<u32, _CYCLE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CYCLE1;
#[doc = "`read()` method returns [cycle1::R](cycle1::R) reader structure"]
impl crate::Readable for CYCLE1 {}
#[doc = "`write(|w| ..)` method takes [cycle1::W](cycle1::W) writer structure"]
impl crate::Writable for CYCLE1 {}
#[doc = "SMC Cycle Register (CS_number = 1)"]
pub mod cycle1;
#[doc = "SMC Mode Register (CS_number = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode1](mode1) module"]
pub type MODE1 = crate::Reg<u32, _MODE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE1;
#[doc = "`read()` method returns [mode1::R](mode1::R) reader structure"]
impl crate::Readable for MODE1 {}
#[doc = "`write(|w| ..)` method takes [mode1::W](mode1::W) writer structure"]
impl crate::Writable for MODE1 {}
#[doc = "SMC Mode Register (CS_number = 1)"]
pub mod mode1;
#[doc = "SMC Setup Register (CS_number = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup2](setup2) module"]
pub type SETUP2 = crate::Reg<u32, _SETUP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP2;
#[doc = "`read()` method returns [setup2::R](setup2::R) reader structure"]
impl crate::Readable for SETUP2 {}
#[doc = "`write(|w| ..)` method takes [setup2::W](setup2::W) writer structure"]
impl crate::Writable for SETUP2 {}
#[doc = "SMC Setup Register (CS_number = 2)"]
pub mod setup2;
#[doc = "SMC Pulse Register (CS_number = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulse2](pulse2) module"]
pub type PULSE2 = crate::Reg<u32, _PULSE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PULSE2;
#[doc = "`read()` method returns [pulse2::R](pulse2::R) reader structure"]
impl crate::Readable for PULSE2 {}
#[doc = "`write(|w| ..)` method takes [pulse2::W](pulse2::W) writer structure"]
impl crate::Writable for PULSE2 {}
#[doc = "SMC Pulse Register (CS_number = 2)"]
pub mod pulse2;
#[doc = "SMC Cycle Register (CS_number = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle2](cycle2) module"]
pub type CYCLE2 = crate::Reg<u32, _CYCLE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CYCLE2;
#[doc = "`read()` method returns [cycle2::R](cycle2::R) reader structure"]
impl crate::Readable for CYCLE2 {}
#[doc = "`write(|w| ..)` method takes [cycle2::W](cycle2::W) writer structure"]
impl crate::Writable for CYCLE2 {}
#[doc = "SMC Cycle Register (CS_number = 2)"]
pub mod cycle2;
#[doc = "SMC Mode Register (CS_number = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode2](mode2) module"]
pub type MODE2 = crate::Reg<u32, _MODE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE2;
#[doc = "`read()` method returns [mode2::R](mode2::R) reader structure"]
impl crate::Readable for MODE2 {}
#[doc = "`write(|w| ..)` method takes [mode2::W](mode2::W) writer structure"]
impl crate::Writable for MODE2 {}
#[doc = "SMC Mode Register (CS_number = 2)"]
pub mod mode2;
#[doc = "SMC Setup Register (CS_number = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup3](setup3) module"]
pub type SETUP3 = crate::Reg<u32, _SETUP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP3;
#[doc = "`read()` method returns [setup3::R](setup3::R) reader structure"]
impl crate::Readable for SETUP3 {}
#[doc = "`write(|w| ..)` method takes [setup3::W](setup3::W) writer structure"]
impl crate::Writable for SETUP3 {}
#[doc = "SMC Setup Register (CS_number = 3)"]
pub mod setup3;
#[doc = "SMC Pulse Register (CS_number = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulse3](pulse3) module"]
pub type PULSE3 = crate::Reg<u32, _PULSE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PULSE3;
#[doc = "`read()` method returns [pulse3::R](pulse3::R) reader structure"]
impl crate::Readable for PULSE3 {}
#[doc = "`write(|w| ..)` method takes [pulse3::W](pulse3::W) writer structure"]
impl crate::Writable for PULSE3 {}
#[doc = "SMC Pulse Register (CS_number = 3)"]
pub mod pulse3;
#[doc = "SMC Cycle Register (CS_number = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle3](cycle3) module"]
pub type CYCLE3 = crate::Reg<u32, _CYCLE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CYCLE3;
#[doc = "`read()` method returns [cycle3::R](cycle3::R) reader structure"]
impl crate::Readable for CYCLE3 {}
#[doc = "`write(|w| ..)` method takes [cycle3::W](cycle3::W) writer structure"]
impl crate::Writable for CYCLE3 {}
#[doc = "SMC Cycle Register (CS_number = 3)"]
pub mod cycle3;
#[doc = "SMC Mode Register (CS_number = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode3](mode3) module"]
pub type MODE3 = crate::Reg<u32, _MODE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE3;
#[doc = "`read()` method returns [mode3::R](mode3::R) reader structure"]
impl crate::Readable for MODE3 {}
#[doc = "`write(|w| ..)` method takes [mode3::W](mode3::W) writer structure"]
impl crate::Writable for MODE3 {}
#[doc = "SMC Mode Register (CS_number = 3)"]
pub mod mode3;
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
#[doc = "SMC Write Protect Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpmr](wpmr) module"]
pub type WPMR = crate::Reg<u32, _WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPMR;
#[doc = "`read()` method returns [wpmr::R](wpmr::R) reader structure"]
impl crate::Readable for WPMR {}
#[doc = "`write(|w| ..)` method takes [wpmr::W](wpmr::W) writer structure"]
impl crate::Writable for WPMR {}
#[doc = "SMC Write Protect Mode Register"]
pub mod wpmr;
#[doc = "SMC Write Protect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](wpsr) module"]
pub type WPSR = crate::Reg<u32, _WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSR;
#[doc = "`read()` method returns [wpsr::R](wpsr::R) reader structure"]
impl crate::Readable for WPSR {}
#[doc = "SMC Write Protect Status Register"]
pub mod wpsr;
