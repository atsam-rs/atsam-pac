#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Memory Address Register"]
    pub mar0: MAR,
    #[doc = "0x04 - Peripheral Select Register"]
    pub psr0: PSR,
    #[doc = "0x08 - Transfer Counter Register"]
    pub tcr0: TCR,
    #[doc = "0x0c - Memory Address Reload Register"]
    pub marr0: MARR,
    #[doc = "0x10 - Transfer Counter Reload Register"]
    pub tcrr0: TCRR,
    #[doc = "0x14 - Control Register"]
    pub cr0: CR,
    #[doc = "0x18 - Mode Register"]
    pub mr0: MR,
    #[doc = "0x1c - Status Register"]
    pub sr0: SR,
    #[doc = "0x20 - Interrupt Enable Register"]
    pub ier0: IER,
    #[doc = "0x24 - Interrupt Disable Register"]
    pub idr0: IDR,
    #[doc = "0x28 - Interrupt Mask Register"]
    pub imr0: IMR,
    #[doc = "0x2c - Interrupt Status Register"]
    pub isr0: ISR,
    _reserved12: [u8; 0x10],
    #[doc = "0x40 - Memory Address Register"]
    pub mar1: MAR,
    #[doc = "0x44 - Peripheral Select Register"]
    pub psr1: PSR,
    #[doc = "0x48 - Transfer Counter Register"]
    pub tcr1: TCR,
    #[doc = "0x4c - Memory Address Reload Register"]
    pub marr1: MARR,
    #[doc = "0x50 - Transfer Counter Reload Register"]
    pub tcrr1: TCRR,
    #[doc = "0x54 - Control Register"]
    pub cr1: CR,
    #[doc = "0x58 - Mode Register"]
    pub mr1: MR,
    #[doc = "0x5c - Status Register"]
    pub sr1: SR,
    #[doc = "0x60 - Interrupt Enable Register"]
    pub ier1: IER,
    #[doc = "0x64 - Interrupt Disable Register"]
    pub idr1: IDR,
    #[doc = "0x68 - Interrupt Mask Register"]
    pub imr1: IMR,
    #[doc = "0x6c - Interrupt Status Register"]
    pub isr1: ISR,
    _reserved24: [u8; 0x10],
    #[doc = "0x80 - Memory Address Register"]
    pub mar2: MAR,
    #[doc = "0x84 - Peripheral Select Register"]
    pub psr2: PSR,
    #[doc = "0x88 - Transfer Counter Register"]
    pub tcr2: TCR,
    #[doc = "0x8c - Memory Address Reload Register"]
    pub marr2: MARR,
    #[doc = "0x90 - Transfer Counter Reload Register"]
    pub tcrr2: TCRR,
    #[doc = "0x94 - Control Register"]
    pub cr2: CR,
    #[doc = "0x98 - Mode Register"]
    pub mr2: MR,
    #[doc = "0x9c - Status Register"]
    pub sr2: SR,
    #[doc = "0xa0 - Interrupt Enable Register"]
    pub ier2: IER,
    #[doc = "0xa4 - Interrupt Disable Register"]
    pub idr2: IDR,
    #[doc = "0xa8 - Interrupt Mask Register"]
    pub imr2: IMR,
    #[doc = "0xac - Interrupt Status Register"]
    pub isr2: ISR,
    _reserved36: [u8; 0x10],
    #[doc = "0xc0 - Memory Address Register"]
    pub mar3: MAR,
    #[doc = "0xc4 - Peripheral Select Register"]
    pub psr3: PSR,
    #[doc = "0xc8 - Transfer Counter Register"]
    pub tcr3: TCR,
    #[doc = "0xcc - Memory Address Reload Register"]
    pub marr3: MARR,
    #[doc = "0xd0 - Transfer Counter Reload Register"]
    pub tcrr3: TCRR,
    #[doc = "0xd4 - Control Register"]
    pub cr3: CR,
    #[doc = "0xd8 - Mode Register"]
    pub mr3: MR,
    #[doc = "0xdc - Status Register"]
    pub sr3: SR,
    #[doc = "0xe0 - Interrupt Enable Register"]
    pub ier3: IER,
    #[doc = "0xe4 - Interrupt Disable Register"]
    pub idr3: IDR,
    #[doc = "0xe8 - Interrupt Mask Register"]
    pub imr3: IMR,
    #[doc = "0xec - Interrupt Status Register"]
    pub isr3: ISR,
    _reserved48: [u8; 0x10],
    #[doc = "0x100 - Memory Address Register"]
    pub mar4: MAR,
    #[doc = "0x104 - Peripheral Select Register"]
    pub psr4: PSR,
    #[doc = "0x108 - Transfer Counter Register"]
    pub tcr4: TCR,
    #[doc = "0x10c - Memory Address Reload Register"]
    pub marr4: MARR,
    #[doc = "0x110 - Transfer Counter Reload Register"]
    pub tcrr4: TCRR,
    #[doc = "0x114 - Control Register"]
    pub cr4: CR,
    #[doc = "0x118 - Mode Register"]
    pub mr4: MR,
    #[doc = "0x11c - Status Register"]
    pub sr4: SR,
    #[doc = "0x120 - Interrupt Enable Register"]
    pub ier4: IER,
    #[doc = "0x124 - Interrupt Disable Register"]
    pub idr4: IDR,
    #[doc = "0x128 - Interrupt Mask Register"]
    pub imr4: IMR,
    #[doc = "0x12c - Interrupt Status Register"]
    pub isr4: ISR,
    _reserved60: [u8; 0x10],
    #[doc = "0x140 - Memory Address Register"]
    pub mar5: MAR,
    #[doc = "0x144 - Peripheral Select Register"]
    pub psr5: PSR,
    #[doc = "0x148 - Transfer Counter Register"]
    pub tcr5: TCR,
    #[doc = "0x14c - Memory Address Reload Register"]
    pub marr5: MARR,
    #[doc = "0x150 - Transfer Counter Reload Register"]
    pub tcrr5: TCRR,
    #[doc = "0x154 - Control Register"]
    pub cr5: CR,
    #[doc = "0x158 - Mode Register"]
    pub mr5: MR,
    #[doc = "0x15c - Status Register"]
    pub sr5: SR,
    #[doc = "0x160 - Interrupt Enable Register"]
    pub ier5: IER,
    #[doc = "0x164 - Interrupt Disable Register"]
    pub idr5: IDR,
    #[doc = "0x168 - Interrupt Mask Register"]
    pub imr5: IMR,
    #[doc = "0x16c - Interrupt Status Register"]
    pub isr5: ISR,
    _reserved72: [u8; 0x10],
    #[doc = "0x180 - Memory Address Register"]
    pub mar6: MAR,
    #[doc = "0x184 - Peripheral Select Register"]
    pub psr6: PSR,
    #[doc = "0x188 - Transfer Counter Register"]
    pub tcr6: TCR,
    #[doc = "0x18c - Memory Address Reload Register"]
    pub marr6: MARR,
    #[doc = "0x190 - Transfer Counter Reload Register"]
    pub tcrr6: TCRR,
    #[doc = "0x194 - Control Register"]
    pub cr6: CR,
    #[doc = "0x198 - Mode Register"]
    pub mr6: MR,
    #[doc = "0x19c - Status Register"]
    pub sr6: SR,
    #[doc = "0x1a0 - Interrupt Enable Register"]
    pub ier6: IER,
    #[doc = "0x1a4 - Interrupt Disable Register"]
    pub idr6: IDR,
    #[doc = "0x1a8 - Interrupt Mask Register"]
    pub imr6: IMR,
    #[doc = "0x1ac - Interrupt Status Register"]
    pub isr6: ISR,
    _reserved84: [u8; 0x10],
    #[doc = "0x1c0 - Memory Address Register"]
    pub mar7: MAR,
    #[doc = "0x1c4 - Peripheral Select Register"]
    pub psr7: PSR,
    #[doc = "0x1c8 - Transfer Counter Register"]
    pub tcr7: TCR,
    #[doc = "0x1cc - Memory Address Reload Register"]
    pub marr7: MARR,
    #[doc = "0x1d0 - Transfer Counter Reload Register"]
    pub tcrr7: TCRR,
    #[doc = "0x1d4 - Control Register"]
    pub cr7: CR,
    #[doc = "0x1d8 - Mode Register"]
    pub mr7: MR,
    #[doc = "0x1dc - Status Register"]
    pub sr7: SR,
    #[doc = "0x1e0 - Interrupt Enable Register"]
    pub ier7: IER,
    #[doc = "0x1e4 - Interrupt Disable Register"]
    pub idr7: IDR,
    #[doc = "0x1e8 - Interrupt Mask Register"]
    pub imr7: IMR,
    #[doc = "0x1ec - Interrupt Status Register"]
    pub isr7: ISR,
    _reserved96: [u8; 0x10],
    #[doc = "0x200 - Memory Address Register"]
    pub mar8: MAR,
    #[doc = "0x204 - Peripheral Select Register"]
    pub psr8: PSR,
    #[doc = "0x208 - Transfer Counter Register"]
    pub tcr8: TCR,
    #[doc = "0x20c - Memory Address Reload Register"]
    pub marr8: MARR,
    #[doc = "0x210 - Transfer Counter Reload Register"]
    pub tcrr8: TCRR,
    #[doc = "0x214 - Control Register"]
    pub cr8: CR,
    #[doc = "0x218 - Mode Register"]
    pub mr8: MR,
    #[doc = "0x21c - Status Register"]
    pub sr8: SR,
    #[doc = "0x220 - Interrupt Enable Register"]
    pub ier8: IER,
    #[doc = "0x224 - Interrupt Disable Register"]
    pub idr8: IDR,
    #[doc = "0x228 - Interrupt Mask Register"]
    pub imr8: IMR,
    #[doc = "0x22c - Interrupt Status Register"]
    pub isr8: ISR,
    _reserved108: [u8; 0x10],
    #[doc = "0x240 - Memory Address Register"]
    pub mar9: MAR,
    #[doc = "0x244 - Peripheral Select Register"]
    pub psr9: PSR,
    #[doc = "0x248 - Transfer Counter Register"]
    pub tcr9: TCR,
    #[doc = "0x24c - Memory Address Reload Register"]
    pub marr9: MARR,
    #[doc = "0x250 - Transfer Counter Reload Register"]
    pub tcrr9: TCRR,
    #[doc = "0x254 - Control Register"]
    pub cr9: CR,
    #[doc = "0x258 - Mode Register"]
    pub mr9: MR,
    #[doc = "0x25c - Status Register"]
    pub sr9: SR,
    #[doc = "0x260 - Interrupt Enable Register"]
    pub ier9: IER,
    #[doc = "0x264 - Interrupt Disable Register"]
    pub idr9: IDR,
    #[doc = "0x268 - Interrupt Mask Register"]
    pub imr9: IMR,
    #[doc = "0x26c - Interrupt Status Register"]
    pub isr9: ISR,
    _reserved120: [u8; 0x10],
    #[doc = "0x280 - Memory Address Register"]
    pub mar10: MAR,
    #[doc = "0x284 - Peripheral Select Register"]
    pub psr10: PSR,
    #[doc = "0x288 - Transfer Counter Register"]
    pub tcr10: TCR,
    #[doc = "0x28c - Memory Address Reload Register"]
    pub marr10: MARR,
    #[doc = "0x290 - Transfer Counter Reload Register"]
    pub tcrr10: TCRR,
    #[doc = "0x294 - Control Register"]
    pub cr10: CR,
    #[doc = "0x298 - Mode Register"]
    pub mr10: MR,
    #[doc = "0x29c - Status Register"]
    pub sr10: SR,
    #[doc = "0x2a0 - Interrupt Enable Register"]
    pub ier10: IER,
    #[doc = "0x2a4 - Interrupt Disable Register"]
    pub idr10: IDR,
    #[doc = "0x2a8 - Interrupt Mask Register"]
    pub imr10: IMR,
    #[doc = "0x2ac - Interrupt Status Register"]
    pub isr10: ISR,
    _reserved132: [u8; 0x10],
    #[doc = "0x2c0 - Memory Address Register"]
    pub mar11: MAR,
    #[doc = "0x2c4 - Peripheral Select Register"]
    pub psr11: PSR,
    #[doc = "0x2c8 - Transfer Counter Register"]
    pub tcr11: TCR,
    #[doc = "0x2cc - Memory Address Reload Register"]
    pub marr11: MARR,
    #[doc = "0x2d0 - Transfer Counter Reload Register"]
    pub tcrr11: TCRR,
    #[doc = "0x2d4 - Control Register"]
    pub cr11: CR,
    #[doc = "0x2d8 - Mode Register"]
    pub mr11: MR,
    #[doc = "0x2dc - Status Register"]
    pub sr11: SR,
    #[doc = "0x2e0 - Interrupt Enable Register"]
    pub ier11: IER,
    #[doc = "0x2e4 - Interrupt Disable Register"]
    pub idr11: IDR,
    #[doc = "0x2e8 - Interrupt Mask Register"]
    pub imr11: IMR,
    #[doc = "0x2ec - Interrupt Status Register"]
    pub isr11: ISR,
    _reserved144: [u8; 0x10],
    #[doc = "0x300 - Memory Address Register"]
    pub mar12: MAR,
    #[doc = "0x304 - Peripheral Select Register"]
    pub psr12: PSR,
    #[doc = "0x308 - Transfer Counter Register"]
    pub tcr12: TCR,
    #[doc = "0x30c - Memory Address Reload Register"]
    pub marr12: MARR,
    #[doc = "0x310 - Transfer Counter Reload Register"]
    pub tcrr12: TCRR,
    #[doc = "0x314 - Control Register"]
    pub cr12: CR,
    #[doc = "0x318 - Mode Register"]
    pub mr12: MR,
    #[doc = "0x31c - Status Register"]
    pub sr12: SR,
    #[doc = "0x320 - Interrupt Enable Register"]
    pub ier12: IER,
    #[doc = "0x324 - Interrupt Disable Register"]
    pub idr12: IDR,
    #[doc = "0x328 - Interrupt Mask Register"]
    pub imr12: IMR,
    #[doc = "0x32c - Interrupt Status Register"]
    pub isr12: ISR,
    _reserved156: [u8; 0x10],
    #[doc = "0x340 - Memory Address Register"]
    pub mar13: MAR,
    #[doc = "0x344 - Peripheral Select Register"]
    pub psr13: PSR,
    #[doc = "0x348 - Transfer Counter Register"]
    pub tcr13: TCR,
    #[doc = "0x34c - Memory Address Reload Register"]
    pub marr13: MARR,
    #[doc = "0x350 - Transfer Counter Reload Register"]
    pub tcrr13: TCRR,
    #[doc = "0x354 - Control Register"]
    pub cr13: CR,
    #[doc = "0x358 - Mode Register"]
    pub mr13: MR,
    #[doc = "0x35c - Status Register"]
    pub sr13: SR,
    #[doc = "0x360 - Interrupt Enable Register"]
    pub ier13: IER,
    #[doc = "0x364 - Interrupt Disable Register"]
    pub idr13: IDR,
    #[doc = "0x368 - Interrupt Mask Register"]
    pub imr13: IMR,
    #[doc = "0x36c - Interrupt Status Register"]
    pub isr13: ISR,
    _reserved168: [u8; 0x10],
    #[doc = "0x380 - Memory Address Register"]
    pub mar14: MAR,
    #[doc = "0x384 - Peripheral Select Register"]
    pub psr14: PSR,
    #[doc = "0x388 - Transfer Counter Register"]
    pub tcr14: TCR,
    #[doc = "0x38c - Memory Address Reload Register"]
    pub marr14: MARR,
    #[doc = "0x390 - Transfer Counter Reload Register"]
    pub tcrr14: TCRR,
    #[doc = "0x394 - Control Register"]
    pub cr14: CR,
    #[doc = "0x398 - Mode Register"]
    pub mr14: MR,
    #[doc = "0x39c - Status Register"]
    pub sr14: SR,
    #[doc = "0x3a0 - Interrupt Enable Register"]
    pub ier14: IER,
    #[doc = "0x3a4 - Interrupt Disable Register"]
    pub idr14: IDR,
    #[doc = "0x3a8 - Interrupt Mask Register"]
    pub imr14: IMR,
    #[doc = "0x3ac - Interrupt Status Register"]
    pub isr14: ISR,
    _reserved180: [u8; 0x10],
    #[doc = "0x3c0 - Memory Address Register"]
    pub mar15: MAR,
    #[doc = "0x3c4 - Peripheral Select Register"]
    pub psr15: PSR,
    #[doc = "0x3c8 - Transfer Counter Register"]
    pub tcr15: TCR,
    #[doc = "0x3cc - Memory Address Reload Register"]
    pub marr15: MARR,
    #[doc = "0x3d0 - Transfer Counter Reload Register"]
    pub tcrr15: TCRR,
    #[doc = "0x3d4 - Control Register"]
    pub cr15: CR,
    #[doc = "0x3d8 - Mode Register"]
    pub mr15: MR,
    #[doc = "0x3dc - Status Register"]
    pub sr15: SR,
    #[doc = "0x3e0 - Interrupt Enable Register"]
    pub ier15: IER,
    #[doc = "0x3e4 - Interrupt Disable Register"]
    pub idr15: IDR,
    #[doc = "0x3e8 - Interrupt Mask Register"]
    pub imr15: IMR,
    #[doc = "0x3ec - Interrupt Status Register"]
    pub isr15: ISR,
    _reserved192: [u8; 0x0410],
    #[doc = "0x800 - Performance Control Register"]
    pub pcontrol: PCONTROL,
    #[doc = "0x804 - Channel 0 Read Data Cycles"]
    pub prdata0: PRDATA0,
    #[doc = "0x808 - Channel 0 Read Stall Cycles"]
    pub prstall0: PRSTALL0,
    #[doc = "0x80c - Channel 0 Read Max Latency"]
    pub prlat0: PRLAT0,
    #[doc = "0x810 - Channel 0 Write Data Cycles"]
    pub pwdata0: PWDATA0,
    #[doc = "0x814 - Channel 0 Write Stall Cycles"]
    pub pwstall0: PWSTALL0,
    #[doc = "0x818 - Channel0 Write Max Latency"]
    pub pwlat0: PWLAT0,
    #[doc = "0x81c - Channel 1 Read Data Cycles"]
    pub prdata1: PRDATA1,
    #[doc = "0x820 - Channel Read Stall Cycles"]
    pub prstall1: PRSTALL1,
    #[doc = "0x824 - Channel 1 Read Max Latency"]
    pub prlat1: PRLAT1,
    #[doc = "0x828 - Channel 1 Write Data Cycles"]
    pub pwdata1: PWDATA1,
    #[doc = "0x82c - Channel 1 Write stall Cycles"]
    pub pwstall1: PWSTALL1,
    #[doc = "0x830 - Channel 1 Read Max Latency"]
    pub pwlat1: PWLAT1,
    #[doc = "0x834 - Version Register"]
    pub version: VERSION,
}
#[doc = "CR (w) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "MARR (rw) register accessor: an alias for `Reg<MARR_SPEC>`"]
pub type MARR = crate::Reg<marr::MARR_SPEC>;
#[doc = "Memory Address Reload Register"]
pub mod marr;
#[doc = "MAR (rw) register accessor: an alias for `Reg<MAR_SPEC>`"]
pub type MAR = crate::Reg<mar::MAR_SPEC>;
#[doc = "Memory Address Register"]
pub mod mar;
#[doc = "MR (rw) register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "PCONTROL (rw) register accessor: an alias for `Reg<PCONTROL_SPEC>`"]
pub type PCONTROL = crate::Reg<pcontrol::PCONTROL_SPEC>;
#[doc = "Performance Control Register"]
pub mod pcontrol;
#[doc = "PRDATA0 (r) register accessor: an alias for `Reg<PRDATA0_SPEC>`"]
pub type PRDATA0 = crate::Reg<prdata0::PRDATA0_SPEC>;
#[doc = "Channel 0 Read Data Cycles"]
pub mod prdata0;
#[doc = "PRDATA1 (r) register accessor: an alias for `Reg<PRDATA1_SPEC>`"]
pub type PRDATA1 = crate::Reg<prdata1::PRDATA1_SPEC>;
#[doc = "Channel 1 Read Data Cycles"]
pub mod prdata1;
#[doc = "PRLAT0 (r) register accessor: an alias for `Reg<PRLAT0_SPEC>`"]
pub type PRLAT0 = crate::Reg<prlat0::PRLAT0_SPEC>;
#[doc = "Channel 0 Read Max Latency"]
pub mod prlat0;
#[doc = "PRLAT1 (r) register accessor: an alias for `Reg<PRLAT1_SPEC>`"]
pub type PRLAT1 = crate::Reg<prlat1::PRLAT1_SPEC>;
#[doc = "Channel 1 Read Max Latency"]
pub mod prlat1;
#[doc = "PRSTALL0 (r) register accessor: an alias for `Reg<PRSTALL0_SPEC>`"]
pub type PRSTALL0 = crate::Reg<prstall0::PRSTALL0_SPEC>;
#[doc = "Channel 0 Read Stall Cycles"]
pub mod prstall0;
#[doc = "PRSTALL1 (r) register accessor: an alias for `Reg<PRSTALL1_SPEC>`"]
pub type PRSTALL1 = crate::Reg<prstall1::PRSTALL1_SPEC>;
#[doc = "Channel Read Stall Cycles"]
pub mod prstall1;
#[doc = "PSR (rw) register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "Peripheral Select Register"]
pub mod psr;
#[doc = "PWDATA0 (r) register accessor: an alias for `Reg<PWDATA0_SPEC>`"]
pub type PWDATA0 = crate::Reg<pwdata0::PWDATA0_SPEC>;
#[doc = "Channel 0 Write Data Cycles"]
pub mod pwdata0;
#[doc = "PWDATA1 (r) register accessor: an alias for `Reg<PWDATA1_SPEC>`"]
pub type PWDATA1 = crate::Reg<pwdata1::PWDATA1_SPEC>;
#[doc = "Channel 1 Write Data Cycles"]
pub mod pwdata1;
#[doc = "PWLAT0 (r) register accessor: an alias for `Reg<PWLAT0_SPEC>`"]
pub type PWLAT0 = crate::Reg<pwlat0::PWLAT0_SPEC>;
#[doc = "Channel0 Write Max Latency"]
pub mod pwlat0;
#[doc = "PWLAT1 (r) register accessor: an alias for `Reg<PWLAT1_SPEC>`"]
pub type PWLAT1 = crate::Reg<pwlat1::PWLAT1_SPEC>;
#[doc = "Channel 1 Read Max Latency"]
pub mod pwlat1;
#[doc = "PWSTALL0 (r) register accessor: an alias for `Reg<PWSTALL0_SPEC>`"]
pub type PWSTALL0 = crate::Reg<pwstall0::PWSTALL0_SPEC>;
#[doc = "Channel 0 Write Stall Cycles"]
pub mod pwstall0;
#[doc = "PWSTALL1 (r) register accessor: an alias for `Reg<PWSTALL1_SPEC>`"]
pub type PWSTALL1 = crate::Reg<pwstall1::PWSTALL1_SPEC>;
#[doc = "Channel 1 Write stall Cycles"]
pub mod pwstall1;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "TCRR (rw) register accessor: an alias for `Reg<TCRR_SPEC>`"]
pub type TCRR = crate::Reg<tcrr::TCRR_SPEC>;
#[doc = "Transfer Counter Reload Register"]
pub mod tcrr;
#[doc = "TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Transfer Counter Register"]
pub mod tcr;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
