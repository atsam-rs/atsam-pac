#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device General Control Register"]
    pub udcon: UDCON,
    #[doc = "0x04 - Device Global Interupt Register"]
    pub udint: UDINT,
    #[doc = "0x08 - Device Global Interrupt Clear Register"]
    pub udintclr: UDINTCLR,
    #[doc = "0x0c - Device Global Interrupt Set Regsiter"]
    pub udintset: UDINTSET,
    #[doc = "0x10 - Device Global Interrupt Enable Register"]
    pub udinte: UDINTE,
    #[doc = "0x14 - Device Global Interrupt Enable Clear Register"]
    pub udinteclr: UDINTECLR,
    #[doc = "0x18 - Device Global Interrupt Enable Set Register"]
    pub udinteset: UDINTESET,
    #[doc = "0x1c - Endpoint Enable/Reset Register"]
    pub uerst: UERST,
    #[doc = "0x20 - Device Frame Number Register"]
    pub udfnum: UDFNUM,
    _reserved9: [u8; 220usize],
    #[doc = "0x100 - Endpoint Configuration Register"]
    pub uecfg0: UECFG0,
    #[doc = "0x104 - Endpoint Configuration Register"]
    pub uecfg1: UECFG1,
    #[doc = "0x108 - Endpoint Configuration Register"]
    pub uecfg2: UECFG2,
    #[doc = "0x10c - Endpoint Configuration Register"]
    pub uecfg3: UECFG3,
    #[doc = "0x110 - Endpoint Configuration Register"]
    pub uecfg4: UECFG4,
    #[doc = "0x114 - Endpoint Configuration Register"]
    pub uecfg5: UECFG5,
    #[doc = "0x118 - Endpoint Configuration Register"]
    pub uecfg6: UECFG6,
    #[doc = "0x11c - Endpoint Configuration Register"]
    pub uecfg7: UECFG7,
    _reserved17: [u8; 16usize],
    #[doc = "0x130 - Endpoint Status Register"]
    pub uesta0: UESTA0,
    #[doc = "0x134 - Endpoint Status Register"]
    pub uesta1: UESTA1,
    #[doc = "0x138 - Endpoint Status Register"]
    pub uesta2: UESTA2,
    #[doc = "0x13c - Endpoint Status Register"]
    pub uesta3: UESTA3,
    #[doc = "0x140 - Endpoint Status Register"]
    pub uesta4: UESTA4,
    #[doc = "0x144 - Endpoint Status Register"]
    pub uesta5: UESTA5,
    #[doc = "0x148 - Endpoint Status Register"]
    pub uesta6: UESTA6,
    #[doc = "0x14c - Endpoint Status Register"]
    pub uesta7: UESTA7,
    _reserved25: [u8; 16usize],
    #[doc = "0x160 - Endpoint Status Clear Register"]
    pub uesta0clr: UESTA0CLR,
    #[doc = "0x164 - Endpoint Status Clear Register"]
    pub uesta1clr: UESTA1CLR,
    #[doc = "0x168 - Endpoint Status Clear Register"]
    pub uesta2clr: UESTA2CLR,
    #[doc = "0x16c - Endpoint Status Clear Register"]
    pub uesta3clr: UESTA3CLR,
    #[doc = "0x170 - Endpoint Status Clear Register"]
    pub uesta4clr: UESTA4CLR,
    #[doc = "0x174 - Endpoint Status Clear Register"]
    pub uesta5clr: UESTA5CLR,
    #[doc = "0x178 - Endpoint Status Clear Register"]
    pub uesta6clr: UESTA6CLR,
    #[doc = "0x17c - Endpoint Status Clear Register"]
    pub uesta7clr: UESTA7CLR,
    _reserved33: [u8; 16usize],
    #[doc = "0x190 - Endpoint Status Set Register"]
    pub uesta0set: UESTA0SET,
    #[doc = "0x194 - Endpoint Status Set Register"]
    pub uesta1set: UESTA1SET,
    #[doc = "0x198 - Endpoint Status Set Register"]
    pub uesta2set: UESTA2SET,
    #[doc = "0x19c - Endpoint Status Set Register"]
    pub uesta3set: UESTA3SET,
    #[doc = "0x1a0 - Endpoint Status Set Register"]
    pub uesta4set: UESTA4SET,
    #[doc = "0x1a4 - Endpoint Status Set Register"]
    pub uesta5set: UESTA5SET,
    #[doc = "0x1a8 - Endpoint Status Set Register"]
    pub uesta6set: UESTA6SET,
    #[doc = "0x1ac - Endpoint Status Set Register"]
    pub uesta7set: UESTA7SET,
    _reserved41: [u8; 16usize],
    #[doc = "0x1c0 - Endpoint Control Register"]
    pub uecon0: UECON0,
    #[doc = "0x1c4 - Endpoint Control Register"]
    pub uecon1: UECON1,
    #[doc = "0x1c8 - Endpoint Control Register"]
    pub uecon2: UECON2,
    #[doc = "0x1cc - Endpoint Control Register"]
    pub uecon3: UECON3,
    #[doc = "0x1d0 - Endpoint Control Register"]
    pub uecon4: UECON4,
    #[doc = "0x1d4 - Endpoint Control Register"]
    pub uecon5: UECON5,
    #[doc = "0x1d8 - Endpoint Control Register"]
    pub uecon6: UECON6,
    #[doc = "0x1dc - Endpoint Control Register"]
    pub uecon7: UECON7,
    _reserved49: [u8; 16usize],
    #[doc = "0x1f0 - Endpoint Control Set Register"]
    pub uecon0set: UECON0SET,
    #[doc = "0x1f4 - Endpoint Control Set Register"]
    pub uecon1set: UECON1SET,
    #[doc = "0x1f8 - Endpoint Control Set Register"]
    pub uecon2set: UECON2SET,
    #[doc = "0x1fc - Endpoint Control Set Register"]
    pub uecon3set: UECON3SET,
    #[doc = "0x200 - Endpoint Control Set Register"]
    pub uecon4set: UECON4SET,
    #[doc = "0x204 - Endpoint Control Set Register"]
    pub uecon5set: UECON5SET,
    #[doc = "0x208 - Endpoint Control Set Register"]
    pub uecon6set: UECON6SET,
    #[doc = "0x20c - Endpoint Control Set Register"]
    pub uecon7set: UECON7SET,
    _reserved57: [u8; 16usize],
    #[doc = "0x220 - Endpoint Control Clear Register"]
    pub uecon0clr: UECON0CLR,
    #[doc = "0x224 - TXINE Clear"]
    pub uecon1clr: UECON1CLR,
    #[doc = "0x228 - TXINE Clear"]
    pub uecon2clr: UECON2CLR,
    #[doc = "0x22c - TXINE Clear"]
    pub uecon3clr: UECON3CLR,
    #[doc = "0x230 - TXINE Clear"]
    pub uecon4clr: UECON4CLR,
    #[doc = "0x234 - TXINE Clear"]
    pub uecon5clr: UECON5CLR,
    #[doc = "0x238 - TXINE Clear"]
    pub uecon6clr: UECON6CLR,
    #[doc = "0x23c - TXINE Clear"]
    pub uecon7clr: UECON7CLR,
    _reserved65: [u8; 448usize],
    #[doc = "0x400 - Host General Control Register"]
    pub uhcon: UHCON,
    #[doc = "0x404 - Host Global Interrupt Register"]
    pub uhint: UHINT,
    #[doc = "0x408 - Host Global Interrrupt Clear Register"]
    pub uhintclr: UHINTCLR,
    #[doc = "0x40c - Host Global Interrupt Set Register"]
    pub uhintset: UHINTSET,
    #[doc = "0x410 - Host Global Interrupt Enable Register"]
    pub uhinte: UHINTE,
    #[doc = "0x414 - Host Global Interrupt Enable Clear Register"]
    pub uhinteclr: UHINTECLR,
    #[doc = "0x418 - Host Global Interrupt Enable Set Register"]
    pub uhinteset: UHINTESET,
    #[doc = "0x41c - Pipe Reset Register"]
    pub uprst: UPRST,
    #[doc = "0x420 - Host Frame Number Register"]
    pub uhfnum: UHFNUM,
    #[doc = "0x424 - Host Start of Frame Control Register"]
    pub uhsofc: UHSOFC,
    _reserved75: [u8; 216usize],
    #[doc = "0x500 - Pipe Configuration Register"]
    pub upcfg0: UPCFG0,
    #[doc = "0x504 - Pipe Configuration Register"]
    pub upcfg1: UPCFG1,
    #[doc = "0x508 - Pipe Configuration Register"]
    pub upcfg2: UPCFG2,
    #[doc = "0x50c - Pipe Configuration Register"]
    pub upcfg3: UPCFG3,
    #[doc = "0x510 - Pipe Configuration Register"]
    pub upcfg4: UPCFG4,
    #[doc = "0x514 - Pipe Configuration Register"]
    pub upcfg5: UPCFG5,
    #[doc = "0x518 - Pipe Configuration Register"]
    pub upcfg6: UPCFG6,
    #[doc = "0x51c - Pipe Configuration Register"]
    pub upcfg7: UPCFG7,
    _reserved83: [u8; 16usize],
    #[doc = "0x530 - Pipe Status Register"]
    pub upsta0: UPSTA0,
    #[doc = "0x534 - Pipe Status Register"]
    pub upsta1: UPSTA1,
    #[doc = "0x538 - Pipe Status Register"]
    pub upsta2: UPSTA2,
    #[doc = "0x53c - Pipe Status Register"]
    pub upsta3: UPSTA3,
    #[doc = "0x540 - Pipe Status Register"]
    pub upsta4: UPSTA4,
    #[doc = "0x544 - Pipe Status Register"]
    pub upsta5: UPSTA5,
    #[doc = "0x548 - Pipe Status Register"]
    pub upsta6: UPSTA6,
    #[doc = "0x54c - Pipe Status Register"]
    pub upsta7: UPSTA7,
    _reserved91: [u8; 16usize],
    #[doc = "0x560 - Pipe Status Clear Register"]
    pub upsta0clr: UPSTA0CLR,
    #[doc = "0x564 - Pipe Status Clear Register"]
    pub upsta1clr: UPSTA1CLR,
    #[doc = "0x568 - Pipe Status Clear Register"]
    pub upsta2clr: UPSTA2CLR,
    #[doc = "0x56c - Pipe Status Clear Register"]
    pub upsta3clr: UPSTA3CLR,
    #[doc = "0x570 - Pipe Status Clear Register"]
    pub upsta4clr: UPSTA4CLR,
    #[doc = "0x574 - Pipe Status Clear Register"]
    pub upsta5clr: UPSTA5CLR,
    #[doc = "0x578 - Pipe Status Clear Register"]
    pub upsta6clr: UPSTA6CLR,
    #[doc = "0x57c - Pipe Status Clear Register"]
    pub upsta7clr: UPSTA7CLR,
    _reserved99: [u8; 16usize],
    #[doc = "0x590 - Pipe Status Set Register"]
    pub upsta0set: UPSTA0SET,
    #[doc = "0x594 - Pipe Status Set Register"]
    pub upsta1set: UPSTA1SET,
    #[doc = "0x598 - Pipe Status Set Register"]
    pub upsta2set: UPSTA2SET,
    #[doc = "0x59c - Pipe Status Set Register"]
    pub upsta3set: UPSTA3SET,
    #[doc = "0x5a0 - Pipe Status Set Register"]
    pub upsta4set: UPSTA4SET,
    #[doc = "0x5a4 - Pipe Status Set Register"]
    pub upsta5set: UPSTA5SET,
    #[doc = "0x5a8 - Pipe Status Set Register"]
    pub upsta6set: UPSTA6SET,
    #[doc = "0x5ac - Pipe Status Set Register"]
    pub upsta7set: UPSTA7SET,
    _reserved107: [u8; 16usize],
    #[doc = "0x5c0 - Pipe Control Register"]
    pub upcon0: UPCON0,
    #[doc = "0x5c4 - Pipe Control Register"]
    pub upcon1: UPCON1,
    #[doc = "0x5c8 - Pipe Control Register"]
    pub upcon2: UPCON2,
    #[doc = "0x5cc - Pipe Control Register"]
    pub upcon3: UPCON3,
    #[doc = "0x5d0 - Pipe Control Register"]
    pub upcon4: UPCON4,
    #[doc = "0x5d4 - Pipe Control Register"]
    pub upcon5: UPCON5,
    #[doc = "0x5d8 - Pipe Control Register"]
    pub upcon6: UPCON6,
    #[doc = "0x5dc - Pipe Control Register"]
    pub upcon7: UPCON7,
    _reserved115: [u8; 16usize],
    #[doc = "0x5f0 - Pipe Control Set Register"]
    pub upcon0set: UPCON0SET,
    #[doc = "0x5f4 - Pipe Control Set Register"]
    pub upcon1set: UPCON1SET,
    #[doc = "0x5f8 - Pipe Control Set Register"]
    pub upcon2set: UPCON2SET,
    #[doc = "0x5fc - Pipe Control Set Register"]
    pub upcon3set: UPCON3SET,
    #[doc = "0x600 - Pipe Control Set Register"]
    pub upcon4set: UPCON4SET,
    #[doc = "0x604 - Pipe Control Set Register"]
    pub upcon5set: UPCON5SET,
    #[doc = "0x608 - Pipe Control Set Register"]
    pub upcon6set: UPCON6SET,
    #[doc = "0x60c - Pipe Control Set Register"]
    pub upcon7set: UPCON7SET,
    _reserved123: [u8; 16usize],
    #[doc = "0x620 - Pipe Control Clear Register"]
    pub upcon0clr: UPCON0CLR,
    #[doc = "0x624 - Pipe Control Clear Register"]
    pub upcon1clr: UPCON1CLR,
    #[doc = "0x628 - Pipe Control Clear Register"]
    pub upcon2clr: UPCON2CLR,
    #[doc = "0x62c - Pipe Control Clear Register"]
    pub upcon3clr: UPCON3CLR,
    #[doc = "0x630 - Pipe Control Clear Register"]
    pub upcon4clr: UPCON4CLR,
    #[doc = "0x634 - Pipe Control Clear Register"]
    pub upcon5clr: UPCON5CLR,
    #[doc = "0x638 - Pipe Control Clear Register"]
    pub upcon6clr: UPCON6CLR,
    #[doc = "0x63c - Pipe Control Clear Register"]
    pub upcon7clr: UPCON7CLR,
    _reserved131: [u8; 16usize],
    #[doc = "0x650 - Pipe In Request"]
    pub upinrq0: UPINRQ0,
    #[doc = "0x654 - Pipe In Request"]
    pub upinrq1: UPINRQ1,
    #[doc = "0x658 - Pipe In Request"]
    pub upinrq2: UPINRQ2,
    #[doc = "0x65c - Pipe In Request"]
    pub upinrq3: UPINRQ3,
    #[doc = "0x660 - Pipe In Request"]
    pub upinrq4: UPINRQ4,
    #[doc = "0x664 - Pipe In Request"]
    pub upinrq5: UPINRQ5,
    #[doc = "0x668 - Pipe In Request"]
    pub upinrq6: UPINRQ6,
    #[doc = "0x66c - Pipe In Request"]
    pub upinrq7: UPINRQ7,
    _reserved139: [u8; 400usize],
    #[doc = "0x800 - General Control Register"]
    pub usbcon: USBCON,
    #[doc = "0x804 - General Status Register"]
    pub usbsta: USBSTA,
    #[doc = "0x808 - General Status Clear Register"]
    pub usbstaclr: USBSTACLR,
    #[doc = "0x80c - General Status Set Register"]
    pub usbstaset: USBSTASET,
    _reserved143: [u8; 8usize],
    #[doc = "0x818 - IP Version Register"]
    pub uvers: UVERS,
    #[doc = "0x81c - IP Features Register"]
    pub ufeatures: UFEATURES,
    #[doc = "0x820 - IP PB address size Register"]
    pub uaddrsize: UADDRSIZE,
    #[doc = "0x824 - IP Name Part One: HUSB"]
    pub uname1: UNAME1,
    #[doc = "0x828 - IP Name Part Two: HOST"]
    pub uname2: UNAME2,
    #[doc = "0x82c - USB internal finite state machine"]
    pub usbfsm: USBFSM,
    #[doc = "0x830 - Endpoint descriptor table"]
    pub udesc: UDESC,
}
#[doc = "IP PB address size Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uaddrsize](uaddrsize) module"]
pub type UADDRSIZE = crate::Reg<u32, _UADDRSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UADDRSIZE;
#[doc = "`read()` method returns [uaddrsize::R](uaddrsize::R) reader structure"]
impl crate::Readable for UADDRSIZE {}
#[doc = "IP PB address size Register"]
pub mod uaddrsize;
#[doc = "Device General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udcon](udcon) module"]
pub type UDCON = crate::Reg<u32, _UDCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDCON;
#[doc = "`read()` method returns [udcon::R](udcon::R) reader structure"]
impl crate::Readable for UDCON {}
#[doc = "`write(|w| ..)` method takes [udcon::W](udcon::W) writer structure"]
impl crate::Writable for UDCON {}
#[doc = "Device General Control Register"]
pub mod udcon;
#[doc = "Endpoint descriptor table\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udesc](udesc) module"]
pub type UDESC = crate::Reg<u32, _UDESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDESC;
#[doc = "`read()` method returns [udesc::R](udesc::R) reader structure"]
impl crate::Readable for UDESC {}
#[doc = "`write(|w| ..)` method takes [udesc::W](udesc::W) writer structure"]
impl crate::Writable for UDESC {}
#[doc = "Endpoint descriptor table"]
pub mod udesc;
#[doc = "Device Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udfnum](udfnum) module"]
pub type UDFNUM = crate::Reg<u32, _UDFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDFNUM;
#[doc = "`read()` method returns [udfnum::R](udfnum::R) reader structure"]
impl crate::Readable for UDFNUM {}
#[doc = "Device Frame Number Register"]
pub mod udfnum;
#[doc = "Device Global Interupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udint](udint) module"]
pub type UDINT = crate::Reg<u32, _UDINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDINT;
#[doc = "`read()` method returns [udint::R](udint::R) reader structure"]
impl crate::Readable for UDINT {}
#[doc = "Device Global Interupt Register"]
pub mod udint;
#[doc = "Device Global Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udintclr](udintclr) module"]
pub type UDINTCLR = crate::Reg<u32, _UDINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDINTCLR;
#[doc = "`write(|w| ..)` method takes [udintclr::W](udintclr::W) writer structure"]
impl crate::Writable for UDINTCLR {}
#[doc = "Device Global Interrupt Clear Register"]
pub mod udintclr;
#[doc = "Device Global Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udinte](udinte) module"]
pub type UDINTE = crate::Reg<u32, _UDINTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDINTE;
#[doc = "`read()` method returns [udinte::R](udinte::R) reader structure"]
impl crate::Readable for UDINTE {}
#[doc = "Device Global Interrupt Enable Register"]
pub mod udinte;
#[doc = "Device Global Interrupt Enable Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udinteclr](udinteclr) module"]
pub type UDINTECLR = crate::Reg<u32, _UDINTECLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDINTECLR;
#[doc = "`write(|w| ..)` method takes [udinteclr::W](udinteclr::W) writer structure"]
impl crate::Writable for UDINTECLR {}
#[doc = "Device Global Interrupt Enable Clear Register"]
pub mod udinteclr;
#[doc = "Device Global Interrupt Enable Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udinteset](udinteset) module"]
pub type UDINTESET = crate::Reg<u32, _UDINTESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDINTESET;
#[doc = "`write(|w| ..)` method takes [udinteset::W](udinteset::W) writer structure"]
impl crate::Writable for UDINTESET {}
#[doc = "Device Global Interrupt Enable Set Register"]
pub mod udinteset;
#[doc = "Device Global Interrupt Set Regsiter\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udintset](udintset) module"]
pub type UDINTSET = crate::Reg<u32, _UDINTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDINTSET;
#[doc = "`write(|w| ..)` method takes [udintset::W](udintset::W) writer structure"]
impl crate::Writable for UDINTSET {}
#[doc = "Device Global Interrupt Set Regsiter"]
pub mod udintset;
#[doc = "Endpoint Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecfg0](uecfg0) module"]
pub type UECFG0 = crate::Reg<u32, _UECFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECFG0;
#[doc = "`read()` method returns [uecfg0::R](uecfg0::R) reader structure"]
impl crate::Readable for UECFG0 {}
#[doc = "`write(|w| ..)` method takes [uecfg0::W](uecfg0::W) writer structure"]
impl crate::Writable for UECFG0 {}
#[doc = "Endpoint Configuration Register"]
pub mod uecfg0;
#[doc = "Endpoint Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecfg1](uecfg1) module"]
pub type UECFG1 = crate::Reg<u32, _UECFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECFG1;
#[doc = "`read()` method returns [uecfg1::R](uecfg1::R) reader structure"]
impl crate::Readable for UECFG1 {}
#[doc = "`write(|w| ..)` method takes [uecfg1::W](uecfg1::W) writer structure"]
impl crate::Writable for UECFG1 {}
#[doc = "Endpoint Configuration Register"]
pub mod uecfg1;
#[doc = "Endpoint Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecfg2](uecfg2) module"]
pub type UECFG2 = crate::Reg<u32, _UECFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECFG2;
#[doc = "`read()` method returns [uecfg2::R](uecfg2::R) reader structure"]
impl crate::Readable for UECFG2 {}
#[doc = "`write(|w| ..)` method takes [uecfg2::W](uecfg2::W) writer structure"]
impl crate::Writable for UECFG2 {}
#[doc = "Endpoint Configuration Register"]
pub mod uecfg2;
#[doc = "Endpoint Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecfg3](uecfg3) module"]
pub type UECFG3 = crate::Reg<u32, _UECFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECFG3;
#[doc = "`read()` method returns [uecfg3::R](uecfg3::R) reader structure"]
impl crate::Readable for UECFG3 {}
#[doc = "`write(|w| ..)` method takes [uecfg3::W](uecfg3::W) writer structure"]
impl crate::Writable for UECFG3 {}
#[doc = "Endpoint Configuration Register"]
pub mod uecfg3;
#[doc = "Endpoint Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecfg4](uecfg4) module"]
pub type UECFG4 = crate::Reg<u32, _UECFG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECFG4;
#[doc = "`read()` method returns [uecfg4::R](uecfg4::R) reader structure"]
impl crate::Readable for UECFG4 {}
#[doc = "`write(|w| ..)` method takes [uecfg4::W](uecfg4::W) writer structure"]
impl crate::Writable for UECFG4 {}
#[doc = "Endpoint Configuration Register"]
pub mod uecfg4;
#[doc = "Endpoint Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecfg5](uecfg5) module"]
pub type UECFG5 = crate::Reg<u32, _UECFG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECFG5;
#[doc = "`read()` method returns [uecfg5::R](uecfg5::R) reader structure"]
impl crate::Readable for UECFG5 {}
#[doc = "`write(|w| ..)` method takes [uecfg5::W](uecfg5::W) writer structure"]
impl crate::Writable for UECFG5 {}
#[doc = "Endpoint Configuration Register"]
pub mod uecfg5;
#[doc = "Endpoint Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecfg6](uecfg6) module"]
pub type UECFG6 = crate::Reg<u32, _UECFG6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECFG6;
#[doc = "`read()` method returns [uecfg6::R](uecfg6::R) reader structure"]
impl crate::Readable for UECFG6 {}
#[doc = "`write(|w| ..)` method takes [uecfg6::W](uecfg6::W) writer structure"]
impl crate::Writable for UECFG6 {}
#[doc = "Endpoint Configuration Register"]
pub mod uecfg6;
#[doc = "Endpoint Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecfg7](uecfg7) module"]
pub type UECFG7 = crate::Reg<u32, _UECFG7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECFG7;
#[doc = "`read()` method returns [uecfg7::R](uecfg7::R) reader structure"]
impl crate::Readable for UECFG7 {}
#[doc = "`write(|w| ..)` method takes [uecfg7::W](uecfg7::W) writer structure"]
impl crate::Writable for UECFG7 {}
#[doc = "Endpoint Configuration Register"]
pub mod uecfg7;
#[doc = "Endpoint Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon0](uecon0) module"]
pub type UECON0 = crate::Reg<u32, _UECON0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON0;
#[doc = "`read()` method returns [uecon0::R](uecon0::R) reader structure"]
impl crate::Readable for UECON0 {}
#[doc = "Endpoint Control Register"]
pub mod uecon0;
#[doc = "Endpoint Control Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon0clr](uecon0clr) module"]
pub type UECON0CLR = crate::Reg<u32, _UECON0CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON0CLR;
#[doc = "`write(|w| ..)` method takes [uecon0clr::W](uecon0clr::W) writer structure"]
impl crate::Writable for UECON0CLR {}
#[doc = "Endpoint Control Clear Register"]
pub mod uecon0clr;
#[doc = "Endpoint Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon0set](uecon0set) module"]
pub type UECON0SET = crate::Reg<u32, _UECON0SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON0SET;
#[doc = "`write(|w| ..)` method takes [uecon0set::W](uecon0set::W) writer structure"]
impl crate::Writable for UECON0SET {}
#[doc = "Endpoint Control Set Register"]
pub mod uecon0set;
#[doc = "Endpoint Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon1](uecon1) module"]
pub type UECON1 = crate::Reg<u32, _UECON1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON1;
#[doc = "`read()` method returns [uecon1::R](uecon1::R) reader structure"]
impl crate::Readable for UECON1 {}
#[doc = "Endpoint Control Register"]
pub mod uecon1;
#[doc = "TXINE Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon1clr](uecon1clr) module"]
pub type UECON1CLR = crate::Reg<u32, _UECON1CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON1CLR;
#[doc = "`write(|w| ..)` method takes [uecon1clr::W](uecon1clr::W) writer structure"]
impl crate::Writable for UECON1CLR {}
#[doc = "TXINE Clear"]
pub mod uecon1clr;
#[doc = "Endpoint Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon1set](uecon1set) module"]
pub type UECON1SET = crate::Reg<u32, _UECON1SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON1SET;
#[doc = "`write(|w| ..)` method takes [uecon1set::W](uecon1set::W) writer structure"]
impl crate::Writable for UECON1SET {}
#[doc = "Endpoint Control Set Register"]
pub mod uecon1set;
#[doc = "Endpoint Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon2](uecon2) module"]
pub type UECON2 = crate::Reg<u32, _UECON2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON2;
#[doc = "`read()` method returns [uecon2::R](uecon2::R) reader structure"]
impl crate::Readable for UECON2 {}
#[doc = "Endpoint Control Register"]
pub mod uecon2;
#[doc = "TXINE Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon2clr](uecon2clr) module"]
pub type UECON2CLR = crate::Reg<u32, _UECON2CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON2CLR;
#[doc = "`write(|w| ..)` method takes [uecon2clr::W](uecon2clr::W) writer structure"]
impl crate::Writable for UECON2CLR {}
#[doc = "TXINE Clear"]
pub mod uecon2clr;
#[doc = "Endpoint Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon2set](uecon2set) module"]
pub type UECON2SET = crate::Reg<u32, _UECON2SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON2SET;
#[doc = "`write(|w| ..)` method takes [uecon2set::W](uecon2set::W) writer structure"]
impl crate::Writable for UECON2SET {}
#[doc = "Endpoint Control Set Register"]
pub mod uecon2set;
#[doc = "Endpoint Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon3](uecon3) module"]
pub type UECON3 = crate::Reg<u32, _UECON3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON3;
#[doc = "`read()` method returns [uecon3::R](uecon3::R) reader structure"]
impl crate::Readable for UECON3 {}
#[doc = "Endpoint Control Register"]
pub mod uecon3;
#[doc = "TXINE Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon3clr](uecon3clr) module"]
pub type UECON3CLR = crate::Reg<u32, _UECON3CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON3CLR;
#[doc = "`write(|w| ..)` method takes [uecon3clr::W](uecon3clr::W) writer structure"]
impl crate::Writable for UECON3CLR {}
#[doc = "TXINE Clear"]
pub mod uecon3clr;
#[doc = "Endpoint Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon3set](uecon3set) module"]
pub type UECON3SET = crate::Reg<u32, _UECON3SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON3SET;
#[doc = "`write(|w| ..)` method takes [uecon3set::W](uecon3set::W) writer structure"]
impl crate::Writable for UECON3SET {}
#[doc = "Endpoint Control Set Register"]
pub mod uecon3set;
#[doc = "Endpoint Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon4](uecon4) module"]
pub type UECON4 = crate::Reg<u32, _UECON4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON4;
#[doc = "`read()` method returns [uecon4::R](uecon4::R) reader structure"]
impl crate::Readable for UECON4 {}
#[doc = "Endpoint Control Register"]
pub mod uecon4;
#[doc = "TXINE Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon4clr](uecon4clr) module"]
pub type UECON4CLR = crate::Reg<u32, _UECON4CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON4CLR;
#[doc = "`write(|w| ..)` method takes [uecon4clr::W](uecon4clr::W) writer structure"]
impl crate::Writable for UECON4CLR {}
#[doc = "TXINE Clear"]
pub mod uecon4clr;
#[doc = "Endpoint Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon4set](uecon4set) module"]
pub type UECON4SET = crate::Reg<u32, _UECON4SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON4SET;
#[doc = "`write(|w| ..)` method takes [uecon4set::W](uecon4set::W) writer structure"]
impl crate::Writable for UECON4SET {}
#[doc = "Endpoint Control Set Register"]
pub mod uecon4set;
#[doc = "Endpoint Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon5](uecon5) module"]
pub type UECON5 = crate::Reg<u32, _UECON5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON5;
#[doc = "`read()` method returns [uecon5::R](uecon5::R) reader structure"]
impl crate::Readable for UECON5 {}
#[doc = "Endpoint Control Register"]
pub mod uecon5;
#[doc = "TXINE Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon5clr](uecon5clr) module"]
pub type UECON5CLR = crate::Reg<u32, _UECON5CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON5CLR;
#[doc = "`write(|w| ..)` method takes [uecon5clr::W](uecon5clr::W) writer structure"]
impl crate::Writable for UECON5CLR {}
#[doc = "TXINE Clear"]
pub mod uecon5clr;
#[doc = "Endpoint Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon5set](uecon5set) module"]
pub type UECON5SET = crate::Reg<u32, _UECON5SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON5SET;
#[doc = "`write(|w| ..)` method takes [uecon5set::W](uecon5set::W) writer structure"]
impl crate::Writable for UECON5SET {}
#[doc = "Endpoint Control Set Register"]
pub mod uecon5set;
#[doc = "Endpoint Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon6](uecon6) module"]
pub type UECON6 = crate::Reg<u32, _UECON6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON6;
#[doc = "`read()` method returns [uecon6::R](uecon6::R) reader structure"]
impl crate::Readable for UECON6 {}
#[doc = "Endpoint Control Register"]
pub mod uecon6;
#[doc = "TXINE Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon6clr](uecon6clr) module"]
pub type UECON6CLR = crate::Reg<u32, _UECON6CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON6CLR;
#[doc = "`write(|w| ..)` method takes [uecon6clr::W](uecon6clr::W) writer structure"]
impl crate::Writable for UECON6CLR {}
#[doc = "TXINE Clear"]
pub mod uecon6clr;
#[doc = "Endpoint Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon6set](uecon6set) module"]
pub type UECON6SET = crate::Reg<u32, _UECON6SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON6SET;
#[doc = "`write(|w| ..)` method takes [uecon6set::W](uecon6set::W) writer structure"]
impl crate::Writable for UECON6SET {}
#[doc = "Endpoint Control Set Register"]
pub mod uecon6set;
#[doc = "Endpoint Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon7](uecon7) module"]
pub type UECON7 = crate::Reg<u32, _UECON7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON7;
#[doc = "`read()` method returns [uecon7::R](uecon7::R) reader structure"]
impl crate::Readable for UECON7 {}
#[doc = "Endpoint Control Register"]
pub mod uecon7;
#[doc = "TXINE Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon7clr](uecon7clr) module"]
pub type UECON7CLR = crate::Reg<u32, _UECON7CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON7CLR;
#[doc = "`write(|w| ..)` method takes [uecon7clr::W](uecon7clr::W) writer structure"]
impl crate::Writable for UECON7CLR {}
#[doc = "TXINE Clear"]
pub mod uecon7clr;
#[doc = "Endpoint Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon7set](uecon7set) module"]
pub type UECON7SET = crate::Reg<u32, _UECON7SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UECON7SET;
#[doc = "`write(|w| ..)` method takes [uecon7set::W](uecon7set::W) writer structure"]
impl crate::Writable for UECON7SET {}
#[doc = "Endpoint Control Set Register"]
pub mod uecon7set;
#[doc = "Endpoint Enable/Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uerst](uerst) module"]
pub type UERST = crate::Reg<u32, _UERST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UERST;
#[doc = "`read()` method returns [uerst::R](uerst::R) reader structure"]
impl crate::Readable for UERST {}
#[doc = "`write(|w| ..)` method takes [uerst::W](uerst::W) writer structure"]
impl crate::Writable for UERST {}
#[doc = "Endpoint Enable/Reset Register"]
pub mod uerst;
#[doc = "Endpoint Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta0](uesta0) module"]
pub type UESTA0 = crate::Reg<u32, _UESTA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA0;
#[doc = "`read()` method returns [uesta0::R](uesta0::R) reader structure"]
impl crate::Readable for UESTA0 {}
#[doc = "Endpoint Status Register"]
pub mod uesta0;
#[doc = "Endpoint Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta0clr](uesta0clr) module"]
pub type UESTA0CLR = crate::Reg<u32, _UESTA0CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA0CLR;
#[doc = "`write(|w| ..)` method takes [uesta0clr::W](uesta0clr::W) writer structure"]
impl crate::Writable for UESTA0CLR {}
#[doc = "Endpoint Status Clear Register"]
pub mod uesta0clr;
#[doc = "Endpoint Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta0set](uesta0set) module"]
pub type UESTA0SET = crate::Reg<u32, _UESTA0SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA0SET;
#[doc = "`write(|w| ..)` method takes [uesta0set::W](uesta0set::W) writer structure"]
impl crate::Writable for UESTA0SET {}
#[doc = "Endpoint Status Set Register"]
pub mod uesta0set;
#[doc = "Endpoint Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta1](uesta1) module"]
pub type UESTA1 = crate::Reg<u32, _UESTA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA1;
#[doc = "`read()` method returns [uesta1::R](uesta1::R) reader structure"]
impl crate::Readable for UESTA1 {}
#[doc = "Endpoint Status Register"]
pub mod uesta1;
#[doc = "Endpoint Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta1clr](uesta1clr) module"]
pub type UESTA1CLR = crate::Reg<u32, _UESTA1CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA1CLR;
#[doc = "`write(|w| ..)` method takes [uesta1clr::W](uesta1clr::W) writer structure"]
impl crate::Writable for UESTA1CLR {}
#[doc = "Endpoint Status Clear Register"]
pub mod uesta1clr;
#[doc = "Endpoint Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta1set](uesta1set) module"]
pub type UESTA1SET = crate::Reg<u32, _UESTA1SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA1SET;
#[doc = "`write(|w| ..)` method takes [uesta1set::W](uesta1set::W) writer structure"]
impl crate::Writable for UESTA1SET {}
#[doc = "Endpoint Status Set Register"]
pub mod uesta1set;
#[doc = "Endpoint Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta2](uesta2) module"]
pub type UESTA2 = crate::Reg<u32, _UESTA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA2;
#[doc = "`read()` method returns [uesta2::R](uesta2::R) reader structure"]
impl crate::Readable for UESTA2 {}
#[doc = "Endpoint Status Register"]
pub mod uesta2;
#[doc = "Endpoint Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta2clr](uesta2clr) module"]
pub type UESTA2CLR = crate::Reg<u32, _UESTA2CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA2CLR;
#[doc = "`write(|w| ..)` method takes [uesta2clr::W](uesta2clr::W) writer structure"]
impl crate::Writable for UESTA2CLR {}
#[doc = "Endpoint Status Clear Register"]
pub mod uesta2clr;
#[doc = "Endpoint Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta2set](uesta2set) module"]
pub type UESTA2SET = crate::Reg<u32, _UESTA2SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA2SET;
#[doc = "`write(|w| ..)` method takes [uesta2set::W](uesta2set::W) writer structure"]
impl crate::Writable for UESTA2SET {}
#[doc = "Endpoint Status Set Register"]
pub mod uesta2set;
#[doc = "Endpoint Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta3](uesta3) module"]
pub type UESTA3 = crate::Reg<u32, _UESTA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA3;
#[doc = "`read()` method returns [uesta3::R](uesta3::R) reader structure"]
impl crate::Readable for UESTA3 {}
#[doc = "Endpoint Status Register"]
pub mod uesta3;
#[doc = "Endpoint Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta3clr](uesta3clr) module"]
pub type UESTA3CLR = crate::Reg<u32, _UESTA3CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA3CLR;
#[doc = "`write(|w| ..)` method takes [uesta3clr::W](uesta3clr::W) writer structure"]
impl crate::Writable for UESTA3CLR {}
#[doc = "Endpoint Status Clear Register"]
pub mod uesta3clr;
#[doc = "Endpoint Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta3set](uesta3set) module"]
pub type UESTA3SET = crate::Reg<u32, _UESTA3SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA3SET;
#[doc = "`write(|w| ..)` method takes [uesta3set::W](uesta3set::W) writer structure"]
impl crate::Writable for UESTA3SET {}
#[doc = "Endpoint Status Set Register"]
pub mod uesta3set;
#[doc = "Endpoint Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta4](uesta4) module"]
pub type UESTA4 = crate::Reg<u32, _UESTA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA4;
#[doc = "`read()` method returns [uesta4::R](uesta4::R) reader structure"]
impl crate::Readable for UESTA4 {}
#[doc = "Endpoint Status Register"]
pub mod uesta4;
#[doc = "Endpoint Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta4clr](uesta4clr) module"]
pub type UESTA4CLR = crate::Reg<u32, _UESTA4CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA4CLR;
#[doc = "`write(|w| ..)` method takes [uesta4clr::W](uesta4clr::W) writer structure"]
impl crate::Writable for UESTA4CLR {}
#[doc = "Endpoint Status Clear Register"]
pub mod uesta4clr;
#[doc = "Endpoint Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta4set](uesta4set) module"]
pub type UESTA4SET = crate::Reg<u32, _UESTA4SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA4SET;
#[doc = "`write(|w| ..)` method takes [uesta4set::W](uesta4set::W) writer structure"]
impl crate::Writable for UESTA4SET {}
#[doc = "Endpoint Status Set Register"]
pub mod uesta4set;
#[doc = "Endpoint Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta5](uesta5) module"]
pub type UESTA5 = crate::Reg<u32, _UESTA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA5;
#[doc = "`read()` method returns [uesta5::R](uesta5::R) reader structure"]
impl crate::Readable for UESTA5 {}
#[doc = "Endpoint Status Register"]
pub mod uesta5;
#[doc = "Endpoint Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta5clr](uesta5clr) module"]
pub type UESTA5CLR = crate::Reg<u32, _UESTA5CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA5CLR;
#[doc = "`write(|w| ..)` method takes [uesta5clr::W](uesta5clr::W) writer structure"]
impl crate::Writable for UESTA5CLR {}
#[doc = "Endpoint Status Clear Register"]
pub mod uesta5clr;
#[doc = "Endpoint Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta5set](uesta5set) module"]
pub type UESTA5SET = crate::Reg<u32, _UESTA5SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA5SET;
#[doc = "`write(|w| ..)` method takes [uesta5set::W](uesta5set::W) writer structure"]
impl crate::Writable for UESTA5SET {}
#[doc = "Endpoint Status Set Register"]
pub mod uesta5set;
#[doc = "Endpoint Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta6](uesta6) module"]
pub type UESTA6 = crate::Reg<u32, _UESTA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA6;
#[doc = "`read()` method returns [uesta6::R](uesta6::R) reader structure"]
impl crate::Readable for UESTA6 {}
#[doc = "Endpoint Status Register"]
pub mod uesta6;
#[doc = "Endpoint Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta6clr](uesta6clr) module"]
pub type UESTA6CLR = crate::Reg<u32, _UESTA6CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA6CLR;
#[doc = "`write(|w| ..)` method takes [uesta6clr::W](uesta6clr::W) writer structure"]
impl crate::Writable for UESTA6CLR {}
#[doc = "Endpoint Status Clear Register"]
pub mod uesta6clr;
#[doc = "Endpoint Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta6set](uesta6set) module"]
pub type UESTA6SET = crate::Reg<u32, _UESTA6SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA6SET;
#[doc = "`write(|w| ..)` method takes [uesta6set::W](uesta6set::W) writer structure"]
impl crate::Writable for UESTA6SET {}
#[doc = "Endpoint Status Set Register"]
pub mod uesta6set;
#[doc = "Endpoint Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta7](uesta7) module"]
pub type UESTA7 = crate::Reg<u32, _UESTA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA7;
#[doc = "`read()` method returns [uesta7::R](uesta7::R) reader structure"]
impl crate::Readable for UESTA7 {}
#[doc = "Endpoint Status Register"]
pub mod uesta7;
#[doc = "Endpoint Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta7clr](uesta7clr) module"]
pub type UESTA7CLR = crate::Reg<u32, _UESTA7CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA7CLR;
#[doc = "`write(|w| ..)` method takes [uesta7clr::W](uesta7clr::W) writer structure"]
impl crate::Writable for UESTA7CLR {}
#[doc = "Endpoint Status Clear Register"]
pub mod uesta7clr;
#[doc = "Endpoint Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta7set](uesta7set) module"]
pub type UESTA7SET = crate::Reg<u32, _UESTA7SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UESTA7SET;
#[doc = "`write(|w| ..)` method takes [uesta7set::W](uesta7set::W) writer structure"]
impl crate::Writable for UESTA7SET {}
#[doc = "Endpoint Status Set Register"]
pub mod uesta7set;
#[doc = "IP Features Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ufeatures](ufeatures) module"]
pub type UFEATURES = crate::Reg<u32, _UFEATURES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UFEATURES;
#[doc = "`read()` method returns [ufeatures::R](ufeatures::R) reader structure"]
impl crate::Readable for UFEATURES {}
#[doc = "IP Features Register"]
pub mod ufeatures;
#[doc = "Host General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhcon](uhcon) module"]
pub type UHCON = crate::Reg<u32, _UHCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCON;
#[doc = "`read()` method returns [uhcon::R](uhcon::R) reader structure"]
impl crate::Readable for UHCON {}
#[doc = "`write(|w| ..)` method takes [uhcon::W](uhcon::W) writer structure"]
impl crate::Writable for UHCON {}
#[doc = "Host General Control Register"]
pub mod uhcon;
#[doc = "Host Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhfnum](uhfnum) module"]
pub type UHFNUM = crate::Reg<u32, _UHFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHFNUM;
#[doc = "`read()` method returns [uhfnum::R](uhfnum::R) reader structure"]
impl crate::Readable for UHFNUM {}
#[doc = "`write(|w| ..)` method takes [uhfnum::W](uhfnum::W) writer structure"]
impl crate::Writable for UHFNUM {}
#[doc = "Host Frame Number Register"]
pub mod uhfnum;
#[doc = "Host Global Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhint](uhint) module"]
pub type UHINT = crate::Reg<u32, _UHINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHINT;
#[doc = "`read()` method returns [uhint::R](uhint::R) reader structure"]
impl crate::Readable for UHINT {}
#[doc = "Host Global Interrupt Register"]
pub mod uhint;
#[doc = "Host Global Interrrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhintclr](uhintclr) module"]
pub type UHINTCLR = crate::Reg<u32, _UHINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHINTCLR;
#[doc = "`write(|w| ..)` method takes [uhintclr::W](uhintclr::W) writer structure"]
impl crate::Writable for UHINTCLR {}
#[doc = "Host Global Interrrupt Clear Register"]
pub mod uhintclr;
#[doc = "Host Global Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhinte](uhinte) module"]
pub type UHINTE = crate::Reg<u32, _UHINTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHINTE;
#[doc = "`read()` method returns [uhinte::R](uhinte::R) reader structure"]
impl crate::Readable for UHINTE {}
#[doc = "Host Global Interrupt Enable Register"]
pub mod uhinte;
#[doc = "Host Global Interrupt Enable Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhinteclr](uhinteclr) module"]
pub type UHINTECLR = crate::Reg<u32, _UHINTECLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHINTECLR;
#[doc = "`write(|w| ..)` method takes [uhinteclr::W](uhinteclr::W) writer structure"]
impl crate::Writable for UHINTECLR {}
#[doc = "Host Global Interrupt Enable Clear Register"]
pub mod uhinteclr;
#[doc = "Host Global Interrupt Enable Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhinteset](uhinteset) module"]
pub type UHINTESET = crate::Reg<u32, _UHINTESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHINTESET;
#[doc = "`write(|w| ..)` method takes [uhinteset::W](uhinteset::W) writer structure"]
impl crate::Writable for UHINTESET {}
#[doc = "Host Global Interrupt Enable Set Register"]
pub mod uhinteset;
#[doc = "Host Global Interrupt Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhintset](uhintset) module"]
pub type UHINTSET = crate::Reg<u32, _UHINTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHINTSET;
#[doc = "`write(|w| ..)` method takes [uhintset::W](uhintset::W) writer structure"]
impl crate::Writable for UHINTSET {}
#[doc = "Host Global Interrupt Set Register"]
pub mod uhintset;
#[doc = "Host Start of Frame Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhsofc](uhsofc) module"]
pub type UHSOFC = crate::Reg<u32, _UHSOFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHSOFC;
#[doc = "`read()` method returns [uhsofc::R](uhsofc::R) reader structure"]
impl crate::Readable for UHSOFC {}
#[doc = "`write(|w| ..)` method takes [uhsofc::W](uhsofc::W) writer structure"]
impl crate::Writable for UHSOFC {}
#[doc = "Host Start of Frame Control Register"]
pub mod uhsofc;
#[doc = "IP Name Part One: HUSB\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uname1](uname1) module"]
pub type UNAME1 = crate::Reg<u32, _UNAME1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNAME1;
#[doc = "`read()` method returns [uname1::R](uname1::R) reader structure"]
impl crate::Readable for UNAME1 {}
#[doc = "IP Name Part One: HUSB"]
pub mod uname1;
#[doc = "IP Name Part Two: HOST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uname2](uname2) module"]
pub type UNAME2 = crate::Reg<u32, _UNAME2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNAME2;
#[doc = "`read()` method returns [uname2::R](uname2::R) reader structure"]
impl crate::Readable for UNAME2 {}
#[doc = "IP Name Part Two: HOST"]
pub mod uname2;
#[doc = "Pipe Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcfg0](upcfg0) module"]
pub type UPCFG0 = crate::Reg<u32, _UPCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCFG0;
#[doc = "`read()` method returns [upcfg0::R](upcfg0::R) reader structure"]
impl crate::Readable for UPCFG0 {}
#[doc = "`write(|w| ..)` method takes [upcfg0::W](upcfg0::W) writer structure"]
impl crate::Writable for UPCFG0 {}
#[doc = "Pipe Configuration Register"]
pub mod upcfg0;
#[doc = "Pipe Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcfg1](upcfg1) module"]
pub type UPCFG1 = crate::Reg<u32, _UPCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCFG1;
#[doc = "`read()` method returns [upcfg1::R](upcfg1::R) reader structure"]
impl crate::Readable for UPCFG1 {}
#[doc = "`write(|w| ..)` method takes [upcfg1::W](upcfg1::W) writer structure"]
impl crate::Writable for UPCFG1 {}
#[doc = "Pipe Configuration Register"]
pub mod upcfg1;
#[doc = "Pipe Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcfg2](upcfg2) module"]
pub type UPCFG2 = crate::Reg<u32, _UPCFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCFG2;
#[doc = "`read()` method returns [upcfg2::R](upcfg2::R) reader structure"]
impl crate::Readable for UPCFG2 {}
#[doc = "`write(|w| ..)` method takes [upcfg2::W](upcfg2::W) writer structure"]
impl crate::Writable for UPCFG2 {}
#[doc = "Pipe Configuration Register"]
pub mod upcfg2;
#[doc = "Pipe Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcfg3](upcfg3) module"]
pub type UPCFG3 = crate::Reg<u32, _UPCFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCFG3;
#[doc = "`read()` method returns [upcfg3::R](upcfg3::R) reader structure"]
impl crate::Readable for UPCFG3 {}
#[doc = "`write(|w| ..)` method takes [upcfg3::W](upcfg3::W) writer structure"]
impl crate::Writable for UPCFG3 {}
#[doc = "Pipe Configuration Register"]
pub mod upcfg3;
#[doc = "Pipe Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcfg4](upcfg4) module"]
pub type UPCFG4 = crate::Reg<u32, _UPCFG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCFG4;
#[doc = "`read()` method returns [upcfg4::R](upcfg4::R) reader structure"]
impl crate::Readable for UPCFG4 {}
#[doc = "`write(|w| ..)` method takes [upcfg4::W](upcfg4::W) writer structure"]
impl crate::Writable for UPCFG4 {}
#[doc = "Pipe Configuration Register"]
pub mod upcfg4;
#[doc = "Pipe Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcfg5](upcfg5) module"]
pub type UPCFG5 = crate::Reg<u32, _UPCFG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCFG5;
#[doc = "`read()` method returns [upcfg5::R](upcfg5::R) reader structure"]
impl crate::Readable for UPCFG5 {}
#[doc = "`write(|w| ..)` method takes [upcfg5::W](upcfg5::W) writer structure"]
impl crate::Writable for UPCFG5 {}
#[doc = "Pipe Configuration Register"]
pub mod upcfg5;
#[doc = "Pipe Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcfg6](upcfg6) module"]
pub type UPCFG6 = crate::Reg<u32, _UPCFG6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCFG6;
#[doc = "`read()` method returns [upcfg6::R](upcfg6::R) reader structure"]
impl crate::Readable for UPCFG6 {}
#[doc = "`write(|w| ..)` method takes [upcfg6::W](upcfg6::W) writer structure"]
impl crate::Writable for UPCFG6 {}
#[doc = "Pipe Configuration Register"]
pub mod upcfg6;
#[doc = "Pipe Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcfg7](upcfg7) module"]
pub type UPCFG7 = crate::Reg<u32, _UPCFG7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCFG7;
#[doc = "`read()` method returns [upcfg7::R](upcfg7::R) reader structure"]
impl crate::Readable for UPCFG7 {}
#[doc = "`write(|w| ..)` method takes [upcfg7::W](upcfg7::W) writer structure"]
impl crate::Writable for UPCFG7 {}
#[doc = "Pipe Configuration Register"]
pub mod upcfg7;
#[doc = "Pipe Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon0](upcon0) module"]
pub type UPCON0 = crate::Reg<u32, _UPCON0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON0;
#[doc = "`read()` method returns [upcon0::R](upcon0::R) reader structure"]
impl crate::Readable for UPCON0 {}
#[doc = "Pipe Control Register"]
pub mod upcon0;
#[doc = "Pipe Control Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon0clr](upcon0clr) module"]
pub type UPCON0CLR = crate::Reg<u32, _UPCON0CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON0CLR;
#[doc = "`write(|w| ..)` method takes [upcon0clr::W](upcon0clr::W) writer structure"]
impl crate::Writable for UPCON0CLR {}
#[doc = "Pipe Control Clear Register"]
pub mod upcon0clr;
#[doc = "Pipe Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon0set](upcon0set) module"]
pub type UPCON0SET = crate::Reg<u32, _UPCON0SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON0SET;
#[doc = "`write(|w| ..)` method takes [upcon0set::W](upcon0set::W) writer structure"]
impl crate::Writable for UPCON0SET {}
#[doc = "Pipe Control Set Register"]
pub mod upcon0set;
#[doc = "Pipe Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon1](upcon1) module"]
pub type UPCON1 = crate::Reg<u32, _UPCON1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON1;
#[doc = "`read()` method returns [upcon1::R](upcon1::R) reader structure"]
impl crate::Readable for UPCON1 {}
#[doc = "Pipe Control Register"]
pub mod upcon1;
#[doc = "Pipe Control Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon1clr](upcon1clr) module"]
pub type UPCON1CLR = crate::Reg<u32, _UPCON1CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON1CLR;
#[doc = "`write(|w| ..)` method takes [upcon1clr::W](upcon1clr::W) writer structure"]
impl crate::Writable for UPCON1CLR {}
#[doc = "Pipe Control Clear Register"]
pub mod upcon1clr;
#[doc = "Pipe Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon1set](upcon1set) module"]
pub type UPCON1SET = crate::Reg<u32, _UPCON1SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON1SET;
#[doc = "`write(|w| ..)` method takes [upcon1set::W](upcon1set::W) writer structure"]
impl crate::Writable for UPCON1SET {}
#[doc = "Pipe Control Set Register"]
pub mod upcon1set;
#[doc = "Pipe Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon2](upcon2) module"]
pub type UPCON2 = crate::Reg<u32, _UPCON2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON2;
#[doc = "`read()` method returns [upcon2::R](upcon2::R) reader structure"]
impl crate::Readable for UPCON2 {}
#[doc = "Pipe Control Register"]
pub mod upcon2;
#[doc = "Pipe Control Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon2clr](upcon2clr) module"]
pub type UPCON2CLR = crate::Reg<u32, _UPCON2CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON2CLR;
#[doc = "`write(|w| ..)` method takes [upcon2clr::W](upcon2clr::W) writer structure"]
impl crate::Writable for UPCON2CLR {}
#[doc = "Pipe Control Clear Register"]
pub mod upcon2clr;
#[doc = "Pipe Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon2set](upcon2set) module"]
pub type UPCON2SET = crate::Reg<u32, _UPCON2SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON2SET;
#[doc = "`write(|w| ..)` method takes [upcon2set::W](upcon2set::W) writer structure"]
impl crate::Writable for UPCON2SET {}
#[doc = "Pipe Control Set Register"]
pub mod upcon2set;
#[doc = "Pipe Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon3](upcon3) module"]
pub type UPCON3 = crate::Reg<u32, _UPCON3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON3;
#[doc = "`read()` method returns [upcon3::R](upcon3::R) reader structure"]
impl crate::Readable for UPCON3 {}
#[doc = "Pipe Control Register"]
pub mod upcon3;
#[doc = "Pipe Control Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon3clr](upcon3clr) module"]
pub type UPCON3CLR = crate::Reg<u32, _UPCON3CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON3CLR;
#[doc = "`write(|w| ..)` method takes [upcon3clr::W](upcon3clr::W) writer structure"]
impl crate::Writable for UPCON3CLR {}
#[doc = "Pipe Control Clear Register"]
pub mod upcon3clr;
#[doc = "Pipe Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon3set](upcon3set) module"]
pub type UPCON3SET = crate::Reg<u32, _UPCON3SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON3SET;
#[doc = "`write(|w| ..)` method takes [upcon3set::W](upcon3set::W) writer structure"]
impl crate::Writable for UPCON3SET {}
#[doc = "Pipe Control Set Register"]
pub mod upcon3set;
#[doc = "Pipe Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon4](upcon4) module"]
pub type UPCON4 = crate::Reg<u32, _UPCON4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON4;
#[doc = "`read()` method returns [upcon4::R](upcon4::R) reader structure"]
impl crate::Readable for UPCON4 {}
#[doc = "Pipe Control Register"]
pub mod upcon4;
#[doc = "Pipe Control Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon4clr](upcon4clr) module"]
pub type UPCON4CLR = crate::Reg<u32, _UPCON4CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON4CLR;
#[doc = "`write(|w| ..)` method takes [upcon4clr::W](upcon4clr::W) writer structure"]
impl crate::Writable for UPCON4CLR {}
#[doc = "Pipe Control Clear Register"]
pub mod upcon4clr;
#[doc = "Pipe Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon4set](upcon4set) module"]
pub type UPCON4SET = crate::Reg<u32, _UPCON4SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON4SET;
#[doc = "`write(|w| ..)` method takes [upcon4set::W](upcon4set::W) writer structure"]
impl crate::Writable for UPCON4SET {}
#[doc = "Pipe Control Set Register"]
pub mod upcon4set;
#[doc = "Pipe Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon5](upcon5) module"]
pub type UPCON5 = crate::Reg<u32, _UPCON5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON5;
#[doc = "`read()` method returns [upcon5::R](upcon5::R) reader structure"]
impl crate::Readable for UPCON5 {}
#[doc = "Pipe Control Register"]
pub mod upcon5;
#[doc = "Pipe Control Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon5clr](upcon5clr) module"]
pub type UPCON5CLR = crate::Reg<u32, _UPCON5CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON5CLR;
#[doc = "`write(|w| ..)` method takes [upcon5clr::W](upcon5clr::W) writer structure"]
impl crate::Writable for UPCON5CLR {}
#[doc = "Pipe Control Clear Register"]
pub mod upcon5clr;
#[doc = "Pipe Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon5set](upcon5set) module"]
pub type UPCON5SET = crate::Reg<u32, _UPCON5SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON5SET;
#[doc = "`write(|w| ..)` method takes [upcon5set::W](upcon5set::W) writer structure"]
impl crate::Writable for UPCON5SET {}
#[doc = "Pipe Control Set Register"]
pub mod upcon5set;
#[doc = "Pipe Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon6](upcon6) module"]
pub type UPCON6 = crate::Reg<u32, _UPCON6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON6;
#[doc = "`read()` method returns [upcon6::R](upcon6::R) reader structure"]
impl crate::Readable for UPCON6 {}
#[doc = "Pipe Control Register"]
pub mod upcon6;
#[doc = "Pipe Control Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon6clr](upcon6clr) module"]
pub type UPCON6CLR = crate::Reg<u32, _UPCON6CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON6CLR;
#[doc = "`write(|w| ..)` method takes [upcon6clr::W](upcon6clr::W) writer structure"]
impl crate::Writable for UPCON6CLR {}
#[doc = "Pipe Control Clear Register"]
pub mod upcon6clr;
#[doc = "Pipe Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon6set](upcon6set) module"]
pub type UPCON6SET = crate::Reg<u32, _UPCON6SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON6SET;
#[doc = "`write(|w| ..)` method takes [upcon6set::W](upcon6set::W) writer structure"]
impl crate::Writable for UPCON6SET {}
#[doc = "Pipe Control Set Register"]
pub mod upcon6set;
#[doc = "Pipe Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon7](upcon7) module"]
pub type UPCON7 = crate::Reg<u32, _UPCON7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON7;
#[doc = "`read()` method returns [upcon7::R](upcon7::R) reader structure"]
impl crate::Readable for UPCON7 {}
#[doc = "Pipe Control Register"]
pub mod upcon7;
#[doc = "Pipe Control Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon7clr](upcon7clr) module"]
pub type UPCON7CLR = crate::Reg<u32, _UPCON7CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON7CLR;
#[doc = "`write(|w| ..)` method takes [upcon7clr::W](upcon7clr::W) writer structure"]
impl crate::Writable for UPCON7CLR {}
#[doc = "Pipe Control Clear Register"]
pub mod upcon7clr;
#[doc = "Pipe Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon7set](upcon7set) module"]
pub type UPCON7SET = crate::Reg<u32, _UPCON7SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCON7SET;
#[doc = "`write(|w| ..)` method takes [upcon7set::W](upcon7set::W) writer structure"]
impl crate::Writable for UPCON7SET {}
#[doc = "Pipe Control Set Register"]
pub mod upcon7set;
#[doc = "Pipe In Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upinrq0](upinrq0) module"]
pub type UPINRQ0 = crate::Reg<u32, _UPINRQ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPINRQ0;
#[doc = "`read()` method returns [upinrq0::R](upinrq0::R) reader structure"]
impl crate::Readable for UPINRQ0 {}
#[doc = "`write(|w| ..)` method takes [upinrq0::W](upinrq0::W) writer structure"]
impl crate::Writable for UPINRQ0 {}
#[doc = "Pipe In Request"]
pub mod upinrq0;
#[doc = "Pipe In Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upinrq1](upinrq1) module"]
pub type UPINRQ1 = crate::Reg<u32, _UPINRQ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPINRQ1;
#[doc = "`read()` method returns [upinrq1::R](upinrq1::R) reader structure"]
impl crate::Readable for UPINRQ1 {}
#[doc = "`write(|w| ..)` method takes [upinrq1::W](upinrq1::W) writer structure"]
impl crate::Writable for UPINRQ1 {}
#[doc = "Pipe In Request"]
pub mod upinrq1;
#[doc = "Pipe In Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upinrq2](upinrq2) module"]
pub type UPINRQ2 = crate::Reg<u32, _UPINRQ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPINRQ2;
#[doc = "`read()` method returns [upinrq2::R](upinrq2::R) reader structure"]
impl crate::Readable for UPINRQ2 {}
#[doc = "`write(|w| ..)` method takes [upinrq2::W](upinrq2::W) writer structure"]
impl crate::Writable for UPINRQ2 {}
#[doc = "Pipe In Request"]
pub mod upinrq2;
#[doc = "Pipe In Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upinrq3](upinrq3) module"]
pub type UPINRQ3 = crate::Reg<u32, _UPINRQ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPINRQ3;
#[doc = "`read()` method returns [upinrq3::R](upinrq3::R) reader structure"]
impl crate::Readable for UPINRQ3 {}
#[doc = "`write(|w| ..)` method takes [upinrq3::W](upinrq3::W) writer structure"]
impl crate::Writable for UPINRQ3 {}
#[doc = "Pipe In Request"]
pub mod upinrq3;
#[doc = "Pipe In Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upinrq4](upinrq4) module"]
pub type UPINRQ4 = crate::Reg<u32, _UPINRQ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPINRQ4;
#[doc = "`read()` method returns [upinrq4::R](upinrq4::R) reader structure"]
impl crate::Readable for UPINRQ4 {}
#[doc = "`write(|w| ..)` method takes [upinrq4::W](upinrq4::W) writer structure"]
impl crate::Writable for UPINRQ4 {}
#[doc = "Pipe In Request"]
pub mod upinrq4;
#[doc = "Pipe In Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upinrq5](upinrq5) module"]
pub type UPINRQ5 = crate::Reg<u32, _UPINRQ5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPINRQ5;
#[doc = "`read()` method returns [upinrq5::R](upinrq5::R) reader structure"]
impl crate::Readable for UPINRQ5 {}
#[doc = "`write(|w| ..)` method takes [upinrq5::W](upinrq5::W) writer structure"]
impl crate::Writable for UPINRQ5 {}
#[doc = "Pipe In Request"]
pub mod upinrq5;
#[doc = "Pipe In Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upinrq6](upinrq6) module"]
pub type UPINRQ6 = crate::Reg<u32, _UPINRQ6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPINRQ6;
#[doc = "`read()` method returns [upinrq6::R](upinrq6::R) reader structure"]
impl crate::Readable for UPINRQ6 {}
#[doc = "`write(|w| ..)` method takes [upinrq6::W](upinrq6::W) writer structure"]
impl crate::Writable for UPINRQ6 {}
#[doc = "Pipe In Request"]
pub mod upinrq6;
#[doc = "Pipe In Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upinrq7](upinrq7) module"]
pub type UPINRQ7 = crate::Reg<u32, _UPINRQ7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPINRQ7;
#[doc = "`read()` method returns [upinrq7::R](upinrq7::R) reader structure"]
impl crate::Readable for UPINRQ7 {}
#[doc = "`write(|w| ..)` method takes [upinrq7::W](upinrq7::W) writer structure"]
impl crate::Writable for UPINRQ7 {}
#[doc = "Pipe In Request"]
pub mod upinrq7;
#[doc = "Pipe Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uprst](uprst) module"]
pub type UPRST = crate::Reg<u32, _UPRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPRST;
#[doc = "`read()` method returns [uprst::R](uprst::R) reader structure"]
impl crate::Readable for UPRST {}
#[doc = "`write(|w| ..)` method takes [uprst::W](uprst::W) writer structure"]
impl crate::Writable for UPRST {}
#[doc = "Pipe Reset Register"]
pub mod uprst;
#[doc = "Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta0](upsta0) module"]
pub type UPSTA0 = crate::Reg<u32, _UPSTA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA0;
#[doc = "`read()` method returns [upsta0::R](upsta0::R) reader structure"]
impl crate::Readable for UPSTA0 {}
#[doc = "Pipe Status Register"]
pub mod upsta0;
#[doc = "Pipe Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta0clr](upsta0clr) module"]
pub type UPSTA0CLR = crate::Reg<u32, _UPSTA0CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA0CLR;
#[doc = "`write(|w| ..)` method takes [upsta0clr::W](upsta0clr::W) writer structure"]
impl crate::Writable for UPSTA0CLR {}
#[doc = "Pipe Status Clear Register"]
pub mod upsta0clr;
#[doc = "Pipe Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta0set](upsta0set) module"]
pub type UPSTA0SET = crate::Reg<u32, _UPSTA0SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA0SET;
#[doc = "`write(|w| ..)` method takes [upsta0set::W](upsta0set::W) writer structure"]
impl crate::Writable for UPSTA0SET {}
#[doc = "Pipe Status Set Register"]
pub mod upsta0set;
#[doc = "Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta1](upsta1) module"]
pub type UPSTA1 = crate::Reg<u32, _UPSTA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA1;
#[doc = "`read()` method returns [upsta1::R](upsta1::R) reader structure"]
impl crate::Readable for UPSTA1 {}
#[doc = "Pipe Status Register"]
pub mod upsta1;
#[doc = "Pipe Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta1clr](upsta1clr) module"]
pub type UPSTA1CLR = crate::Reg<u32, _UPSTA1CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA1CLR;
#[doc = "`write(|w| ..)` method takes [upsta1clr::W](upsta1clr::W) writer structure"]
impl crate::Writable for UPSTA1CLR {}
#[doc = "Pipe Status Clear Register"]
pub mod upsta1clr;
#[doc = "Pipe Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta1set](upsta1set) module"]
pub type UPSTA1SET = crate::Reg<u32, _UPSTA1SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA1SET;
#[doc = "`write(|w| ..)` method takes [upsta1set::W](upsta1set::W) writer structure"]
impl crate::Writable for UPSTA1SET {}
#[doc = "Pipe Status Set Register"]
pub mod upsta1set;
#[doc = "Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta2](upsta2) module"]
pub type UPSTA2 = crate::Reg<u32, _UPSTA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA2;
#[doc = "`read()` method returns [upsta2::R](upsta2::R) reader structure"]
impl crate::Readable for UPSTA2 {}
#[doc = "Pipe Status Register"]
pub mod upsta2;
#[doc = "Pipe Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta2clr](upsta2clr) module"]
pub type UPSTA2CLR = crate::Reg<u32, _UPSTA2CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA2CLR;
#[doc = "`write(|w| ..)` method takes [upsta2clr::W](upsta2clr::W) writer structure"]
impl crate::Writable for UPSTA2CLR {}
#[doc = "Pipe Status Clear Register"]
pub mod upsta2clr;
#[doc = "Pipe Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta2set](upsta2set) module"]
pub type UPSTA2SET = crate::Reg<u32, _UPSTA2SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA2SET;
#[doc = "`write(|w| ..)` method takes [upsta2set::W](upsta2set::W) writer structure"]
impl crate::Writable for UPSTA2SET {}
#[doc = "Pipe Status Set Register"]
pub mod upsta2set;
#[doc = "Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta3](upsta3) module"]
pub type UPSTA3 = crate::Reg<u32, _UPSTA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA3;
#[doc = "`read()` method returns [upsta3::R](upsta3::R) reader structure"]
impl crate::Readable for UPSTA3 {}
#[doc = "Pipe Status Register"]
pub mod upsta3;
#[doc = "Pipe Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta3clr](upsta3clr) module"]
pub type UPSTA3CLR = crate::Reg<u32, _UPSTA3CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA3CLR;
#[doc = "`write(|w| ..)` method takes [upsta3clr::W](upsta3clr::W) writer structure"]
impl crate::Writable for UPSTA3CLR {}
#[doc = "Pipe Status Clear Register"]
pub mod upsta3clr;
#[doc = "Pipe Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta3set](upsta3set) module"]
pub type UPSTA3SET = crate::Reg<u32, _UPSTA3SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA3SET;
#[doc = "`write(|w| ..)` method takes [upsta3set::W](upsta3set::W) writer structure"]
impl crate::Writable for UPSTA3SET {}
#[doc = "Pipe Status Set Register"]
pub mod upsta3set;
#[doc = "Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta4](upsta4) module"]
pub type UPSTA4 = crate::Reg<u32, _UPSTA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA4;
#[doc = "`read()` method returns [upsta4::R](upsta4::R) reader structure"]
impl crate::Readable for UPSTA4 {}
#[doc = "Pipe Status Register"]
pub mod upsta4;
#[doc = "Pipe Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta4clr](upsta4clr) module"]
pub type UPSTA4CLR = crate::Reg<u32, _UPSTA4CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA4CLR;
#[doc = "`write(|w| ..)` method takes [upsta4clr::W](upsta4clr::W) writer structure"]
impl crate::Writable for UPSTA4CLR {}
#[doc = "Pipe Status Clear Register"]
pub mod upsta4clr;
#[doc = "Pipe Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta4set](upsta4set) module"]
pub type UPSTA4SET = crate::Reg<u32, _UPSTA4SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA4SET;
#[doc = "`write(|w| ..)` method takes [upsta4set::W](upsta4set::W) writer structure"]
impl crate::Writable for UPSTA4SET {}
#[doc = "Pipe Status Set Register"]
pub mod upsta4set;
#[doc = "Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta5](upsta5) module"]
pub type UPSTA5 = crate::Reg<u32, _UPSTA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA5;
#[doc = "`read()` method returns [upsta5::R](upsta5::R) reader structure"]
impl crate::Readable for UPSTA5 {}
#[doc = "Pipe Status Register"]
pub mod upsta5;
#[doc = "Pipe Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta5clr](upsta5clr) module"]
pub type UPSTA5CLR = crate::Reg<u32, _UPSTA5CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA5CLR;
#[doc = "`write(|w| ..)` method takes [upsta5clr::W](upsta5clr::W) writer structure"]
impl crate::Writable for UPSTA5CLR {}
#[doc = "Pipe Status Clear Register"]
pub mod upsta5clr;
#[doc = "Pipe Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta5set](upsta5set) module"]
pub type UPSTA5SET = crate::Reg<u32, _UPSTA5SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA5SET;
#[doc = "`write(|w| ..)` method takes [upsta5set::W](upsta5set::W) writer structure"]
impl crate::Writable for UPSTA5SET {}
#[doc = "Pipe Status Set Register"]
pub mod upsta5set;
#[doc = "Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta6](upsta6) module"]
pub type UPSTA6 = crate::Reg<u32, _UPSTA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA6;
#[doc = "`read()` method returns [upsta6::R](upsta6::R) reader structure"]
impl crate::Readable for UPSTA6 {}
#[doc = "Pipe Status Register"]
pub mod upsta6;
#[doc = "Pipe Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta6clr](upsta6clr) module"]
pub type UPSTA6CLR = crate::Reg<u32, _UPSTA6CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA6CLR;
#[doc = "`write(|w| ..)` method takes [upsta6clr::W](upsta6clr::W) writer structure"]
impl crate::Writable for UPSTA6CLR {}
#[doc = "Pipe Status Clear Register"]
pub mod upsta6clr;
#[doc = "Pipe Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta6set](upsta6set) module"]
pub type UPSTA6SET = crate::Reg<u32, _UPSTA6SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA6SET;
#[doc = "`write(|w| ..)` method takes [upsta6set::W](upsta6set::W) writer structure"]
impl crate::Writable for UPSTA6SET {}
#[doc = "Pipe Status Set Register"]
pub mod upsta6set;
#[doc = "Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta7](upsta7) module"]
pub type UPSTA7 = crate::Reg<u32, _UPSTA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA7;
#[doc = "`read()` method returns [upsta7::R](upsta7::R) reader structure"]
impl crate::Readable for UPSTA7 {}
#[doc = "Pipe Status Register"]
pub mod upsta7;
#[doc = "Pipe Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta7clr](upsta7clr) module"]
pub type UPSTA7CLR = crate::Reg<u32, _UPSTA7CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA7CLR;
#[doc = "`write(|w| ..)` method takes [upsta7clr::W](upsta7clr::W) writer structure"]
impl crate::Writable for UPSTA7CLR {}
#[doc = "Pipe Status Clear Register"]
pub mod upsta7clr;
#[doc = "Pipe Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta7set](upsta7set) module"]
pub type UPSTA7SET = crate::Reg<u32, _UPSTA7SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPSTA7SET;
#[doc = "`write(|w| ..)` method takes [upsta7set::W](upsta7set::W) writer structure"]
impl crate::Writable for UPSTA7SET {}
#[doc = "Pipe Status Set Register"]
pub mod upsta7set;
#[doc = "General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbcon](usbcon) module"]
pub type USBCON = crate::Reg<u32, _USBCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCON;
#[doc = "`read()` method returns [usbcon::R](usbcon::R) reader structure"]
impl crate::Readable for USBCON {}
#[doc = "`write(|w| ..)` method takes [usbcon::W](usbcon::W) writer structure"]
impl crate::Writable for USBCON {}
#[doc = "General Control Register"]
pub mod usbcon;
#[doc = "USB internal finite state machine\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbfsm](usbfsm) module"]
pub type USBFSM = crate::Reg<u32, _USBFSM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBFSM;
#[doc = "`read()` method returns [usbfsm::R](usbfsm::R) reader structure"]
impl crate::Readable for USBFSM {}
#[doc = "USB internal finite state machine"]
pub mod usbfsm;
#[doc = "General Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbsta](usbsta) module"]
pub type USBSTA = crate::Reg<u32, _USBSTA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBSTA;
#[doc = "`read()` method returns [usbsta::R](usbsta::R) reader structure"]
impl crate::Readable for USBSTA {}
#[doc = "General Status Register"]
pub mod usbsta;
#[doc = "General Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbstaclr](usbstaclr) module"]
pub type USBSTACLR = crate::Reg<u32, _USBSTACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBSTACLR;
#[doc = "`write(|w| ..)` method takes [usbstaclr::W](usbstaclr::W) writer structure"]
impl crate::Writable for USBSTACLR {}
#[doc = "General Status Clear Register"]
pub mod usbstaclr;
#[doc = "General Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbstaset](usbstaset) module"]
pub type USBSTASET = crate::Reg<u32, _USBSTASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBSTASET;
#[doc = "`write(|w| ..)` method takes [usbstaset::W](usbstaset::W) writer structure"]
impl crate::Writable for USBSTASET {}
#[doc = "General Status Set Register"]
pub mod usbstaset;
#[doc = "IP Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uvers](uvers) module"]
pub type UVERS = crate::Reg<u32, _UVERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UVERS;
#[doc = "`read()` method returns [uvers::R](uvers::R) reader structure"]
impl crate::Readable for UVERS {}
#[doc = "IP Version Register"]
pub mod uvers;
