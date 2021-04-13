#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device General Control Register"]
    pub devctrl: DEVCTRL,
    #[doc = "0x04 - Device Global Interrupt Status Register"]
    pub devisr: DEVISR,
    #[doc = "0x08 - Device Global Interrupt Clear Register"]
    pub devicr: DEVICR,
    #[doc = "0x0c - Device Global Interrupt Set Register"]
    pub devifr: DEVIFR,
    #[doc = "0x10 - Device Global Interrupt Mask Register"]
    pub devimr: DEVIMR,
    #[doc = "0x14 - Device Global Interrupt Disable Register"]
    pub devidr: DEVIDR,
    #[doc = "0x18 - Device Global Interrupt Enable Register"]
    pub devier: DEVIER,
    #[doc = "0x1c - Device Endpoint Register"]
    pub devept: DEVEPT,
    #[doc = "0x20 - Device Frame Number Register"]
    pub devfnum: DEVFNUM,
    _reserved9: [u8; 220usize],
    #[doc = "0x100 - Device Endpoint Configuration Register (n = 0) 0"]
    pub deveptcfg: [DEVEPTCFG; 10],
    _reserved10: [u8; 8usize],
    #[doc = "0x130 - Device Endpoint Status Register (n = 0) 0"]
    pub deveptisr: [DEVEPTISR; 10],
    _reserved11: [u8; 8usize],
    #[doc = "0x160 - Device Endpoint Clear Register (n = 0) 0"]
    pub devepticr: [DEVEPTICR; 10],
    _reserved12: [u8; 8usize],
    #[doc = "0x190 - Device Endpoint Set Register (n = 0) 0"]
    pub deveptifr: [DEVEPTIFR; 10],
    _reserved13: [u8; 8usize],
    #[doc = "0x1c0 - Device Endpoint Mask Register (n = 0) 0"]
    pub deveptimr: [DEVEPTIMR; 10],
    _reserved14: [u8; 8usize],
    #[doc = "0x1f0 - Device Endpoint Enable Register (n = 0) 0"]
    pub deveptier: [DEVEPTIER; 10],
    _reserved15: [u8; 8usize],
    #[doc = "0x220 - Device Endpoint Disable Register (n = 0) 0"]
    pub deveptidr: [DEVEPTIDR; 10],
    _reserved16: [u8; 200usize],
    #[doc = "0x310 - Device DMA Channel Next Descriptor Address Register (n = 1)"]
    pub usbhs_devdma: [USBHS_DEVDMA; 7],
    _reserved17: [u8; 128usize],
    #[doc = "0x400 - Host General Control Register"]
    pub hstctrl: HSTCTRL,
    #[doc = "0x404 - Host Global Interrupt Status Register"]
    pub hstisr: HSTISR,
    #[doc = "0x408 - Host Global Interrupt Clear Register"]
    pub hsticr: HSTICR,
    #[doc = "0x40c - Host Global Interrupt Set Register"]
    pub hstifr: HSTIFR,
    #[doc = "0x410 - Host Global Interrupt Mask Register"]
    pub hstimr: HSTIMR,
    #[doc = "0x414 - Host Global Interrupt Disable Register"]
    pub hstidr: HSTIDR,
    #[doc = "0x418 - Host Global Interrupt Enable Register"]
    pub hstier: HSTIER,
    #[doc = "0x41c - Host Pipe Register"]
    pub hstpip: HSTPIP,
    #[doc = "0x420 - Host Frame Number Register"]
    pub hstfnum: HSTFNUM,
    #[doc = "0x424 - Host Address 1 Register"]
    pub hstaddr1: HSTADDR1,
    #[doc = "0x428 - Host Address 2 Register"]
    pub hstaddr2: HSTADDR2,
    #[doc = "0x42c - Host Address 3 Register"]
    pub hstaddr3: HSTADDR3,
    _reserved29: [u8; 208usize],
    #[doc = "0x500 - Host Pipe Configuration Register (n = 0) 0"]
    pub hstpipcfg: [HSTPIPCFG; 10],
    _reserved30: [u8; 8usize],
    #[doc = "0x530 - Host Pipe Status Register (n = 0) 0"]
    pub hstpipisr: [HSTPIPISR; 10],
    _reserved31: [u8; 8usize],
    #[doc = "0x560 - Host Pipe Clear Register (n = 0) 0"]
    pub hstpipicr: [HSTPIPICR; 10],
    _reserved32: [u8; 8usize],
    #[doc = "0x590 - Host Pipe Set Register (n = 0) 0"]
    pub hstpipifr: [HSTPIPIFR; 10],
    _reserved33: [u8; 8usize],
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0) 0"]
    pub hstpipimr: [HSTPIPIMR; 10],
    _reserved34: [u8; 8usize],
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0) 0"]
    pub hstpipier: [HSTPIPIER; 10],
    _reserved35: [u8; 8usize],
    #[doc = "0x620 - Host Pipe Disable Register (n = 0) 0"]
    pub hstpipidr: [HSTPIPIDR; 10],
    _reserved36: [u8; 8usize],
    #[doc = "0x650 - Host Pipe IN Request Register (n = 0) 0"]
    pub hstpipinrq: [HSTPIPINRQ; 10],
    _reserved37: [u8; 8usize],
    #[doc = "0x680 - Host Pipe Error Register (n = 0) 0"]
    pub hstpiperr: [HSTPIPERR; 10],
    _reserved38: [u8; 104usize],
    #[doc = "0x710 - Host DMA Channel Next Descriptor Address Register (n = 1)"]
    pub usbhs_hstdma: [USBHS_HSTDMA; 7],
    _reserved39: [u8; 128usize],
    #[doc = "0x800 - General Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x804 - General Status Register"]
    pub sr: SR,
    #[doc = "0x808 - General Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x80c - General Status Set Register"]
    pub sfr: SFR,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct USBHS_DEVDMA {
    #[doc = "0x00 - Device DMA Channel Next Descriptor Address Register (n = 1)"]
    pub devdmanxtdsc: self::usbhs_devdma::DEVDMANXTDSC,
    #[doc = "0x04 - Device DMA Channel Address Register (n = 1)"]
    pub devdmaaddress: self::usbhs_devdma::DEVDMAADDRESS,
    #[doc = "0x08 - Device DMA Channel Control Register (n = 1)"]
    pub devdmacontrol: self::usbhs_devdma::DEVDMACONTROL,
    #[doc = "0x0c - Device DMA Channel Status Register (n = 1)"]
    pub devdmastatus: self::usbhs_devdma::DEVDMASTATUS,
}
#[doc = r"Register block"]
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod usbhs_devdma;
#[doc = r"Register block"]
#[repr(C)]
pub struct USBHS_HSTDMA {
    #[doc = "0x00 - Host DMA Channel Next Descriptor Address Register (n = 1)"]
    pub hstdmanxtdsc: self::usbhs_hstdma::HSTDMANXTDSC,
    #[doc = "0x04 - Host DMA Channel Address Register (n = 1)"]
    pub hstdmaaddress: self::usbhs_hstdma::HSTDMAADDRESS,
    #[doc = "0x08 - Host DMA Channel Control Register (n = 1)"]
    pub hstdmacontrol: self::usbhs_hstdma::HSTDMACONTROL,
    #[doc = "0x0c - Host DMA Channel Status Register (n = 1)"]
    pub hstdmastatus: self::usbhs_hstdma::HSTDMASTATUS,
}
#[doc = r"Register block"]
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod usbhs_hstdma;
#[doc = "Device General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devctrl](devctrl) module"]
pub type DEVCTRL = crate::Reg<u32, _DEVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVCTRL;
#[doc = "`read()` method returns [devctrl::R](devctrl::R) reader structure"]
impl crate::Readable for DEVCTRL {}
#[doc = "`write(|w| ..)` method takes [devctrl::W](devctrl::W) writer structure"]
impl crate::Writable for DEVCTRL {}
#[doc = "Device General Control Register"]
pub mod devctrl;
#[doc = "Device Global Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devisr](devisr) module"]
pub type DEVISR = crate::Reg<u32, _DEVISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVISR;
#[doc = "`read()` method returns [devisr::R](devisr::R) reader structure"]
impl crate::Readable for DEVISR {}
#[doc = "Device Global Interrupt Status Register"]
pub mod devisr;
#[doc = "Device Global Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devicr](devicr) module"]
pub type DEVICR = crate::Reg<u32, _DEVICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICR;
#[doc = "`write(|w| ..)` method takes [devicr::W](devicr::W) writer structure"]
impl crate::Writable for DEVICR {}
#[doc = "Device Global Interrupt Clear Register"]
pub mod devicr;
#[doc = "Device Global Interrupt Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devifr](devifr) module"]
pub type DEVIFR = crate::Reg<u32, _DEVIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVIFR;
#[doc = "`write(|w| ..)` method takes [devifr::W](devifr::W) writer structure"]
impl crate::Writable for DEVIFR {}
#[doc = "Device Global Interrupt Set Register"]
pub mod devifr;
#[doc = "Device Global Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devimr](devimr) module"]
pub type DEVIMR = crate::Reg<u32, _DEVIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVIMR;
#[doc = "`read()` method returns [devimr::R](devimr::R) reader structure"]
impl crate::Readable for DEVIMR {}
#[doc = "Device Global Interrupt Mask Register"]
pub mod devimr;
#[doc = "Device Global Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devidr](devidr) module"]
pub type DEVIDR = crate::Reg<u32, _DEVIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVIDR;
#[doc = "`write(|w| ..)` method takes [devidr::W](devidr::W) writer structure"]
impl crate::Writable for DEVIDR {}
#[doc = "Device Global Interrupt Disable Register"]
pub mod devidr;
#[doc = "Device Global Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devier](devier) module"]
pub type DEVIER = crate::Reg<u32, _DEVIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVIER;
#[doc = "`write(|w| ..)` method takes [devier::W](devier::W) writer structure"]
impl crate::Writable for DEVIER {}
#[doc = "Device Global Interrupt Enable Register"]
pub mod devier;
#[doc = "Device Endpoint Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devept](devept) module"]
pub type DEVEPT = crate::Reg<u32, _DEVEPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPT;
#[doc = "`read()` method returns [devept::R](devept::R) reader structure"]
impl crate::Readable for DEVEPT {}
#[doc = "`write(|w| ..)` method takes [devept::W](devept::W) writer structure"]
impl crate::Writable for DEVEPT {}
#[doc = "Device Endpoint Register"]
pub mod devept;
#[doc = "Device Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devfnum](devfnum) module"]
pub type DEVFNUM = crate::Reg<u32, _DEVFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVFNUM;
#[doc = "`read()` method returns [devfnum::R](devfnum::R) reader structure"]
impl crate::Readable for DEVFNUM {}
#[doc = "Device Frame Number Register"]
pub mod devfnum;
#[doc = "Device Endpoint Configuration Register (n = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptcfg](deveptcfg) module"]
pub type DEVEPTCFG = crate::Reg<u32, _DEVEPTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTCFG;
#[doc = "`read()` method returns [deveptcfg::R](deveptcfg::R) reader structure"]
impl crate::Readable for DEVEPTCFG {}
#[doc = "`write(|w| ..)` method takes [deveptcfg::W](deveptcfg::W) writer structure"]
impl crate::Writable for DEVEPTCFG {}
#[doc = "Device Endpoint Configuration Register (n = 0) 0"]
pub mod deveptcfg;
#[doc = "Device Endpoint Status Register (n = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptisr](deveptisr) module"]
pub type DEVEPTISR = crate::Reg<u32, _DEVEPTISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTISR;
#[doc = "`read()` method returns [deveptisr::R](deveptisr::R) reader structure"]
impl crate::Readable for DEVEPTISR {}
#[doc = "Device Endpoint Status Register (n = 0) 0"]
pub mod deveptisr;
#[doc = "Device Endpoint Clear Register (n = 0) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devepticr](devepticr) module"]
pub type DEVEPTICR = crate::Reg<u32, _DEVEPTICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTICR;
#[doc = "`write(|w| ..)` method takes [devepticr::W](devepticr::W) writer structure"]
impl crate::Writable for DEVEPTICR {}
#[doc = "Device Endpoint Clear Register (n = 0) 0"]
pub mod devepticr;
#[doc = "Device Endpoint Set Register (n = 0) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptifr](deveptifr) module"]
pub type DEVEPTIFR = crate::Reg<u32, _DEVEPTIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIFR;
#[doc = "`write(|w| ..)` method takes [deveptifr::W](deveptifr::W) writer structure"]
impl crate::Writable for DEVEPTIFR {}
#[doc = "Device Endpoint Set Register (n = 0) 0"]
pub mod deveptifr;
#[doc = "Device Endpoint Mask Register (n = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptimr](deveptimr) module"]
pub type DEVEPTIMR = crate::Reg<u32, _DEVEPTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIMR;
#[doc = "`read()` method returns [deveptimr::R](deveptimr::R) reader structure"]
impl crate::Readable for DEVEPTIMR {}
#[doc = "Device Endpoint Mask Register (n = 0) 0"]
pub mod deveptimr;
#[doc = "Device Endpoint Enable Register (n = 0) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptier](deveptier) module"]
pub type DEVEPTIER = crate::Reg<u32, _DEVEPTIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIER;
#[doc = "`write(|w| ..)` method takes [deveptier::W](deveptier::W) writer structure"]
impl crate::Writable for DEVEPTIER {}
#[doc = "Device Endpoint Enable Register (n = 0) 0"]
pub mod deveptier;
#[doc = "Device Endpoint Disable Register (n = 0) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptidr](deveptidr) module"]
pub type DEVEPTIDR = crate::Reg<u32, _DEVEPTIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIDR;
#[doc = "`write(|w| ..)` method takes [deveptidr::W](deveptidr::W) writer structure"]
impl crate::Writable for DEVEPTIDR {}
#[doc = "Device Endpoint Disable Register (n = 0) 0"]
pub mod deveptidr;
#[doc = "Host General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstctrl](hstctrl) module"]
pub type HSTCTRL = crate::Reg<u32, _HSTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTCTRL;
#[doc = "`read()` method returns [hstctrl::R](hstctrl::R) reader structure"]
impl crate::Readable for HSTCTRL {}
#[doc = "`write(|w| ..)` method takes [hstctrl::W](hstctrl::W) writer structure"]
impl crate::Writable for HSTCTRL {}
#[doc = "Host General Control Register"]
pub mod hstctrl;
#[doc = "Host Global Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstisr](hstisr) module"]
pub type HSTISR = crate::Reg<u32, _HSTISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTISR;
#[doc = "`read()` method returns [hstisr::R](hstisr::R) reader structure"]
impl crate::Readable for HSTISR {}
#[doc = "Host Global Interrupt Status Register"]
pub mod hstisr;
#[doc = "Host Global Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsticr](hsticr) module"]
pub type HSTICR = crate::Reg<u32, _HSTICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTICR;
#[doc = "`write(|w| ..)` method takes [hsticr::W](hsticr::W) writer structure"]
impl crate::Writable for HSTICR {}
#[doc = "Host Global Interrupt Clear Register"]
pub mod hsticr;
#[doc = "Host Global Interrupt Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstifr](hstifr) module"]
pub type HSTIFR = crate::Reg<u32, _HSTIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTIFR;
#[doc = "`write(|w| ..)` method takes [hstifr::W](hstifr::W) writer structure"]
impl crate::Writable for HSTIFR {}
#[doc = "Host Global Interrupt Set Register"]
pub mod hstifr;
#[doc = "Host Global Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstimr](hstimr) module"]
pub type HSTIMR = crate::Reg<u32, _HSTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTIMR;
#[doc = "`read()` method returns [hstimr::R](hstimr::R) reader structure"]
impl crate::Readable for HSTIMR {}
#[doc = "Host Global Interrupt Mask Register"]
pub mod hstimr;
#[doc = "Host Global Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstidr](hstidr) module"]
pub type HSTIDR = crate::Reg<u32, _HSTIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTIDR;
#[doc = "`write(|w| ..)` method takes [hstidr::W](hstidr::W) writer structure"]
impl crate::Writable for HSTIDR {}
#[doc = "Host Global Interrupt Disable Register"]
pub mod hstidr;
#[doc = "Host Global Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstier](hstier) module"]
pub type HSTIER = crate::Reg<u32, _HSTIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTIER;
#[doc = "`write(|w| ..)` method takes [hstier::W](hstier::W) writer structure"]
impl crate::Writable for HSTIER {}
#[doc = "Host Global Interrupt Enable Register"]
pub mod hstier;
#[doc = "Host Pipe Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpip](hstpip) module"]
pub type HSTPIP = crate::Reg<u32, _HSTPIP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIP;
#[doc = "`read()` method returns [hstpip::R](hstpip::R) reader structure"]
impl crate::Readable for HSTPIP {}
#[doc = "`write(|w| ..)` method takes [hstpip::W](hstpip::W) writer structure"]
impl crate::Writable for HSTPIP {}
#[doc = "Host Pipe Register"]
pub mod hstpip;
#[doc = "Host Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstfnum](hstfnum) module"]
pub type HSTFNUM = crate::Reg<u32, _HSTFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTFNUM;
#[doc = "`read()` method returns [hstfnum::R](hstfnum::R) reader structure"]
impl crate::Readable for HSTFNUM {}
#[doc = "`write(|w| ..)` method takes [hstfnum::W](hstfnum::W) writer structure"]
impl crate::Writable for HSTFNUM {}
#[doc = "Host Frame Number Register"]
pub mod hstfnum;
#[doc = "Host Address 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstaddr1](hstaddr1) module"]
pub type HSTADDR1 = crate::Reg<u32, _HSTADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTADDR1;
#[doc = "`read()` method returns [hstaddr1::R](hstaddr1::R) reader structure"]
impl crate::Readable for HSTADDR1 {}
#[doc = "`write(|w| ..)` method takes [hstaddr1::W](hstaddr1::W) writer structure"]
impl crate::Writable for HSTADDR1 {}
#[doc = "Host Address 1 Register"]
pub mod hstaddr1;
#[doc = "Host Address 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstaddr2](hstaddr2) module"]
pub type HSTADDR2 = crate::Reg<u32, _HSTADDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTADDR2;
#[doc = "`read()` method returns [hstaddr2::R](hstaddr2::R) reader structure"]
impl crate::Readable for HSTADDR2 {}
#[doc = "`write(|w| ..)` method takes [hstaddr2::W](hstaddr2::W) writer structure"]
impl crate::Writable for HSTADDR2 {}
#[doc = "Host Address 2 Register"]
pub mod hstaddr2;
#[doc = "Host Address 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstaddr3](hstaddr3) module"]
pub type HSTADDR3 = crate::Reg<u32, _HSTADDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTADDR3;
#[doc = "`read()` method returns [hstaddr3::R](hstaddr3::R) reader structure"]
impl crate::Readable for HSTADDR3 {}
#[doc = "`write(|w| ..)` method takes [hstaddr3::W](hstaddr3::W) writer structure"]
impl crate::Writable for HSTADDR3 {}
#[doc = "Host Address 3 Register"]
pub mod hstaddr3;
#[doc = "Host Pipe Configuration Register (n = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipcfg](hstpipcfg) module"]
pub type HSTPIPCFG = crate::Reg<u32, _HSTPIPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPCFG;
#[doc = "`read()` method returns [hstpipcfg::R](hstpipcfg::R) reader structure"]
impl crate::Readable for HSTPIPCFG {}
#[doc = "`write(|w| ..)` method takes [hstpipcfg::W](hstpipcfg::W) writer structure"]
impl crate::Writable for HSTPIPCFG {}
#[doc = "Host Pipe Configuration Register (n = 0) 0"]
pub mod hstpipcfg;
#[doc = "Host Pipe Status Register (n = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipisr](hstpipisr) module"]
pub type HSTPIPISR = crate::Reg<u32, _HSTPIPISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPISR;
#[doc = "`read()` method returns [hstpipisr::R](hstpipisr::R) reader structure"]
impl crate::Readable for HSTPIPISR {}
#[doc = "Host Pipe Status Register (n = 0) 0"]
pub mod hstpipisr;
#[doc = "Host Pipe Clear Register (n = 0) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipicr](hstpipicr) module"]
pub type HSTPIPICR = crate::Reg<u32, _HSTPIPICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPICR;
#[doc = "`write(|w| ..)` method takes [hstpipicr::W](hstpipicr::W) writer structure"]
impl crate::Writable for HSTPIPICR {}
#[doc = "Host Pipe Clear Register (n = 0) 0"]
pub mod hstpipicr;
#[doc = "Host Pipe Set Register (n = 0) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipifr](hstpipifr) module"]
pub type HSTPIPIFR = crate::Reg<u32, _HSTPIPIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIFR;
#[doc = "`write(|w| ..)` method takes [hstpipifr::W](hstpipifr::W) writer structure"]
impl crate::Writable for HSTPIPIFR {}
#[doc = "Host Pipe Set Register (n = 0) 0"]
pub mod hstpipifr;
#[doc = "Host Pipe Mask Register (n = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipimr](hstpipimr) module"]
pub type HSTPIPIMR = crate::Reg<u32, _HSTPIPIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIMR;
#[doc = "`read()` method returns [hstpipimr::R](hstpipimr::R) reader structure"]
impl crate::Readable for HSTPIPIMR {}
#[doc = "Host Pipe Mask Register (n = 0) 0"]
pub mod hstpipimr;
#[doc = "Host Pipe Enable Register (n = 0) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipier](hstpipier) module"]
pub type HSTPIPIER = crate::Reg<u32, _HSTPIPIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIER;
#[doc = "`write(|w| ..)` method takes [hstpipier::W](hstpipier::W) writer structure"]
impl crate::Writable for HSTPIPIER {}
#[doc = "Host Pipe Enable Register (n = 0) 0"]
pub mod hstpipier;
#[doc = "Host Pipe Disable Register (n = 0) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipidr](hstpipidr) module"]
pub type HSTPIPIDR = crate::Reg<u32, _HSTPIPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIDR;
#[doc = "`write(|w| ..)` method takes [hstpipidr::W](hstpipidr::W) writer structure"]
impl crate::Writable for HSTPIPIDR {}
#[doc = "Host Pipe Disable Register (n = 0) 0"]
pub mod hstpipidr;
#[doc = "Host Pipe IN Request Register (n = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipinrq](hstpipinrq) module"]
pub type HSTPIPINRQ = crate::Reg<u32, _HSTPIPINRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPINRQ;
#[doc = "`read()` method returns [hstpipinrq::R](hstpipinrq::R) reader structure"]
impl crate::Readable for HSTPIPINRQ {}
#[doc = "`write(|w| ..)` method takes [hstpipinrq::W](hstpipinrq::W) writer structure"]
impl crate::Writable for HSTPIPINRQ {}
#[doc = "Host Pipe IN Request Register (n = 0) 0"]
pub mod hstpipinrq;
#[doc = "Host Pipe Error Register (n = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpiperr](hstpiperr) module"]
pub type HSTPIPERR = crate::Reg<u32, _HSTPIPERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPERR;
#[doc = "`read()` method returns [hstpiperr::R](hstpiperr::R) reader structure"]
impl crate::Readable for HSTPIPERR {}
#[doc = "`write(|w| ..)` method takes [hstpiperr::W](hstpiperr::W) writer structure"]
impl crate::Writable for HSTPIPERR {}
#[doc = "Host Pipe Error Register (n = 0) 0"]
pub mod hstpiperr;
#[doc = "General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "General Control Register"]
pub mod ctrl;
#[doc = "General Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "General Status Register"]
pub mod sr;
#[doc = "General Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "General Status Clear Register"]
pub mod scr;
#[doc = "General Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfr](sfr) module"]
pub type SFR = crate::Reg<u32, _SFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFR;
#[doc = "`write(|w| ..)` method takes [sfr::W](sfr::W) writer structure"]
impl crate::Writable for SFR {}
#[doc = "General Status Set Register"]
pub mod sfr;
