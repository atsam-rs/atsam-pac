#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x08 - Status Register"]
    pub sr: SR,
    #[doc = "0x0c - Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x10 - Resistive Touch Screen Register"]
    pub rts: RTS,
    #[doc = "0x14 - Sequencer Configuration Register"]
    pub seqcfg: SEQCFG,
    _reserved_6_cdma: [u8; 4usize],
    #[doc = "0x1c - Timing Configuration Register"]
    pub tim: TIM,
    #[doc = "0x20 - Internal Timer Register"]
    pub itimer: ITIMER,
    #[doc = "0x24 - Window Monitor Configuration Register"]
    pub wcfg: WCFG,
    #[doc = "0x28 - Window Monitor Threshold Configuration Register"]
    pub wth: WTH,
    #[doc = "0x2c - Sequencer Last Converted Value Register"]
    pub lcv: LCV,
    #[doc = "0x30 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x34 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x38 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x3c - Calibration Register"]
    pub calib: CALIB,
    #[doc = "0x40 - Version Register"]
    pub version: VERSION,
    #[doc = "0x44 - Parameter Register"]
    pub parameter: PARAMETER,
}
impl RegisterBlock {
    #[doc = "0x18 - Configuration Direct Memory Access Register"]
    #[inline(always)]
    pub fn cdma_alt(&self) -> &CDMA_ALT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CDMA_ALT) }
    }
    #[doc = "0x18 - Configuration Direct Memory Access Register"]
    #[inline(always)]
    pub fn cdma_alt_mut(&self) -> &mut CDMA_ALT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut CDMA_ALT) }
    }
    #[doc = "0x18 - Configuration Direct Memory Access Register"]
    #[inline(always)]
    pub fn cdma(&self) -> &CDMA {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CDMA) }
    }
    #[doc = "0x18 - Configuration Direct Memory Access Register"]
    #[inline(always)]
    pub fn cdma_mut(&self) -> &mut CDMA {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut CDMA) }
    }
}
#[doc = "Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calib](calib) module"]
pub type CALIB = crate::Reg<u32, _CALIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALIB;
#[doc = "`read()` method returns [calib::R](calib::R) reader structure"]
impl crate::Readable for CALIB {}
#[doc = "`write(|w| ..)` method takes [calib::W](calib::W) writer structure"]
impl crate::Writable for CALIB {}
#[doc = "Calibration Register"]
pub mod calib;
#[doc = "Configuration Direct Memory Access Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdma](cdma) module"]
pub type CDMA = crate::Reg<u32, _CDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDMA;
#[doc = "`write(|w| ..)` method takes [cdma::W](cdma::W) writer structure"]
impl crate::Writable for CDMA {}
#[doc = "Configuration Direct Memory Access Register"]
pub mod cdma;
#[doc = "Configuration Direct Memory Access Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdma_alt](cdma_alt) module"]
pub type CDMA_ALT = crate::Reg<u32, _CDMA_ALT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDMA_ALT;
#[doc = "`write(|w| ..)` method takes [cdma_alt::W](cdma_alt::W) writer structure"]
impl crate::Writable for CDMA_ALT {}
#[doc = "Configuration Direct Memory Access Register"]
pub mod cdma_alt;
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
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
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
#[doc = "Internal Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itimer](itimer) module"]
pub type ITIMER = crate::Reg<u32, _ITIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITIMER;
#[doc = "`read()` method returns [itimer::R](itimer::R) reader structure"]
impl crate::Readable for ITIMER {}
#[doc = "`write(|w| ..)` method takes [itimer::W](itimer::W) writer structure"]
impl crate::Writable for ITIMER {}
#[doc = "Internal Timer Register"]
pub mod itimer;
#[doc = "Sequencer Last Converted Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcv](lcv) module"]
pub type LCV = crate::Reg<u32, _LCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCV;
#[doc = "`read()` method returns [lcv::R](lcv::R) reader structure"]
impl crate::Readable for LCV {}
#[doc = "Sequencer Last Converted Value Register"]
pub mod lcv;
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [parameter](parameter) module"]
pub type PARAMETER = crate::Reg<u32, _PARAMETER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PARAMETER;
#[doc = "`read()` method returns [parameter::R](parameter::R) reader structure"]
impl crate::Readable for PARAMETER {}
#[doc = "Parameter Register"]
pub mod parameter;
#[doc = "Resistive Touch Screen Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rts](rts) module"]
pub type RTS = crate::Reg<u32, _RTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTS;
#[doc = "`read()` method returns [rts::R](rts::R) reader structure"]
impl crate::Readable for RTS {}
#[doc = "`write(|w| ..)` method takes [rts::W](rts::W) writer structure"]
impl crate::Writable for RTS {}
#[doc = "Resistive Touch Screen Register"]
pub mod rts;
#[doc = "Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "Status Clear Register"]
pub mod scr;
#[doc = "Sequencer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqcfg](seqcfg) module"]
pub type SEQCFG = crate::Reg<u32, _SEQCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQCFG;
#[doc = "`read()` method returns [seqcfg::R](seqcfg::R) reader structure"]
impl crate::Readable for SEQCFG {}
#[doc = "`write(|w| ..)` method takes [seqcfg::W](seqcfg::W) writer structure"]
impl crate::Writable for SEQCFG {}
#[doc = "Sequencer Configuration Register"]
pub mod seqcfg;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Timing Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim](tim) module"]
pub type TIM = crate::Reg<u32, _TIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM;
#[doc = "`read()` method returns [tim::R](tim::R) reader structure"]
impl crate::Readable for TIM {}
#[doc = "`write(|w| ..)` method takes [tim::W](tim::W) writer structure"]
impl crate::Writable for TIM {}
#[doc = "Timing Configuration Register"]
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
#[doc = "Window Monitor Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcfg](wcfg) module"]
pub type WCFG = crate::Reg<u32, _WCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WCFG;
#[doc = "`read()` method returns [wcfg::R](wcfg::R) reader structure"]
impl crate::Readable for WCFG {}
#[doc = "`write(|w| ..)` method takes [wcfg::W](wcfg::W) writer structure"]
impl crate::Writable for WCFG {}
#[doc = "Window Monitor Configuration Register"]
pub mod wcfg;
#[doc = "Window Monitor Threshold Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wth](wth) module"]
pub type WTH = crate::Reg<u32, _WTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTH;
#[doc = "`read()` method returns [wth::R](wth::R) reader structure"]
impl crate::Readable for WTH {}
#[doc = "`write(|w| ..)` method takes [wth::W](wth::W) writer structure"]
impl crate::Writable for WTH {}
#[doc = "Window Monitor Threshold Configuration Register"]
pub mod wth;
