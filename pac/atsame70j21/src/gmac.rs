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
    _reserved19: [u8; 0x34],
    #[doc = "0x80 - Hash Register Bottom"]
    pub hrb: HRB,
    #[doc = "0x84 - Hash Register Top"]
    pub hrt: HRT,
    #[doc = "0x88..0xa8 - Specific Address 1 Bottom Register"]
    pub gmac_sa: [GMAC_SA; 4],
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
    _reserved32: [u8; 0x0c],
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
    _reserved39: [u8; 0x08],
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
    _reserved84: [u8; 0x08],
    #[doc = "0x1bc - 1588 Timer Increment Sub-nanoseconds Register"]
    pub tisubn: TISUBN,
    #[doc = "0x1c0 - 1588 Timer Seconds High Register"]
    pub tsh: TSH,
    _reserved86: [u8; 0x0c],
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
    _reserved98: [u8; 0x01fc],
    #[doc = "0x3fc..0x404 - Interrupt Status Register Priority Queue (index = 1) 0"]
    pub isrpq: [ISRPQ; 2],
    _reserved99: [u8; 0x38],
    #[doc = "0x43c..0x444 - Transmit Buffer Queue Base Address Register Priority Queue (index = 1) 0"]
    pub tbqbapq: [TBQBAPQ; 2],
    _reserved100: [u8; 0x38],
    #[doc = "0x47c..0x484 - Receive Buffer Queue Base Address Register Priority Queue (index = 1) 0"]
    pub rbqbapq: [RBQBAPQ; 2],
    _reserved101: [u8; 0x18],
    #[doc = "0x49c..0x4a4 - Receive Buffer Size Register Priority Queue (index = 1) 0"]
    pub rbsrpq: [RBSRPQ; 2],
    _reserved102: [u8; 0x18],
    #[doc = "0x4bc - Credit-Based Shaping Control Register"]
    pub cbscr: CBSCR,
    #[doc = "0x4c0 - Credit-Based Shaping IdleSlope Register for Queue A"]
    pub cbsisqa: CBSISQA,
    #[doc = "0x4c4 - Credit-Based Shaping IdleSlope Register for Queue B"]
    pub cbsisqb: CBSISQB,
    _reserved105: [u8; 0x38],
    #[doc = "0x500..0x510 - Screening Type 1 Register Priority Queue (index = 0) 0"]
    pub st1rpq: [ST1RPQ; 4],
    _reserved106: [u8; 0x30],
    #[doc = "0x540..0x560 - Screening Type 2 Register Priority Queue (index = 0) 0"]
    pub st2rpq: [ST2RPQ; 8],
    _reserved107: [u8; 0x9c],
    #[doc = "0x5fc..0x604 - Interrupt Enable Register Priority Queue (index = 1) 0"]
    pub ierpq: [IERPQ; 2],
    _reserved108: [u8; 0x18],
    #[doc = "0x61c..0x624 - Interrupt Disable Register Priority Queue (index = 1) 0"]
    pub idrpq: [IDRPQ; 2],
    _reserved109: [u8; 0x18],
    #[doc = "0x63c..0x644 - Interrupt Mask Register Priority Queue (index = 1) 0"]
    pub imrpq: [IMRPQ; 2],
    _reserved110: [u8; 0x9c],
    #[doc = "0x6e0..0x6f0 - Screening Type 2 Ethertype Register (index = 0) 0"]
    pub st2er: [ST2ER; 4],
    _reserved111: [u8; 0x10],
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
impl RegisterBlock {
    #[doc = "0x88..0x90 - Specific Address 1 Bottom Register"]
    #[inline(always)]
    pub fn gmac_sa1(&self) -> &GMAC_SA {
        &self.gmac_sa[0]
    }
    #[doc = "0x90..0x98 - Specific Address 1 Bottom Register"]
    #[inline(always)]
    pub fn gmac_sa2(&self) -> &GMAC_SA {
        &self.gmac_sa[1]
    }
    #[doc = "0x98..0xa0 - Specific Address 1 Bottom Register"]
    #[inline(always)]
    pub fn gmac_sa3(&self) -> &GMAC_SA {
        &self.gmac_sa[2]
    }
    #[doc = "0xa0..0xa8 - Specific Address 1 Bottom Register"]
    #[inline(always)]
    pub fn gmac_sa4(&self) -> &GMAC_SA {
        &self.gmac_sa[3]
    }
}
#[doc = "NCR (rw) register accessor: an alias for `Reg<NCR_SPEC>`"]
pub type NCR = crate::Reg<ncr::NCR_SPEC>;
#[doc = "Network Control Register"]
pub mod ncr;
#[doc = "NCFGR (rw) register accessor: an alias for `Reg<NCFGR_SPEC>`"]
pub type NCFGR = crate::Reg<ncfgr::NCFGR_SPEC>;
#[doc = "Network Configuration Register"]
pub mod ncfgr;
#[doc = "NSR (r) register accessor: an alias for `Reg<NSR_SPEC>`"]
pub type NSR = crate::Reg<nsr::NSR_SPEC>;
#[doc = "Network Status Register"]
pub mod nsr;
#[doc = "UR (rw) register accessor: an alias for `Reg<UR_SPEC>`"]
pub type UR = crate::Reg<ur::UR_SPEC>;
#[doc = "User Register"]
pub mod ur;
#[doc = "DCFGR (rw) register accessor: an alias for `Reg<DCFGR_SPEC>`"]
pub type DCFGR = crate::Reg<dcfgr::DCFGR_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod dcfgr;
#[doc = "TSR (rw) register accessor: an alias for `Reg<TSR_SPEC>`"]
pub type TSR = crate::Reg<tsr::TSR_SPEC>;
#[doc = "Transmit Status Register"]
pub mod tsr;
#[doc = "RBQB (rw) register accessor: an alias for `Reg<RBQB_SPEC>`"]
pub type RBQB = crate::Reg<rbqb::RBQB_SPEC>;
#[doc = "Receive Buffer Queue Base Address Register"]
pub mod rbqb;
#[doc = "TBQB (rw) register accessor: an alias for `Reg<TBQB_SPEC>`"]
pub type TBQB = crate::Reg<tbqb::TBQB_SPEC>;
#[doc = "Transmit Buffer Queue Base Address Register"]
pub mod tbqb;
#[doc = "RSR (rw) register accessor: an alias for `Reg<RSR_SPEC>`"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "MAN (rw) register accessor: an alias for `Reg<MAN_SPEC>`"]
pub type MAN = crate::Reg<man::MAN_SPEC>;
#[doc = "PHY Maintenance Register"]
pub mod man;
#[doc = "RPQ (r) register accessor: an alias for `Reg<RPQ_SPEC>`"]
pub type RPQ = crate::Reg<rpq::RPQ_SPEC>;
#[doc = "Received Pause Quantum Register"]
pub mod rpq;
#[doc = "TPQ (rw) register accessor: an alias for `Reg<TPQ_SPEC>`"]
pub type TPQ = crate::Reg<tpq::TPQ_SPEC>;
#[doc = "Transmit Pause Quantum Register"]
pub mod tpq;
#[doc = "TPSF (rw) register accessor: an alias for `Reg<TPSF_SPEC>`"]
pub type TPSF = crate::Reg<tpsf::TPSF_SPEC>;
#[doc = "TX Partial Store and Forward Register"]
pub mod tpsf;
#[doc = "RPSF (rw) register accessor: an alias for `Reg<RPSF_SPEC>`"]
pub type RPSF = crate::Reg<rpsf::RPSF_SPEC>;
#[doc = "RX Partial Store and Forward Register"]
pub mod rpsf;
#[doc = "RJFML (rw) register accessor: an alias for `Reg<RJFML_SPEC>`"]
pub type RJFML = crate::Reg<rjfml::RJFML_SPEC>;
#[doc = "RX Jumbo Frame Max Length Register"]
pub mod rjfml;
#[doc = "HRB (rw) register accessor: an alias for `Reg<HRB_SPEC>`"]
pub type HRB = crate::Reg<hrb::HRB_SPEC>;
#[doc = "Hash Register Bottom"]
pub mod hrb;
#[doc = "HRT (rw) register accessor: an alias for `Reg<HRT_SPEC>`"]
pub type HRT = crate::Reg<hrt::HRT_SPEC>;
#[doc = "Hash Register Top"]
pub mod hrt;
#[doc = "Specific Address 1 Bottom Register"]
pub use self::gmac_sa::GMAC_SA;
#[doc = r"Cluster"]
#[doc = "Specific Address 1 Bottom Register"]
pub mod gmac_sa;
#[doc = "TIDM1 (rw) register accessor: an alias for `Reg<TIDM1_SPEC>`"]
pub type TIDM1 = crate::Reg<tidm1::TIDM1_SPEC>;
#[doc = "Type ID Match 1 Register"]
pub mod tidm1;
#[doc = "TIDM2 (rw) register accessor: an alias for `Reg<TIDM2_SPEC>`"]
pub type TIDM2 = crate::Reg<tidm2::TIDM2_SPEC>;
#[doc = "Type ID Match 2 Register"]
pub mod tidm2;
#[doc = "TIDM3 (rw) register accessor: an alias for `Reg<TIDM3_SPEC>`"]
pub type TIDM3 = crate::Reg<tidm3::TIDM3_SPEC>;
#[doc = "Type ID Match 3 Register"]
pub mod tidm3;
#[doc = "TIDM4 (rw) register accessor: an alias for `Reg<TIDM4_SPEC>`"]
pub type TIDM4 = crate::Reg<tidm4::TIDM4_SPEC>;
#[doc = "Type ID Match 4 Register"]
pub mod tidm4;
#[doc = "WOL (rw) register accessor: an alias for `Reg<WOL_SPEC>`"]
pub type WOL = crate::Reg<wol::WOL_SPEC>;
#[doc = "Wake on LAN Register"]
pub mod wol;
#[doc = "IPGS (rw) register accessor: an alias for `Reg<IPGS_SPEC>`"]
pub type IPGS = crate::Reg<ipgs::IPGS_SPEC>;
#[doc = "IPG Stretch Register"]
pub mod ipgs;
#[doc = "SVLAN (rw) register accessor: an alias for `Reg<SVLAN_SPEC>`"]
pub type SVLAN = crate::Reg<svlan::SVLAN_SPEC>;
#[doc = "Stacked VLAN Register"]
pub mod svlan;
#[doc = "TPFCP (rw) register accessor: an alias for `Reg<TPFCP_SPEC>`"]
pub type TPFCP = crate::Reg<tpfcp::TPFCP_SPEC>;
#[doc = "Transmit PFC Pause Register"]
pub mod tpfcp;
#[doc = "SAMB1 (rw) register accessor: an alias for `Reg<SAMB1_SPEC>`"]
pub type SAMB1 = crate::Reg<samb1::SAMB1_SPEC>;
#[doc = "Specific Address 1 Mask Bottom Register"]
pub mod samb1;
#[doc = "SAMT1 (rw) register accessor: an alias for `Reg<SAMT1_SPEC>`"]
pub type SAMT1 = crate::Reg<samt1::SAMT1_SPEC>;
#[doc = "Specific Address 1 Mask Top Register"]
pub mod samt1;
#[doc = "NSC (rw) register accessor: an alias for `Reg<NSC_SPEC>`"]
pub type NSC = crate::Reg<nsc::NSC_SPEC>;
#[doc = "1588 Timer Nanosecond Comparison Register"]
pub mod nsc;
#[doc = "SCL (rw) register accessor: an alias for `Reg<SCL_SPEC>`"]
pub type SCL = crate::Reg<scl::SCL_SPEC>;
#[doc = "1588 Timer Second Comparison Low Register"]
pub mod scl;
#[doc = "SCH (rw) register accessor: an alias for `Reg<SCH_SPEC>`"]
pub type SCH = crate::Reg<sch::SCH_SPEC>;
#[doc = "1588 Timer Second Comparison High Register"]
pub mod sch;
#[doc = "EFTSH (r) register accessor: an alias for `Reg<EFTSH_SPEC>`"]
pub type EFTSH = crate::Reg<eftsh::EFTSH_SPEC>;
#[doc = "PTP Event Frame Transmitted Seconds High Register"]
pub mod eftsh;
#[doc = "EFRSH (r) register accessor: an alias for `Reg<EFRSH_SPEC>`"]
pub type EFRSH = crate::Reg<efrsh::EFRSH_SPEC>;
#[doc = "PTP Event Frame Received Seconds High Register"]
pub mod efrsh;
#[doc = "PEFTSH (r) register accessor: an alias for `Reg<PEFTSH_SPEC>`"]
pub type PEFTSH = crate::Reg<peftsh::PEFTSH_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Seconds High Register"]
pub mod peftsh;
#[doc = "PEFRSH (r) register accessor: an alias for `Reg<PEFRSH_SPEC>`"]
pub type PEFRSH = crate::Reg<pefrsh::PEFRSH_SPEC>;
#[doc = "PTP Peer Event Frame Received Seconds High Register"]
pub mod pefrsh;
#[doc = "OTLO (r) register accessor: an alias for `Reg<OTLO_SPEC>`"]
pub type OTLO = crate::Reg<otlo::OTLO_SPEC>;
#[doc = "Octets Transmitted Low Register"]
pub mod otlo;
#[doc = "OTHI (r) register accessor: an alias for `Reg<OTHI_SPEC>`"]
pub type OTHI = crate::Reg<othi::OTHI_SPEC>;
#[doc = "Octets Transmitted High Register"]
pub mod othi;
#[doc = "FT (r) register accessor: an alias for `Reg<FT_SPEC>`"]
pub type FT = crate::Reg<ft::FT_SPEC>;
#[doc = "Frames Transmitted Register"]
pub mod ft;
#[doc = "BCFT (r) register accessor: an alias for `Reg<BCFT_SPEC>`"]
pub type BCFT = crate::Reg<bcft::BCFT_SPEC>;
#[doc = "Broadcast Frames Transmitted Register"]
pub mod bcft;
#[doc = "MFT (r) register accessor: an alias for `Reg<MFT_SPEC>`"]
pub type MFT = crate::Reg<mft::MFT_SPEC>;
#[doc = "Multicast Frames Transmitted Register"]
pub mod mft;
#[doc = "PFT (r) register accessor: an alias for `Reg<PFT_SPEC>`"]
pub type PFT = crate::Reg<pft::PFT_SPEC>;
#[doc = "Pause Frames Transmitted Register"]
pub mod pft;
#[doc = "BFT64 (r) register accessor: an alias for `Reg<BFT64_SPEC>`"]
pub type BFT64 = crate::Reg<bft64::BFT64_SPEC>;
#[doc = "64 Byte Frames Transmitted Register"]
pub mod bft64;
#[doc = "TBFT127 (r) register accessor: an alias for `Reg<TBFT127_SPEC>`"]
pub type TBFT127 = crate::Reg<tbft127::TBFT127_SPEC>;
#[doc = "65 to 127 Byte Frames Transmitted Register"]
pub mod tbft127;
#[doc = "TBFT255 (r) register accessor: an alias for `Reg<TBFT255_SPEC>`"]
pub type TBFT255 = crate::Reg<tbft255::TBFT255_SPEC>;
#[doc = "128 to 255 Byte Frames Transmitted Register"]
pub mod tbft255;
#[doc = "TBFT511 (r) register accessor: an alias for `Reg<TBFT511_SPEC>`"]
pub type TBFT511 = crate::Reg<tbft511::TBFT511_SPEC>;
#[doc = "256 to 511 Byte Frames Transmitted Register"]
pub mod tbft511;
#[doc = "TBFT1023 (r) register accessor: an alias for `Reg<TBFT1023_SPEC>`"]
pub type TBFT1023 = crate::Reg<tbft1023::TBFT1023_SPEC>;
#[doc = "512 to 1023 Byte Frames Transmitted Register"]
pub mod tbft1023;
#[doc = "TBFT1518 (r) register accessor: an alias for `Reg<TBFT1518_SPEC>`"]
pub type TBFT1518 = crate::Reg<tbft1518::TBFT1518_SPEC>;
#[doc = "1024 to 1518 Byte Frames Transmitted Register"]
pub mod tbft1518;
#[doc = "GTBFT1518 (r) register accessor: an alias for `Reg<GTBFT1518_SPEC>`"]
pub type GTBFT1518 = crate::Reg<gtbft1518::GTBFT1518_SPEC>;
#[doc = "Greater Than 1518 Byte Frames Transmitted Register"]
pub mod gtbft1518;
#[doc = "TUR (r) register accessor: an alias for `Reg<TUR_SPEC>`"]
pub type TUR = crate::Reg<tur::TUR_SPEC>;
#[doc = "Transmit Underruns Register"]
pub mod tur;
#[doc = "SCF (r) register accessor: an alias for `Reg<SCF_SPEC>`"]
pub type SCF = crate::Reg<scf::SCF_SPEC>;
#[doc = "Single Collision Frames Register"]
pub mod scf;
#[doc = "MCF (r) register accessor: an alias for `Reg<MCF_SPEC>`"]
pub type MCF = crate::Reg<mcf::MCF_SPEC>;
#[doc = "Multiple Collision Frames Register"]
pub mod mcf;
#[doc = "EC (r) register accessor: an alias for `Reg<EC_SPEC>`"]
pub type EC = crate::Reg<ec::EC_SPEC>;
#[doc = "Excessive Collisions Register"]
pub mod ec;
#[doc = "LC (r) register accessor: an alias for `Reg<LC_SPEC>`"]
pub type LC = crate::Reg<lc::LC_SPEC>;
#[doc = "Late Collisions Register"]
pub mod lc;
#[doc = "DTF (r) register accessor: an alias for `Reg<DTF_SPEC>`"]
pub type DTF = crate::Reg<dtf::DTF_SPEC>;
#[doc = "Deferred Transmission Frames Register"]
pub mod dtf;
#[doc = "CSE (r) register accessor: an alias for `Reg<CSE_SPEC>`"]
pub type CSE = crate::Reg<cse::CSE_SPEC>;
#[doc = "Carrier Sense Errors Register"]
pub mod cse;
#[doc = "ORLO (r) register accessor: an alias for `Reg<ORLO_SPEC>`"]
pub type ORLO = crate::Reg<orlo::ORLO_SPEC>;
#[doc = "Octets Received Low Received Register"]
pub mod orlo;
#[doc = "ORHI (r) register accessor: an alias for `Reg<ORHI_SPEC>`"]
pub type ORHI = crate::Reg<orhi::ORHI_SPEC>;
#[doc = "Octets Received High Received Register"]
pub mod orhi;
#[doc = "FR (r) register accessor: an alias for `Reg<FR_SPEC>`"]
pub type FR = crate::Reg<fr::FR_SPEC>;
#[doc = "Frames Received Register"]
pub mod fr;
#[doc = "BCFR (r) register accessor: an alias for `Reg<BCFR_SPEC>`"]
pub type BCFR = crate::Reg<bcfr::BCFR_SPEC>;
#[doc = "Broadcast Frames Received Register"]
pub mod bcfr;
#[doc = "MFR (r) register accessor: an alias for `Reg<MFR_SPEC>`"]
pub type MFR = crate::Reg<mfr::MFR_SPEC>;
#[doc = "Multicast Frames Received Register"]
pub mod mfr;
#[doc = "PFR (r) register accessor: an alias for `Reg<PFR_SPEC>`"]
pub type PFR = crate::Reg<pfr::PFR_SPEC>;
#[doc = "Pause Frames Received Register"]
pub mod pfr;
#[doc = "BFR64 (r) register accessor: an alias for `Reg<BFR64_SPEC>`"]
pub type BFR64 = crate::Reg<bfr64::BFR64_SPEC>;
#[doc = "64 Byte Frames Received Register"]
pub mod bfr64;
#[doc = "TBFR127 (r) register accessor: an alias for `Reg<TBFR127_SPEC>`"]
pub type TBFR127 = crate::Reg<tbfr127::TBFR127_SPEC>;
#[doc = "65 to 127 Byte Frames Received Register"]
pub mod tbfr127;
#[doc = "TBFR255 (r) register accessor: an alias for `Reg<TBFR255_SPEC>`"]
pub type TBFR255 = crate::Reg<tbfr255::TBFR255_SPEC>;
#[doc = "128 to 255 Byte Frames Received Register"]
pub mod tbfr255;
#[doc = "TBFR511 (r) register accessor: an alias for `Reg<TBFR511_SPEC>`"]
pub type TBFR511 = crate::Reg<tbfr511::TBFR511_SPEC>;
#[doc = "256 to 511 Byte Frames Received Register"]
pub mod tbfr511;
#[doc = "TBFR1023 (r) register accessor: an alias for `Reg<TBFR1023_SPEC>`"]
pub type TBFR1023 = crate::Reg<tbfr1023::TBFR1023_SPEC>;
#[doc = "512 to 1023 Byte Frames Received Register"]
pub mod tbfr1023;
#[doc = "TBFR1518 (r) register accessor: an alias for `Reg<TBFR1518_SPEC>`"]
pub type TBFR1518 = crate::Reg<tbfr1518::TBFR1518_SPEC>;
#[doc = "1024 to 1518 Byte Frames Received Register"]
pub mod tbfr1518;
#[doc = "TMXBFR (r) register accessor: an alias for `Reg<TMXBFR_SPEC>`"]
pub type TMXBFR = crate::Reg<tmxbfr::TMXBFR_SPEC>;
#[doc = "1519 to Maximum Byte Frames Received Register"]
pub mod tmxbfr;
#[doc = "UFR (r) register accessor: an alias for `Reg<UFR_SPEC>`"]
pub type UFR = crate::Reg<ufr::UFR_SPEC>;
#[doc = "Undersize Frames Received Register"]
pub mod ufr;
#[doc = "OFR (r) register accessor: an alias for `Reg<OFR_SPEC>`"]
pub type OFR = crate::Reg<ofr::OFR_SPEC>;
#[doc = "Oversize Frames Received Register"]
pub mod ofr;
#[doc = "JR (r) register accessor: an alias for `Reg<JR_SPEC>`"]
pub type JR = crate::Reg<jr::JR_SPEC>;
#[doc = "Jabbers Received Register"]
pub mod jr;
#[doc = "FCSE (r) register accessor: an alias for `Reg<FCSE_SPEC>`"]
pub type FCSE = crate::Reg<fcse::FCSE_SPEC>;
#[doc = "Frame Check Sequence Errors Register"]
pub mod fcse;
#[doc = "LFFE (r) register accessor: an alias for `Reg<LFFE_SPEC>`"]
pub type LFFE = crate::Reg<lffe::LFFE_SPEC>;
#[doc = "Length Field Frame Errors Register"]
pub mod lffe;
#[doc = "RSE (r) register accessor: an alias for `Reg<RSE_SPEC>`"]
pub type RSE = crate::Reg<rse::RSE_SPEC>;
#[doc = "Receive Symbol Errors Register"]
pub mod rse;
#[doc = "AE (r) register accessor: an alias for `Reg<AE_SPEC>`"]
pub type AE = crate::Reg<ae::AE_SPEC>;
#[doc = "Alignment Errors Register"]
pub mod ae;
#[doc = "RRE (r) register accessor: an alias for `Reg<RRE_SPEC>`"]
pub type RRE = crate::Reg<rre::RRE_SPEC>;
#[doc = "Receive Resource Errors Register"]
pub mod rre;
#[doc = "ROE (r) register accessor: an alias for `Reg<ROE_SPEC>`"]
pub type ROE = crate::Reg<roe::ROE_SPEC>;
#[doc = "Receive Overrun Register"]
pub mod roe;
#[doc = "IHCE (r) register accessor: an alias for `Reg<IHCE_SPEC>`"]
pub type IHCE = crate::Reg<ihce::IHCE_SPEC>;
#[doc = "IP Header Checksum Errors Register"]
pub mod ihce;
#[doc = "TCE (r) register accessor: an alias for `Reg<TCE_SPEC>`"]
pub type TCE = crate::Reg<tce::TCE_SPEC>;
#[doc = "TCP Checksum Errors Register"]
pub mod tce;
#[doc = "UCE (r) register accessor: an alias for `Reg<UCE_SPEC>`"]
pub type UCE = crate::Reg<uce::UCE_SPEC>;
#[doc = "UDP Checksum Errors Register"]
pub mod uce;
#[doc = "TISUBN (rw) register accessor: an alias for `Reg<TISUBN_SPEC>`"]
pub type TISUBN = crate::Reg<tisubn::TISUBN_SPEC>;
#[doc = "1588 Timer Increment Sub-nanoseconds Register"]
pub mod tisubn;
#[doc = "TSH (rw) register accessor: an alias for `Reg<TSH_SPEC>`"]
pub type TSH = crate::Reg<tsh::TSH_SPEC>;
#[doc = "1588 Timer Seconds High Register"]
pub mod tsh;
#[doc = "TSL (rw) register accessor: an alias for `Reg<TSL_SPEC>`"]
pub type TSL = crate::Reg<tsl::TSL_SPEC>;
#[doc = "1588 Timer Seconds Low Register"]
pub mod tsl;
#[doc = "TN (rw) register accessor: an alias for `Reg<TN_SPEC>`"]
pub type TN = crate::Reg<tn::TN_SPEC>;
#[doc = "1588 Timer Nanoseconds Register"]
pub mod tn;
#[doc = "TA (w) register accessor: an alias for `Reg<TA_SPEC>`"]
pub type TA = crate::Reg<ta::TA_SPEC>;
#[doc = "1588 Timer Adjust Register"]
pub mod ta;
#[doc = "TI (rw) register accessor: an alias for `Reg<TI_SPEC>`"]
pub type TI = crate::Reg<ti::TI_SPEC>;
#[doc = "1588 Timer Increment Register"]
pub mod ti;
#[doc = "EFTSL (r) register accessor: an alias for `Reg<EFTSL_SPEC>`"]
pub type EFTSL = crate::Reg<eftsl::EFTSL_SPEC>;
#[doc = "PTP Event Frame Transmitted Seconds Low Register"]
pub mod eftsl;
#[doc = "EFTN (r) register accessor: an alias for `Reg<EFTN_SPEC>`"]
pub type EFTN = crate::Reg<eftn::EFTN_SPEC>;
#[doc = "PTP Event Frame Transmitted Nanoseconds Register"]
pub mod eftn;
#[doc = "EFRSL (r) register accessor: an alias for `Reg<EFRSL_SPEC>`"]
pub type EFRSL = crate::Reg<efrsl::EFRSL_SPEC>;
#[doc = "PTP Event Frame Received Seconds Low Register"]
pub mod efrsl;
#[doc = "EFRN (r) register accessor: an alias for `Reg<EFRN_SPEC>`"]
pub type EFRN = crate::Reg<efrn::EFRN_SPEC>;
#[doc = "PTP Event Frame Received Nanoseconds Register"]
pub mod efrn;
#[doc = "PEFTSL (r) register accessor: an alias for `Reg<PEFTSL_SPEC>`"]
pub type PEFTSL = crate::Reg<peftsl::PEFTSL_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Seconds Low Register"]
pub mod peftsl;
#[doc = "PEFTN (r) register accessor: an alias for `Reg<PEFTN_SPEC>`"]
pub type PEFTN = crate::Reg<peftn::PEFTN_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register"]
pub mod peftn;
#[doc = "PEFRSL (r) register accessor: an alias for `Reg<PEFRSL_SPEC>`"]
pub type PEFRSL = crate::Reg<pefrsl::PEFRSL_SPEC>;
#[doc = "PTP Peer Event Frame Received Seconds Low Register"]
pub mod pefrsl;
#[doc = "PEFRN (r) register accessor: an alias for `Reg<PEFRN_SPEC>`"]
pub type PEFRN = crate::Reg<pefrn::PEFRN_SPEC>;
#[doc = "PTP Peer Event Frame Received Nanoseconds Register"]
pub mod pefrn;
#[doc = "ISRPQ (r) register accessor: an alias for `Reg<ISRPQ_SPEC>`"]
pub type ISRPQ = crate::Reg<isrpq::ISRPQ_SPEC>;
#[doc = "Interrupt Status Register Priority Queue (index = 1) 0"]
pub mod isrpq;
#[doc = "TBQBAPQ (rw) register accessor: an alias for `Reg<TBQBAPQ_SPEC>`"]
pub type TBQBAPQ = crate::Reg<tbqbapq::TBQBAPQ_SPEC>;
#[doc = "Transmit Buffer Queue Base Address Register Priority Queue (index = 1) 0"]
pub mod tbqbapq;
#[doc = "RBQBAPQ (rw) register accessor: an alias for `Reg<RBQBAPQ_SPEC>`"]
pub type RBQBAPQ = crate::Reg<rbqbapq::RBQBAPQ_SPEC>;
#[doc = "Receive Buffer Queue Base Address Register Priority Queue (index = 1) 0"]
pub mod rbqbapq;
#[doc = "RBSRPQ (rw) register accessor: an alias for `Reg<RBSRPQ_SPEC>`"]
pub type RBSRPQ = crate::Reg<rbsrpq::RBSRPQ_SPEC>;
#[doc = "Receive Buffer Size Register Priority Queue (index = 1) 0"]
pub mod rbsrpq;
#[doc = "CBSCR (rw) register accessor: an alias for `Reg<CBSCR_SPEC>`"]
pub type CBSCR = crate::Reg<cbscr::CBSCR_SPEC>;
#[doc = "Credit-Based Shaping Control Register"]
pub mod cbscr;
#[doc = "CBSISQA (rw) register accessor: an alias for `Reg<CBSISQA_SPEC>`"]
pub type CBSISQA = crate::Reg<cbsisqa::CBSISQA_SPEC>;
#[doc = "Credit-Based Shaping IdleSlope Register for Queue A"]
pub mod cbsisqa;
#[doc = "CBSISQB (rw) register accessor: an alias for `Reg<CBSISQB_SPEC>`"]
pub type CBSISQB = crate::Reg<cbsisqb::CBSISQB_SPEC>;
#[doc = "Credit-Based Shaping IdleSlope Register for Queue B"]
pub mod cbsisqb;
#[doc = "ST1RPQ (rw) register accessor: an alias for `Reg<ST1RPQ_SPEC>`"]
pub type ST1RPQ = crate::Reg<st1rpq::ST1RPQ_SPEC>;
#[doc = "Screening Type 1 Register Priority Queue (index = 0) 0"]
pub mod st1rpq;
#[doc = "ST2RPQ (rw) register accessor: an alias for `Reg<ST2RPQ_SPEC>`"]
pub type ST2RPQ = crate::Reg<st2rpq::ST2RPQ_SPEC>;
#[doc = "Screening Type 2 Register Priority Queue (index = 0) 0"]
pub mod st2rpq;
#[doc = "IERPQ (w) register accessor: an alias for `Reg<IERPQ_SPEC>`"]
pub type IERPQ = crate::Reg<ierpq::IERPQ_SPEC>;
#[doc = "Interrupt Enable Register Priority Queue (index = 1) 0"]
pub mod ierpq;
#[doc = "IDRPQ (w) register accessor: an alias for `Reg<IDRPQ_SPEC>`"]
pub type IDRPQ = crate::Reg<idrpq::IDRPQ_SPEC>;
#[doc = "Interrupt Disable Register Priority Queue (index = 1) 0"]
pub mod idrpq;
#[doc = "IMRPQ (rw) register accessor: an alias for `Reg<IMRPQ_SPEC>`"]
pub type IMRPQ = crate::Reg<imrpq::IMRPQ_SPEC>;
#[doc = "Interrupt Mask Register Priority Queue (index = 1) 0"]
pub mod imrpq;
#[doc = "ST2ER (rw) register accessor: an alias for `Reg<ST2ER_SPEC>`"]
pub type ST2ER = crate::Reg<st2er::ST2ER_SPEC>;
#[doc = "Screening Type 2 Ethertype Register (index = 0) 0"]
pub mod st2er;
#[doc = "ST2CW00 (rw) register accessor: an alias for `Reg<ST2CW00_SPEC>`"]
pub type ST2CW00 = crate::Reg<st2cw00::ST2CW00_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 0)"]
pub mod st2cw00;
#[doc = "ST2CW10 (rw) register accessor: an alias for `Reg<ST2CW10_SPEC>`"]
pub type ST2CW10 = crate::Reg<st2cw10::ST2CW10_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 0)"]
pub mod st2cw10;
#[doc = "ST2CW01 (rw) register accessor: an alias for `Reg<ST2CW01_SPEC>`"]
pub type ST2CW01 = crate::Reg<st2cw01::ST2CW01_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 1)"]
pub mod st2cw01;
#[doc = "ST2CW11 (rw) register accessor: an alias for `Reg<ST2CW11_SPEC>`"]
pub type ST2CW11 = crate::Reg<st2cw11::ST2CW11_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 1)"]
pub mod st2cw11;
#[doc = "ST2CW02 (rw) register accessor: an alias for `Reg<ST2CW02_SPEC>`"]
pub type ST2CW02 = crate::Reg<st2cw02::ST2CW02_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 2)"]
pub mod st2cw02;
#[doc = "ST2CW12 (rw) register accessor: an alias for `Reg<ST2CW12_SPEC>`"]
pub type ST2CW12 = crate::Reg<st2cw12::ST2CW12_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 2)"]
pub mod st2cw12;
#[doc = "ST2CW03 (rw) register accessor: an alias for `Reg<ST2CW03_SPEC>`"]
pub type ST2CW03 = crate::Reg<st2cw03::ST2CW03_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 3)"]
pub mod st2cw03;
#[doc = "ST2CW13 (rw) register accessor: an alias for `Reg<ST2CW13_SPEC>`"]
pub type ST2CW13 = crate::Reg<st2cw13::ST2CW13_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 3)"]
pub mod st2cw13;
#[doc = "ST2CW04 (rw) register accessor: an alias for `Reg<ST2CW04_SPEC>`"]
pub type ST2CW04 = crate::Reg<st2cw04::ST2CW04_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 4)"]
pub mod st2cw04;
#[doc = "ST2CW14 (rw) register accessor: an alias for `Reg<ST2CW14_SPEC>`"]
pub type ST2CW14 = crate::Reg<st2cw14::ST2CW14_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 4)"]
pub mod st2cw14;
#[doc = "ST2CW05 (rw) register accessor: an alias for `Reg<ST2CW05_SPEC>`"]
pub type ST2CW05 = crate::Reg<st2cw05::ST2CW05_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 5)"]
pub mod st2cw05;
#[doc = "ST2CW15 (rw) register accessor: an alias for `Reg<ST2CW15_SPEC>`"]
pub type ST2CW15 = crate::Reg<st2cw15::ST2CW15_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 5)"]
pub mod st2cw15;
#[doc = "ST2CW06 (rw) register accessor: an alias for `Reg<ST2CW06_SPEC>`"]
pub type ST2CW06 = crate::Reg<st2cw06::ST2CW06_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 6)"]
pub mod st2cw06;
#[doc = "ST2CW16 (rw) register accessor: an alias for `Reg<ST2CW16_SPEC>`"]
pub type ST2CW16 = crate::Reg<st2cw16::ST2CW16_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 6)"]
pub mod st2cw16;
#[doc = "ST2CW07 (rw) register accessor: an alias for `Reg<ST2CW07_SPEC>`"]
pub type ST2CW07 = crate::Reg<st2cw07::ST2CW07_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 7)"]
pub mod st2cw07;
#[doc = "ST2CW17 (rw) register accessor: an alias for `Reg<ST2CW17_SPEC>`"]
pub type ST2CW17 = crate::Reg<st2cw17::ST2CW17_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 7)"]
pub mod st2cw17;
#[doc = "ST2CW08 (rw) register accessor: an alias for `Reg<ST2CW08_SPEC>`"]
pub type ST2CW08 = crate::Reg<st2cw08::ST2CW08_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 8)"]
pub mod st2cw08;
#[doc = "ST2CW18 (rw) register accessor: an alias for `Reg<ST2CW18_SPEC>`"]
pub type ST2CW18 = crate::Reg<st2cw18::ST2CW18_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 8)"]
pub mod st2cw18;
#[doc = "ST2CW09 (rw) register accessor: an alias for `Reg<ST2CW09_SPEC>`"]
pub type ST2CW09 = crate::Reg<st2cw09::ST2CW09_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 9)"]
pub mod st2cw09;
#[doc = "ST2CW19 (rw) register accessor: an alias for `Reg<ST2CW19_SPEC>`"]
pub type ST2CW19 = crate::Reg<st2cw19::ST2CW19_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 9)"]
pub mod st2cw19;
#[doc = "ST2CW010 (rw) register accessor: an alias for `Reg<ST2CW010_SPEC>`"]
pub type ST2CW010 = crate::Reg<st2cw010::ST2CW010_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 10)"]
pub mod st2cw010;
#[doc = "ST2CW110 (rw) register accessor: an alias for `Reg<ST2CW110_SPEC>`"]
pub type ST2CW110 = crate::Reg<st2cw110::ST2CW110_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 10)"]
pub mod st2cw110;
#[doc = "ST2CW011 (rw) register accessor: an alias for `Reg<ST2CW011_SPEC>`"]
pub type ST2CW011 = crate::Reg<st2cw011::ST2CW011_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 11)"]
pub mod st2cw011;
#[doc = "ST2CW111 (rw) register accessor: an alias for `Reg<ST2CW111_SPEC>`"]
pub type ST2CW111 = crate::Reg<st2cw111::ST2CW111_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 11)"]
pub mod st2cw111;
#[doc = "ST2CW012 (rw) register accessor: an alias for `Reg<ST2CW012_SPEC>`"]
pub type ST2CW012 = crate::Reg<st2cw012::ST2CW012_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 12)"]
pub mod st2cw012;
#[doc = "ST2CW112 (rw) register accessor: an alias for `Reg<ST2CW112_SPEC>`"]
pub type ST2CW112 = crate::Reg<st2cw112::ST2CW112_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 12)"]
pub mod st2cw112;
#[doc = "ST2CW013 (rw) register accessor: an alias for `Reg<ST2CW013_SPEC>`"]
pub type ST2CW013 = crate::Reg<st2cw013::ST2CW013_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 13)"]
pub mod st2cw013;
#[doc = "ST2CW113 (rw) register accessor: an alias for `Reg<ST2CW113_SPEC>`"]
pub type ST2CW113 = crate::Reg<st2cw113::ST2CW113_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 13)"]
pub mod st2cw113;
#[doc = "ST2CW014 (rw) register accessor: an alias for `Reg<ST2CW014_SPEC>`"]
pub type ST2CW014 = crate::Reg<st2cw014::ST2CW014_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 14)"]
pub mod st2cw014;
#[doc = "ST2CW114 (rw) register accessor: an alias for `Reg<ST2CW114_SPEC>`"]
pub type ST2CW114 = crate::Reg<st2cw114::ST2CW114_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 14)"]
pub mod st2cw114;
#[doc = "ST2CW015 (rw) register accessor: an alias for `Reg<ST2CW015_SPEC>`"]
pub type ST2CW015 = crate::Reg<st2cw015::ST2CW015_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 15)"]
pub mod st2cw015;
#[doc = "ST2CW115 (rw) register accessor: an alias for `Reg<ST2CW115_SPEC>`"]
pub type ST2CW115 = crate::Reg<st2cw115::ST2CW115_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 15)"]
pub mod st2cw115;
#[doc = "ST2CW016 (rw) register accessor: an alias for `Reg<ST2CW016_SPEC>`"]
pub type ST2CW016 = crate::Reg<st2cw016::ST2CW016_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 16)"]
pub mod st2cw016;
#[doc = "ST2CW116 (rw) register accessor: an alias for `Reg<ST2CW116_SPEC>`"]
pub type ST2CW116 = crate::Reg<st2cw116::ST2CW116_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 16)"]
pub mod st2cw116;
#[doc = "ST2CW017 (rw) register accessor: an alias for `Reg<ST2CW017_SPEC>`"]
pub type ST2CW017 = crate::Reg<st2cw017::ST2CW017_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 17)"]
pub mod st2cw017;
#[doc = "ST2CW117 (rw) register accessor: an alias for `Reg<ST2CW117_SPEC>`"]
pub type ST2CW117 = crate::Reg<st2cw117::ST2CW117_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 17)"]
pub mod st2cw117;
#[doc = "ST2CW018 (rw) register accessor: an alias for `Reg<ST2CW018_SPEC>`"]
pub type ST2CW018 = crate::Reg<st2cw018::ST2CW018_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 18)"]
pub mod st2cw018;
#[doc = "ST2CW118 (rw) register accessor: an alias for `Reg<ST2CW118_SPEC>`"]
pub type ST2CW118 = crate::Reg<st2cw118::ST2CW118_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 18)"]
pub mod st2cw118;
#[doc = "ST2CW019 (rw) register accessor: an alias for `Reg<ST2CW019_SPEC>`"]
pub type ST2CW019 = crate::Reg<st2cw019::ST2CW019_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 19)"]
pub mod st2cw019;
#[doc = "ST2CW119 (rw) register accessor: an alias for `Reg<ST2CW119_SPEC>`"]
pub type ST2CW119 = crate::Reg<st2cw119::ST2CW119_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 19)"]
pub mod st2cw119;
#[doc = "ST2CW020 (rw) register accessor: an alias for `Reg<ST2CW020_SPEC>`"]
pub type ST2CW020 = crate::Reg<st2cw020::ST2CW020_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 20)"]
pub mod st2cw020;
#[doc = "ST2CW120 (rw) register accessor: an alias for `Reg<ST2CW120_SPEC>`"]
pub type ST2CW120 = crate::Reg<st2cw120::ST2CW120_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 20)"]
pub mod st2cw120;
#[doc = "ST2CW021 (rw) register accessor: an alias for `Reg<ST2CW021_SPEC>`"]
pub type ST2CW021 = crate::Reg<st2cw021::ST2CW021_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 21)"]
pub mod st2cw021;
#[doc = "ST2CW121 (rw) register accessor: an alias for `Reg<ST2CW121_SPEC>`"]
pub type ST2CW121 = crate::Reg<st2cw121::ST2CW121_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 21)"]
pub mod st2cw121;
#[doc = "ST2CW022 (rw) register accessor: an alias for `Reg<ST2CW022_SPEC>`"]
pub type ST2CW022 = crate::Reg<st2cw022::ST2CW022_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 22)"]
pub mod st2cw022;
#[doc = "ST2CW122 (rw) register accessor: an alias for `Reg<ST2CW122_SPEC>`"]
pub type ST2CW122 = crate::Reg<st2cw122::ST2CW122_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 22)"]
pub mod st2cw122;
#[doc = "ST2CW023 (rw) register accessor: an alias for `Reg<ST2CW023_SPEC>`"]
pub type ST2CW023 = crate::Reg<st2cw023::ST2CW023_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register (index = 23)"]
pub mod st2cw023;
#[doc = "ST2CW123 (rw) register accessor: an alias for `Reg<ST2CW123_SPEC>`"]
pub type ST2CW123 = crate::Reg<st2cw123::ST2CW123_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register (index = 23)"]
pub mod st2cw123;
