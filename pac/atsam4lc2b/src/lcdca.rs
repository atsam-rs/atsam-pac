#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x08 - Timing Register"]
    pub tim: TIM,
    #[doc = "0x0c - Status Register"]
    pub sr: SR,
    #[doc = "0x10 - Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x14 - Data Register Low 0"]
    pub drl0: DRL0,
    #[doc = "0x18 - Data Register High 0"]
    pub drh0: DRH0,
    #[doc = "0x1c - Data Register Low 1"]
    pub drl1: DRL1,
    #[doc = "0x20 - Data Register High 1"]
    pub drh1: DRH1,
    #[doc = "0x24 - Data Register Low 2"]
    pub drl2: DRL2,
    #[doc = "0x28 - Data Register High 2"]
    pub drh2: DRH2,
    #[doc = "0x2c - Data Register Low 3"]
    pub drl3: DRL3,
    #[doc = "0x30 - Data Register High 3"]
    pub drh3: DRH3,
    #[doc = "0x34 - Indirect Access Data Register"]
    pub iadr: IADR,
    #[doc = "0x38 - Blink Configuration Register"]
    pub bcfg: BCFG,
    #[doc = "0x3c - Circular Shift Register Configuration"]
    pub csrcfg: CSRCFG,
    #[doc = "0x40 - Character Mapping Configuration Register"]
    pub cmcfg: CMCFG,
    #[doc = "0x44 - Character Mapping Data Register"]
    pub cmdr: CMDR,
    #[doc = "0x48 - Automated Character Mapping Configuration Register"]
    pub acmcfg: ACMCFG,
    #[doc = "0x4c - Automated Character Mapping Data Register"]
    pub acmdr: ACMDR,
    #[doc = "0x50 - Automated Bit Mapping Configuration Register"]
    pub abmcfg: ABMCFG,
    #[doc = "0x54 - Automated Bit Mapping Data Register"]
    pub abmdr: ABMDR,
    #[doc = "0x58 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x5c - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x60 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x64 - Version Register"]
    pub version: VERSION,
}
#[doc = "Automated Bit Mapping Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abmcfg](abmcfg) module"]
pub type ABMCFG = crate::Reg<u32, _ABMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ABMCFG;
#[doc = "`read()` method returns [abmcfg::R](abmcfg::R) reader structure"]
impl crate::Readable for ABMCFG {}
#[doc = "`write(|w| ..)` method takes [abmcfg::W](abmcfg::W) writer structure"]
impl crate::Writable for ABMCFG {}
#[doc = "Automated Bit Mapping Configuration Register"]
pub mod abmcfg;
#[doc = "Automated Bit Mapping Data Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abmdr](abmdr) module"]
pub type ABMDR = crate::Reg<u32, _ABMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ABMDR;
#[doc = "`write(|w| ..)` method takes [abmdr::W](abmdr::W) writer structure"]
impl crate::Writable for ABMDR {}
#[doc = "Automated Bit Mapping Data Register"]
pub mod abmdr;
#[doc = "Automated Character Mapping Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmcfg](acmcfg) module"]
pub type ACMCFG = crate::Reg<u32, _ACMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACMCFG;
#[doc = "`read()` method returns [acmcfg::R](acmcfg::R) reader structure"]
impl crate::Readable for ACMCFG {}
#[doc = "`write(|w| ..)` method takes [acmcfg::W](acmcfg::W) writer structure"]
impl crate::Writable for ACMCFG {}
#[doc = "Automated Character Mapping Configuration Register"]
pub mod acmcfg;
#[doc = "Automated Character Mapping Data Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmdr](acmdr) module"]
pub type ACMDR = crate::Reg<u32, _ACMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACMDR;
#[doc = "`write(|w| ..)` method takes [acmdr::W](acmdr::W) writer structure"]
impl crate::Writable for ACMDR {}
#[doc = "Automated Character Mapping Data Register"]
pub mod acmdr;
#[doc = "Blink Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcfg](bcfg) module"]
pub type BCFG = crate::Reg<u32, _BCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCFG;
#[doc = "`read()` method returns [bcfg::R](bcfg::R) reader structure"]
impl crate::Readable for BCFG {}
#[doc = "`write(|w| ..)` method takes [bcfg::W](bcfg::W) writer structure"]
impl crate::Writable for BCFG {}
#[doc = "Blink Configuration Register"]
pub mod bcfg;
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "Character Mapping Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmcfg](cmcfg) module"]
pub type CMCFG = crate::Reg<u32, _CMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMCFG;
#[doc = "`read()` method returns [cmcfg::R](cmcfg::R) reader structure"]
impl crate::Readable for CMCFG {}
#[doc = "`write(|w| ..)` method takes [cmcfg::W](cmcfg::W) writer structure"]
impl crate::Writable for CMCFG {}
#[doc = "Character Mapping Configuration Register"]
pub mod cmcfg;
#[doc = "Character Mapping Data Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdr](cmdr) module"]
pub type CMDR = crate::Reg<u32, _CMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDR;
#[doc = "`write(|w| ..)` method takes [cmdr::W](cmdr::W) writer structure"]
impl crate::Writable for CMDR {}
#[doc = "Character Mapping Data Register"]
pub mod cmdr;
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Circular Shift Register Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csrcfg](csrcfg) module"]
pub type CSRCFG = crate::Reg<u32, _CSRCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSRCFG;
#[doc = "`read()` method returns [csrcfg::R](csrcfg::R) reader structure"]
impl crate::Readable for CSRCFG {}
#[doc = "`write(|w| ..)` method takes [csrcfg::W](csrcfg::W) writer structure"]
impl crate::Writable for CSRCFG {}
#[doc = "Circular Shift Register Configuration"]
pub mod csrcfg;
#[doc = "Data Register High 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drh0](drh0) module"]
pub type DRH0 = crate::Reg<u32, _DRH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DRH0;
#[doc = "`read()` method returns [drh0::R](drh0::R) reader structure"]
impl crate::Readable for DRH0 {}
#[doc = "`write(|w| ..)` method takes [drh0::W](drh0::W) writer structure"]
impl crate::Writable for DRH0 {}
#[doc = "Data Register High 0"]
pub mod drh0;
#[doc = "Data Register High 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drh1](drh1) module"]
pub type DRH1 = crate::Reg<u32, _DRH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DRH1;
#[doc = "`read()` method returns [drh1::R](drh1::R) reader structure"]
impl crate::Readable for DRH1 {}
#[doc = "`write(|w| ..)` method takes [drh1::W](drh1::W) writer structure"]
impl crate::Writable for DRH1 {}
#[doc = "Data Register High 1"]
pub mod drh1;
#[doc = "Data Register High 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drh2](drh2) module"]
pub type DRH2 = crate::Reg<u32, _DRH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DRH2;
#[doc = "`read()` method returns [drh2::R](drh2::R) reader structure"]
impl crate::Readable for DRH2 {}
#[doc = "`write(|w| ..)` method takes [drh2::W](drh2::W) writer structure"]
impl crate::Writable for DRH2 {}
#[doc = "Data Register High 2"]
pub mod drh2;
#[doc = "Data Register High 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drh3](drh3) module"]
pub type DRH3 = crate::Reg<u32, _DRH3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DRH3;
#[doc = "`read()` method returns [drh3::R](drh3::R) reader structure"]
impl crate::Readable for DRH3 {}
#[doc = "`write(|w| ..)` method takes [drh3::W](drh3::W) writer structure"]
impl crate::Writable for DRH3 {}
#[doc = "Data Register High 3"]
pub mod drh3;
#[doc = "Data Register Low 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drl0](drl0) module"]
pub type DRL0 = crate::Reg<u32, _DRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DRL0;
#[doc = "`read()` method returns [drl0::R](drl0::R) reader structure"]
impl crate::Readable for DRL0 {}
#[doc = "`write(|w| ..)` method takes [drl0::W](drl0::W) writer structure"]
impl crate::Writable for DRL0 {}
#[doc = "Data Register Low 0"]
pub mod drl0;
#[doc = "Data Register Low 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drl1](drl1) module"]
pub type DRL1 = crate::Reg<u32, _DRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DRL1;
#[doc = "`read()` method returns [drl1::R](drl1::R) reader structure"]
impl crate::Readable for DRL1 {}
#[doc = "`write(|w| ..)` method takes [drl1::W](drl1::W) writer structure"]
impl crate::Writable for DRL1 {}
#[doc = "Data Register Low 1"]
pub mod drl1;
#[doc = "Data Register Low 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drl2](drl2) module"]
pub type DRL2 = crate::Reg<u32, _DRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DRL2;
#[doc = "`read()` method returns [drl2::R](drl2::R) reader structure"]
impl crate::Readable for DRL2 {}
#[doc = "`write(|w| ..)` method takes [drl2::W](drl2::W) writer structure"]
impl crate::Writable for DRL2 {}
#[doc = "Data Register Low 2"]
pub mod drl2;
#[doc = "Data Register Low 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drl3](drl3) module"]
pub type DRL3 = crate::Reg<u32, _DRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DRL3;
#[doc = "`read()` method returns [drl3::R](drl3::R) reader structure"]
impl crate::Readable for DRL3 {}
#[doc = "`write(|w| ..)` method takes [drl3::W](drl3::W) writer structure"]
impl crate::Writable for DRL3 {}
#[doc = "Data Register Low 3"]
pub mod drl3;
#[doc = "Indirect Access Data Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iadr](iadr) module"]
pub type IADR = crate::Reg<u32, _IADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IADR;
#[doc = "`write(|w| ..)` method takes [iadr::W](iadr::W) writer structure"]
impl crate::Writable for IADR {}
#[doc = "Indirect Access Data Register"]
pub mod iadr;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "Status Clear Register"]
pub mod scr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim](tim) module"]
pub type TIM = crate::Reg<u32, _TIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM;
#[doc = "`read()` method returns [tim::R](tim::R) reader structure"]
impl crate::Readable for TIM {}
#[doc = "`write(|w| ..)` method takes [tim::W](tim::W) writer structure"]
impl crate::Writable for TIM {}
#[doc = "Timing Register"]
pub mod tim;
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "Version Register"]
pub mod version;
