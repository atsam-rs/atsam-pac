#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Memory Address Register"]
    pub mar0: crate::Reg<mar::MAR_SPEC>,
    #[doc = "0x04 - Peripheral Select Register"]
    pub psr0: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x08 - Transfer Counter Register"]
    pub tcr0: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x0c - Memory Address Reload Register"]
    pub marr0: crate::Reg<marr::MARR_SPEC>,
    #[doc = "0x10 - Transfer Counter Reload Register"]
    pub tcrr0: crate::Reg<tcrr::TCRR_SPEC>,
    #[doc = "0x14 - Control Register"]
    pub cr0: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x18 - Mode Register"]
    pub mr0: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x1c - Status Register"]
    pub sr0: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x20 - Interrupt Enable Register"]
    pub ier0: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x24 - Interrupt Disable Register"]
    pub idr0: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x28 - Interrupt Mask Register"]
    pub imr0: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x2c - Interrupt Status Register"]
    pub isr0: crate::Reg<isr::ISR_SPEC>,
    _reserved12: [u8; 0x10],
    #[doc = "0x40 - Memory Address Register"]
    pub mar1: crate::Reg<mar::MAR_SPEC>,
    #[doc = "0x44 - Peripheral Select Register"]
    pub psr1: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x48 - Transfer Counter Register"]
    pub tcr1: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x4c - Memory Address Reload Register"]
    pub marr1: crate::Reg<marr::MARR_SPEC>,
    #[doc = "0x50 - Transfer Counter Reload Register"]
    pub tcrr1: crate::Reg<tcrr::TCRR_SPEC>,
    #[doc = "0x54 - Control Register"]
    pub cr1: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x58 - Mode Register"]
    pub mr1: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x5c - Status Register"]
    pub sr1: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x60 - Interrupt Enable Register"]
    pub ier1: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x64 - Interrupt Disable Register"]
    pub idr1: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x68 - Interrupt Mask Register"]
    pub imr1: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x6c - Interrupt Status Register"]
    pub isr1: crate::Reg<isr::ISR_SPEC>,
    _reserved24: [u8; 0x10],
    #[doc = "0x80 - Memory Address Register"]
    pub mar2: crate::Reg<mar::MAR_SPEC>,
    #[doc = "0x84 - Peripheral Select Register"]
    pub psr2: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x88 - Transfer Counter Register"]
    pub tcr2: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x8c - Memory Address Reload Register"]
    pub marr2: crate::Reg<marr::MARR_SPEC>,
    #[doc = "0x90 - Transfer Counter Reload Register"]
    pub tcrr2: crate::Reg<tcrr::TCRR_SPEC>,
    #[doc = "0x94 - Control Register"]
    pub cr2: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x98 - Mode Register"]
    pub mr2: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x9c - Status Register"]
    pub sr2: crate::Reg<sr::SR_SPEC>,
    #[doc = "0xa0 - Interrupt Enable Register"]
    pub ier2: crate::Reg<ier::IER_SPEC>,
    #[doc = "0xa4 - Interrupt Disable Register"]
    pub idr2: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0xa8 - Interrupt Mask Register"]
    pub imr2: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0xac - Interrupt Status Register"]
    pub isr2: crate::Reg<isr::ISR_SPEC>,
    _reserved36: [u8; 0x10],
    #[doc = "0xc0 - Memory Address Register"]
    pub mar3: crate::Reg<mar::MAR_SPEC>,
    #[doc = "0xc4 - Peripheral Select Register"]
    pub psr3: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0xc8 - Transfer Counter Register"]
    pub tcr3: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0xcc - Memory Address Reload Register"]
    pub marr3: crate::Reg<marr::MARR_SPEC>,
    #[doc = "0xd0 - Transfer Counter Reload Register"]
    pub tcrr3: crate::Reg<tcrr::TCRR_SPEC>,
    #[doc = "0xd4 - Control Register"]
    pub cr3: crate::Reg<cr::CR_SPEC>,
    #[doc = "0xd8 - Mode Register"]
    pub mr3: crate::Reg<mr::MR_SPEC>,
    #[doc = "0xdc - Status Register"]
    pub sr3: crate::Reg<sr::SR_SPEC>,
    #[doc = "0xe0 - Interrupt Enable Register"]
    pub ier3: crate::Reg<ier::IER_SPEC>,
    #[doc = "0xe4 - Interrupt Disable Register"]
    pub idr3: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0xe8 - Interrupt Mask Register"]
    pub imr3: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0xec - Interrupt Status Register"]
    pub isr3: crate::Reg<isr::ISR_SPEC>,
    _reserved48: [u8; 0x10],
    #[doc = "0x100 - Memory Address Register"]
    pub mar4: crate::Reg<mar::MAR_SPEC>,
    #[doc = "0x104 - Peripheral Select Register"]
    pub psr4: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x108 - Transfer Counter Register"]
    pub tcr4: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x10c - Memory Address Reload Register"]
    pub marr4: crate::Reg<marr::MARR_SPEC>,
    #[doc = "0x110 - Transfer Counter Reload Register"]
    pub tcrr4: crate::Reg<tcrr::TCRR_SPEC>,
    #[doc = "0x114 - Control Register"]
    pub cr4: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x118 - Mode Register"]
    pub mr4: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x11c - Status Register"]
    pub sr4: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x120 - Interrupt Enable Register"]
    pub ier4: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x124 - Interrupt Disable Register"]
    pub idr4: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x128 - Interrupt Mask Register"]
    pub imr4: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x12c - Interrupt Status Register"]
    pub isr4: crate::Reg<isr::ISR_SPEC>,
    _reserved60: [u8; 0x10],
    #[doc = "0x140 - Memory Address Register"]
    pub mar5: crate::Reg<mar::MAR_SPEC>,
    #[doc = "0x144 - Peripheral Select Register"]
    pub psr5: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x148 - Transfer Counter Register"]
    pub tcr5: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x14c - Memory Address Reload Register"]
    pub marr5: crate::Reg<marr::MARR_SPEC>,
    #[doc = "0x150 - Transfer Counter Reload Register"]
    pub tcrr5: crate::Reg<tcrr::TCRR_SPEC>,
    #[doc = "0x154 - Control Register"]
    pub cr5: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x158 - Mode Register"]
    pub mr5: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x15c - Status Register"]
    pub sr5: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x160 - Interrupt Enable Register"]
    pub ier5: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x164 - Interrupt Disable Register"]
    pub idr5: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x168 - Interrupt Mask Register"]
    pub imr5: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x16c - Interrupt Status Register"]
    pub isr5: crate::Reg<isr::ISR_SPEC>,
    _reserved72: [u8; 0x10],
    #[doc = "0x180 - Memory Address Register"]
    pub mar6: crate::Reg<mar::MAR_SPEC>,
    #[doc = "0x184 - Peripheral Select Register"]
    pub psr6: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x188 - Transfer Counter Register"]
    pub tcr6: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x18c - Memory Address Reload Register"]
    pub marr6: crate::Reg<marr::MARR_SPEC>,
    #[doc = "0x190 - Transfer Counter Reload Register"]
    pub tcrr6: crate::Reg<tcrr::TCRR_SPEC>,
    #[doc = "0x194 - Control Register"]
    pub cr6: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x198 - Mode Register"]
    pub mr6: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x19c - Status Register"]
    pub sr6: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x1a0 - Interrupt Enable Register"]
    pub ier6: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x1a4 - Interrupt Disable Register"]
    pub idr6: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x1a8 - Interrupt Mask Register"]
    pub imr6: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x1ac - Interrupt Status Register"]
    pub isr6: crate::Reg<isr::ISR_SPEC>,
    _reserved84: [u8; 0x10],
    #[doc = "0x1c0 - Memory Address Register"]
    pub mar7: crate::Reg<mar::MAR_SPEC>,
    #[doc = "0x1c4 - Peripheral Select Register"]
    pub psr7: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x1c8 - Transfer Counter Register"]
    pub tcr7: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x1cc - Memory Address Reload Register"]
    pub marr7: crate::Reg<marr::MARR_SPEC>,
    #[doc = "0x1d0 - Transfer Counter Reload Register"]
    pub tcrr7: crate::Reg<tcrr::TCRR_SPEC>,
    #[doc = "0x1d4 - Control Register"]
    pub cr7: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x1d8 - Mode Register"]
    pub mr7: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x1dc - Status Register"]
    pub sr7: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x1e0 - Interrupt Enable Register"]
    pub ier7: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x1e4 - Interrupt Disable Register"]
    pub idr7: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x1e8 - Interrupt Mask Register"]
    pub imr7: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x1ec - Interrupt Status Register"]
    pub isr7: crate::Reg<isr::ISR_SPEC>,
    _reserved96: [u8; 0x10],
    #[doc = "0x200 - Memory Address Register"]
    pub mar8: crate::Reg<mar::MAR_SPEC>,
    #[doc = "0x204 - Peripheral Select Register"]
    pub psr8: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x208 - Transfer Counter Register"]
    pub tcr8: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x20c - Memory Address Reload Register"]
    pub marr8: crate::Reg<marr::MARR_SPEC>,
    #[doc = "0x210 - Transfer Counter Reload Register"]
    pub tcrr8: crate::Reg<tcrr::TCRR_SPEC>,
    #[doc = "0x214 - Control Register"]
    pub cr8: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x218 - Mode Register"]
    pub mr8: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x21c - Status Register"]
    pub sr8: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x220 - Interrupt Enable Register"]
    pub ier8: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x224 - Interrupt Disable Register"]
    pub idr8: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x228 - Interrupt Mask Register"]
    pub imr8: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x22c - Interrupt Status Register"]
    pub isr8: crate::Reg<isr::ISR_SPEC>,
    _reserved108: [u8; 0x10],
    #[doc = "0x240 - Memory Address Register"]
    pub mar9: crate::Reg<mar::MAR_SPEC>,
    #[doc = "0x244 - Peripheral Select Register"]
    pub psr9: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x248 - Transfer Counter Register"]
    pub tcr9: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x24c - Memory Address Reload Register"]
    pub marr9: crate::Reg<marr::MARR_SPEC>,
    #[doc = "0x250 - Transfer Counter Reload Register"]
    pub tcrr9: crate::Reg<tcrr::TCRR_SPEC>,
    #[doc = "0x254 - Control Register"]
    pub cr9: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x258 - Mode Register"]
    pub mr9: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x25c - Status Register"]
    pub sr9: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x260 - Interrupt Enable Register"]
    pub ier9: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x264 - Interrupt Disable Register"]
    pub idr9: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x268 - Interrupt Mask Register"]
    pub imr9: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x26c - Interrupt Status Register"]
    pub isr9: crate::Reg<isr::ISR_SPEC>,
    _reserved120: [u8; 0x10],
    #[doc = "0x280 - Memory Address Register"]
    pub mar10: crate::Reg<mar::MAR_SPEC>,
    #[doc = "0x284 - Peripheral Select Register"]
    pub psr10: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x288 - Transfer Counter Register"]
    pub tcr10: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x28c - Memory Address Reload Register"]
    pub marr10: crate::Reg<marr::MARR_SPEC>,
    #[doc = "0x290 - Transfer Counter Reload Register"]
    pub tcrr10: crate::Reg<tcrr::TCRR_SPEC>,
    #[doc = "0x294 - Control Register"]
    pub cr10: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x298 - Mode Register"]
    pub mr10: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x29c - Status Register"]
    pub sr10: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x2a0 - Interrupt Enable Register"]
    pub ier10: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x2a4 - Interrupt Disable Register"]
    pub idr10: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x2a8 - Interrupt Mask Register"]
    pub imr10: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x2ac - Interrupt Status Register"]
    pub isr10: crate::Reg<isr::ISR_SPEC>,
    _reserved132: [u8; 0x10],
    #[doc = "0x2c0 - Memory Address Register"]
    pub mar11: crate::Reg<mar::MAR_SPEC>,
    #[doc = "0x2c4 - Peripheral Select Register"]
    pub psr11: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x2c8 - Transfer Counter Register"]
    pub tcr11: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x2cc - Memory Address Reload Register"]
    pub marr11: crate::Reg<marr::MARR_SPEC>,
    #[doc = "0x2d0 - Transfer Counter Reload Register"]
    pub tcrr11: crate::Reg<tcrr::TCRR_SPEC>,
    #[doc = "0x2d4 - Control Register"]
    pub cr11: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x2d8 - Mode Register"]
    pub mr11: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x2dc - Status Register"]
    pub sr11: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x2e0 - Interrupt Enable Register"]
    pub ier11: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x2e4 - Interrupt Disable Register"]
    pub idr11: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x2e8 - Interrupt Mask Register"]
    pub imr11: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x2ec - Interrupt Status Register"]
    pub isr11: crate::Reg<isr::ISR_SPEC>,
    _reserved144: [u8; 0x10],
    #[doc = "0x300 - Memory Address Register"]
    pub mar12: crate::Reg<mar::MAR_SPEC>,
    #[doc = "0x304 - Peripheral Select Register"]
    pub psr12: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x308 - Transfer Counter Register"]
    pub tcr12: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x30c - Memory Address Reload Register"]
    pub marr12: crate::Reg<marr::MARR_SPEC>,
    #[doc = "0x310 - Transfer Counter Reload Register"]
    pub tcrr12: crate::Reg<tcrr::TCRR_SPEC>,
    #[doc = "0x314 - Control Register"]
    pub cr12: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x318 - Mode Register"]
    pub mr12: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x31c - Status Register"]
    pub sr12: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x320 - Interrupt Enable Register"]
    pub ier12: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x324 - Interrupt Disable Register"]
    pub idr12: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x328 - Interrupt Mask Register"]
    pub imr12: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x32c - Interrupt Status Register"]
    pub isr12: crate::Reg<isr::ISR_SPEC>,
    _reserved156: [u8; 0x10],
    #[doc = "0x340 - Memory Address Register"]
    pub mar13: crate::Reg<mar::MAR_SPEC>,
    #[doc = "0x344 - Peripheral Select Register"]
    pub psr13: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x348 - Transfer Counter Register"]
    pub tcr13: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x34c - Memory Address Reload Register"]
    pub marr13: crate::Reg<marr::MARR_SPEC>,
    #[doc = "0x350 - Transfer Counter Reload Register"]
    pub tcrr13: crate::Reg<tcrr::TCRR_SPEC>,
    #[doc = "0x354 - Control Register"]
    pub cr13: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x358 - Mode Register"]
    pub mr13: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x35c - Status Register"]
    pub sr13: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x360 - Interrupt Enable Register"]
    pub ier13: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x364 - Interrupt Disable Register"]
    pub idr13: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x368 - Interrupt Mask Register"]
    pub imr13: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x36c - Interrupt Status Register"]
    pub isr13: crate::Reg<isr::ISR_SPEC>,
    _reserved168: [u8; 0x10],
    #[doc = "0x380 - Memory Address Register"]
    pub mar14: crate::Reg<mar::MAR_SPEC>,
    #[doc = "0x384 - Peripheral Select Register"]
    pub psr14: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x388 - Transfer Counter Register"]
    pub tcr14: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x38c - Memory Address Reload Register"]
    pub marr14: crate::Reg<marr::MARR_SPEC>,
    #[doc = "0x390 - Transfer Counter Reload Register"]
    pub tcrr14: crate::Reg<tcrr::TCRR_SPEC>,
    #[doc = "0x394 - Control Register"]
    pub cr14: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x398 - Mode Register"]
    pub mr14: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x39c - Status Register"]
    pub sr14: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x3a0 - Interrupt Enable Register"]
    pub ier14: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x3a4 - Interrupt Disable Register"]
    pub idr14: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x3a8 - Interrupt Mask Register"]
    pub imr14: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x3ac - Interrupt Status Register"]
    pub isr14: crate::Reg<isr::ISR_SPEC>,
    _reserved180: [u8; 0x10],
    #[doc = "0x3c0 - Memory Address Register"]
    pub mar15: crate::Reg<mar::MAR_SPEC>,
    #[doc = "0x3c4 - Peripheral Select Register"]
    pub psr15: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x3c8 - Transfer Counter Register"]
    pub tcr15: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x3cc - Memory Address Reload Register"]
    pub marr15: crate::Reg<marr::MARR_SPEC>,
    #[doc = "0x3d0 - Transfer Counter Reload Register"]
    pub tcrr15: crate::Reg<tcrr::TCRR_SPEC>,
    #[doc = "0x3d4 - Control Register"]
    pub cr15: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x3d8 - Mode Register"]
    pub mr15: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x3dc - Status Register"]
    pub sr15: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x3e0 - Interrupt Enable Register"]
    pub ier15: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x3e4 - Interrupt Disable Register"]
    pub idr15: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x3e8 - Interrupt Mask Register"]
    pub imr15: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x3ec - Interrupt Status Register"]
    pub isr15: crate::Reg<isr::ISR_SPEC>,
    _reserved192: [u8; 0x0410],
    #[doc = "0x800 - Performance Control Register"]
    pub pcontrol: crate::Reg<pcontrol::PCONTROL_SPEC>,
    #[doc = "0x804 - Channel 0 Read Data Cycles"]
    pub prdata0: crate::Reg<prdata0::PRDATA0_SPEC>,
    #[doc = "0x808 - Channel 0 Read Stall Cycles"]
    pub prstall0: crate::Reg<prstall0::PRSTALL0_SPEC>,
    #[doc = "0x80c - Channel 0 Read Max Latency"]
    pub prlat0: crate::Reg<prlat0::PRLAT0_SPEC>,
    #[doc = "0x810 - Channel 0 Write Data Cycles"]
    pub pwdata0: crate::Reg<pwdata0::PWDATA0_SPEC>,
    #[doc = "0x814 - Channel 0 Write Stall Cycles"]
    pub pwstall0: crate::Reg<pwstall0::PWSTALL0_SPEC>,
    #[doc = "0x818 - Channel0 Write Max Latency"]
    pub pwlat0: crate::Reg<pwlat0::PWLAT0_SPEC>,
    #[doc = "0x81c - Channel 1 Read Data Cycles"]
    pub prdata1: crate::Reg<prdata1::PRDATA1_SPEC>,
    #[doc = "0x820 - Channel Read Stall Cycles"]
    pub prstall1: crate::Reg<prstall1::PRSTALL1_SPEC>,
    #[doc = "0x824 - Channel 1 Read Max Latency"]
    pub prlat1: crate::Reg<prlat1::PRLAT1_SPEC>,
    #[doc = "0x828 - Channel 1 Write Data Cycles"]
    pub pwdata1: crate::Reg<pwdata1::PWDATA1_SPEC>,
    #[doc = "0x82c - Channel 1 Write stall Cycles"]
    pub pwstall1: crate::Reg<pwstall1::PWSTALL1_SPEC>,
    #[doc = "0x830 - Channel 1 Read Max Latency"]
    pub pwlat1: crate::Reg<pwlat1::PWLAT1_SPEC>,
    #[doc = "0x834 - Version Register"]
    pub version: crate::Reg<version::VERSION_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "MARR register accessor: an alias for `Reg<MARR_SPEC>`"]
pub type MARR = crate::Reg<marr::MARR_SPEC>;
#[doc = "Memory Address Reload Register"]
pub mod marr;
#[doc = "MAR register accessor: an alias for `Reg<MAR_SPEC>`"]
pub type MAR = crate::Reg<mar::MAR_SPEC>;
#[doc = "Memory Address Register"]
pub mod mar;
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "PCONTROL register accessor: an alias for `Reg<PCONTROL_SPEC>`"]
pub type PCONTROL = crate::Reg<pcontrol::PCONTROL_SPEC>;
#[doc = "Performance Control Register"]
pub mod pcontrol;
#[doc = "PRDATA0 register accessor: an alias for `Reg<PRDATA0_SPEC>`"]
pub type PRDATA0 = crate::Reg<prdata0::PRDATA0_SPEC>;
#[doc = "Channel 0 Read Data Cycles"]
pub mod prdata0;
#[doc = "PRDATA1 register accessor: an alias for `Reg<PRDATA1_SPEC>`"]
pub type PRDATA1 = crate::Reg<prdata1::PRDATA1_SPEC>;
#[doc = "Channel 1 Read Data Cycles"]
pub mod prdata1;
#[doc = "PRLAT0 register accessor: an alias for `Reg<PRLAT0_SPEC>`"]
pub type PRLAT0 = crate::Reg<prlat0::PRLAT0_SPEC>;
#[doc = "Channel 0 Read Max Latency"]
pub mod prlat0;
#[doc = "PRLAT1 register accessor: an alias for `Reg<PRLAT1_SPEC>`"]
pub type PRLAT1 = crate::Reg<prlat1::PRLAT1_SPEC>;
#[doc = "Channel 1 Read Max Latency"]
pub mod prlat1;
#[doc = "PRSTALL0 register accessor: an alias for `Reg<PRSTALL0_SPEC>`"]
pub type PRSTALL0 = crate::Reg<prstall0::PRSTALL0_SPEC>;
#[doc = "Channel 0 Read Stall Cycles"]
pub mod prstall0;
#[doc = "PRSTALL1 register accessor: an alias for `Reg<PRSTALL1_SPEC>`"]
pub type PRSTALL1 = crate::Reg<prstall1::PRSTALL1_SPEC>;
#[doc = "Channel Read Stall Cycles"]
pub mod prstall1;
#[doc = "PSR register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "Peripheral Select Register"]
pub mod psr;
#[doc = "PWDATA0 register accessor: an alias for `Reg<PWDATA0_SPEC>`"]
pub type PWDATA0 = crate::Reg<pwdata0::PWDATA0_SPEC>;
#[doc = "Channel 0 Write Data Cycles"]
pub mod pwdata0;
#[doc = "PWDATA1 register accessor: an alias for `Reg<PWDATA1_SPEC>`"]
pub type PWDATA1 = crate::Reg<pwdata1::PWDATA1_SPEC>;
#[doc = "Channel 1 Write Data Cycles"]
pub mod pwdata1;
#[doc = "PWLAT0 register accessor: an alias for `Reg<PWLAT0_SPEC>`"]
pub type PWLAT0 = crate::Reg<pwlat0::PWLAT0_SPEC>;
#[doc = "Channel0 Write Max Latency"]
pub mod pwlat0;
#[doc = "PWLAT1 register accessor: an alias for `Reg<PWLAT1_SPEC>`"]
pub type PWLAT1 = crate::Reg<pwlat1::PWLAT1_SPEC>;
#[doc = "Channel 1 Read Max Latency"]
pub mod pwlat1;
#[doc = "PWSTALL0 register accessor: an alias for `Reg<PWSTALL0_SPEC>`"]
pub type PWSTALL0 = crate::Reg<pwstall0::PWSTALL0_SPEC>;
#[doc = "Channel 0 Write Stall Cycles"]
pub mod pwstall0;
#[doc = "PWSTALL1 register accessor: an alias for `Reg<PWSTALL1_SPEC>`"]
pub type PWSTALL1 = crate::Reg<pwstall1::PWSTALL1_SPEC>;
#[doc = "Channel 1 Write stall Cycles"]
pub mod pwstall1;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "TCRR register accessor: an alias for `Reg<TCRR_SPEC>`"]
pub type TCRR = crate::Reg<tcrr::TCRR_SPEC>;
#[doc = "Transfer Counter Reload Register"]
pub mod tcrr;
#[doc = "TCR register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Transfer Counter Register"]
pub mod tcr;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
