#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Network Control Register"]
    pub ncr: NCR,
    #[doc = "0x04 - Network Configuration Register"]
    pub ncfgr: NCFGR,
    #[doc = "0x08 - Network Status Register"]
    pub nsr: NSR,
    #[doc = "0x0c - User Register"]
    pub ur: UR,
    #[doc = "0x10 - DMA Configuration Register"]
    pub dcfgr: DCFGR,
    #[doc = "0x14 - Transmit Status Register"]
    pub tsr: TSR,
    #[doc = "0x18 - Receive Buffer Queue Base Address Register"]
    pub rbqb: RBQB,
    #[doc = "0x1c - Transmit Buffer Queue Base Address Register"]
    pub tbqb: TBQB,
    #[doc = "0x20 - Receive Status Register"]
    pub rsr: RSR,
    #[doc = "0x24 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x2c - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x30 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x34 - PHY Maintenance Register"]
    pub man: MAN,
    #[doc = "0x38 - Received Pause Quantum Register"]
    pub rpq: RPQ,
    #[doc = "0x3c - Transmit Pause Quantum Register"]
    pub tpq: TPQ,
    #[doc = "0x40 - TX Partial Store and Forward Register"]
    pub tpsf: TPSF,
    #[doc = "0x44 - RX Partial Store and Forward Register"]
    pub rpsf: RPSF,
    #[doc = "0x48 - RX Jumbo Frame Max Length Register"]
    pub rjfml: RJFML,
    _reserved19: [u8; 52usize],
    #[doc = "0x80 - Hash Register Bottom"]
    pub hrb: HRB,
    #[doc = "0x84 - Hash Register Top"]
    pub hrt: HRT,
    #[doc = "0x88 - Specific Address 1 Bottom Register"]
    pub gmac_sa1: GMAC_SA,
    #[doc = "0x90 - Specific Address 1 Bottom Register"]
    pub gmac_sa2: GMAC_SA,
    #[doc = "0x98 - Specific Address 1 Bottom Register"]
    pub gmac_sa3: GMAC_SA,
    #[doc = "0xa0 - Specific Address 1 Bottom Register"]
    pub gmac_sa4: GMAC_SA,
    #[doc = "0xa8 - Type ID Match 1 Register"]
    pub tidm1: TIDM1,
    #[doc = "0xac - Type ID Match 2 Register"]
    pub tidm2: TIDM2,
    #[doc = "0xb0 - Type ID Match 3 Register"]
    pub tidm3: TIDM3,
    #[doc = "0xb4 - Type ID Match 4 Register"]
    pub tidm4: TIDM4,
    #[doc = "0xb8 - Wake on LAN Register"]
    pub wol: WOL,
    #[doc = "0xbc - IPG Stretch Register"]
    pub ipgs: IPGS,
    #[doc = "0xc0 - Stacked VLAN Register"]
    pub svlan: SVLAN,
    #[doc = "0xc4 - Transmit PFC Pause Register"]
    pub tpfcp: TPFCP,
    #[doc = "0xc8 - Specific Address 1 Mask Bottom Register"]
    pub samb1: SAMB1,
    #[doc = "0xcc - Specific Address 1 Mask Top Register"]
    pub samt1: SAMT1,
    _reserved35: [u8; 12usize],
    #[doc = "0xdc - 1588 Timer Nanosecond Comparison Register"]
    pub nsc: NSC,
    #[doc = "0xe0 - 1588 Timer Second Comparison Low Register"]
    pub scl: SCL,
    #[doc = "0xe4 - 1588 Timer Second Comparison High Register"]
    pub sch: SCH,
    #[doc = "0xe8 - PTP Event Frame Transmitted Seconds High Register"]
    pub eftsh: EFTSH,
    #[doc = "0xec - PTP Event Frame Received Seconds High Register"]
    pub efrsh: EFRSH,
    #[doc = "0xf0 - PTP Peer Event Frame Transmitted Seconds High Register"]
    pub peftsh: PEFTSH,
    #[doc = "0xf4 - PTP Peer Event Frame Received Seconds High Register"]
    pub pefrsh: PEFRSH,
    _reserved42: [u8; 8usize],
    #[doc = "0x100 - Octets Transmitted Low Register"]
    pub otlo: OTLO,
    #[doc = "0x104 - Octets Transmitted High Register"]
    pub othi: OTHI,
    #[doc = "0x108 - Frames Transmitted Register"]
    pub ft: FT,
    #[doc = "0x10c - Broadcast Frames Transmitted Register"]
    pub bcft: BCFT,
    #[doc = "0x110 - Multicast Frames Transmitted Register"]
    pub mft: MFT,
    #[doc = "0x114 - Pause Frames Transmitted Register"]
    pub pft: PFT,
    #[doc = "0x118 - 64 Byte Frames Transmitted Register"]
    pub bft64: BFT64,
    #[doc = "0x11c - 65 to 127 Byte Frames Transmitted Register"]
    pub tbft127: TBFT127,
    #[doc = "0x120 - 128 to 255 Byte Frames Transmitted Register"]
    pub tbft255: TBFT255,
    #[doc = "0x124 - 256 to 511 Byte Frames Transmitted Register"]
    pub tbft511: TBFT511,
    #[doc = "0x128 - 512 to 1023 Byte Frames Transmitted Register"]
    pub tbft1023: TBFT1023,
    #[doc = "0x12c - 1024 to 1518 Byte Frames Transmitted Register"]
    pub tbft1518: TBFT1518,
    #[doc = "0x130 - Greater Than 1518 Byte Frames Transmitted Register"]
    pub gtbft1518: GTBFT1518,
    #[doc = "0x134 - Transmit Underruns Register"]
    pub tur: TUR,
    #[doc = "0x138 - Single Collision Frames Register"]
    pub scf: SCF,
    #[doc = "0x13c - Multiple Collision Frames Register"]
    pub mcf: MCF,
    #[doc = "0x140 - Excessive Collisions Register"]
    pub ec: EC,
    #[doc = "0x144 - Late Collisions Register"]
    pub lc: LC,
    #[doc = "0x148 - Deferred Transmission Frames Register"]
    pub dtf: DTF,
    #[doc = "0x14c - Carrier Sense Errors Register"]
    pub cse: CSE,
    #[doc = "0x150 - Octets Received Low Received Register"]
    pub orlo: ORLO,
    #[doc = "0x154 - Octets Received High Received Register"]
    pub orhi: ORHI,
    #[doc = "0x158 - Frames Received Register"]
    pub fr: FR,
    #[doc = "0x15c - Broadcast Frames Received Register"]
    pub bcfr: BCFR,
    #[doc = "0x160 - Multicast Frames Received Register"]
    pub mfr: MFR,
    #[doc = "0x164 - Pause Frames Received Register"]
    pub pfr: PFR,
    #[doc = "0x168 - 64 Byte Frames Received Register"]
    pub bfr64: BFR64,
    #[doc = "0x16c - 65 to 127 Byte Frames Received Register"]
    pub tbfr127: TBFR127,
    #[doc = "0x170 - 128 to 255 Byte Frames Received Register"]
    pub tbfr255: TBFR255,
    #[doc = "0x174 - 256 to 511 Byte Frames Received Register"]
    pub tbfr511: TBFR511,
    #[doc = "0x178 - 512 to 1023 Byte Frames Received Register"]
    pub tbfr1023: TBFR1023,
    #[doc = "0x17c - 1024 to 1518 Byte Frames Received Register"]
    pub tbfr1518: TBFR1518,
    #[doc = "0x180 - 1519 to Maximum Byte Frames Received Register"]
    pub tmxbfr: TMXBFR,
    #[doc = "0x184 - Undersize Frames Received Register"]
    pub ufr: UFR,
    #[doc = "0x188 - Oversize Frames Received Register"]
    pub ofr: OFR,
    #[doc = "0x18c - Jabbers Received Register"]
    pub jr: JR,
    #[doc = "0x190 - Frame Check Sequence Errors Register"]
    pub fcse: FCSE,
    #[doc = "0x194 - Length Field Frame Errors Register"]
    pub lffe: LFFE,
    #[doc = "0x198 - Receive Symbol Errors Register"]
    pub rse: RSE,
    #[doc = "0x19c - Alignment Errors Register"]
    pub ae: AE,
    #[doc = "0x1a0 - Receive Resource Errors Register"]
    pub rre: RRE,
    #[doc = "0x1a4 - Receive Overrun Register"]
    pub roe: ROE,
    #[doc = "0x1a8 - IP Header Checksum Errors Register"]
    pub ihce: IHCE,
    #[doc = "0x1ac - TCP Checksum Errors Register"]
    pub tce: TCE,
    #[doc = "0x1b0 - UDP Checksum Errors Register"]
    pub uce: UCE,
    _reserved87: [u8; 8usize],
    #[doc = "0x1bc - 1588 Timer Increment Sub-nanoseconds Register"]
    pub tisubn: TISUBN,
    #[doc = "0x1c0 - 1588 Timer Seconds High Register"]
    pub tsh: TSH,
    _reserved89: [u8; 12usize],
    #[doc = "0x1d0 - 1588 Timer Seconds Low Register"]
    pub tsl: TSL,
    #[doc = "0x1d4 - 1588 Timer Nanoseconds Register"]
    pub tn: TN,
    #[doc = "0x1d8 - 1588 Timer Adjust Register"]
    pub ta: TA,
    #[doc = "0x1dc - 1588 Timer Increment Register"]
    pub ti: TI,
    #[doc = "0x1e0 - PTP Event Frame Transmitted Seconds Low Register"]
    pub eftsl: EFTSL,
    #[doc = "0x1e4 - PTP Event Frame Transmitted Nanoseconds Register"]
    pub eftn: EFTN,
    #[doc = "0x1e8 - PTP Event Frame Received Seconds Low Register"]
    pub efrsl: EFRSL,
    #[doc = "0x1ec - PTP Event Frame Received Nanoseconds Register"]
    pub efrn: EFRN,
    #[doc = "0x1f0 - PTP Peer Event Frame Transmitted Seconds Low Register"]
    pub peftsl: PEFTSL,
    #[doc = "0x1f4 - PTP Peer Event Frame Transmitted Nanoseconds Register"]
    pub peftn: PEFTN,
    #[doc = "0x1f8 - PTP Peer Event Frame Received Seconds Low Register"]
    pub pefrsl: PEFRSL,
    #[doc = "0x1fc - PTP Peer Event Frame Received Nanoseconds Register"]
    pub pefrn: PEFRN,
    _reserved101: [u8; 508usize],
    #[doc = "0x3fc - Interrupt Status Register Priority Queue (index = 1) 0"]
    pub isrpq: [ISRPQ; 2],
    _reserved102: [u8; 56usize],
    #[doc = "0x43c - Transmit Buffer Queue Base Address Register Priority Queue (index = 1) 0"]
    pub tbqbapq: [TBQBAPQ; 2],
    _reserved103: [u8; 56usize],
    #[doc = "0x47c - Receive Buffer Queue Base Address Register Priority Queue (index = 1) 0"]
    pub rbqbapq: [RBQBAPQ; 2],
    _reserved104: [u8; 24usize],
    #[doc = "0x49c - Receive Buffer Size Register Priority Queue (index = 1) 0"]
    pub rbsrpq: [RBSRPQ; 2],
    _reserved105: [u8; 24usize],
    #[doc = "0x4bc - Credit-Based Shaping Control Register"]
    pub cbscr: CBSCR,
    #[doc = "0x4c0 - Credit-Based Shaping IdleSlope Register for Queue A"]
    pub cbsisqa: CBSISQA,
    #[doc = "0x4c4 - Credit-Based Shaping IdleSlope Register for Queue B"]
    pub cbsisqb: CBSISQB,
    _reserved108: [u8; 56usize],
    #[doc = "0x500 - Screening Type 1 Register Priority Queue (index = 0) 0"]
    pub st1rpq: [ST1RPQ; 4],
    _reserved109: [u8; 48usize],
    #[doc = "0x540 - Screening Type 2 Register Priority Queue (index = 0) 0"]
    pub st2rpq: [ST2RPQ; 8],
    _reserved110: [u8; 156usize],
    #[doc = "0x5fc - Interrupt Enable Register Priority Queue (index = 1) 0"]
    pub ierpq: [IERPQ; 2],
    _reserved111: [u8; 24usize],
    #[doc = "0x61c - Interrupt Disable Register Priority Queue (index = 1) 0"]
    pub idrpq: [IDRPQ; 2],
    _reserved112: [u8; 24usize],
    #[doc = "0x63c - Interrupt Mask Register Priority Queue (index = 1) 0"]
    pub imrpq: [IMRPQ; 2],
    _reserved113: [u8; 156usize],
    #[doc = "0x6e0 - Screening Type 2 Ethertype Register (index = 0) 0"]
    pub st2er: [ST2ER; 4],
    _reserved114: [u8; 16usize],
    #[doc = "0x700 - Screening Type 2 Compare Word 0 Register (index = 0)"]
    pub st2cw00: ST2CW00,
    #[doc = "0x704 - Screening Type 2 Compare Word 1 Register (index = 0)"]
    pub st2cw10: ST2CW10,
    #[doc = "0x708 - Screening Type 2 Compare Word 0 Register (index = 1)"]
    pub st2cw01: ST2CW01,
    #[doc = "0x70c - Screening Type 2 Compare Word 1 Register (index = 1)"]
    pub st2cw11: ST2CW11,
    #[doc = "0x710 - Screening Type 2 Compare Word 0 Register (index = 2)"]
    pub st2cw02: ST2CW02,
    #[doc = "0x714 - Screening Type 2 Compare Word 1 Register (index = 2)"]
    pub st2cw12: ST2CW12,
    #[doc = "0x718 - Screening Type 2 Compare Word 0 Register (index = 3)"]
    pub st2cw03: ST2CW03,
    #[doc = "0x71c - Screening Type 2 Compare Word 1 Register (index = 3)"]
    pub st2cw13: ST2CW13,
    #[doc = "0x720 - Screening Type 2 Compare Word 0 Register (index = 4)"]
    pub st2cw04: ST2CW04,
    #[doc = "0x724 - Screening Type 2 Compare Word 1 Register (index = 4)"]
    pub st2cw14: ST2CW14,
    #[doc = "0x728 - Screening Type 2 Compare Word 0 Register (index = 5)"]
    pub st2cw05: ST2CW05,
    #[doc = "0x72c - Screening Type 2 Compare Word 1 Register (index = 5)"]
    pub st2cw15: ST2CW15,
    #[doc = "0x730 - Screening Type 2 Compare Word 0 Register (index = 6)"]
    pub st2cw06: ST2CW06,
    #[doc = "0x734 - Screening Type 2 Compare Word 1 Register (index = 6)"]
    pub st2cw16: ST2CW16,
    #[doc = "0x738 - Screening Type 2 Compare Word 0 Register (index = 7)"]
    pub st2cw07: ST2CW07,
    #[doc = "0x73c - Screening Type 2 Compare Word 1 Register (index = 7)"]
    pub st2cw17: ST2CW17,
    #[doc = "0x740 - Screening Type 2 Compare Word 0 Register (index = 8)"]
    pub st2cw08: ST2CW08,
    #[doc = "0x744 - Screening Type 2 Compare Word 1 Register (index = 8)"]
    pub st2cw18: ST2CW18,
    #[doc = "0x748 - Screening Type 2 Compare Word 0 Register (index = 9)"]
    pub st2cw09: ST2CW09,
    #[doc = "0x74c - Screening Type 2 Compare Word 1 Register (index = 9)"]
    pub st2cw19: ST2CW19,
    #[doc = "0x750 - Screening Type 2 Compare Word 0 Register (index = 10)"]
    pub st2cw010: ST2CW010,
    #[doc = "0x754 - Screening Type 2 Compare Word 1 Register (index = 10)"]
    pub st2cw110: ST2CW110,
    #[doc = "0x758 - Screening Type 2 Compare Word 0 Register (index = 11)"]
    pub st2cw011: ST2CW011,
    #[doc = "0x75c - Screening Type 2 Compare Word 1 Register (index = 11)"]
    pub st2cw111: ST2CW111,
    #[doc = "0x760 - Screening Type 2 Compare Word 0 Register (index = 12)"]
    pub st2cw012: ST2CW012,
    #[doc = "0x764 - Screening Type 2 Compare Word 1 Register (index = 12)"]
    pub st2cw112: ST2CW112,
    #[doc = "0x768 - Screening Type 2 Compare Word 0 Register (index = 13)"]
    pub st2cw013: ST2CW013,
    #[doc = "0x76c - Screening Type 2 Compare Word 1 Register (index = 13)"]
    pub st2cw113: ST2CW113,
    #[doc = "0x770 - Screening Type 2 Compare Word 0 Register (index = 14)"]
    pub st2cw014: ST2CW014,
    #[doc = "0x774 - Screening Type 2 Compare Word 1 Register (index = 14)"]
    pub st2cw114: ST2CW114,
    #[doc = "0x778 - Screening Type 2 Compare Word 0 Register (index = 15)"]
    pub st2cw015: ST2CW015,
    #[doc = "0x77c - Screening Type 2 Compare Word 1 Register (index = 15)"]
    pub st2cw115: ST2CW115,
    #[doc = "0x780 - Screening Type 2 Compare Word 0 Register (index = 16)"]
    pub st2cw016: ST2CW016,
    #[doc = "0x784 - Screening Type 2 Compare Word 1 Register (index = 16)"]
    pub st2cw116: ST2CW116,
    #[doc = "0x788 - Screening Type 2 Compare Word 0 Register (index = 17)"]
    pub st2cw017: ST2CW017,
    #[doc = "0x78c - Screening Type 2 Compare Word 1 Register (index = 17)"]
    pub st2cw117: ST2CW117,
    #[doc = "0x790 - Screening Type 2 Compare Word 0 Register (index = 18)"]
    pub st2cw018: ST2CW018,
    #[doc = "0x794 - Screening Type 2 Compare Word 1 Register (index = 18)"]
    pub st2cw118: ST2CW118,
    #[doc = "0x798 - Screening Type 2 Compare Word 0 Register (index = 19)"]
    pub st2cw019: ST2CW019,
    #[doc = "0x79c - Screening Type 2 Compare Word 1 Register (index = 19)"]
    pub st2cw119: ST2CW119,
    #[doc = "0x7a0 - Screening Type 2 Compare Word 0 Register (index = 20)"]
    pub st2cw020: ST2CW020,
    #[doc = "0x7a4 - Screening Type 2 Compare Word 1 Register (index = 20)"]
    pub st2cw120: ST2CW120,
    #[doc = "0x7a8 - Screening Type 2 Compare Word 0 Register (index = 21)"]
    pub st2cw021: ST2CW021,
    #[doc = "0x7ac - Screening Type 2 Compare Word 1 Register (index = 21)"]
    pub st2cw121: ST2CW121,
    #[doc = "0x7b0 - Screening Type 2 Compare Word 0 Register (index = 22)"]
    pub st2cw022: ST2CW022,
    #[doc = "0x7b4 - Screening Type 2 Compare Word 1 Register (index = 22)"]
    pub st2cw122: ST2CW122,
    #[doc = "0x7b8 - Screening Type 2 Compare Word 0 Register (index = 23)"]
    pub st2cw023: ST2CW023,
    #[doc = "0x7bc - Screening Type 2 Compare Word 1 Register (index = 23)"]
    pub st2cw123: ST2CW123,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct GMAC_SA {
    #[doc = "0x00 - Specific Address 1 Bottom Register"]
    pub sab: self::gmac_sa::SAB,
    #[doc = "0x04 - Specific Address 1 Top Register"]
    pub sat: self::gmac_sa::SAT,
}
#[doc = r"Register block"]
#[doc = "Specific Address 1 Bottom Register"]
pub mod gmac_sa;
#[doc = "Network Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncr](ncr) module"]
pub type NCR = crate::Reg<u32, _NCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NCR;
#[doc = "`read()` method returns [ncr::R](ncr::R) reader structure"]
impl crate::Readable for NCR {}
#[doc = "`write(|w| ..)` method takes [ncr::W](ncr::W) writer structure"]
impl crate::Writable for NCR {}
#[doc = "Network Control Register"]
pub mod ncr;
#[doc = "Network Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncfgr](ncfgr) module"]
pub type NCFGR = crate::Reg<u32, _NCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NCFGR;
#[doc = "`read()` method returns [ncfgr::R](ncfgr::R) reader structure"]
impl crate::Readable for NCFGR {}
#[doc = "`write(|w| ..)` method takes [ncfgr::W](ncfgr::W) writer structure"]
impl crate::Writable for NCFGR {}
#[doc = "Network Configuration Register"]
pub mod ncfgr;
#[doc = "Network Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsr](nsr) module"]
pub type NSR = crate::Reg<u32, _NSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSR;
#[doc = "`read()` method returns [nsr::R](nsr::R) reader structure"]
impl crate::Readable for NSR {}
#[doc = "Network Status Register"]
pub mod nsr;
#[doc = "User Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur](ur) module"]
pub type UR = crate::Reg<u32, _UR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR;
#[doc = "`read()` method returns [ur::R](ur::R) reader structure"]
impl crate::Readable for UR {}
#[doc = "`write(|w| ..)` method takes [ur::W](ur::W) writer structure"]
impl crate::Writable for UR {}
#[doc = "User Register"]
pub mod ur;
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfgr](dcfgr) module"]
pub type DCFGR = crate::Reg<u32, _DCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCFGR;
#[doc = "`read()` method returns [dcfgr::R](dcfgr::R) reader structure"]
impl crate::Readable for DCFGR {}
#[doc = "`write(|w| ..)` method takes [dcfgr::W](dcfgr::W) writer structure"]
impl crate::Writable for DCFGR {}
#[doc = "DMA Configuration Register"]
pub mod dcfgr;
#[doc = "Transmit Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsr](tsr) module"]
pub type TSR = crate::Reg<u32, _TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSR;
#[doc = "`read()` method returns [tsr::R](tsr::R) reader structure"]
impl crate::Readable for TSR {}
#[doc = "`write(|w| ..)` method takes [tsr::W](tsr::W) writer structure"]
impl crate::Writable for TSR {}
#[doc = "Transmit Status Register"]
pub mod tsr;
#[doc = "Receive Buffer Queue Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbqb](rbqb) module"]
pub type RBQB = crate::Reg<u32, _RBQB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBQB;
#[doc = "`read()` method returns [rbqb::R](rbqb::R) reader structure"]
impl crate::Readable for RBQB {}
#[doc = "`write(|w| ..)` method takes [rbqb::W](rbqb::W) writer structure"]
impl crate::Writable for RBQB {}
#[doc = "Receive Buffer Queue Base Address Register"]
pub mod rbqb;
#[doc = "Transmit Buffer Queue Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbqb](tbqb) module"]
pub type TBQB = crate::Reg<u32, _TBQB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBQB;
#[doc = "`read()` method returns [tbqb::R](tbqb::R) reader structure"]
impl crate::Readable for TBQB {}
#[doc = "`write(|w| ..)` method takes [tbqb::W](tbqb::W) writer structure"]
impl crate::Writable for TBQB {}
#[doc = "Transmit Buffer Queue Base Address Register"]
pub mod tbqb;
#[doc = "Receive Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsr](rsr) module"]
pub type RSR = crate::Reg<u32, _RSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSR;
#[doc = "`read()` method returns [rsr::R](rsr::R) reader structure"]
impl crate::Readable for RSR {}
#[doc = "`write(|w| ..)` method takes [rsr::W](rsr::W) writer structure"]
impl crate::Writable for RSR {}
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "`write(|w| ..)` method takes [imr::W](imr::W) writer structure"]
impl crate::Writable for IMR {}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "PHY Maintenance Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [man](man) module"]
pub type MAN = crate::Reg<u32, _MAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAN;
#[doc = "`read()` method returns [man::R](man::R) reader structure"]
impl crate::Readable for MAN {}
#[doc = "`write(|w| ..)` method takes [man::W](man::W) writer structure"]
impl crate::Writable for MAN {}
#[doc = "PHY Maintenance Register"]
pub mod man;
#[doc = "Received Pause Quantum Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpq](rpq) module"]
pub type RPQ = crate::Reg<u32, _RPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPQ;
#[doc = "`read()` method returns [rpq::R](rpq::R) reader structure"]
impl crate::Readable for RPQ {}
#[doc = "Received Pause Quantum Register"]
pub mod rpq;
#[doc = "Transmit Pause Quantum Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpq](tpq) module"]
pub type TPQ = crate::Reg<u32, _TPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPQ;
#[doc = "`read()` method returns [tpq::R](tpq::R) reader structure"]
impl crate::Readable for TPQ {}
#[doc = "`write(|w| ..)` method takes [tpq::W](tpq::W) writer structure"]
impl crate::Writable for TPQ {}
#[doc = "Transmit Pause Quantum Register"]
pub mod tpq;
#[doc = "TX Partial Store and Forward Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpsf](tpsf) module"]
pub type TPSF = crate::Reg<u32, _TPSF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPSF;
#[doc = "`read()` method returns [tpsf::R](tpsf::R) reader structure"]
impl crate::Readable for TPSF {}
#[doc = "`write(|w| ..)` method takes [tpsf::W](tpsf::W) writer structure"]
impl crate::Writable for TPSF {}
#[doc = "TX Partial Store and Forward Register"]
pub mod tpsf;
#[doc = "RX Partial Store and Forward Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpsf](rpsf) module"]
pub type RPSF = crate::Reg<u32, _RPSF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPSF;
#[doc = "`read()` method returns [rpsf::R](rpsf::R) reader structure"]
impl crate::Readable for RPSF {}
#[doc = "`write(|w| ..)` method takes [rpsf::W](rpsf::W) writer structure"]
impl crate::Writable for RPSF {}
#[doc = "RX Partial Store and Forward Register"]
pub mod rpsf;
#[doc = "RX Jumbo Frame Max Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rjfml](rjfml) module"]
pub type RJFML = crate::Reg<u32, _RJFML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RJFML;
#[doc = "`read()` method returns [rjfml::R](rjfml::R) reader structure"]
impl crate::Readable for RJFML {}
#[doc = "`write(|w| ..)` method takes [rjfml::W](rjfml::W) writer structure"]
impl crate::Writable for RJFML {}
#[doc = "RX Jumbo Frame Max Length Register"]
pub mod rjfml;
#[doc = "Hash Register Bottom\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrb](hrb) module"]
pub type HRB = crate::Reg<u32, _HRB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRB;
#[doc = "`read()` method returns [hrb::R](hrb::R) reader structure"]
impl crate::Readable for HRB {}
#[doc = "`write(|w| ..)` method takes [hrb::W](hrb::W) writer structure"]
impl crate::Writable for HRB {}
#[doc = "Hash Register Bottom"]
pub mod hrb;
#[doc = "Hash Register Top\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrt](hrt) module"]
pub type HRT = crate::Reg<u32, _HRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRT;
#[doc = "`read()` method returns [hrt::R](hrt::R) reader structure"]
impl crate::Readable for HRT {}
#[doc = "`write(|w| ..)` method takes [hrt::W](hrt::W) writer structure"]
impl crate::Writable for HRT {}
#[doc = "Hash Register Top"]
pub mod hrt;
#[doc = "Type ID Match 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tidm1](tidm1) module"]
pub type TIDM1 = crate::Reg<u32, _TIDM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIDM1;
#[doc = "`read()` method returns [tidm1::R](tidm1::R) reader structure"]
impl crate::Readable for TIDM1 {}
#[doc = "`write(|w| ..)` method takes [tidm1::W](tidm1::W) writer structure"]
impl crate::Writable for TIDM1 {}
#[doc = "Type ID Match 1 Register"]
pub mod tidm1;
#[doc = "Type ID Match 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tidm2](tidm2) module"]
pub type TIDM2 = crate::Reg<u32, _TIDM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIDM2;
#[doc = "`read()` method returns [tidm2::R](tidm2::R) reader structure"]
impl crate::Readable for TIDM2 {}
#[doc = "`write(|w| ..)` method takes [tidm2::W](tidm2::W) writer structure"]
impl crate::Writable for TIDM2 {}
#[doc = "Type ID Match 2 Register"]
pub mod tidm2;
#[doc = "Type ID Match 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tidm3](tidm3) module"]
pub type TIDM3 = crate::Reg<u32, _TIDM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIDM3;
#[doc = "`read()` method returns [tidm3::R](tidm3::R) reader structure"]
impl crate::Readable for TIDM3 {}
#[doc = "`write(|w| ..)` method takes [tidm3::W](tidm3::W) writer structure"]
impl crate::Writable for TIDM3 {}
#[doc = "Type ID Match 3 Register"]
pub mod tidm3;
#[doc = "Type ID Match 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tidm4](tidm4) module"]
pub type TIDM4 = crate::Reg<u32, _TIDM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIDM4;
#[doc = "`read()` method returns [tidm4::R](tidm4::R) reader structure"]
impl crate::Readable for TIDM4 {}
#[doc = "`write(|w| ..)` method takes [tidm4::W](tidm4::W) writer structure"]
impl crate::Writable for TIDM4 {}
#[doc = "Type ID Match 4 Register"]
pub mod tidm4;
#[doc = "Wake on LAN Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wol](wol) module"]
pub type WOL = crate::Reg<u32, _WOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WOL;
#[doc = "`read()` method returns [wol::R](wol::R) reader structure"]
impl crate::Readable for WOL {}
#[doc = "`write(|w| ..)` method takes [wol::W](wol::W) writer structure"]
impl crate::Writable for WOL {}
#[doc = "Wake on LAN Register"]
pub mod wol;
#[doc = "IPG Stretch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipgs](ipgs) module"]
pub type IPGS = crate::Reg<u32, _IPGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPGS;
#[doc = "`read()` method returns [ipgs::R](ipgs::R) reader structure"]
impl crate::Readable for IPGS {}
#[doc = "`write(|w| ..)` method takes [ipgs::W](ipgs::W) writer structure"]
impl crate::Writable for IPGS {}
#[doc = "IPG Stretch Register"]
pub mod ipgs;
#[doc = "Stacked VLAN Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [svlan](svlan) module"]
pub type SVLAN = crate::Reg<u32, _SVLAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SVLAN;
#[doc = "`read()` method returns [svlan::R](svlan::R) reader structure"]
impl crate::Readable for SVLAN {}
#[doc = "`write(|w| ..)` method takes [svlan::W](svlan::W) writer structure"]
impl crate::Writable for SVLAN {}
#[doc = "Stacked VLAN Register"]
pub mod svlan;
#[doc = "Transmit PFC Pause Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpfcp](tpfcp) module"]
pub type TPFCP = crate::Reg<u32, _TPFCP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPFCP;
#[doc = "`read()` method returns [tpfcp::R](tpfcp::R) reader structure"]
impl crate::Readable for TPFCP {}
#[doc = "`write(|w| ..)` method takes [tpfcp::W](tpfcp::W) writer structure"]
impl crate::Writable for TPFCP {}
#[doc = "Transmit PFC Pause Register"]
pub mod tpfcp;
#[doc = "Specific Address 1 Mask Bottom Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [samb1](samb1) module"]
pub type SAMB1 = crate::Reg<u32, _SAMB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAMB1;
#[doc = "`read()` method returns [samb1::R](samb1::R) reader structure"]
impl crate::Readable for SAMB1 {}
#[doc = "`write(|w| ..)` method takes [samb1::W](samb1::W) writer structure"]
impl crate::Writable for SAMB1 {}
#[doc = "Specific Address 1 Mask Bottom Register"]
pub mod samb1;
#[doc = "Specific Address 1 Mask Top Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [samt1](samt1) module"]
pub type SAMT1 = crate::Reg<u32, _SAMT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAMT1;
#[doc = "`read()` method returns [samt1::R](samt1::R) reader structure"]
impl crate::Readable for SAMT1 {}
#[doc = "`write(|w| ..)` method takes [samt1::W](samt1::W) writer structure"]
impl crate::Writable for SAMT1 {}
#[doc = "Specific Address 1 Mask Top Register"]
pub mod samt1;
#[doc = "1588 Timer Nanosecond Comparison Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsc](nsc) module"]
pub type NSC = crate::Reg<u32, _NSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSC;
#[doc = "`read()` method returns [nsc::R](nsc::R) reader structure"]
impl crate::Readable for NSC {}
#[doc = "`write(|w| ..)` method takes [nsc::W](nsc::W) writer structure"]
impl crate::Writable for NSC {}
#[doc = "1588 Timer Nanosecond Comparison Register"]
pub mod nsc;
#[doc = "1588 Timer Second Comparison Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl](scl) module"]
pub type SCL = crate::Reg<u32, _SCL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL;
#[doc = "`read()` method returns [scl::R](scl::R) reader structure"]
impl crate::Readable for SCL {}
#[doc = "`write(|w| ..)` method takes [scl::W](scl::W) writer structure"]
impl crate::Writable for SCL {}
#[doc = "1588 Timer Second Comparison Low Register"]
pub mod scl;
#[doc = "1588 Timer Second Comparison High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sch](sch) module"]
pub type SCH = crate::Reg<u32, _SCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCH;
#[doc = "`read()` method returns [sch::R](sch::R) reader structure"]
impl crate::Readable for SCH {}
#[doc = "`write(|w| ..)` method takes [sch::W](sch::W) writer structure"]
impl crate::Writable for SCH {}
#[doc = "1588 Timer Second Comparison High Register"]
pub mod sch;
#[doc = "PTP Event Frame Transmitted Seconds High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eftsh](eftsh) module"]
pub type EFTSH = crate::Reg<u32, _EFTSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFTSH;
#[doc = "`read()` method returns [eftsh::R](eftsh::R) reader structure"]
impl crate::Readable for EFTSH {}
#[doc = "PTP Event Frame Transmitted Seconds High Register"]
pub mod eftsh;
#[doc = "PTP Event Frame Received Seconds High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efrsh](efrsh) module"]
pub type EFRSH = crate::Reg<u32, _EFRSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFRSH;
#[doc = "`read()` method returns [efrsh::R](efrsh::R) reader structure"]
impl crate::Readable for EFRSH {}
#[doc = "PTP Event Frame Received Seconds High Register"]
pub mod efrsh;
#[doc = "PTP Peer Event Frame Transmitted Seconds High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peftsh](peftsh) module"]
pub type PEFTSH = crate::Reg<u32, _PEFTSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEFTSH;
#[doc = "`read()` method returns [peftsh::R](peftsh::R) reader structure"]
impl crate::Readable for PEFTSH {}
#[doc = "PTP Peer Event Frame Transmitted Seconds High Register"]
pub mod peftsh;
#[doc = "PTP Peer Event Frame Received Seconds High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pefrsh](pefrsh) module"]
pub type PEFRSH = crate::Reg<u32, _PEFRSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEFRSH;
#[doc = "`read()` method returns [pefrsh::R](pefrsh::R) reader structure"]
impl crate::Readable for PEFRSH {}
#[doc = "PTP Peer Event Frame Received Seconds High Register"]
pub mod pefrsh;
#[doc = "Octets Transmitted Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otlo](otlo) module"]
pub type OTLO = crate::Reg<u32, _OTLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTLO;
#[doc = "`read()` method returns [otlo::R](otlo::R) reader structure"]
impl crate::Readable for OTLO {}
#[doc = "Octets Transmitted Low Register"]
pub mod otlo;
#[doc = "Octets Transmitted High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [othi](othi) module"]
pub type OTHI = crate::Reg<u32, _OTHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTHI;
#[doc = "`read()` method returns [othi::R](othi::R) reader structure"]
impl crate::Readable for OTHI {}
#[doc = "Octets Transmitted High Register"]
pub mod othi;
#[doc = "Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ft](ft) module"]
pub type FT = crate::Reg<u32, _FT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FT;
#[doc = "`read()` method returns [ft::R](ft::R) reader structure"]
impl crate::Readable for FT {}
#[doc = "Frames Transmitted Register"]
pub mod ft;
#[doc = "Broadcast Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcft](bcft) module"]
pub type BCFT = crate::Reg<u32, _BCFT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCFT;
#[doc = "`read()` method returns [bcft::R](bcft::R) reader structure"]
impl crate::Readable for BCFT {}
#[doc = "Broadcast Frames Transmitted Register"]
pub mod bcft;
#[doc = "Multicast Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mft](mft) module"]
pub type MFT = crate::Reg<u32, _MFT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MFT;
#[doc = "`read()` method returns [mft::R](mft::R) reader structure"]
impl crate::Readable for MFT {}
#[doc = "Multicast Frames Transmitted Register"]
pub mod mft;
#[doc = "Pause Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pft](pft) module"]
pub type PFT = crate::Reg<u32, _PFT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFT;
#[doc = "`read()` method returns [pft::R](pft::R) reader structure"]
impl crate::Readable for PFT {}
#[doc = "Pause Frames Transmitted Register"]
pub mod pft;
#[doc = "64 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bft64](bft64) module"]
pub type BFT64 = crate::Reg<u32, _BFT64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFT64;
#[doc = "`read()` method returns [bft64::R](bft64::R) reader structure"]
impl crate::Readable for BFT64 {}
#[doc = "64 Byte Frames Transmitted Register"]
pub mod bft64;
#[doc = "65 to 127 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbft127](tbft127) module"]
pub type TBFT127 = crate::Reg<u32, _TBFT127>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFT127;
#[doc = "`read()` method returns [tbft127::R](tbft127::R) reader structure"]
impl crate::Readable for TBFT127 {}
#[doc = "65 to 127 Byte Frames Transmitted Register"]
pub mod tbft127;
#[doc = "128 to 255 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbft255](tbft255) module"]
pub type TBFT255 = crate::Reg<u32, _TBFT255>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFT255;
#[doc = "`read()` method returns [tbft255::R](tbft255::R) reader structure"]
impl crate::Readable for TBFT255 {}
#[doc = "128 to 255 Byte Frames Transmitted Register"]
pub mod tbft255;
#[doc = "256 to 511 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbft511](tbft511) module"]
pub type TBFT511 = crate::Reg<u32, _TBFT511>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFT511;
#[doc = "`read()` method returns [tbft511::R](tbft511::R) reader structure"]
impl crate::Readable for TBFT511 {}
#[doc = "256 to 511 Byte Frames Transmitted Register"]
pub mod tbft511;
#[doc = "512 to 1023 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbft1023](tbft1023) module"]
pub type TBFT1023 = crate::Reg<u32, _TBFT1023>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFT1023;
#[doc = "`read()` method returns [tbft1023::R](tbft1023::R) reader structure"]
impl crate::Readable for TBFT1023 {}
#[doc = "512 to 1023 Byte Frames Transmitted Register"]
pub mod tbft1023;
#[doc = "1024 to 1518 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbft1518](tbft1518) module"]
pub type TBFT1518 = crate::Reg<u32, _TBFT1518>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFT1518;
#[doc = "`read()` method returns [tbft1518::R](tbft1518::R) reader structure"]
impl crate::Readable for TBFT1518 {}
#[doc = "1024 to 1518 Byte Frames Transmitted Register"]
pub mod tbft1518;
#[doc = "Greater Than 1518 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtbft1518](gtbft1518) module"]
pub type GTBFT1518 = crate::Reg<u32, _GTBFT1518>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GTBFT1518;
#[doc = "`read()` method returns [gtbft1518::R](gtbft1518::R) reader structure"]
impl crate::Readable for GTBFT1518 {}
#[doc = "Greater Than 1518 Byte Frames Transmitted Register"]
pub mod gtbft1518;
#[doc = "Transmit Underruns Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tur](tur) module"]
pub type TUR = crate::Reg<u32, _TUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TUR;
#[doc = "`read()` method returns [tur::R](tur::R) reader structure"]
impl crate::Readable for TUR {}
#[doc = "Transmit Underruns Register"]
pub mod tur;
#[doc = "Single Collision Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scf](scf) module"]
pub type SCF = crate::Reg<u32, _SCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCF;
#[doc = "`read()` method returns [scf::R](scf::R) reader structure"]
impl crate::Readable for SCF {}
#[doc = "Single Collision Frames Register"]
pub mod scf;
#[doc = "Multiple Collision Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcf](mcf) module"]
pub type MCF = crate::Reg<u32, _MCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCF;
#[doc = "`read()` method returns [mcf::R](mcf::R) reader structure"]
impl crate::Readable for MCF {}
#[doc = "Multiple Collision Frames Register"]
pub mod mcf;
#[doc = "Excessive Collisions Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ec](ec) module"]
pub type EC = crate::Reg<u32, _EC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EC;
#[doc = "`read()` method returns [ec::R](ec::R) reader structure"]
impl crate::Readable for EC {}
#[doc = "Excessive Collisions Register"]
pub mod ec;
#[doc = "Late Collisions Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc](lc) module"]
pub type LC = crate::Reg<u32, _LC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LC;
#[doc = "`read()` method returns [lc::R](lc::R) reader structure"]
impl crate::Readable for LC {}
#[doc = "Late Collisions Register"]
pub mod lc;
#[doc = "Deferred Transmission Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtf](dtf) module"]
pub type DTF = crate::Reg<u32, _DTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTF;
#[doc = "`read()` method returns [dtf::R](dtf::R) reader structure"]
impl crate::Readable for DTF {}
#[doc = "Deferred Transmission Frames Register"]
pub mod dtf;
#[doc = "Carrier Sense Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cse](cse) module"]
pub type CSE = crate::Reg<u32, _CSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSE;
#[doc = "`read()` method returns [cse::R](cse::R) reader structure"]
impl crate::Readable for CSE {}
#[doc = "Carrier Sense Errors Register"]
pub mod cse;
#[doc = "Octets Received Low Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [orlo](orlo) module"]
pub type ORLO = crate::Reg<u32, _ORLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ORLO;
#[doc = "`read()` method returns [orlo::R](orlo::R) reader structure"]
impl crate::Readable for ORLO {}
#[doc = "Octets Received Low Received Register"]
pub mod orlo;
#[doc = "Octets Received High Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [orhi](orhi) module"]
pub type ORHI = crate::Reg<u32, _ORHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ORHI;
#[doc = "`read()` method returns [orhi::R](orhi::R) reader structure"]
impl crate::Readable for ORHI {}
#[doc = "Octets Received High Received Register"]
pub mod orhi;
#[doc = "Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](fr) module"]
pub type FR = crate::Reg<u32, _FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FR;
#[doc = "`read()` method returns [fr::R](fr::R) reader structure"]
impl crate::Readable for FR {}
#[doc = "Frames Received Register"]
pub mod fr;
#[doc = "Broadcast Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcfr](bcfr) module"]
pub type BCFR = crate::Reg<u32, _BCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCFR;
#[doc = "`read()` method returns [bcfr::R](bcfr::R) reader structure"]
impl crate::Readable for BCFR {}
#[doc = "Broadcast Frames Received Register"]
pub mod bcfr;
#[doc = "Multicast Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mfr](mfr) module"]
pub type MFR = crate::Reg<u32, _MFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MFR;
#[doc = "`read()` method returns [mfr::R](mfr::R) reader structure"]
impl crate::Readable for MFR {}
#[doc = "Multicast Frames Received Register"]
pub mod mfr;
#[doc = "Pause Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfr](pfr) module"]
pub type PFR = crate::Reg<u32, _PFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFR;
#[doc = "`read()` method returns [pfr::R](pfr::R) reader structure"]
impl crate::Readable for PFR {}
#[doc = "Pause Frames Received Register"]
pub mod pfr;
#[doc = "64 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfr64](bfr64) module"]
pub type BFR64 = crate::Reg<u32, _BFR64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFR64;
#[doc = "`read()` method returns [bfr64::R](bfr64::R) reader structure"]
impl crate::Readable for BFR64 {}
#[doc = "64 Byte Frames Received Register"]
pub mod bfr64;
#[doc = "65 to 127 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbfr127](tbfr127) module"]
pub type TBFR127 = crate::Reg<u32, _TBFR127>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFR127;
#[doc = "`read()` method returns [tbfr127::R](tbfr127::R) reader structure"]
impl crate::Readable for TBFR127 {}
#[doc = "65 to 127 Byte Frames Received Register"]
pub mod tbfr127;
#[doc = "128 to 255 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbfr255](tbfr255) module"]
pub type TBFR255 = crate::Reg<u32, _TBFR255>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFR255;
#[doc = "`read()` method returns [tbfr255::R](tbfr255::R) reader structure"]
impl crate::Readable for TBFR255 {}
#[doc = "128 to 255 Byte Frames Received Register"]
pub mod tbfr255;
#[doc = "256 to 511 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbfr511](tbfr511) module"]
pub type TBFR511 = crate::Reg<u32, _TBFR511>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFR511;
#[doc = "`read()` method returns [tbfr511::R](tbfr511::R) reader structure"]
impl crate::Readable for TBFR511 {}
#[doc = "256 to 511 Byte Frames Received Register"]
pub mod tbfr511;
#[doc = "512 to 1023 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbfr1023](tbfr1023) module"]
pub type TBFR1023 = crate::Reg<u32, _TBFR1023>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFR1023;
#[doc = "`read()` method returns [tbfr1023::R](tbfr1023::R) reader structure"]
impl crate::Readable for TBFR1023 {}
#[doc = "512 to 1023 Byte Frames Received Register"]
pub mod tbfr1023;
#[doc = "1024 to 1518 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbfr1518](tbfr1518) module"]
pub type TBFR1518 = crate::Reg<u32, _TBFR1518>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFR1518;
#[doc = "`read()` method returns [tbfr1518::R](tbfr1518::R) reader structure"]
impl crate::Readable for TBFR1518 {}
#[doc = "1024 to 1518 Byte Frames Received Register"]
pub mod tbfr1518;
#[doc = "1519 to Maximum Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmxbfr](tmxbfr) module"]
pub type TMXBFR = crate::Reg<u32, _TMXBFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMXBFR;
#[doc = "`read()` method returns [tmxbfr::R](tmxbfr::R) reader structure"]
impl crate::Readable for TMXBFR {}
#[doc = "1519 to Maximum Byte Frames Received Register"]
pub mod tmxbfr;
#[doc = "Undersize Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ufr](ufr) module"]
pub type UFR = crate::Reg<u32, _UFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UFR;
#[doc = "`read()` method returns [ufr::R](ufr::R) reader structure"]
impl crate::Readable for UFR {}
#[doc = "Undersize Frames Received Register"]
pub mod ufr;
#[doc = "Oversize Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr](ofr) module"]
pub type OFR = crate::Reg<u32, _OFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFR;
#[doc = "`read()` method returns [ofr::R](ofr::R) reader structure"]
impl crate::Readable for OFR {}
#[doc = "Oversize Frames Received Register"]
pub mod ofr;
#[doc = "Jabbers Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jr](jr) module"]
pub type JR = crate::Reg<u32, _JR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JR;
#[doc = "`read()` method returns [jr::R](jr::R) reader structure"]
impl crate::Readable for JR {}
#[doc = "Jabbers Received Register"]
pub mod jr;
#[doc = "Frame Check Sequence Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcse](fcse) module"]
pub type FCSE = crate::Reg<u32, _FCSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCSE;
#[doc = "`read()` method returns [fcse::R](fcse::R) reader structure"]
impl crate::Readable for FCSE {}
#[doc = "Frame Check Sequence Errors Register"]
pub mod fcse;
#[doc = "Length Field Frame Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lffe](lffe) module"]
pub type LFFE = crate::Reg<u32, _LFFE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFFE;
#[doc = "`read()` method returns [lffe::R](lffe::R) reader structure"]
impl crate::Readable for LFFE {}
#[doc = "Length Field Frame Errors Register"]
pub mod lffe;
#[doc = "Receive Symbol Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rse](rse) module"]
pub type RSE = crate::Reg<u32, _RSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSE;
#[doc = "`read()` method returns [rse::R](rse::R) reader structure"]
impl crate::Readable for RSE {}
#[doc = "Receive Symbol Errors Register"]
pub mod rse;
#[doc = "Alignment Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ae](ae) module"]
pub type AE = crate::Reg<u32, _AE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AE;
#[doc = "`read()` method returns [ae::R](ae::R) reader structure"]
impl crate::Readable for AE {}
#[doc = "Alignment Errors Register"]
pub mod ae;
#[doc = "Receive Resource Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rre](rre) module"]
pub type RRE = crate::Reg<u32, _RRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RRE;
#[doc = "`read()` method returns [rre::R](rre::R) reader structure"]
impl crate::Readable for RRE {}
#[doc = "Receive Resource Errors Register"]
pub mod rre;
#[doc = "Receive Overrun Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [roe](roe) module"]
pub type ROE = crate::Reg<u32, _ROE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROE;
#[doc = "`read()` method returns [roe::R](roe::R) reader structure"]
impl crate::Readable for ROE {}
#[doc = "Receive Overrun Register"]
pub mod roe;
#[doc = "IP Header Checksum Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ihce](ihce) module"]
pub type IHCE = crate::Reg<u32, _IHCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IHCE;
#[doc = "`read()` method returns [ihce::R](ihce::R) reader structure"]
impl crate::Readable for IHCE {}
#[doc = "IP Header Checksum Errors Register"]
pub mod ihce;
#[doc = "TCP Checksum Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tce](tce) module"]
pub type TCE = crate::Reg<u32, _TCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCE;
#[doc = "`read()` method returns [tce::R](tce::R) reader structure"]
impl crate::Readable for TCE {}
#[doc = "TCP Checksum Errors Register"]
pub mod tce;
#[doc = "UDP Checksum Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uce](uce) module"]
pub type UCE = crate::Reg<u32, _UCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCE;
#[doc = "`read()` method returns [uce::R](uce::R) reader structure"]
impl crate::Readable for UCE {}
#[doc = "UDP Checksum Errors Register"]
pub mod uce;
#[doc = "1588 Timer Increment Sub-nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tisubn](tisubn) module"]
pub type TISUBN = crate::Reg<u32, _TISUBN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TISUBN;
#[doc = "`read()` method returns [tisubn::R](tisubn::R) reader structure"]
impl crate::Readable for TISUBN {}
#[doc = "`write(|w| ..)` method takes [tisubn::W](tisubn::W) writer structure"]
impl crate::Writable for TISUBN {}
#[doc = "1588 Timer Increment Sub-nanoseconds Register"]
pub mod tisubn;
#[doc = "1588 Timer Seconds High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsh](tsh) module"]
pub type TSH = crate::Reg<u32, _TSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSH;
#[doc = "`read()` method returns [tsh::R](tsh::R) reader structure"]
impl crate::Readable for TSH {}
#[doc = "`write(|w| ..)` method takes [tsh::W](tsh::W) writer structure"]
impl crate::Writable for TSH {}
#[doc = "1588 Timer Seconds High Register"]
pub mod tsh;
#[doc = "1588 Timer Seconds Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsl](tsl) module"]
pub type TSL = crate::Reg<u32, _TSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSL;
#[doc = "`read()` method returns [tsl::R](tsl::R) reader structure"]
impl crate::Readable for TSL {}
#[doc = "`write(|w| ..)` method takes [tsl::W](tsl::W) writer structure"]
impl crate::Writable for TSL {}
#[doc = "1588 Timer Seconds Low Register"]
pub mod tsl;
#[doc = "1588 Timer Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tn](tn) module"]
pub type TN = crate::Reg<u32, _TN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TN;
#[doc = "`read()` method returns [tn::R](tn::R) reader structure"]
impl crate::Readable for TN {}
#[doc = "`write(|w| ..)` method takes [tn::W](tn::W) writer structure"]
impl crate::Writable for TN {}
#[doc = "1588 Timer Nanoseconds Register"]
pub mod tn;
#[doc = "1588 Timer Adjust Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta](ta) module"]
pub type TA = crate::Reg<u32, _TA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA;
#[doc = "`write(|w| ..)` method takes [ta::W](ta::W) writer structure"]
impl crate::Writable for TA {}
#[doc = "1588 Timer Adjust Register"]
pub mod ta;
#[doc = "1588 Timer Increment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ti](ti) module"]
pub type TI = crate::Reg<u32, _TI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TI;
#[doc = "`read()` method returns [ti::R](ti::R) reader structure"]
impl crate::Readable for TI {}
#[doc = "`write(|w| ..)` method takes [ti::W](ti::W) writer structure"]
impl crate::Writable for TI {}
#[doc = "1588 Timer Increment Register"]
pub mod ti;
#[doc = "PTP Event Frame Transmitted Seconds Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eftsl](eftsl) module"]
pub type EFTSL = crate::Reg<u32, _EFTSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFTSL;
#[doc = "`read()` method returns [eftsl::R](eftsl::R) reader structure"]
impl crate::Readable for EFTSL {}
#[doc = "PTP Event Frame Transmitted Seconds Low Register"]
pub mod eftsl;
#[doc = "PTP Event Frame Transmitted Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eftn](eftn) module"]
pub type EFTN = crate::Reg<u32, _EFTN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFTN;
#[doc = "`read()` method returns [eftn::R](eftn::R) reader structure"]
impl crate::Readable for EFTN {}
#[doc = "PTP Event Frame Transmitted Nanoseconds Register"]
pub mod eftn;
#[doc = "PTP Event Frame Received Seconds Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efrsl](efrsl) module"]
pub type EFRSL = crate::Reg<u32, _EFRSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFRSL;
#[doc = "`read()` method returns [efrsl::R](efrsl::R) reader structure"]
impl crate::Readable for EFRSL {}
#[doc = "PTP Event Frame Received Seconds Low Register"]
pub mod efrsl;
#[doc = "PTP Event Frame Received Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efrn](efrn) module"]
pub type EFRN = crate::Reg<u32, _EFRN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFRN;
#[doc = "`read()` method returns [efrn::R](efrn::R) reader structure"]
impl crate::Readable for EFRN {}
#[doc = "PTP Event Frame Received Nanoseconds Register"]
pub mod efrn;
#[doc = "PTP Peer Event Frame Transmitted Seconds Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peftsl](peftsl) module"]
pub type PEFTSL = crate::Reg<u32, _PEFTSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEFTSL;
#[doc = "`read()` method returns [peftsl::R](peftsl::R) reader structure"]
impl crate::Readable for PEFTSL {}
#[doc = "PTP Peer Event Frame Transmitted Seconds Low Register"]
pub mod peftsl;
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peftn](peftn) module"]
pub type PEFTN = crate::Reg<u32, _PEFTN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEFTN;
#[doc = "`read()` method returns [peftn::R](peftn::R) reader structure"]
impl crate::Readable for PEFTN {}
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register"]
pub mod peftn;
#[doc = "PTP Peer Event Frame Received Seconds Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pefrsl](pefrsl) module"]
pub type PEFRSL = crate::Reg<u32, _PEFRSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEFRSL;
#[doc = "`read()` method returns [pefrsl::R](pefrsl::R) reader structure"]
impl crate::Readable for PEFRSL {}
#[doc = "PTP Peer Event Frame Received Seconds Low Register"]
pub mod pefrsl;
#[doc = "PTP Peer Event Frame Received Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pefrn](pefrn) module"]
pub type PEFRN = crate::Reg<u32, _PEFRN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEFRN;
#[doc = "`read()` method returns [pefrn::R](pefrn::R) reader structure"]
impl crate::Readable for PEFRN {}
#[doc = "PTP Peer Event Frame Received Nanoseconds Register"]
pub mod pefrn;
#[doc = "Interrupt Status Register Priority Queue (index = 1) 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isrpq](isrpq) module"]
pub type ISRPQ = crate::Reg<u32, _ISRPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISRPQ;
#[doc = "`read()` method returns [isrpq::R](isrpq::R) reader structure"]
impl crate::Readable for ISRPQ {}
#[doc = "Interrupt Status Register Priority Queue (index = 1) 0"]
pub mod isrpq;
#[doc = "Transmit Buffer Queue Base Address Register Priority Queue (index = 1) 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbqbapq](tbqbapq) module"]
pub type TBQBAPQ = crate::Reg<u32, _TBQBAPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBQBAPQ;
#[doc = "`read()` method returns [tbqbapq::R](tbqbapq::R) reader structure"]
impl crate::Readable for TBQBAPQ {}
#[doc = "`write(|w| ..)` method takes [tbqbapq::W](tbqbapq::W) writer structure"]
impl crate::Writable for TBQBAPQ {}
#[doc = "Transmit Buffer Queue Base Address Register Priority Queue (index = 1) 0"]
pub mod tbqbapq;
#[doc = "Receive Buffer Queue Base Address Register Priority Queue (index = 1) 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbqbapq](rbqbapq) module"]
pub type RBQBAPQ = crate::Reg<u32, _RBQBAPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBQBAPQ;
#[doc = "`read()` method returns [rbqbapq::R](rbqbapq::R) reader structure"]
impl crate::Readable for RBQBAPQ {}
#[doc = "`write(|w| ..)` method takes [rbqbapq::W](rbqbapq::W) writer structure"]
impl crate::Writable for RBQBAPQ {}
#[doc = "Receive Buffer Queue Base Address Register Priority Queue (index = 1) 0"]
pub mod rbqbapq;
#[doc = "Receive Buffer Size Register Priority Queue (index = 1) 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbsrpq](rbsrpq) module"]
pub type RBSRPQ = crate::Reg<u32, _RBSRPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBSRPQ;
#[doc = "`read()` method returns [rbsrpq::R](rbsrpq::R) reader structure"]
impl crate::Readable for RBSRPQ {}
#[doc = "`write(|w| ..)` method takes [rbsrpq::W](rbsrpq::W) writer structure"]
impl crate::Writable for RBSRPQ {}
#[doc = "Receive Buffer Size Register Priority Queue (index = 1) 0"]
pub mod rbsrpq;
#[doc = "Credit-Based Shaping Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbscr](cbscr) module"]
pub type CBSCR = crate::Reg<u32, _CBSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBSCR;
#[doc = "`read()` method returns [cbscr::R](cbscr::R) reader structure"]
impl crate::Readable for CBSCR {}
#[doc = "`write(|w| ..)` method takes [cbscr::W](cbscr::W) writer structure"]
impl crate::Writable for CBSCR {}
#[doc = "Credit-Based Shaping Control Register"]
pub mod cbscr;
#[doc = "Credit-Based Shaping IdleSlope Register for Queue A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbsisqa](cbsisqa) module"]
pub type CBSISQA = crate::Reg<u32, _CBSISQA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBSISQA;
#[doc = "`read()` method returns [cbsisqa::R](cbsisqa::R) reader structure"]
impl crate::Readable for CBSISQA {}
#[doc = "`write(|w| ..)` method takes [cbsisqa::W](cbsisqa::W) writer structure"]
impl crate::Writable for CBSISQA {}
#[doc = "Credit-Based Shaping IdleSlope Register for Queue A"]
pub mod cbsisqa;
#[doc = "Credit-Based Shaping IdleSlope Register for Queue B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbsisqb](cbsisqb) module"]
pub type CBSISQB = crate::Reg<u32, _CBSISQB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBSISQB;
#[doc = "`read()` method returns [cbsisqb::R](cbsisqb::R) reader structure"]
impl crate::Readable for CBSISQB {}
#[doc = "`write(|w| ..)` method takes [cbsisqb::W](cbsisqb::W) writer structure"]
impl crate::Writable for CBSISQB {}
#[doc = "Credit-Based Shaping IdleSlope Register for Queue B"]
pub mod cbsisqb;
#[doc = "Screening Type 1 Register Priority Queue (index = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st1rpq](st1rpq) module"]
pub type ST1RPQ = crate::Reg<u32, _ST1RPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST1RPQ;
#[doc = "`read()` method returns [st1rpq::R](st1rpq::R) reader structure"]
impl crate::Readable for ST1RPQ {}
#[doc = "`write(|w| ..)` method takes [st1rpq::W](st1rpq::W) writer structure"]
impl crate::Writable for ST1RPQ {}
#[doc = "Screening Type 1 Register Priority Queue (index = 0) 0"]
pub mod st1rpq;
#[doc = "Screening Type 2 Register Priority Queue (index = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2rpq](st2rpq) module"]
pub type ST2RPQ = crate::Reg<u32, _ST2RPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2RPQ;
#[doc = "`read()` method returns [st2rpq::R](st2rpq::R) reader structure"]
impl crate::Readable for ST2RPQ {}
#[doc = "`write(|w| ..)` method takes [st2rpq::W](st2rpq::W) writer structure"]
impl crate::Writable for ST2RPQ {}
#[doc = "Screening Type 2 Register Priority Queue (index = 0) 0"]
pub mod st2rpq;
#[doc = "Interrupt Enable Register Priority Queue (index = 1) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ierpq](ierpq) module"]
pub type IERPQ = crate::Reg<u32, _IERPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IERPQ;
#[doc = "`write(|w| ..)` method takes [ierpq::W](ierpq::W) writer structure"]
impl crate::Writable for IERPQ {}
#[doc = "Interrupt Enable Register Priority Queue (index = 1) 0"]
pub mod ierpq;
#[doc = "Interrupt Disable Register Priority Queue (index = 1) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idrpq](idrpq) module"]
pub type IDRPQ = crate::Reg<u32, _IDRPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDRPQ;
#[doc = "`write(|w| ..)` method takes [idrpq::W](idrpq::W) writer structure"]
impl crate::Writable for IDRPQ {}
#[doc = "Interrupt Disable Register Priority Queue (index = 1) 0"]
pub mod idrpq;
#[doc = "Interrupt Mask Register Priority Queue (index = 1) 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imrpq](imrpq) module"]
pub type IMRPQ = crate::Reg<u32, _IMRPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMRPQ;
#[doc = "`read()` method returns [imrpq::R](imrpq::R) reader structure"]
impl crate::Readable for IMRPQ {}
#[doc = "`write(|w| ..)` method takes [imrpq::W](imrpq::W) writer structure"]
impl crate::Writable for IMRPQ {}
#[doc = "Interrupt Mask Register Priority Queue (index = 1) 0"]
pub mod imrpq;
#[doc = "Screening Type 2 Ethertype Register (index = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2er](st2er) module"]
pub type ST2ER = crate::Reg<u32, _ST2ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2ER;
#[doc = "`read()` method returns [st2er::R](st2er::R) reader structure"]
impl crate::Readable for ST2ER {}
#[doc = "`write(|w| ..)` method takes [st2er::W](st2er::W) writer structure"]
impl crate::Writable for ST2ER {}
#[doc = "Screening Type 2 Ethertype Register (index = 0) 0"]
pub mod st2er;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw00](st2cw00) module"]
pub type ST2CW00 = crate::Reg<u32, _ST2CW00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW00;
#[doc = "`read()` method returns [st2cw00::R](st2cw00::R) reader structure"]
impl crate::Readable for ST2CW00 {}
#[doc = "`write(|w| ..)` method takes [st2cw00::W](st2cw00::W) writer structure"]
impl crate::Writable for ST2CW00 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 0)"]
pub mod st2cw00;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw10](st2cw10) module"]
pub type ST2CW10 = crate::Reg<u32, _ST2CW10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW10;
#[doc = "`read()` method returns [st2cw10::R](st2cw10::R) reader structure"]
impl crate::Readable for ST2CW10 {}
#[doc = "`write(|w| ..)` method takes [st2cw10::W](st2cw10::W) writer structure"]
impl crate::Writable for ST2CW10 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 0)"]
pub mod st2cw10;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw01](st2cw01) module"]
pub type ST2CW01 = crate::Reg<u32, _ST2CW01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW01;
#[doc = "`read()` method returns [st2cw01::R](st2cw01::R) reader structure"]
impl crate::Readable for ST2CW01 {}
#[doc = "`write(|w| ..)` method takes [st2cw01::W](st2cw01::W) writer structure"]
impl crate::Writable for ST2CW01 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 1)"]
pub mod st2cw01;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw11](st2cw11) module"]
pub type ST2CW11 = crate::Reg<u32, _ST2CW11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW11;
#[doc = "`read()` method returns [st2cw11::R](st2cw11::R) reader structure"]
impl crate::Readable for ST2CW11 {}
#[doc = "`write(|w| ..)` method takes [st2cw11::W](st2cw11::W) writer structure"]
impl crate::Writable for ST2CW11 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 1)"]
pub mod st2cw11;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw02](st2cw02) module"]
pub type ST2CW02 = crate::Reg<u32, _ST2CW02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW02;
#[doc = "`read()` method returns [st2cw02::R](st2cw02::R) reader structure"]
impl crate::Readable for ST2CW02 {}
#[doc = "`write(|w| ..)` method takes [st2cw02::W](st2cw02::W) writer structure"]
impl crate::Writable for ST2CW02 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 2)"]
pub mod st2cw02;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw12](st2cw12) module"]
pub type ST2CW12 = crate::Reg<u32, _ST2CW12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW12;
#[doc = "`read()` method returns [st2cw12::R](st2cw12::R) reader structure"]
impl crate::Readable for ST2CW12 {}
#[doc = "`write(|w| ..)` method takes [st2cw12::W](st2cw12::W) writer structure"]
impl crate::Writable for ST2CW12 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 2)"]
pub mod st2cw12;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw03](st2cw03) module"]
pub type ST2CW03 = crate::Reg<u32, _ST2CW03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW03;
#[doc = "`read()` method returns [st2cw03::R](st2cw03::R) reader structure"]
impl crate::Readable for ST2CW03 {}
#[doc = "`write(|w| ..)` method takes [st2cw03::W](st2cw03::W) writer structure"]
impl crate::Writable for ST2CW03 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 3)"]
pub mod st2cw03;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw13](st2cw13) module"]
pub type ST2CW13 = crate::Reg<u32, _ST2CW13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW13;
#[doc = "`read()` method returns [st2cw13::R](st2cw13::R) reader structure"]
impl crate::Readable for ST2CW13 {}
#[doc = "`write(|w| ..)` method takes [st2cw13::W](st2cw13::W) writer structure"]
impl crate::Writable for ST2CW13 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 3)"]
pub mod st2cw13;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw04](st2cw04) module"]
pub type ST2CW04 = crate::Reg<u32, _ST2CW04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW04;
#[doc = "`read()` method returns [st2cw04::R](st2cw04::R) reader structure"]
impl crate::Readable for ST2CW04 {}
#[doc = "`write(|w| ..)` method takes [st2cw04::W](st2cw04::W) writer structure"]
impl crate::Writable for ST2CW04 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 4)"]
pub mod st2cw04;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw14](st2cw14) module"]
pub type ST2CW14 = crate::Reg<u32, _ST2CW14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW14;
#[doc = "`read()` method returns [st2cw14::R](st2cw14::R) reader structure"]
impl crate::Readable for ST2CW14 {}
#[doc = "`write(|w| ..)` method takes [st2cw14::W](st2cw14::W) writer structure"]
impl crate::Writable for ST2CW14 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 4)"]
pub mod st2cw14;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw05](st2cw05) module"]
pub type ST2CW05 = crate::Reg<u32, _ST2CW05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW05;
#[doc = "`read()` method returns [st2cw05::R](st2cw05::R) reader structure"]
impl crate::Readable for ST2CW05 {}
#[doc = "`write(|w| ..)` method takes [st2cw05::W](st2cw05::W) writer structure"]
impl crate::Writable for ST2CW05 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 5)"]
pub mod st2cw05;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw15](st2cw15) module"]
pub type ST2CW15 = crate::Reg<u32, _ST2CW15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW15;
#[doc = "`read()` method returns [st2cw15::R](st2cw15::R) reader structure"]
impl crate::Readable for ST2CW15 {}
#[doc = "`write(|w| ..)` method takes [st2cw15::W](st2cw15::W) writer structure"]
impl crate::Writable for ST2CW15 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 5)"]
pub mod st2cw15;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw06](st2cw06) module"]
pub type ST2CW06 = crate::Reg<u32, _ST2CW06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW06;
#[doc = "`read()` method returns [st2cw06::R](st2cw06::R) reader structure"]
impl crate::Readable for ST2CW06 {}
#[doc = "`write(|w| ..)` method takes [st2cw06::W](st2cw06::W) writer structure"]
impl crate::Writable for ST2CW06 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 6)"]
pub mod st2cw06;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw16](st2cw16) module"]
pub type ST2CW16 = crate::Reg<u32, _ST2CW16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW16;
#[doc = "`read()` method returns [st2cw16::R](st2cw16::R) reader structure"]
impl crate::Readable for ST2CW16 {}
#[doc = "`write(|w| ..)` method takes [st2cw16::W](st2cw16::W) writer structure"]
impl crate::Writable for ST2CW16 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 6)"]
pub mod st2cw16;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw07](st2cw07) module"]
pub type ST2CW07 = crate::Reg<u32, _ST2CW07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW07;
#[doc = "`read()` method returns [st2cw07::R](st2cw07::R) reader structure"]
impl crate::Readable for ST2CW07 {}
#[doc = "`write(|w| ..)` method takes [st2cw07::W](st2cw07::W) writer structure"]
impl crate::Writable for ST2CW07 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 7)"]
pub mod st2cw07;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw17](st2cw17) module"]
pub type ST2CW17 = crate::Reg<u32, _ST2CW17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW17;
#[doc = "`read()` method returns [st2cw17::R](st2cw17::R) reader structure"]
impl crate::Readable for ST2CW17 {}
#[doc = "`write(|w| ..)` method takes [st2cw17::W](st2cw17::W) writer structure"]
impl crate::Writable for ST2CW17 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 7)"]
pub mod st2cw17;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 8)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw08](st2cw08) module"]
pub type ST2CW08 = crate::Reg<u32, _ST2CW08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW08;
#[doc = "`read()` method returns [st2cw08::R](st2cw08::R) reader structure"]
impl crate::Readable for ST2CW08 {}
#[doc = "`write(|w| ..)` method takes [st2cw08::W](st2cw08::W) writer structure"]
impl crate::Writable for ST2CW08 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 8)"]
pub mod st2cw08;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 8)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw18](st2cw18) module"]
pub type ST2CW18 = crate::Reg<u32, _ST2CW18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW18;
#[doc = "`read()` method returns [st2cw18::R](st2cw18::R) reader structure"]
impl crate::Readable for ST2CW18 {}
#[doc = "`write(|w| ..)` method takes [st2cw18::W](st2cw18::W) writer structure"]
impl crate::Writable for ST2CW18 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 8)"]
pub mod st2cw18;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 9)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw09](st2cw09) module"]
pub type ST2CW09 = crate::Reg<u32, _ST2CW09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW09;
#[doc = "`read()` method returns [st2cw09::R](st2cw09::R) reader structure"]
impl crate::Readable for ST2CW09 {}
#[doc = "`write(|w| ..)` method takes [st2cw09::W](st2cw09::W) writer structure"]
impl crate::Writable for ST2CW09 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 9)"]
pub mod st2cw09;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 9)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw19](st2cw19) module"]
pub type ST2CW19 = crate::Reg<u32, _ST2CW19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW19;
#[doc = "`read()` method returns [st2cw19::R](st2cw19::R) reader structure"]
impl crate::Readable for ST2CW19 {}
#[doc = "`write(|w| ..)` method takes [st2cw19::W](st2cw19::W) writer structure"]
impl crate::Writable for ST2CW19 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 9)"]
pub mod st2cw19;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 10)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw010](st2cw010) module"]
pub type ST2CW010 = crate::Reg<u32, _ST2CW010>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW010;
#[doc = "`read()` method returns [st2cw010::R](st2cw010::R) reader structure"]
impl crate::Readable for ST2CW010 {}
#[doc = "`write(|w| ..)` method takes [st2cw010::W](st2cw010::W) writer structure"]
impl crate::Writable for ST2CW010 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 10)"]
pub mod st2cw010;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 10)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw110](st2cw110) module"]
pub type ST2CW110 = crate::Reg<u32, _ST2CW110>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW110;
#[doc = "`read()` method returns [st2cw110::R](st2cw110::R) reader structure"]
impl crate::Readable for ST2CW110 {}
#[doc = "`write(|w| ..)` method takes [st2cw110::W](st2cw110::W) writer structure"]
impl crate::Writable for ST2CW110 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 10)"]
pub mod st2cw110;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 11)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw011](st2cw011) module"]
pub type ST2CW011 = crate::Reg<u32, _ST2CW011>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW011;
#[doc = "`read()` method returns [st2cw011::R](st2cw011::R) reader structure"]
impl crate::Readable for ST2CW011 {}
#[doc = "`write(|w| ..)` method takes [st2cw011::W](st2cw011::W) writer structure"]
impl crate::Writable for ST2CW011 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 11)"]
pub mod st2cw011;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 11)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw111](st2cw111) module"]
pub type ST2CW111 = crate::Reg<u32, _ST2CW111>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW111;
#[doc = "`read()` method returns [st2cw111::R](st2cw111::R) reader structure"]
impl crate::Readable for ST2CW111 {}
#[doc = "`write(|w| ..)` method takes [st2cw111::W](st2cw111::W) writer structure"]
impl crate::Writable for ST2CW111 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 11)"]
pub mod st2cw111;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 12)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw012](st2cw012) module"]
pub type ST2CW012 = crate::Reg<u32, _ST2CW012>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW012;
#[doc = "`read()` method returns [st2cw012::R](st2cw012::R) reader structure"]
impl crate::Readable for ST2CW012 {}
#[doc = "`write(|w| ..)` method takes [st2cw012::W](st2cw012::W) writer structure"]
impl crate::Writable for ST2CW012 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 12)"]
pub mod st2cw012;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 12)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw112](st2cw112) module"]
pub type ST2CW112 = crate::Reg<u32, _ST2CW112>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW112;
#[doc = "`read()` method returns [st2cw112::R](st2cw112::R) reader structure"]
impl crate::Readable for ST2CW112 {}
#[doc = "`write(|w| ..)` method takes [st2cw112::W](st2cw112::W) writer structure"]
impl crate::Writable for ST2CW112 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 12)"]
pub mod st2cw112;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 13)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw013](st2cw013) module"]
pub type ST2CW013 = crate::Reg<u32, _ST2CW013>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW013;
#[doc = "`read()` method returns [st2cw013::R](st2cw013::R) reader structure"]
impl crate::Readable for ST2CW013 {}
#[doc = "`write(|w| ..)` method takes [st2cw013::W](st2cw013::W) writer structure"]
impl crate::Writable for ST2CW013 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 13)"]
pub mod st2cw013;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 13)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw113](st2cw113) module"]
pub type ST2CW113 = crate::Reg<u32, _ST2CW113>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW113;
#[doc = "`read()` method returns [st2cw113::R](st2cw113::R) reader structure"]
impl crate::Readable for ST2CW113 {}
#[doc = "`write(|w| ..)` method takes [st2cw113::W](st2cw113::W) writer structure"]
impl crate::Writable for ST2CW113 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 13)"]
pub mod st2cw113;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 14)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw014](st2cw014) module"]
pub type ST2CW014 = crate::Reg<u32, _ST2CW014>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW014;
#[doc = "`read()` method returns [st2cw014::R](st2cw014::R) reader structure"]
impl crate::Readable for ST2CW014 {}
#[doc = "`write(|w| ..)` method takes [st2cw014::W](st2cw014::W) writer structure"]
impl crate::Writable for ST2CW014 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 14)"]
pub mod st2cw014;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 14)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw114](st2cw114) module"]
pub type ST2CW114 = crate::Reg<u32, _ST2CW114>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW114;
#[doc = "`read()` method returns [st2cw114::R](st2cw114::R) reader structure"]
impl crate::Readable for ST2CW114 {}
#[doc = "`write(|w| ..)` method takes [st2cw114::W](st2cw114::W) writer structure"]
impl crate::Writable for ST2CW114 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 14)"]
pub mod st2cw114;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 15)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw015](st2cw015) module"]
pub type ST2CW015 = crate::Reg<u32, _ST2CW015>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW015;
#[doc = "`read()` method returns [st2cw015::R](st2cw015::R) reader structure"]
impl crate::Readable for ST2CW015 {}
#[doc = "`write(|w| ..)` method takes [st2cw015::W](st2cw015::W) writer structure"]
impl crate::Writable for ST2CW015 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 15)"]
pub mod st2cw015;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 15)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw115](st2cw115) module"]
pub type ST2CW115 = crate::Reg<u32, _ST2CW115>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW115;
#[doc = "`read()` method returns [st2cw115::R](st2cw115::R) reader structure"]
impl crate::Readable for ST2CW115 {}
#[doc = "`write(|w| ..)` method takes [st2cw115::W](st2cw115::W) writer structure"]
impl crate::Writable for ST2CW115 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 15)"]
pub mod st2cw115;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 16)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw016](st2cw016) module"]
pub type ST2CW016 = crate::Reg<u32, _ST2CW016>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW016;
#[doc = "`read()` method returns [st2cw016::R](st2cw016::R) reader structure"]
impl crate::Readable for ST2CW016 {}
#[doc = "`write(|w| ..)` method takes [st2cw016::W](st2cw016::W) writer structure"]
impl crate::Writable for ST2CW016 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 16)"]
pub mod st2cw016;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 16)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw116](st2cw116) module"]
pub type ST2CW116 = crate::Reg<u32, _ST2CW116>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW116;
#[doc = "`read()` method returns [st2cw116::R](st2cw116::R) reader structure"]
impl crate::Readable for ST2CW116 {}
#[doc = "`write(|w| ..)` method takes [st2cw116::W](st2cw116::W) writer structure"]
impl crate::Writable for ST2CW116 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 16)"]
pub mod st2cw116;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 17)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw017](st2cw017) module"]
pub type ST2CW017 = crate::Reg<u32, _ST2CW017>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW017;
#[doc = "`read()` method returns [st2cw017::R](st2cw017::R) reader structure"]
impl crate::Readable for ST2CW017 {}
#[doc = "`write(|w| ..)` method takes [st2cw017::W](st2cw017::W) writer structure"]
impl crate::Writable for ST2CW017 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 17)"]
pub mod st2cw017;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 17)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw117](st2cw117) module"]
pub type ST2CW117 = crate::Reg<u32, _ST2CW117>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW117;
#[doc = "`read()` method returns [st2cw117::R](st2cw117::R) reader structure"]
impl crate::Readable for ST2CW117 {}
#[doc = "`write(|w| ..)` method takes [st2cw117::W](st2cw117::W) writer structure"]
impl crate::Writable for ST2CW117 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 17)"]
pub mod st2cw117;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 18)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw018](st2cw018) module"]
pub type ST2CW018 = crate::Reg<u32, _ST2CW018>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW018;
#[doc = "`read()` method returns [st2cw018::R](st2cw018::R) reader structure"]
impl crate::Readable for ST2CW018 {}
#[doc = "`write(|w| ..)` method takes [st2cw018::W](st2cw018::W) writer structure"]
impl crate::Writable for ST2CW018 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 18)"]
pub mod st2cw018;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 18)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw118](st2cw118) module"]
pub type ST2CW118 = crate::Reg<u32, _ST2CW118>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW118;
#[doc = "`read()` method returns [st2cw118::R](st2cw118::R) reader structure"]
impl crate::Readable for ST2CW118 {}
#[doc = "`write(|w| ..)` method takes [st2cw118::W](st2cw118::W) writer structure"]
impl crate::Writable for ST2CW118 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 18)"]
pub mod st2cw118;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 19)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw019](st2cw019) module"]
pub type ST2CW019 = crate::Reg<u32, _ST2CW019>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW019;
#[doc = "`read()` method returns [st2cw019::R](st2cw019::R) reader structure"]
impl crate::Readable for ST2CW019 {}
#[doc = "`write(|w| ..)` method takes [st2cw019::W](st2cw019::W) writer structure"]
impl crate::Writable for ST2CW019 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 19)"]
pub mod st2cw019;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 19)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw119](st2cw119) module"]
pub type ST2CW119 = crate::Reg<u32, _ST2CW119>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW119;
#[doc = "`read()` method returns [st2cw119::R](st2cw119::R) reader structure"]
impl crate::Readable for ST2CW119 {}
#[doc = "`write(|w| ..)` method takes [st2cw119::W](st2cw119::W) writer structure"]
impl crate::Writable for ST2CW119 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 19)"]
pub mod st2cw119;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 20)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw020](st2cw020) module"]
pub type ST2CW020 = crate::Reg<u32, _ST2CW020>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW020;
#[doc = "`read()` method returns [st2cw020::R](st2cw020::R) reader structure"]
impl crate::Readable for ST2CW020 {}
#[doc = "`write(|w| ..)` method takes [st2cw020::W](st2cw020::W) writer structure"]
impl crate::Writable for ST2CW020 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 20)"]
pub mod st2cw020;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 20)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw120](st2cw120) module"]
pub type ST2CW120 = crate::Reg<u32, _ST2CW120>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW120;
#[doc = "`read()` method returns [st2cw120::R](st2cw120::R) reader structure"]
impl crate::Readable for ST2CW120 {}
#[doc = "`write(|w| ..)` method takes [st2cw120::W](st2cw120::W) writer structure"]
impl crate::Writable for ST2CW120 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 20)"]
pub mod st2cw120;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 21)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw021](st2cw021) module"]
pub type ST2CW021 = crate::Reg<u32, _ST2CW021>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW021;
#[doc = "`read()` method returns [st2cw021::R](st2cw021::R) reader structure"]
impl crate::Readable for ST2CW021 {}
#[doc = "`write(|w| ..)` method takes [st2cw021::W](st2cw021::W) writer structure"]
impl crate::Writable for ST2CW021 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 21)"]
pub mod st2cw021;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 21)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw121](st2cw121) module"]
pub type ST2CW121 = crate::Reg<u32, _ST2CW121>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW121;
#[doc = "`read()` method returns [st2cw121::R](st2cw121::R) reader structure"]
impl crate::Readable for ST2CW121 {}
#[doc = "`write(|w| ..)` method takes [st2cw121::W](st2cw121::W) writer structure"]
impl crate::Writable for ST2CW121 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 21)"]
pub mod st2cw121;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 22)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw022](st2cw022) module"]
pub type ST2CW022 = crate::Reg<u32, _ST2CW022>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW022;
#[doc = "`read()` method returns [st2cw022::R](st2cw022::R) reader structure"]
impl crate::Readable for ST2CW022 {}
#[doc = "`write(|w| ..)` method takes [st2cw022::W](st2cw022::W) writer structure"]
impl crate::Writable for ST2CW022 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 22)"]
pub mod st2cw022;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 22)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw122](st2cw122) module"]
pub type ST2CW122 = crate::Reg<u32, _ST2CW122>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW122;
#[doc = "`read()` method returns [st2cw122::R](st2cw122::R) reader structure"]
impl crate::Readable for ST2CW122 {}
#[doc = "`write(|w| ..)` method takes [st2cw122::W](st2cw122::W) writer structure"]
impl crate::Writable for ST2CW122 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 22)"]
pub mod st2cw122;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 23)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw023](st2cw023) module"]
pub type ST2CW023 = crate::Reg<u32, _ST2CW023>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW023;
#[doc = "`read()` method returns [st2cw023::R](st2cw023::R) reader structure"]
impl crate::Readable for ST2CW023 {}
#[doc = "`write(|w| ..)` method takes [st2cw023::W](st2cw023::W) writer structure"]
impl crate::Writable for ST2CW023 {}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 23)"]
pub mod st2cw023;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 23)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw123](st2cw123) module"]
pub type ST2CW123 = crate::Reg<u32, _ST2CW123>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2CW123;
#[doc = "`read()` method returns [st2cw123::R](st2cw123::R) reader structure"]
impl crate::Readable for ST2CW123 {}
#[doc = "`write(|w| ..)` method takes [st2cw123::W](st2cw123::W) writer structure"]
impl crate::Writable for ST2CW123 {}
#[doc = "Screening Type 2 Compare Word 1 Register (index = 23)"]
pub mod st2cw123;
