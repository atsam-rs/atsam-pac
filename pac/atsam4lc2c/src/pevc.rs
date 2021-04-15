#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Status Register"]
    pub chsr: CHSR,
    #[doc = "0x04 - Channel Enable Register"]
    pub cher: CHER,
    #[doc = "0x08 - Channel Disable Register"]
    pub chdr: CHDR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Software Event"]
    pub sev: SEV,
    #[doc = "0x14 - Channel / User Busy"]
    pub busy: BUSY,
    _reserved5: [u8; 8usize],
    #[doc = "0x20 - Trigger Interrupt Mask Enable Register"]
    pub trier: TRIER,
    #[doc = "0x24 - Trigger Interrupt Mask Disable Register"]
    pub tridr: TRIDR,
    #[doc = "0x28 - Trigger Interrupt Mask Register"]
    pub trimr: TRIMR,
    _reserved8: [u8; 4usize],
    #[doc = "0x30 - Trigger Status Register"]
    pub trsr: TRSR,
    #[doc = "0x34 - Trigger Status Clear Register"]
    pub trscr: TRSCR,
    _reserved10: [u8; 8usize],
    #[doc = "0x40 - Overrun Interrupt Mask Enable Register"]
    pub ovier: OVIER,
    #[doc = "0x44 - Overrun Interrupt Mask Disable Register"]
    pub ovidr: OVIDR,
    #[doc = "0x48 - Overrun Interrupt Mask Register"]
    pub ovimr: OVIMR,
    _reserved13: [u8; 4usize],
    #[doc = "0x50 - Overrun Status Register"]
    pub ovsr: OVSR,
    #[doc = "0x54 - Overrun Status Clear Register"]
    pub ovscr: OVSCR,
    _reserved15: [u8; 168usize],
    #[doc = "0x100 - Channel Multiplexer"]
    pub chmx: [CHMX; 19],
    _reserved16: [u8; 180usize],
    #[doc = "0x200 - Event Shaper"]
    pub evs: [EVS; 31],
    _reserved17: [u8; 132usize],
    #[doc = "0x300 - Input Glitch Filter Divider Register"]
    pub igfdr: IGFDR,
    _reserved18: [u8; 244usize],
    #[doc = "0x3f8 - Parameter"]
    pub parameter: PARAMETER,
    #[doc = "0x3fc - Version"]
    pub version: VERSION,
}
#[doc = "Channel / User Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busy](busy) module"]
pub type BUSY = crate::Reg<u32, _BUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSY;
#[doc = "`read()` method returns [busy::R](busy::R) reader structure"]
impl crate::Readable for BUSY {}
#[doc = "Channel / User Busy"]
pub mod busy;
#[doc = "Channel Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdr](chdr) module"]
pub type CHDR = crate::Reg<u32, _CHDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDR;
#[doc = "`write(|w| ..)` method takes [chdr::W](chdr::W) writer structure"]
impl crate::Writable for CHDR {}
#[doc = "Channel Disable Register"]
pub mod chdr;
#[doc = "Channel Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cher](cher) module"]
pub type CHER = crate::Reg<u32, _CHER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHER;
#[doc = "`write(|w| ..)` method takes [cher::W](cher::W) writer structure"]
impl crate::Writable for CHER {}
#[doc = "Channel Enable Register"]
pub mod cher;
#[doc = "Channel Multiplexer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmx](chmx) module"]
pub type CHMX = crate::Reg<u32, _CHMX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHMX;
#[doc = "`read()` method returns [chmx::R](chmx::R) reader structure"]
impl crate::Readable for CHMX {}
#[doc = "`write(|w| ..)` method takes [chmx::W](chmx::W) writer structure"]
impl crate::Writable for CHMX {}
#[doc = "Channel Multiplexer"]
pub mod chmx;
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chsr](chsr) module"]
pub type CHSR = crate::Reg<u32, _CHSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHSR;
#[doc = "`read()` method returns [chsr::R](chsr::R) reader structure"]
impl crate::Readable for CHSR {}
#[doc = "Channel Status Register"]
pub mod chsr;
#[doc = "Event Shaper\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evs](evs) module"]
pub type EVS = crate::Reg<u32, _EVS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVS;
#[doc = "`read()` method returns [evs::R](evs::R) reader structure"]
impl crate::Readable for EVS {}
#[doc = "`write(|w| ..)` method takes [evs::W](evs::W) writer structure"]
impl crate::Writable for EVS {}
#[doc = "Event Shaper"]
pub mod evs;
#[doc = "Input Glitch Filter Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [igfdr](igfdr) module"]
pub type IGFDR = crate::Reg<u32, _IGFDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IGFDR;
#[doc = "`read()` method returns [igfdr::R](igfdr::R) reader structure"]
impl crate::Readable for IGFDR {}
#[doc = "`write(|w| ..)` method takes [igfdr::W](igfdr::W) writer structure"]
impl crate::Writable for IGFDR {}
#[doc = "Input Glitch Filter Divider Register"]
pub mod igfdr;
#[doc = "Overrun Interrupt Mask Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovidr](ovidr) module"]
pub type OVIDR = crate::Reg<u32, _OVIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OVIDR;
#[doc = "`write(|w| ..)` method takes [ovidr::W](ovidr::W) writer structure"]
impl crate::Writable for OVIDR {}
#[doc = "Overrun Interrupt Mask Disable Register"]
pub mod ovidr;
#[doc = "Overrun Interrupt Mask Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovier](ovier) module"]
pub type OVIER = crate::Reg<u32, _OVIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OVIER;
#[doc = "`write(|w| ..)` method takes [ovier::W](ovier::W) writer structure"]
impl crate::Writable for OVIER {}
#[doc = "Overrun Interrupt Mask Enable Register"]
pub mod ovier;
#[doc = "Overrun Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovimr](ovimr) module"]
pub type OVIMR = crate::Reg<u32, _OVIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OVIMR;
#[doc = "`read()` method returns [ovimr::R](ovimr::R) reader structure"]
impl crate::Readable for OVIMR {}
#[doc = "Overrun Interrupt Mask Register"]
pub mod ovimr;
#[doc = "Overrun Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovscr](ovscr) module"]
pub type OVSCR = crate::Reg<u32, _OVSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OVSCR;
#[doc = "`write(|w| ..)` method takes [ovscr::W](ovscr::W) writer structure"]
impl crate::Writable for OVSCR {}
#[doc = "Overrun Status Clear Register"]
pub mod ovscr;
#[doc = "Overrun Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovsr](ovsr) module"]
pub type OVSR = crate::Reg<u32, _OVSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OVSR;
#[doc = "`read()` method returns [ovsr::R](ovsr::R) reader structure"]
impl crate::Readable for OVSR {}
#[doc = "Overrun Status Register"]
pub mod ovsr;
#[doc = "Parameter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [parameter](parameter) module"]
pub type PARAMETER = crate::Reg<u32, _PARAMETER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PARAMETER;
#[doc = "`read()` method returns [parameter::R](parameter::R) reader structure"]
impl crate::Readable for PARAMETER {}
#[doc = "Parameter"]
pub mod parameter;
#[doc = "Software Event\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sev](sev) module"]
pub type SEV = crate::Reg<u32, _SEV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEV;
#[doc = "`write(|w| ..)` method takes [sev::W](sev::W) writer structure"]
impl crate::Writable for SEV {}
#[doc = "Software Event"]
pub mod sev;
#[doc = "Trigger Interrupt Mask Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tridr](tridr) module"]
pub type TRIDR = crate::Reg<u32, _TRIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIDR;
#[doc = "`write(|w| ..)` method takes [tridr::W](tridr::W) writer structure"]
impl crate::Writable for TRIDR {}
#[doc = "Trigger Interrupt Mask Disable Register"]
pub mod tridr;
#[doc = "Trigger Interrupt Mask Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trier](trier) module"]
pub type TRIER = crate::Reg<u32, _TRIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIER;
#[doc = "`write(|w| ..)` method takes [trier::W](trier::W) writer structure"]
impl crate::Writable for TRIER {}
#[doc = "Trigger Interrupt Mask Enable Register"]
pub mod trier;
#[doc = "Trigger Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trimr](trimr) module"]
pub type TRIMR = crate::Reg<u32, _TRIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIMR;
#[doc = "`read()` method returns [trimr::R](trimr::R) reader structure"]
impl crate::Readable for TRIMR {}
#[doc = "Trigger Interrupt Mask Register"]
pub mod trimr;
#[doc = "Trigger Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trscr](trscr) module"]
pub type TRSCR = crate::Reg<u32, _TRSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRSCR;
#[doc = "`write(|w| ..)` method takes [trscr::W](trscr::W) writer structure"]
impl crate::Writable for TRSCR {}
#[doc = "Trigger Status Clear Register"]
pub mod trscr;
#[doc = "Trigger Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trsr](trsr) module"]
pub type TRSR = crate::Reg<u32, _TRSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRSR;
#[doc = "`read()` method returns [trsr::R](trsr::R) reader structure"]
impl crate::Readable for TRSR {}
#[doc = "Trigger Status Register"]
pub mod trsr;
#[doc = "Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "Version"]
pub mod version;
