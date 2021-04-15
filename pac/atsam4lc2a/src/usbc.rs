#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device General Control Register"]
    pub udcon: crate::Reg<udcon::UDCON_SPEC>,
    #[doc = "0x04 - Device Global Interupt Register"]
    pub udint: crate::Reg<udint::UDINT_SPEC>,
    #[doc = "0x08 - Device Global Interrupt Clear Register"]
    pub udintclr: crate::Reg<udintclr::UDINTCLR_SPEC>,
    #[doc = "0x0c - Device Global Interrupt Set Regsiter"]
    pub udintset: crate::Reg<udintset::UDINTSET_SPEC>,
    #[doc = "0x10 - Device Global Interrupt Enable Register"]
    pub udinte: crate::Reg<udinte::UDINTE_SPEC>,
    #[doc = "0x14 - Device Global Interrupt Enable Clear Register"]
    pub udinteclr: crate::Reg<udinteclr::UDINTECLR_SPEC>,
    #[doc = "0x18 - Device Global Interrupt Enable Set Register"]
    pub udinteset: crate::Reg<udinteset::UDINTESET_SPEC>,
    #[doc = "0x1c - Endpoint Enable/Reset Register"]
    pub uerst: crate::Reg<uerst::UERST_SPEC>,
    #[doc = "0x20 - Device Frame Number Register"]
    pub udfnum: crate::Reg<udfnum::UDFNUM_SPEC>,
    _reserved9: [u8; 220usize],
    #[doc = "0x100 - Endpoint Configuration Register"]
    pub uecfg0: crate::Reg<uecfg0::UECFG0_SPEC>,
    #[doc = "0x104 - Endpoint Configuration Register"]
    pub uecfg1: crate::Reg<uecfg1::UECFG1_SPEC>,
    #[doc = "0x108 - Endpoint Configuration Register"]
    pub uecfg2: crate::Reg<uecfg2::UECFG2_SPEC>,
    #[doc = "0x10c - Endpoint Configuration Register"]
    pub uecfg3: crate::Reg<uecfg3::UECFG3_SPEC>,
    #[doc = "0x110 - Endpoint Configuration Register"]
    pub uecfg4: crate::Reg<uecfg4::UECFG4_SPEC>,
    #[doc = "0x114 - Endpoint Configuration Register"]
    pub uecfg5: crate::Reg<uecfg5::UECFG5_SPEC>,
    #[doc = "0x118 - Endpoint Configuration Register"]
    pub uecfg6: crate::Reg<uecfg6::UECFG6_SPEC>,
    #[doc = "0x11c - Endpoint Configuration Register"]
    pub uecfg7: crate::Reg<uecfg7::UECFG7_SPEC>,
    _reserved17: [u8; 16usize],
    #[doc = "0x130 - Endpoint Status Register"]
    pub uesta0: crate::Reg<uesta0::UESTA0_SPEC>,
    #[doc = "0x134 - Endpoint Status Register"]
    pub uesta1: crate::Reg<uesta1::UESTA1_SPEC>,
    #[doc = "0x138 - Endpoint Status Register"]
    pub uesta2: crate::Reg<uesta2::UESTA2_SPEC>,
    #[doc = "0x13c - Endpoint Status Register"]
    pub uesta3: crate::Reg<uesta3::UESTA3_SPEC>,
    #[doc = "0x140 - Endpoint Status Register"]
    pub uesta4: crate::Reg<uesta4::UESTA4_SPEC>,
    #[doc = "0x144 - Endpoint Status Register"]
    pub uesta5: crate::Reg<uesta5::UESTA5_SPEC>,
    #[doc = "0x148 - Endpoint Status Register"]
    pub uesta6: crate::Reg<uesta6::UESTA6_SPEC>,
    #[doc = "0x14c - Endpoint Status Register"]
    pub uesta7: crate::Reg<uesta7::UESTA7_SPEC>,
    _reserved25: [u8; 16usize],
    #[doc = "0x160 - Endpoint Status Clear Register"]
    pub uesta0clr: crate::Reg<uesta0clr::UESTA0CLR_SPEC>,
    #[doc = "0x164 - Endpoint Status Clear Register"]
    pub uesta1clr: crate::Reg<uesta1clr::UESTA1CLR_SPEC>,
    #[doc = "0x168 - Endpoint Status Clear Register"]
    pub uesta2clr: crate::Reg<uesta2clr::UESTA2CLR_SPEC>,
    #[doc = "0x16c - Endpoint Status Clear Register"]
    pub uesta3clr: crate::Reg<uesta3clr::UESTA3CLR_SPEC>,
    #[doc = "0x170 - Endpoint Status Clear Register"]
    pub uesta4clr: crate::Reg<uesta4clr::UESTA4CLR_SPEC>,
    #[doc = "0x174 - Endpoint Status Clear Register"]
    pub uesta5clr: crate::Reg<uesta5clr::UESTA5CLR_SPEC>,
    #[doc = "0x178 - Endpoint Status Clear Register"]
    pub uesta6clr: crate::Reg<uesta6clr::UESTA6CLR_SPEC>,
    #[doc = "0x17c - Endpoint Status Clear Register"]
    pub uesta7clr: crate::Reg<uesta7clr::UESTA7CLR_SPEC>,
    _reserved33: [u8; 16usize],
    #[doc = "0x190 - Endpoint Status Set Register"]
    pub uesta0set: crate::Reg<uesta0set::UESTA0SET_SPEC>,
    #[doc = "0x194 - Endpoint Status Set Register"]
    pub uesta1set: crate::Reg<uesta1set::UESTA1SET_SPEC>,
    #[doc = "0x198 - Endpoint Status Set Register"]
    pub uesta2set: crate::Reg<uesta2set::UESTA2SET_SPEC>,
    #[doc = "0x19c - Endpoint Status Set Register"]
    pub uesta3set: crate::Reg<uesta3set::UESTA3SET_SPEC>,
    #[doc = "0x1a0 - Endpoint Status Set Register"]
    pub uesta4set: crate::Reg<uesta4set::UESTA4SET_SPEC>,
    #[doc = "0x1a4 - Endpoint Status Set Register"]
    pub uesta5set: crate::Reg<uesta5set::UESTA5SET_SPEC>,
    #[doc = "0x1a8 - Endpoint Status Set Register"]
    pub uesta6set: crate::Reg<uesta6set::UESTA6SET_SPEC>,
    #[doc = "0x1ac - Endpoint Status Set Register"]
    pub uesta7set: crate::Reg<uesta7set::UESTA7SET_SPEC>,
    _reserved41: [u8; 16usize],
    #[doc = "0x1c0 - Endpoint Control Register"]
    pub uecon0: crate::Reg<uecon0::UECON0_SPEC>,
    #[doc = "0x1c4 - Endpoint Control Register"]
    pub uecon1: crate::Reg<uecon1::UECON1_SPEC>,
    #[doc = "0x1c8 - Endpoint Control Register"]
    pub uecon2: crate::Reg<uecon2::UECON2_SPEC>,
    #[doc = "0x1cc - Endpoint Control Register"]
    pub uecon3: crate::Reg<uecon3::UECON3_SPEC>,
    #[doc = "0x1d0 - Endpoint Control Register"]
    pub uecon4: crate::Reg<uecon4::UECON4_SPEC>,
    #[doc = "0x1d4 - Endpoint Control Register"]
    pub uecon5: crate::Reg<uecon5::UECON5_SPEC>,
    #[doc = "0x1d8 - Endpoint Control Register"]
    pub uecon6: crate::Reg<uecon6::UECON6_SPEC>,
    #[doc = "0x1dc - Endpoint Control Register"]
    pub uecon7: crate::Reg<uecon7::UECON7_SPEC>,
    _reserved49: [u8; 16usize],
    #[doc = "0x1f0 - Endpoint Control Set Register"]
    pub uecon0set: crate::Reg<uecon0set::UECON0SET_SPEC>,
    #[doc = "0x1f4 - Endpoint Control Set Register"]
    pub uecon1set: crate::Reg<uecon1set::UECON1SET_SPEC>,
    #[doc = "0x1f8 - Endpoint Control Set Register"]
    pub uecon2set: crate::Reg<uecon2set::UECON2SET_SPEC>,
    #[doc = "0x1fc - Endpoint Control Set Register"]
    pub uecon3set: crate::Reg<uecon3set::UECON3SET_SPEC>,
    #[doc = "0x200 - Endpoint Control Set Register"]
    pub uecon4set: crate::Reg<uecon4set::UECON4SET_SPEC>,
    #[doc = "0x204 - Endpoint Control Set Register"]
    pub uecon5set: crate::Reg<uecon5set::UECON5SET_SPEC>,
    #[doc = "0x208 - Endpoint Control Set Register"]
    pub uecon6set: crate::Reg<uecon6set::UECON6SET_SPEC>,
    #[doc = "0x20c - Endpoint Control Set Register"]
    pub uecon7set: crate::Reg<uecon7set::UECON7SET_SPEC>,
    _reserved57: [u8; 16usize],
    #[doc = "0x220 - Endpoint Control Clear Register"]
    pub uecon0clr: crate::Reg<uecon0clr::UECON0CLR_SPEC>,
    #[doc = "0x224 - TXINE Clear"]
    pub uecon1clr: crate::Reg<uecon1clr::UECON1CLR_SPEC>,
    #[doc = "0x228 - TXINE Clear"]
    pub uecon2clr: crate::Reg<uecon2clr::UECON2CLR_SPEC>,
    #[doc = "0x22c - TXINE Clear"]
    pub uecon3clr: crate::Reg<uecon3clr::UECON3CLR_SPEC>,
    #[doc = "0x230 - TXINE Clear"]
    pub uecon4clr: crate::Reg<uecon4clr::UECON4CLR_SPEC>,
    #[doc = "0x234 - TXINE Clear"]
    pub uecon5clr: crate::Reg<uecon5clr::UECON5CLR_SPEC>,
    #[doc = "0x238 - TXINE Clear"]
    pub uecon6clr: crate::Reg<uecon6clr::UECON6CLR_SPEC>,
    #[doc = "0x23c - TXINE Clear"]
    pub uecon7clr: crate::Reg<uecon7clr::UECON7CLR_SPEC>,
    _reserved65: [u8; 448usize],
    #[doc = "0x400 - Host General Control Register"]
    pub uhcon: crate::Reg<uhcon::UHCON_SPEC>,
    #[doc = "0x404 - Host Global Interrupt Register"]
    pub uhint: crate::Reg<uhint::UHINT_SPEC>,
    #[doc = "0x408 - Host Global Interrrupt Clear Register"]
    pub uhintclr: crate::Reg<uhintclr::UHINTCLR_SPEC>,
    #[doc = "0x40c - Host Global Interrupt Set Register"]
    pub uhintset: crate::Reg<uhintset::UHINTSET_SPEC>,
    #[doc = "0x410 - Host Global Interrupt Enable Register"]
    pub uhinte: crate::Reg<uhinte::UHINTE_SPEC>,
    #[doc = "0x414 - Host Global Interrupt Enable Clear Register"]
    pub uhinteclr: crate::Reg<uhinteclr::UHINTECLR_SPEC>,
    #[doc = "0x418 - Host Global Interrupt Enable Set Register"]
    pub uhinteset: crate::Reg<uhinteset::UHINTESET_SPEC>,
    #[doc = "0x41c - Pipe Reset Register"]
    pub uprst: crate::Reg<uprst::UPRST_SPEC>,
    #[doc = "0x420 - Host Frame Number Register"]
    pub uhfnum: crate::Reg<uhfnum::UHFNUM_SPEC>,
    #[doc = "0x424 - Host Start of Frame Control Register"]
    pub uhsofc: crate::Reg<uhsofc::UHSOFC_SPEC>,
    _reserved75: [u8; 216usize],
    #[doc = "0x500 - Pipe Configuration Register"]
    pub upcfg0: crate::Reg<upcfg0::UPCFG0_SPEC>,
    #[doc = "0x504 - Pipe Configuration Register"]
    pub upcfg1: crate::Reg<upcfg1::UPCFG1_SPEC>,
    #[doc = "0x508 - Pipe Configuration Register"]
    pub upcfg2: crate::Reg<upcfg2::UPCFG2_SPEC>,
    #[doc = "0x50c - Pipe Configuration Register"]
    pub upcfg3: crate::Reg<upcfg3::UPCFG3_SPEC>,
    #[doc = "0x510 - Pipe Configuration Register"]
    pub upcfg4: crate::Reg<upcfg4::UPCFG4_SPEC>,
    #[doc = "0x514 - Pipe Configuration Register"]
    pub upcfg5: crate::Reg<upcfg5::UPCFG5_SPEC>,
    #[doc = "0x518 - Pipe Configuration Register"]
    pub upcfg6: crate::Reg<upcfg6::UPCFG6_SPEC>,
    #[doc = "0x51c - Pipe Configuration Register"]
    pub upcfg7: crate::Reg<upcfg7::UPCFG7_SPEC>,
    _reserved83: [u8; 16usize],
    #[doc = "0x530 - Pipe Status Register"]
    pub upsta0: crate::Reg<upsta0::UPSTA0_SPEC>,
    #[doc = "0x534 - Pipe Status Register"]
    pub upsta1: crate::Reg<upsta1::UPSTA1_SPEC>,
    #[doc = "0x538 - Pipe Status Register"]
    pub upsta2: crate::Reg<upsta2::UPSTA2_SPEC>,
    #[doc = "0x53c - Pipe Status Register"]
    pub upsta3: crate::Reg<upsta3::UPSTA3_SPEC>,
    #[doc = "0x540 - Pipe Status Register"]
    pub upsta4: crate::Reg<upsta4::UPSTA4_SPEC>,
    #[doc = "0x544 - Pipe Status Register"]
    pub upsta5: crate::Reg<upsta5::UPSTA5_SPEC>,
    #[doc = "0x548 - Pipe Status Register"]
    pub upsta6: crate::Reg<upsta6::UPSTA6_SPEC>,
    #[doc = "0x54c - Pipe Status Register"]
    pub upsta7: crate::Reg<upsta7::UPSTA7_SPEC>,
    _reserved91: [u8; 16usize],
    #[doc = "0x560 - Pipe Status Clear Register"]
    pub upsta0clr: crate::Reg<upsta0clr::UPSTA0CLR_SPEC>,
    #[doc = "0x564 - Pipe Status Clear Register"]
    pub upsta1clr: crate::Reg<upsta1clr::UPSTA1CLR_SPEC>,
    #[doc = "0x568 - Pipe Status Clear Register"]
    pub upsta2clr: crate::Reg<upsta2clr::UPSTA2CLR_SPEC>,
    #[doc = "0x56c - Pipe Status Clear Register"]
    pub upsta3clr: crate::Reg<upsta3clr::UPSTA3CLR_SPEC>,
    #[doc = "0x570 - Pipe Status Clear Register"]
    pub upsta4clr: crate::Reg<upsta4clr::UPSTA4CLR_SPEC>,
    #[doc = "0x574 - Pipe Status Clear Register"]
    pub upsta5clr: crate::Reg<upsta5clr::UPSTA5CLR_SPEC>,
    #[doc = "0x578 - Pipe Status Clear Register"]
    pub upsta6clr: crate::Reg<upsta6clr::UPSTA6CLR_SPEC>,
    #[doc = "0x57c - Pipe Status Clear Register"]
    pub upsta7clr: crate::Reg<upsta7clr::UPSTA7CLR_SPEC>,
    _reserved99: [u8; 16usize],
    #[doc = "0x590 - Pipe Status Set Register"]
    pub upsta0set: crate::Reg<upsta0set::UPSTA0SET_SPEC>,
    #[doc = "0x594 - Pipe Status Set Register"]
    pub upsta1set: crate::Reg<upsta1set::UPSTA1SET_SPEC>,
    #[doc = "0x598 - Pipe Status Set Register"]
    pub upsta2set: crate::Reg<upsta2set::UPSTA2SET_SPEC>,
    #[doc = "0x59c - Pipe Status Set Register"]
    pub upsta3set: crate::Reg<upsta3set::UPSTA3SET_SPEC>,
    #[doc = "0x5a0 - Pipe Status Set Register"]
    pub upsta4set: crate::Reg<upsta4set::UPSTA4SET_SPEC>,
    #[doc = "0x5a4 - Pipe Status Set Register"]
    pub upsta5set: crate::Reg<upsta5set::UPSTA5SET_SPEC>,
    #[doc = "0x5a8 - Pipe Status Set Register"]
    pub upsta6set: crate::Reg<upsta6set::UPSTA6SET_SPEC>,
    #[doc = "0x5ac - Pipe Status Set Register"]
    pub upsta7set: crate::Reg<upsta7set::UPSTA7SET_SPEC>,
    _reserved107: [u8; 16usize],
    #[doc = "0x5c0 - Pipe Control Register"]
    pub upcon0: crate::Reg<upcon0::UPCON0_SPEC>,
    #[doc = "0x5c4 - Pipe Control Register"]
    pub upcon1: crate::Reg<upcon1::UPCON1_SPEC>,
    #[doc = "0x5c8 - Pipe Control Register"]
    pub upcon2: crate::Reg<upcon2::UPCON2_SPEC>,
    #[doc = "0x5cc - Pipe Control Register"]
    pub upcon3: crate::Reg<upcon3::UPCON3_SPEC>,
    #[doc = "0x5d0 - Pipe Control Register"]
    pub upcon4: crate::Reg<upcon4::UPCON4_SPEC>,
    #[doc = "0x5d4 - Pipe Control Register"]
    pub upcon5: crate::Reg<upcon5::UPCON5_SPEC>,
    #[doc = "0x5d8 - Pipe Control Register"]
    pub upcon6: crate::Reg<upcon6::UPCON6_SPEC>,
    #[doc = "0x5dc - Pipe Control Register"]
    pub upcon7: crate::Reg<upcon7::UPCON7_SPEC>,
    _reserved115: [u8; 16usize],
    #[doc = "0x5f0 - Pipe Control Set Register"]
    pub upcon0set: crate::Reg<upcon0set::UPCON0SET_SPEC>,
    #[doc = "0x5f4 - Pipe Control Set Register"]
    pub upcon1set: crate::Reg<upcon1set::UPCON1SET_SPEC>,
    #[doc = "0x5f8 - Pipe Control Set Register"]
    pub upcon2set: crate::Reg<upcon2set::UPCON2SET_SPEC>,
    #[doc = "0x5fc - Pipe Control Set Register"]
    pub upcon3set: crate::Reg<upcon3set::UPCON3SET_SPEC>,
    #[doc = "0x600 - Pipe Control Set Register"]
    pub upcon4set: crate::Reg<upcon4set::UPCON4SET_SPEC>,
    #[doc = "0x604 - Pipe Control Set Register"]
    pub upcon5set: crate::Reg<upcon5set::UPCON5SET_SPEC>,
    #[doc = "0x608 - Pipe Control Set Register"]
    pub upcon6set: crate::Reg<upcon6set::UPCON6SET_SPEC>,
    #[doc = "0x60c - Pipe Control Set Register"]
    pub upcon7set: crate::Reg<upcon7set::UPCON7SET_SPEC>,
    _reserved123: [u8; 16usize],
    #[doc = "0x620 - Pipe Control Clear Register"]
    pub upcon0clr: crate::Reg<upcon0clr::UPCON0CLR_SPEC>,
    #[doc = "0x624 - Pipe Control Clear Register"]
    pub upcon1clr: crate::Reg<upcon1clr::UPCON1CLR_SPEC>,
    #[doc = "0x628 - Pipe Control Clear Register"]
    pub upcon2clr: crate::Reg<upcon2clr::UPCON2CLR_SPEC>,
    #[doc = "0x62c - Pipe Control Clear Register"]
    pub upcon3clr: crate::Reg<upcon3clr::UPCON3CLR_SPEC>,
    #[doc = "0x630 - Pipe Control Clear Register"]
    pub upcon4clr: crate::Reg<upcon4clr::UPCON4CLR_SPEC>,
    #[doc = "0x634 - Pipe Control Clear Register"]
    pub upcon5clr: crate::Reg<upcon5clr::UPCON5CLR_SPEC>,
    #[doc = "0x638 - Pipe Control Clear Register"]
    pub upcon6clr: crate::Reg<upcon6clr::UPCON6CLR_SPEC>,
    #[doc = "0x63c - Pipe Control Clear Register"]
    pub upcon7clr: crate::Reg<upcon7clr::UPCON7CLR_SPEC>,
    _reserved131: [u8; 16usize],
    #[doc = "0x650 - Pipe In Request"]
    pub upinrq0: crate::Reg<upinrq0::UPINRQ0_SPEC>,
    #[doc = "0x654 - Pipe In Request"]
    pub upinrq1: crate::Reg<upinrq1::UPINRQ1_SPEC>,
    #[doc = "0x658 - Pipe In Request"]
    pub upinrq2: crate::Reg<upinrq2::UPINRQ2_SPEC>,
    #[doc = "0x65c - Pipe In Request"]
    pub upinrq3: crate::Reg<upinrq3::UPINRQ3_SPEC>,
    #[doc = "0x660 - Pipe In Request"]
    pub upinrq4: crate::Reg<upinrq4::UPINRQ4_SPEC>,
    #[doc = "0x664 - Pipe In Request"]
    pub upinrq5: crate::Reg<upinrq5::UPINRQ5_SPEC>,
    #[doc = "0x668 - Pipe In Request"]
    pub upinrq6: crate::Reg<upinrq6::UPINRQ6_SPEC>,
    #[doc = "0x66c - Pipe In Request"]
    pub upinrq7: crate::Reg<upinrq7::UPINRQ7_SPEC>,
    _reserved139: [u8; 400usize],
    #[doc = "0x800 - General Control Register"]
    pub usbcon: crate::Reg<usbcon::USBCON_SPEC>,
    #[doc = "0x804 - General Status Register"]
    pub usbsta: crate::Reg<usbsta::USBSTA_SPEC>,
    #[doc = "0x808 - General Status Clear Register"]
    pub usbstaclr: crate::Reg<usbstaclr::USBSTACLR_SPEC>,
    #[doc = "0x80c - General Status Set Register"]
    pub usbstaset: crate::Reg<usbstaset::USBSTASET_SPEC>,
    _reserved143: [u8; 8usize],
    #[doc = "0x818 - IP Version Register"]
    pub uvers: crate::Reg<uvers::UVERS_SPEC>,
    #[doc = "0x81c - IP Features Register"]
    pub ufeatures: crate::Reg<ufeatures::UFEATURES_SPEC>,
    #[doc = "0x820 - IP PB address size Register"]
    pub uaddrsize: crate::Reg<uaddrsize::UADDRSIZE_SPEC>,
    #[doc = "0x824 - IP Name Part One: HUSB"]
    pub uname1: crate::Reg<uname1::UNAME1_SPEC>,
    #[doc = "0x828 - IP Name Part Two: HOST"]
    pub uname2: crate::Reg<uname2::UNAME2_SPEC>,
    #[doc = "0x82c - USB internal finite state machine"]
    pub usbfsm: crate::Reg<usbfsm::USBFSM_SPEC>,
    #[doc = "0x830 - Endpoint descriptor table"]
    pub udesc: crate::Reg<udesc::UDESC_SPEC>,
}
#[doc = "UADDRSIZE register accessor: an alias for `Reg<UADDRSIZE_SPEC>`"]
pub type UADDRSIZE = crate::Reg<uaddrsize::UADDRSIZE_SPEC>;
#[doc = "IP PB address size Register"]
pub mod uaddrsize;
#[doc = "UDCON register accessor: an alias for `Reg<UDCON_SPEC>`"]
pub type UDCON = crate::Reg<udcon::UDCON_SPEC>;
#[doc = "Device General Control Register"]
pub mod udcon;
#[doc = "UDESC register accessor: an alias for `Reg<UDESC_SPEC>`"]
pub type UDESC = crate::Reg<udesc::UDESC_SPEC>;
#[doc = "Endpoint descriptor table"]
pub mod udesc;
#[doc = "UDFNUM register accessor: an alias for `Reg<UDFNUM_SPEC>`"]
pub type UDFNUM = crate::Reg<udfnum::UDFNUM_SPEC>;
#[doc = "Device Frame Number Register"]
pub mod udfnum;
#[doc = "UDINT register accessor: an alias for `Reg<UDINT_SPEC>`"]
pub type UDINT = crate::Reg<udint::UDINT_SPEC>;
#[doc = "Device Global Interupt Register"]
pub mod udint;
#[doc = "UDINTCLR register accessor: an alias for `Reg<UDINTCLR_SPEC>`"]
pub type UDINTCLR = crate::Reg<udintclr::UDINTCLR_SPEC>;
#[doc = "Device Global Interrupt Clear Register"]
pub mod udintclr;
#[doc = "UDINTE register accessor: an alias for `Reg<UDINTE_SPEC>`"]
pub type UDINTE = crate::Reg<udinte::UDINTE_SPEC>;
#[doc = "Device Global Interrupt Enable Register"]
pub mod udinte;
#[doc = "UDINTECLR register accessor: an alias for `Reg<UDINTECLR_SPEC>`"]
pub type UDINTECLR = crate::Reg<udinteclr::UDINTECLR_SPEC>;
#[doc = "Device Global Interrupt Enable Clear Register"]
pub mod udinteclr;
#[doc = "UDINTESET register accessor: an alias for `Reg<UDINTESET_SPEC>`"]
pub type UDINTESET = crate::Reg<udinteset::UDINTESET_SPEC>;
#[doc = "Device Global Interrupt Enable Set Register"]
pub mod udinteset;
#[doc = "UDINTSET register accessor: an alias for `Reg<UDINTSET_SPEC>`"]
pub type UDINTSET = crate::Reg<udintset::UDINTSET_SPEC>;
#[doc = "Device Global Interrupt Set Regsiter"]
pub mod udintset;
#[doc = "UECFG0 register accessor: an alias for `Reg<UECFG0_SPEC>`"]
pub type UECFG0 = crate::Reg<uecfg0::UECFG0_SPEC>;
#[doc = "Endpoint Configuration Register"]
pub mod uecfg0;
#[doc = "UECFG1 register accessor: an alias for `Reg<UECFG1_SPEC>`"]
pub type UECFG1 = crate::Reg<uecfg1::UECFG1_SPEC>;
#[doc = "Endpoint Configuration Register"]
pub mod uecfg1;
#[doc = "UECFG2 register accessor: an alias for `Reg<UECFG2_SPEC>`"]
pub type UECFG2 = crate::Reg<uecfg2::UECFG2_SPEC>;
#[doc = "Endpoint Configuration Register"]
pub mod uecfg2;
#[doc = "UECFG3 register accessor: an alias for `Reg<UECFG3_SPEC>`"]
pub type UECFG3 = crate::Reg<uecfg3::UECFG3_SPEC>;
#[doc = "Endpoint Configuration Register"]
pub mod uecfg3;
#[doc = "UECFG4 register accessor: an alias for `Reg<UECFG4_SPEC>`"]
pub type UECFG4 = crate::Reg<uecfg4::UECFG4_SPEC>;
#[doc = "Endpoint Configuration Register"]
pub mod uecfg4;
#[doc = "UECFG5 register accessor: an alias for `Reg<UECFG5_SPEC>`"]
pub type UECFG5 = crate::Reg<uecfg5::UECFG5_SPEC>;
#[doc = "Endpoint Configuration Register"]
pub mod uecfg5;
#[doc = "UECFG6 register accessor: an alias for `Reg<UECFG6_SPEC>`"]
pub type UECFG6 = crate::Reg<uecfg6::UECFG6_SPEC>;
#[doc = "Endpoint Configuration Register"]
pub mod uecfg6;
#[doc = "UECFG7 register accessor: an alias for `Reg<UECFG7_SPEC>`"]
pub type UECFG7 = crate::Reg<uecfg7::UECFG7_SPEC>;
#[doc = "Endpoint Configuration Register"]
pub mod uecfg7;
#[doc = "UECON0 register accessor: an alias for `Reg<UECON0_SPEC>`"]
pub type UECON0 = crate::Reg<uecon0::UECON0_SPEC>;
#[doc = "Endpoint Control Register"]
pub mod uecon0;
#[doc = "UECON0CLR register accessor: an alias for `Reg<UECON0CLR_SPEC>`"]
pub type UECON0CLR = crate::Reg<uecon0clr::UECON0CLR_SPEC>;
#[doc = "Endpoint Control Clear Register"]
pub mod uecon0clr;
#[doc = "UECON0SET register accessor: an alias for `Reg<UECON0SET_SPEC>`"]
pub type UECON0SET = crate::Reg<uecon0set::UECON0SET_SPEC>;
#[doc = "Endpoint Control Set Register"]
pub mod uecon0set;
#[doc = "UECON1 register accessor: an alias for `Reg<UECON1_SPEC>`"]
pub type UECON1 = crate::Reg<uecon1::UECON1_SPEC>;
#[doc = "Endpoint Control Register"]
pub mod uecon1;
#[doc = "UECON1CLR register accessor: an alias for `Reg<UECON1CLR_SPEC>`"]
pub type UECON1CLR = crate::Reg<uecon1clr::UECON1CLR_SPEC>;
#[doc = "TXINE Clear"]
pub mod uecon1clr;
#[doc = "UECON1SET register accessor: an alias for `Reg<UECON1SET_SPEC>`"]
pub type UECON1SET = crate::Reg<uecon1set::UECON1SET_SPEC>;
#[doc = "Endpoint Control Set Register"]
pub mod uecon1set;
#[doc = "UECON2 register accessor: an alias for `Reg<UECON2_SPEC>`"]
pub type UECON2 = crate::Reg<uecon2::UECON2_SPEC>;
#[doc = "Endpoint Control Register"]
pub mod uecon2;
#[doc = "UECON2CLR register accessor: an alias for `Reg<UECON2CLR_SPEC>`"]
pub type UECON2CLR = crate::Reg<uecon2clr::UECON2CLR_SPEC>;
#[doc = "TXINE Clear"]
pub mod uecon2clr;
#[doc = "UECON2SET register accessor: an alias for `Reg<UECON2SET_SPEC>`"]
pub type UECON2SET = crate::Reg<uecon2set::UECON2SET_SPEC>;
#[doc = "Endpoint Control Set Register"]
pub mod uecon2set;
#[doc = "UECON3 register accessor: an alias for `Reg<UECON3_SPEC>`"]
pub type UECON3 = crate::Reg<uecon3::UECON3_SPEC>;
#[doc = "Endpoint Control Register"]
pub mod uecon3;
#[doc = "UECON3CLR register accessor: an alias for `Reg<UECON3CLR_SPEC>`"]
pub type UECON3CLR = crate::Reg<uecon3clr::UECON3CLR_SPEC>;
#[doc = "TXINE Clear"]
pub mod uecon3clr;
#[doc = "UECON3SET register accessor: an alias for `Reg<UECON3SET_SPEC>`"]
pub type UECON3SET = crate::Reg<uecon3set::UECON3SET_SPEC>;
#[doc = "Endpoint Control Set Register"]
pub mod uecon3set;
#[doc = "UECON4 register accessor: an alias for `Reg<UECON4_SPEC>`"]
pub type UECON4 = crate::Reg<uecon4::UECON4_SPEC>;
#[doc = "Endpoint Control Register"]
pub mod uecon4;
#[doc = "UECON4CLR register accessor: an alias for `Reg<UECON4CLR_SPEC>`"]
pub type UECON4CLR = crate::Reg<uecon4clr::UECON4CLR_SPEC>;
#[doc = "TXINE Clear"]
pub mod uecon4clr;
#[doc = "UECON4SET register accessor: an alias for `Reg<UECON4SET_SPEC>`"]
pub type UECON4SET = crate::Reg<uecon4set::UECON4SET_SPEC>;
#[doc = "Endpoint Control Set Register"]
pub mod uecon4set;
#[doc = "UECON5 register accessor: an alias for `Reg<UECON5_SPEC>`"]
pub type UECON5 = crate::Reg<uecon5::UECON5_SPEC>;
#[doc = "Endpoint Control Register"]
pub mod uecon5;
#[doc = "UECON5CLR register accessor: an alias for `Reg<UECON5CLR_SPEC>`"]
pub type UECON5CLR = crate::Reg<uecon5clr::UECON5CLR_SPEC>;
#[doc = "TXINE Clear"]
pub mod uecon5clr;
#[doc = "UECON5SET register accessor: an alias for `Reg<UECON5SET_SPEC>`"]
pub type UECON5SET = crate::Reg<uecon5set::UECON5SET_SPEC>;
#[doc = "Endpoint Control Set Register"]
pub mod uecon5set;
#[doc = "UECON6 register accessor: an alias for `Reg<UECON6_SPEC>`"]
pub type UECON6 = crate::Reg<uecon6::UECON6_SPEC>;
#[doc = "Endpoint Control Register"]
pub mod uecon6;
#[doc = "UECON6CLR register accessor: an alias for `Reg<UECON6CLR_SPEC>`"]
pub type UECON6CLR = crate::Reg<uecon6clr::UECON6CLR_SPEC>;
#[doc = "TXINE Clear"]
pub mod uecon6clr;
#[doc = "UECON6SET register accessor: an alias for `Reg<UECON6SET_SPEC>`"]
pub type UECON6SET = crate::Reg<uecon6set::UECON6SET_SPEC>;
#[doc = "Endpoint Control Set Register"]
pub mod uecon6set;
#[doc = "UECON7 register accessor: an alias for `Reg<UECON7_SPEC>`"]
pub type UECON7 = crate::Reg<uecon7::UECON7_SPEC>;
#[doc = "Endpoint Control Register"]
pub mod uecon7;
#[doc = "UECON7CLR register accessor: an alias for `Reg<UECON7CLR_SPEC>`"]
pub type UECON7CLR = crate::Reg<uecon7clr::UECON7CLR_SPEC>;
#[doc = "TXINE Clear"]
pub mod uecon7clr;
#[doc = "UECON7SET register accessor: an alias for `Reg<UECON7SET_SPEC>`"]
pub type UECON7SET = crate::Reg<uecon7set::UECON7SET_SPEC>;
#[doc = "Endpoint Control Set Register"]
pub mod uecon7set;
#[doc = "UERST register accessor: an alias for `Reg<UERST_SPEC>`"]
pub type UERST = crate::Reg<uerst::UERST_SPEC>;
#[doc = "Endpoint Enable/Reset Register"]
pub mod uerst;
#[doc = "UESTA0 register accessor: an alias for `Reg<UESTA0_SPEC>`"]
pub type UESTA0 = crate::Reg<uesta0::UESTA0_SPEC>;
#[doc = "Endpoint Status Register"]
pub mod uesta0;
#[doc = "UESTA0CLR register accessor: an alias for `Reg<UESTA0CLR_SPEC>`"]
pub type UESTA0CLR = crate::Reg<uesta0clr::UESTA0CLR_SPEC>;
#[doc = "Endpoint Status Clear Register"]
pub mod uesta0clr;
#[doc = "UESTA0SET register accessor: an alias for `Reg<UESTA0SET_SPEC>`"]
pub type UESTA0SET = crate::Reg<uesta0set::UESTA0SET_SPEC>;
#[doc = "Endpoint Status Set Register"]
pub mod uesta0set;
#[doc = "UESTA1 register accessor: an alias for `Reg<UESTA1_SPEC>`"]
pub type UESTA1 = crate::Reg<uesta1::UESTA1_SPEC>;
#[doc = "Endpoint Status Register"]
pub mod uesta1;
#[doc = "UESTA1CLR register accessor: an alias for `Reg<UESTA1CLR_SPEC>`"]
pub type UESTA1CLR = crate::Reg<uesta1clr::UESTA1CLR_SPEC>;
#[doc = "Endpoint Status Clear Register"]
pub mod uesta1clr;
#[doc = "UESTA1SET register accessor: an alias for `Reg<UESTA1SET_SPEC>`"]
pub type UESTA1SET = crate::Reg<uesta1set::UESTA1SET_SPEC>;
#[doc = "Endpoint Status Set Register"]
pub mod uesta1set;
#[doc = "UESTA2 register accessor: an alias for `Reg<UESTA2_SPEC>`"]
pub type UESTA2 = crate::Reg<uesta2::UESTA2_SPEC>;
#[doc = "Endpoint Status Register"]
pub mod uesta2;
#[doc = "UESTA2CLR register accessor: an alias for `Reg<UESTA2CLR_SPEC>`"]
pub type UESTA2CLR = crate::Reg<uesta2clr::UESTA2CLR_SPEC>;
#[doc = "Endpoint Status Clear Register"]
pub mod uesta2clr;
#[doc = "UESTA2SET register accessor: an alias for `Reg<UESTA2SET_SPEC>`"]
pub type UESTA2SET = crate::Reg<uesta2set::UESTA2SET_SPEC>;
#[doc = "Endpoint Status Set Register"]
pub mod uesta2set;
#[doc = "UESTA3 register accessor: an alias for `Reg<UESTA3_SPEC>`"]
pub type UESTA3 = crate::Reg<uesta3::UESTA3_SPEC>;
#[doc = "Endpoint Status Register"]
pub mod uesta3;
#[doc = "UESTA3CLR register accessor: an alias for `Reg<UESTA3CLR_SPEC>`"]
pub type UESTA3CLR = crate::Reg<uesta3clr::UESTA3CLR_SPEC>;
#[doc = "Endpoint Status Clear Register"]
pub mod uesta3clr;
#[doc = "UESTA3SET register accessor: an alias for `Reg<UESTA3SET_SPEC>`"]
pub type UESTA3SET = crate::Reg<uesta3set::UESTA3SET_SPEC>;
#[doc = "Endpoint Status Set Register"]
pub mod uesta3set;
#[doc = "UESTA4 register accessor: an alias for `Reg<UESTA4_SPEC>`"]
pub type UESTA4 = crate::Reg<uesta4::UESTA4_SPEC>;
#[doc = "Endpoint Status Register"]
pub mod uesta4;
#[doc = "UESTA4CLR register accessor: an alias for `Reg<UESTA4CLR_SPEC>`"]
pub type UESTA4CLR = crate::Reg<uesta4clr::UESTA4CLR_SPEC>;
#[doc = "Endpoint Status Clear Register"]
pub mod uesta4clr;
#[doc = "UESTA4SET register accessor: an alias for `Reg<UESTA4SET_SPEC>`"]
pub type UESTA4SET = crate::Reg<uesta4set::UESTA4SET_SPEC>;
#[doc = "Endpoint Status Set Register"]
pub mod uesta4set;
#[doc = "UESTA5 register accessor: an alias for `Reg<UESTA5_SPEC>`"]
pub type UESTA5 = crate::Reg<uesta5::UESTA5_SPEC>;
#[doc = "Endpoint Status Register"]
pub mod uesta5;
#[doc = "UESTA5CLR register accessor: an alias for `Reg<UESTA5CLR_SPEC>`"]
pub type UESTA5CLR = crate::Reg<uesta5clr::UESTA5CLR_SPEC>;
#[doc = "Endpoint Status Clear Register"]
pub mod uesta5clr;
#[doc = "UESTA5SET register accessor: an alias for `Reg<UESTA5SET_SPEC>`"]
pub type UESTA5SET = crate::Reg<uesta5set::UESTA5SET_SPEC>;
#[doc = "Endpoint Status Set Register"]
pub mod uesta5set;
#[doc = "UESTA6 register accessor: an alias for `Reg<UESTA6_SPEC>`"]
pub type UESTA6 = crate::Reg<uesta6::UESTA6_SPEC>;
#[doc = "Endpoint Status Register"]
pub mod uesta6;
#[doc = "UESTA6CLR register accessor: an alias for `Reg<UESTA6CLR_SPEC>`"]
pub type UESTA6CLR = crate::Reg<uesta6clr::UESTA6CLR_SPEC>;
#[doc = "Endpoint Status Clear Register"]
pub mod uesta6clr;
#[doc = "UESTA6SET register accessor: an alias for `Reg<UESTA6SET_SPEC>`"]
pub type UESTA6SET = crate::Reg<uesta6set::UESTA6SET_SPEC>;
#[doc = "Endpoint Status Set Register"]
pub mod uesta6set;
#[doc = "UESTA7 register accessor: an alias for `Reg<UESTA7_SPEC>`"]
pub type UESTA7 = crate::Reg<uesta7::UESTA7_SPEC>;
#[doc = "Endpoint Status Register"]
pub mod uesta7;
#[doc = "UESTA7CLR register accessor: an alias for `Reg<UESTA7CLR_SPEC>`"]
pub type UESTA7CLR = crate::Reg<uesta7clr::UESTA7CLR_SPEC>;
#[doc = "Endpoint Status Clear Register"]
pub mod uesta7clr;
#[doc = "UESTA7SET register accessor: an alias for `Reg<UESTA7SET_SPEC>`"]
pub type UESTA7SET = crate::Reg<uesta7set::UESTA7SET_SPEC>;
#[doc = "Endpoint Status Set Register"]
pub mod uesta7set;
#[doc = "UFEATURES register accessor: an alias for `Reg<UFEATURES_SPEC>`"]
pub type UFEATURES = crate::Reg<ufeatures::UFEATURES_SPEC>;
#[doc = "IP Features Register"]
pub mod ufeatures;
#[doc = "UHCON register accessor: an alias for `Reg<UHCON_SPEC>`"]
pub type UHCON = crate::Reg<uhcon::UHCON_SPEC>;
#[doc = "Host General Control Register"]
pub mod uhcon;
#[doc = "UHFNUM register accessor: an alias for `Reg<UHFNUM_SPEC>`"]
pub type UHFNUM = crate::Reg<uhfnum::UHFNUM_SPEC>;
#[doc = "Host Frame Number Register"]
pub mod uhfnum;
#[doc = "UHINT register accessor: an alias for `Reg<UHINT_SPEC>`"]
pub type UHINT = crate::Reg<uhint::UHINT_SPEC>;
#[doc = "Host Global Interrupt Register"]
pub mod uhint;
#[doc = "UHINTCLR register accessor: an alias for `Reg<UHINTCLR_SPEC>`"]
pub type UHINTCLR = crate::Reg<uhintclr::UHINTCLR_SPEC>;
#[doc = "Host Global Interrrupt Clear Register"]
pub mod uhintclr;
#[doc = "UHINTE register accessor: an alias for `Reg<UHINTE_SPEC>`"]
pub type UHINTE = crate::Reg<uhinte::UHINTE_SPEC>;
#[doc = "Host Global Interrupt Enable Register"]
pub mod uhinte;
#[doc = "UHINTECLR register accessor: an alias for `Reg<UHINTECLR_SPEC>`"]
pub type UHINTECLR = crate::Reg<uhinteclr::UHINTECLR_SPEC>;
#[doc = "Host Global Interrupt Enable Clear Register"]
pub mod uhinteclr;
#[doc = "UHINTESET register accessor: an alias for `Reg<UHINTESET_SPEC>`"]
pub type UHINTESET = crate::Reg<uhinteset::UHINTESET_SPEC>;
#[doc = "Host Global Interrupt Enable Set Register"]
pub mod uhinteset;
#[doc = "UHINTSET register accessor: an alias for `Reg<UHINTSET_SPEC>`"]
pub type UHINTSET = crate::Reg<uhintset::UHINTSET_SPEC>;
#[doc = "Host Global Interrupt Set Register"]
pub mod uhintset;
#[doc = "UHSOFC register accessor: an alias for `Reg<UHSOFC_SPEC>`"]
pub type UHSOFC = crate::Reg<uhsofc::UHSOFC_SPEC>;
#[doc = "Host Start of Frame Control Register"]
pub mod uhsofc;
#[doc = "UNAME1 register accessor: an alias for `Reg<UNAME1_SPEC>`"]
pub type UNAME1 = crate::Reg<uname1::UNAME1_SPEC>;
#[doc = "IP Name Part One: HUSB"]
pub mod uname1;
#[doc = "UNAME2 register accessor: an alias for `Reg<UNAME2_SPEC>`"]
pub type UNAME2 = crate::Reg<uname2::UNAME2_SPEC>;
#[doc = "IP Name Part Two: HOST"]
pub mod uname2;
#[doc = "UPCFG0 register accessor: an alias for `Reg<UPCFG0_SPEC>`"]
pub type UPCFG0 = crate::Reg<upcfg0::UPCFG0_SPEC>;
#[doc = "Pipe Configuration Register"]
pub mod upcfg0;
#[doc = "UPCFG1 register accessor: an alias for `Reg<UPCFG1_SPEC>`"]
pub type UPCFG1 = crate::Reg<upcfg1::UPCFG1_SPEC>;
#[doc = "Pipe Configuration Register"]
pub mod upcfg1;
#[doc = "UPCFG2 register accessor: an alias for `Reg<UPCFG2_SPEC>`"]
pub type UPCFG2 = crate::Reg<upcfg2::UPCFG2_SPEC>;
#[doc = "Pipe Configuration Register"]
pub mod upcfg2;
#[doc = "UPCFG3 register accessor: an alias for `Reg<UPCFG3_SPEC>`"]
pub type UPCFG3 = crate::Reg<upcfg3::UPCFG3_SPEC>;
#[doc = "Pipe Configuration Register"]
pub mod upcfg3;
#[doc = "UPCFG4 register accessor: an alias for `Reg<UPCFG4_SPEC>`"]
pub type UPCFG4 = crate::Reg<upcfg4::UPCFG4_SPEC>;
#[doc = "Pipe Configuration Register"]
pub mod upcfg4;
#[doc = "UPCFG5 register accessor: an alias for `Reg<UPCFG5_SPEC>`"]
pub type UPCFG5 = crate::Reg<upcfg5::UPCFG5_SPEC>;
#[doc = "Pipe Configuration Register"]
pub mod upcfg5;
#[doc = "UPCFG6 register accessor: an alias for `Reg<UPCFG6_SPEC>`"]
pub type UPCFG6 = crate::Reg<upcfg6::UPCFG6_SPEC>;
#[doc = "Pipe Configuration Register"]
pub mod upcfg6;
#[doc = "UPCFG7 register accessor: an alias for `Reg<UPCFG7_SPEC>`"]
pub type UPCFG7 = crate::Reg<upcfg7::UPCFG7_SPEC>;
#[doc = "Pipe Configuration Register"]
pub mod upcfg7;
#[doc = "UPCON0 register accessor: an alias for `Reg<UPCON0_SPEC>`"]
pub type UPCON0 = crate::Reg<upcon0::UPCON0_SPEC>;
#[doc = "Pipe Control Register"]
pub mod upcon0;
#[doc = "UPCON0CLR register accessor: an alias for `Reg<UPCON0CLR_SPEC>`"]
pub type UPCON0CLR = crate::Reg<upcon0clr::UPCON0CLR_SPEC>;
#[doc = "Pipe Control Clear Register"]
pub mod upcon0clr;
#[doc = "UPCON0SET register accessor: an alias for `Reg<UPCON0SET_SPEC>`"]
pub type UPCON0SET = crate::Reg<upcon0set::UPCON0SET_SPEC>;
#[doc = "Pipe Control Set Register"]
pub mod upcon0set;
#[doc = "UPCON1 register accessor: an alias for `Reg<UPCON1_SPEC>`"]
pub type UPCON1 = crate::Reg<upcon1::UPCON1_SPEC>;
#[doc = "Pipe Control Register"]
pub mod upcon1;
#[doc = "UPCON1CLR register accessor: an alias for `Reg<UPCON1CLR_SPEC>`"]
pub type UPCON1CLR = crate::Reg<upcon1clr::UPCON1CLR_SPEC>;
#[doc = "Pipe Control Clear Register"]
pub mod upcon1clr;
#[doc = "UPCON1SET register accessor: an alias for `Reg<UPCON1SET_SPEC>`"]
pub type UPCON1SET = crate::Reg<upcon1set::UPCON1SET_SPEC>;
#[doc = "Pipe Control Set Register"]
pub mod upcon1set;
#[doc = "UPCON2 register accessor: an alias for `Reg<UPCON2_SPEC>`"]
pub type UPCON2 = crate::Reg<upcon2::UPCON2_SPEC>;
#[doc = "Pipe Control Register"]
pub mod upcon2;
#[doc = "UPCON2CLR register accessor: an alias for `Reg<UPCON2CLR_SPEC>`"]
pub type UPCON2CLR = crate::Reg<upcon2clr::UPCON2CLR_SPEC>;
#[doc = "Pipe Control Clear Register"]
pub mod upcon2clr;
#[doc = "UPCON2SET register accessor: an alias for `Reg<UPCON2SET_SPEC>`"]
pub type UPCON2SET = crate::Reg<upcon2set::UPCON2SET_SPEC>;
#[doc = "Pipe Control Set Register"]
pub mod upcon2set;
#[doc = "UPCON3 register accessor: an alias for `Reg<UPCON3_SPEC>`"]
pub type UPCON3 = crate::Reg<upcon3::UPCON3_SPEC>;
#[doc = "Pipe Control Register"]
pub mod upcon3;
#[doc = "UPCON3CLR register accessor: an alias for `Reg<UPCON3CLR_SPEC>`"]
pub type UPCON3CLR = crate::Reg<upcon3clr::UPCON3CLR_SPEC>;
#[doc = "Pipe Control Clear Register"]
pub mod upcon3clr;
#[doc = "UPCON3SET register accessor: an alias for `Reg<UPCON3SET_SPEC>`"]
pub type UPCON3SET = crate::Reg<upcon3set::UPCON3SET_SPEC>;
#[doc = "Pipe Control Set Register"]
pub mod upcon3set;
#[doc = "UPCON4 register accessor: an alias for `Reg<UPCON4_SPEC>`"]
pub type UPCON4 = crate::Reg<upcon4::UPCON4_SPEC>;
#[doc = "Pipe Control Register"]
pub mod upcon4;
#[doc = "UPCON4CLR register accessor: an alias for `Reg<UPCON4CLR_SPEC>`"]
pub type UPCON4CLR = crate::Reg<upcon4clr::UPCON4CLR_SPEC>;
#[doc = "Pipe Control Clear Register"]
pub mod upcon4clr;
#[doc = "UPCON4SET register accessor: an alias for `Reg<UPCON4SET_SPEC>`"]
pub type UPCON4SET = crate::Reg<upcon4set::UPCON4SET_SPEC>;
#[doc = "Pipe Control Set Register"]
pub mod upcon4set;
#[doc = "UPCON5 register accessor: an alias for `Reg<UPCON5_SPEC>`"]
pub type UPCON5 = crate::Reg<upcon5::UPCON5_SPEC>;
#[doc = "Pipe Control Register"]
pub mod upcon5;
#[doc = "UPCON5CLR register accessor: an alias for `Reg<UPCON5CLR_SPEC>`"]
pub type UPCON5CLR = crate::Reg<upcon5clr::UPCON5CLR_SPEC>;
#[doc = "Pipe Control Clear Register"]
pub mod upcon5clr;
#[doc = "UPCON5SET register accessor: an alias for `Reg<UPCON5SET_SPEC>`"]
pub type UPCON5SET = crate::Reg<upcon5set::UPCON5SET_SPEC>;
#[doc = "Pipe Control Set Register"]
pub mod upcon5set;
#[doc = "UPCON6 register accessor: an alias for `Reg<UPCON6_SPEC>`"]
pub type UPCON6 = crate::Reg<upcon6::UPCON6_SPEC>;
#[doc = "Pipe Control Register"]
pub mod upcon6;
#[doc = "UPCON6CLR register accessor: an alias for `Reg<UPCON6CLR_SPEC>`"]
pub type UPCON6CLR = crate::Reg<upcon6clr::UPCON6CLR_SPEC>;
#[doc = "Pipe Control Clear Register"]
pub mod upcon6clr;
#[doc = "UPCON6SET register accessor: an alias for `Reg<UPCON6SET_SPEC>`"]
pub type UPCON6SET = crate::Reg<upcon6set::UPCON6SET_SPEC>;
#[doc = "Pipe Control Set Register"]
pub mod upcon6set;
#[doc = "UPCON7 register accessor: an alias for `Reg<UPCON7_SPEC>`"]
pub type UPCON7 = crate::Reg<upcon7::UPCON7_SPEC>;
#[doc = "Pipe Control Register"]
pub mod upcon7;
#[doc = "UPCON7CLR register accessor: an alias for `Reg<UPCON7CLR_SPEC>`"]
pub type UPCON7CLR = crate::Reg<upcon7clr::UPCON7CLR_SPEC>;
#[doc = "Pipe Control Clear Register"]
pub mod upcon7clr;
#[doc = "UPCON7SET register accessor: an alias for `Reg<UPCON7SET_SPEC>`"]
pub type UPCON7SET = crate::Reg<upcon7set::UPCON7SET_SPEC>;
#[doc = "Pipe Control Set Register"]
pub mod upcon7set;
#[doc = "UPINRQ0 register accessor: an alias for `Reg<UPINRQ0_SPEC>`"]
pub type UPINRQ0 = crate::Reg<upinrq0::UPINRQ0_SPEC>;
#[doc = "Pipe In Request"]
pub mod upinrq0;
#[doc = "UPINRQ1 register accessor: an alias for `Reg<UPINRQ1_SPEC>`"]
pub type UPINRQ1 = crate::Reg<upinrq1::UPINRQ1_SPEC>;
#[doc = "Pipe In Request"]
pub mod upinrq1;
#[doc = "UPINRQ2 register accessor: an alias for `Reg<UPINRQ2_SPEC>`"]
pub type UPINRQ2 = crate::Reg<upinrq2::UPINRQ2_SPEC>;
#[doc = "Pipe In Request"]
pub mod upinrq2;
#[doc = "UPINRQ3 register accessor: an alias for `Reg<UPINRQ3_SPEC>`"]
pub type UPINRQ3 = crate::Reg<upinrq3::UPINRQ3_SPEC>;
#[doc = "Pipe In Request"]
pub mod upinrq3;
#[doc = "UPINRQ4 register accessor: an alias for `Reg<UPINRQ4_SPEC>`"]
pub type UPINRQ4 = crate::Reg<upinrq4::UPINRQ4_SPEC>;
#[doc = "Pipe In Request"]
pub mod upinrq4;
#[doc = "UPINRQ5 register accessor: an alias for `Reg<UPINRQ5_SPEC>`"]
pub type UPINRQ5 = crate::Reg<upinrq5::UPINRQ5_SPEC>;
#[doc = "Pipe In Request"]
pub mod upinrq5;
#[doc = "UPINRQ6 register accessor: an alias for `Reg<UPINRQ6_SPEC>`"]
pub type UPINRQ6 = crate::Reg<upinrq6::UPINRQ6_SPEC>;
#[doc = "Pipe In Request"]
pub mod upinrq6;
#[doc = "UPINRQ7 register accessor: an alias for `Reg<UPINRQ7_SPEC>`"]
pub type UPINRQ7 = crate::Reg<upinrq7::UPINRQ7_SPEC>;
#[doc = "Pipe In Request"]
pub mod upinrq7;
#[doc = "UPRST register accessor: an alias for `Reg<UPRST_SPEC>`"]
pub type UPRST = crate::Reg<uprst::UPRST_SPEC>;
#[doc = "Pipe Reset Register"]
pub mod uprst;
#[doc = "UPSTA0 register accessor: an alias for `Reg<UPSTA0_SPEC>`"]
pub type UPSTA0 = crate::Reg<upsta0::UPSTA0_SPEC>;
#[doc = "Pipe Status Register"]
pub mod upsta0;
#[doc = "UPSTA0CLR register accessor: an alias for `Reg<UPSTA0CLR_SPEC>`"]
pub type UPSTA0CLR = crate::Reg<upsta0clr::UPSTA0CLR_SPEC>;
#[doc = "Pipe Status Clear Register"]
pub mod upsta0clr;
#[doc = "UPSTA0SET register accessor: an alias for `Reg<UPSTA0SET_SPEC>`"]
pub type UPSTA0SET = crate::Reg<upsta0set::UPSTA0SET_SPEC>;
#[doc = "Pipe Status Set Register"]
pub mod upsta0set;
#[doc = "UPSTA1 register accessor: an alias for `Reg<UPSTA1_SPEC>`"]
pub type UPSTA1 = crate::Reg<upsta1::UPSTA1_SPEC>;
#[doc = "Pipe Status Register"]
pub mod upsta1;
#[doc = "UPSTA1CLR register accessor: an alias for `Reg<UPSTA1CLR_SPEC>`"]
pub type UPSTA1CLR = crate::Reg<upsta1clr::UPSTA1CLR_SPEC>;
#[doc = "Pipe Status Clear Register"]
pub mod upsta1clr;
#[doc = "UPSTA1SET register accessor: an alias for `Reg<UPSTA1SET_SPEC>`"]
pub type UPSTA1SET = crate::Reg<upsta1set::UPSTA1SET_SPEC>;
#[doc = "Pipe Status Set Register"]
pub mod upsta1set;
#[doc = "UPSTA2 register accessor: an alias for `Reg<UPSTA2_SPEC>`"]
pub type UPSTA2 = crate::Reg<upsta2::UPSTA2_SPEC>;
#[doc = "Pipe Status Register"]
pub mod upsta2;
#[doc = "UPSTA2CLR register accessor: an alias for `Reg<UPSTA2CLR_SPEC>`"]
pub type UPSTA2CLR = crate::Reg<upsta2clr::UPSTA2CLR_SPEC>;
#[doc = "Pipe Status Clear Register"]
pub mod upsta2clr;
#[doc = "UPSTA2SET register accessor: an alias for `Reg<UPSTA2SET_SPEC>`"]
pub type UPSTA2SET = crate::Reg<upsta2set::UPSTA2SET_SPEC>;
#[doc = "Pipe Status Set Register"]
pub mod upsta2set;
#[doc = "UPSTA3 register accessor: an alias for `Reg<UPSTA3_SPEC>`"]
pub type UPSTA3 = crate::Reg<upsta3::UPSTA3_SPEC>;
#[doc = "Pipe Status Register"]
pub mod upsta3;
#[doc = "UPSTA3CLR register accessor: an alias for `Reg<UPSTA3CLR_SPEC>`"]
pub type UPSTA3CLR = crate::Reg<upsta3clr::UPSTA3CLR_SPEC>;
#[doc = "Pipe Status Clear Register"]
pub mod upsta3clr;
#[doc = "UPSTA3SET register accessor: an alias for `Reg<UPSTA3SET_SPEC>`"]
pub type UPSTA3SET = crate::Reg<upsta3set::UPSTA3SET_SPEC>;
#[doc = "Pipe Status Set Register"]
pub mod upsta3set;
#[doc = "UPSTA4 register accessor: an alias for `Reg<UPSTA4_SPEC>`"]
pub type UPSTA4 = crate::Reg<upsta4::UPSTA4_SPEC>;
#[doc = "Pipe Status Register"]
pub mod upsta4;
#[doc = "UPSTA4CLR register accessor: an alias for `Reg<UPSTA4CLR_SPEC>`"]
pub type UPSTA4CLR = crate::Reg<upsta4clr::UPSTA4CLR_SPEC>;
#[doc = "Pipe Status Clear Register"]
pub mod upsta4clr;
#[doc = "UPSTA4SET register accessor: an alias for `Reg<UPSTA4SET_SPEC>`"]
pub type UPSTA4SET = crate::Reg<upsta4set::UPSTA4SET_SPEC>;
#[doc = "Pipe Status Set Register"]
pub mod upsta4set;
#[doc = "UPSTA5 register accessor: an alias for `Reg<UPSTA5_SPEC>`"]
pub type UPSTA5 = crate::Reg<upsta5::UPSTA5_SPEC>;
#[doc = "Pipe Status Register"]
pub mod upsta5;
#[doc = "UPSTA5CLR register accessor: an alias for `Reg<UPSTA5CLR_SPEC>`"]
pub type UPSTA5CLR = crate::Reg<upsta5clr::UPSTA5CLR_SPEC>;
#[doc = "Pipe Status Clear Register"]
pub mod upsta5clr;
#[doc = "UPSTA5SET register accessor: an alias for `Reg<UPSTA5SET_SPEC>`"]
pub type UPSTA5SET = crate::Reg<upsta5set::UPSTA5SET_SPEC>;
#[doc = "Pipe Status Set Register"]
pub mod upsta5set;
#[doc = "UPSTA6 register accessor: an alias for `Reg<UPSTA6_SPEC>`"]
pub type UPSTA6 = crate::Reg<upsta6::UPSTA6_SPEC>;
#[doc = "Pipe Status Register"]
pub mod upsta6;
#[doc = "UPSTA6CLR register accessor: an alias for `Reg<UPSTA6CLR_SPEC>`"]
pub type UPSTA6CLR = crate::Reg<upsta6clr::UPSTA6CLR_SPEC>;
#[doc = "Pipe Status Clear Register"]
pub mod upsta6clr;
#[doc = "UPSTA6SET register accessor: an alias for `Reg<UPSTA6SET_SPEC>`"]
pub type UPSTA6SET = crate::Reg<upsta6set::UPSTA6SET_SPEC>;
#[doc = "Pipe Status Set Register"]
pub mod upsta6set;
#[doc = "UPSTA7 register accessor: an alias for `Reg<UPSTA7_SPEC>`"]
pub type UPSTA7 = crate::Reg<upsta7::UPSTA7_SPEC>;
#[doc = "Pipe Status Register"]
pub mod upsta7;
#[doc = "UPSTA7CLR register accessor: an alias for `Reg<UPSTA7CLR_SPEC>`"]
pub type UPSTA7CLR = crate::Reg<upsta7clr::UPSTA7CLR_SPEC>;
#[doc = "Pipe Status Clear Register"]
pub mod upsta7clr;
#[doc = "UPSTA7SET register accessor: an alias for `Reg<UPSTA7SET_SPEC>`"]
pub type UPSTA7SET = crate::Reg<upsta7set::UPSTA7SET_SPEC>;
#[doc = "Pipe Status Set Register"]
pub mod upsta7set;
#[doc = "USBCON register accessor: an alias for `Reg<USBCON_SPEC>`"]
pub type USBCON = crate::Reg<usbcon::USBCON_SPEC>;
#[doc = "General Control Register"]
pub mod usbcon;
#[doc = "USBFSM register accessor: an alias for `Reg<USBFSM_SPEC>`"]
pub type USBFSM = crate::Reg<usbfsm::USBFSM_SPEC>;
#[doc = "USB internal finite state machine"]
pub mod usbfsm;
#[doc = "USBSTA register accessor: an alias for `Reg<USBSTA_SPEC>`"]
pub type USBSTA = crate::Reg<usbsta::USBSTA_SPEC>;
#[doc = "General Status Register"]
pub mod usbsta;
#[doc = "USBSTACLR register accessor: an alias for `Reg<USBSTACLR_SPEC>`"]
pub type USBSTACLR = crate::Reg<usbstaclr::USBSTACLR_SPEC>;
#[doc = "General Status Clear Register"]
pub mod usbstaclr;
#[doc = "USBSTASET register accessor: an alias for `Reg<USBSTASET_SPEC>`"]
pub type USBSTASET = crate::Reg<usbstaset::USBSTASET_SPEC>;
#[doc = "General Status Set Register"]
pub mod usbstaset;
#[doc = "UVERS register accessor: an alias for `Reg<UVERS_SPEC>`"]
pub type UVERS = crate::Reg<uvers::UVERS_SPEC>;
#[doc = "IP Version Register"]
pub mod uvers;
