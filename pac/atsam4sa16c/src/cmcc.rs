#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache Controller Type Register"]
    pub type_: TYPE,
    #[doc = "0x04 - Cache Controller Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x08 - Cache Controller Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Cache Controller Status Register"]
    pub sr: SR,
    _reserved4: [u8; 16usize],
    #[doc = "0x20 - Cache Controller Maintenance Register 0"]
    pub maint0: MAINT0,
    #[doc = "0x24 - Cache Controller Maintenance Register 1"]
    pub maint1: MAINT1,
    #[doc = "0x28 - Cache Controller Monitor Configuration Register"]
    pub mcfg: MCFG,
    #[doc = "0x2c - Cache Controller Monitor Enable Register"]
    pub men: MEN,
    #[doc = "0x30 - Cache Controller Monitor Control Register"]
    pub mctrl: MCTRL,
    #[doc = "0x34 - Cache Controller Monitor Status Register"]
    pub msr: MSR,
}
#[doc = "Cache Controller Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [type_](type_) module"]
pub type TYPE = crate::Reg<u32, _TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TYPE;
#[doc = "`read()` method returns [type_::R](type_::R) reader structure"]
impl crate::Readable for TYPE {}
#[doc = "Cache Controller Type Register"]
pub mod type_;
#[doc = "Cache Controller Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Cache Controller Configuration Register"]
pub mod cfg;
#[doc = "Cache Controller Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Cache Controller Control Register"]
pub mod ctrl;
#[doc = "Cache Controller Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Cache Controller Status Register"]
pub mod sr;
#[doc = "Cache Controller Maintenance Register 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maint0](maint0) module"]
pub type MAINT0 = crate::Reg<u32, _MAINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAINT0;
#[doc = "`write(|w| ..)` method takes [maint0::W](maint0::W) writer structure"]
impl crate::Writable for MAINT0 {}
#[doc = "Cache Controller Maintenance Register 0"]
pub mod maint0;
#[doc = "Cache Controller Maintenance Register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maint1](maint1) module"]
pub type MAINT1 = crate::Reg<u32, _MAINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAINT1;
#[doc = "`write(|w| ..)` method takes [maint1::W](maint1::W) writer structure"]
impl crate::Writable for MAINT1 {}
#[doc = "Cache Controller Maintenance Register 1"]
pub mod maint1;
#[doc = "Cache Controller Monitor Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfg](mcfg) module"]
pub type MCFG = crate::Reg<u32, _MCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCFG;
#[doc = "`read()` method returns [mcfg::R](mcfg::R) reader structure"]
impl crate::Readable for MCFG {}
#[doc = "`write(|w| ..)` method takes [mcfg::W](mcfg::W) writer structure"]
impl crate::Writable for MCFG {}
#[doc = "Cache Controller Monitor Configuration Register"]
pub mod mcfg;
#[doc = "Cache Controller Monitor Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [men](men) module"]
pub type MEN = crate::Reg<u32, _MEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEN;
#[doc = "`read()` method returns [men::R](men::R) reader structure"]
impl crate::Readable for MEN {}
#[doc = "`write(|w| ..)` method takes [men::W](men::W) writer structure"]
impl crate::Writable for MEN {}
#[doc = "Cache Controller Monitor Enable Register"]
pub mod men;
#[doc = "Cache Controller Monitor Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctrl](mctrl) module"]
pub type MCTRL = crate::Reg<u32, _MCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTRL;
#[doc = "`write(|w| ..)` method takes [mctrl::W](mctrl::W) writer structure"]
impl crate::Writable for MCTRL {}
#[doc = "Cache Controller Monitor Control Register"]
pub mod mctrl;
#[doc = "Cache Controller Monitor Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](msr) module"]
pub type MSR = crate::Reg<u32, _MSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR;
#[doc = "`read()` method returns [msr::R](msr::R) reader structure"]
impl crate::Readable for MSR {}
#[doc = "Cache Controller Monitor Status Register"]
pub mod msr;
