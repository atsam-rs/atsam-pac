#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO Enable Register"]
    pub gper0: GPER,
    #[doc = "0x04 - GPIO Enable Register - Set"]
    pub gpers0: GPERS,
    #[doc = "0x08 - GPIO Enable Register - Clear"]
    pub gperc0: GPERC,
    #[doc = "0x0c - GPIO Enable Register - Toggle"]
    pub gpert0: GPERT,
    #[doc = "0x10 - Peripheral Mux Register 0"]
    pub pmr00: PMR0,
    #[doc = "0x14 - Peripheral Mux Register 0 - Set"]
    pub pmr0s0: PMR0S,
    #[doc = "0x18 - Peripheral Mux Register 0 - Clear"]
    pub pmr0c0: PMR0C,
    #[doc = "0x1c - Peripheral Mux Register 0 - Toggle"]
    pub pmr0t0: PMR0T,
    #[doc = "0x20 - Peripheral Mux Register 1"]
    pub pmr10: PMR1,
    #[doc = "0x24 - Peripheral Mux Register 1 - Set"]
    pub pmr1s0: PMR1S,
    #[doc = "0x28 - Peripheral Mux Register 1 - Clear"]
    pub pmr1c0: PMR1C,
    #[doc = "0x2c - Peripheral Mux Register 1 - Toggle"]
    pub pmr1t0: PMR1T,
    #[doc = "0x30 - Peripheral Mux Register 2"]
    pub pmr20: PMR2,
    #[doc = "0x34 - Peripheral Mux Register 2 - Set"]
    pub pmr2s0: PMR2S,
    #[doc = "0x38 - Peripheral Mux Register 2 - Clear"]
    pub pmr2c0: PMR2C,
    #[doc = "0x3c - Peripheral Mux Register 2 - Toggle"]
    pub pmr2t0: PMR2T,
    #[doc = "0x40 - Output Driver Enable Register"]
    pub oder0: ODER,
    #[doc = "0x44 - Output Driver Enable Register - Set"]
    pub oders0: ODERS,
    #[doc = "0x48 - Output Driver Enable Register - Clear"]
    pub oderc0: ODERC,
    #[doc = "0x4c - Output Driver Enable Register - Toggle"]
    pub odert0: ODERT,
    #[doc = "0x50 - Output Value Register"]
    pub ovr0: OVR,
    #[doc = "0x54 - Output Value Register - Set"]
    pub ovrs0: OVRS,
    #[doc = "0x58 - Output Value Register - Clear"]
    pub ovrc0: OVRC,
    #[doc = "0x5c - Output Value Register - Toggle"]
    pub ovrt0: OVRT,
    #[doc = "0x60 - Pin Value Register"]
    pub pvr0: PVR,
    _reserved25: [u8; 12usize],
    #[doc = "0x70 - Pull-up Enable Register"]
    pub puer0: PUER,
    #[doc = "0x74 - Pull-up Enable Register - Set"]
    pub puers0: PUERS,
    #[doc = "0x78 - Pull-up Enable Register - Clear"]
    pub puerc0: PUERC,
    #[doc = "0x7c - Pull-up Enable Register - Toggle"]
    pub puert0: PUERT,
    #[doc = "0x80 - Pull-down Enable Register"]
    pub pder0: PDER,
    #[doc = "0x84 - Pull-down Enable Register - Set"]
    pub pders0: PDERS,
    #[doc = "0x88 - Pull-down Enable Register - Clear"]
    pub pderc0: PDERC,
    #[doc = "0x8c - Pull-down Enable Register - Toggle"]
    pub pdert0: PDERT,
    #[doc = "0x90 - Interrupt Enable Register"]
    pub ier0: IER,
    #[doc = "0x94 - Interrupt Enable Register - Set"]
    pub iers0: IERS,
    #[doc = "0x98 - Interrupt Enable Register - Clear"]
    pub ierc0: IERC,
    #[doc = "0x9c - Interrupt Enable Register - Toggle"]
    pub iert0: IERT,
    #[doc = "0xa0 - Interrupt Mode Register 0"]
    pub imr00: IMR0,
    #[doc = "0xa4 - Interrupt Mode Register 0 - Set"]
    pub imr0s0: IMR0S,
    #[doc = "0xa8 - Interrupt Mode Register 0 - Clear"]
    pub imr0c0: IMR0C,
    #[doc = "0xac - Interrupt Mode Register 0 - Toggle"]
    pub imr0t0: IMR0T,
    #[doc = "0xb0 - Interrupt Mode Register 1"]
    pub imr10: IMR1,
    #[doc = "0xb4 - Interrupt Mode Register 1 - Set"]
    pub imr1s0: IMR1S,
    #[doc = "0xb8 - Interrupt Mode Register 1 - Clear"]
    pub imr1c0: IMR1C,
    #[doc = "0xbc - Interrupt Mode Register 1 - Toggle"]
    pub imr1t0: IMR1T,
    #[doc = "0xc0 - Glitch Filter Enable Register"]
    pub gfer0: GFER,
    #[doc = "0xc4 - Glitch Filter Enable Register - Set"]
    pub gfers0: GFERS,
    #[doc = "0xc8 - Glitch Filter Enable Register - Clear"]
    pub gferc0: GFERC,
    #[doc = "0xcc - Glitch Filter Enable Register - Toggle"]
    pub gfert0: GFERT,
    #[doc = "0xd0 - Interrupt Flag Register"]
    pub ifr0: IFR,
    _reserved50: [u8; 4usize],
    #[doc = "0xd8 - Interrupt Flag Register - Clear"]
    pub ifrc0: IFRC,
    _reserved51: [u8; 4usize],
    #[doc = "0xe0 - Open Drain Mode Register"]
    pub odmer0: ODMER,
    #[doc = "0xe4 - Open Drain Mode Register - Set"]
    pub odmers0: ODMERS,
    #[doc = "0xe8 - Open Drain Mode Register - Clear"]
    pub odmerc0: ODMERC,
    #[doc = "0xec - Open Drain Mode Register - Toggle"]
    pub odmert0: ODMERT,
    _reserved55: [u8; 16usize],
    #[doc = "0x100 - Output Driving Capability Register 0"]
    pub odcr00: ODCR0,
    #[doc = "0x104 - Output Driving Capability Register 0 - Set"]
    pub odcr0s0: ODCR0S,
    #[doc = "0x108 - Output Driving Capability Register 0 - Clear"]
    pub odcr0c0: ODCR0C,
    #[doc = "0x10c - Output Driving Capability Register 0 - Toggle"]
    pub odcr0t0: ODCR0T,
    #[doc = "0x110 - Output Driving Capability Register 1"]
    pub odcr10: ODCR1,
    #[doc = "0x114 - Output Driving Capability Register 1 - Set"]
    pub odcr1s0: ODCR1S,
    #[doc = "0x118 - Output Driving Capability Register 1 - Clear"]
    pub odcr1c0: ODCR1C,
    #[doc = "0x11c - Output Driving Capability Register 1 - Toggle"]
    pub odcr1t0: ODCR1T,
    _reserved63: [u8; 16usize],
    #[doc = "0x130 - Output Slew Rate Register 0"]
    pub osrr00: OSRR0,
    #[doc = "0x134 - Output Slew Rate Register 0 - Set"]
    pub osrr0s0: OSRR0S,
    #[doc = "0x138 - Output Slew Rate Register 0 - Clear"]
    pub osrr0c0: OSRR0C,
    #[doc = "0x13c - Output Slew Rate Register 0 - Toggle"]
    pub osrr0t0: OSRR0T,
    _reserved67: [u8; 32usize],
    #[doc = "0x160 - Schmitt Trigger Enable Register"]
    pub ster0: STER,
    #[doc = "0x164 - Schmitt Trigger Enable Register - Set"]
    pub sters0: STERS,
    #[doc = "0x168 - Schmitt Trigger Enable Register - Clear"]
    pub sterc0: STERC,
    #[doc = "0x16c - Schmitt Trigger Enable Register - Toggle"]
    pub stert0: STERT,
    _reserved71: [u8; 16usize],
    #[doc = "0x180 - Event Enable Register"]
    pub ever0: EVER,
    #[doc = "0x184 - Event Enable Register - Set"]
    pub evers0: EVERS,
    #[doc = "0x188 - Event Enable Register - Clear"]
    pub everc0: EVERC,
    #[doc = "0x18c - Event Enable Register - Toggle"]
    pub evert0: EVERT,
    _reserved75: [u8; 16usize],
    #[doc = "0x1a0 - Lock Register"]
    pub lock0: LOCK,
    #[doc = "0x1a4 - Lock Register - Set"]
    pub locks0: LOCKS,
    #[doc = "0x1a8 - Lock Register - Clear"]
    pub lockc0: LOCKC,
    #[doc = "0x1ac - Lock Register - Toggle"]
    pub lockt0: LOCKT,
    _reserved79: [u8; 48usize],
    #[doc = "0x1e0 - Unlock Register"]
    pub unlock0: UNLOCK,
    #[doc = "0x1e4 - Access Status Register"]
    pub asr0: ASR,
    _reserved81: [u8; 16usize],
    #[doc = "0x1f8 - Parameter Register"]
    pub parameter0: PARAMETER,
    #[doc = "0x1fc - Version Register"]
    pub version0: VERSION,
    #[doc = "0x200 - GPIO Enable Register"]
    pub gper1: GPER,
    #[doc = "0x204 - GPIO Enable Register - Set"]
    pub gpers1: GPERS,
    #[doc = "0x208 - GPIO Enable Register - Clear"]
    pub gperc1: GPERC,
    #[doc = "0x20c - GPIO Enable Register - Toggle"]
    pub gpert1: GPERT,
    #[doc = "0x210 - Peripheral Mux Register 0"]
    pub pmr01: PMR0,
    #[doc = "0x214 - Peripheral Mux Register 0 - Set"]
    pub pmr0s1: PMR0S,
    #[doc = "0x218 - Peripheral Mux Register 0 - Clear"]
    pub pmr0c1: PMR0C,
    #[doc = "0x21c - Peripheral Mux Register 0 - Toggle"]
    pub pmr0t1: PMR0T,
    #[doc = "0x220 - Peripheral Mux Register 1"]
    pub pmr11: PMR1,
    #[doc = "0x224 - Peripheral Mux Register 1 - Set"]
    pub pmr1s1: PMR1S,
    #[doc = "0x228 - Peripheral Mux Register 1 - Clear"]
    pub pmr1c1: PMR1C,
    #[doc = "0x22c - Peripheral Mux Register 1 - Toggle"]
    pub pmr1t1: PMR1T,
    #[doc = "0x230 - Peripheral Mux Register 2"]
    pub pmr21: PMR2,
    #[doc = "0x234 - Peripheral Mux Register 2 - Set"]
    pub pmr2s1: PMR2S,
    #[doc = "0x238 - Peripheral Mux Register 2 - Clear"]
    pub pmr2c1: PMR2C,
    #[doc = "0x23c - Peripheral Mux Register 2 - Toggle"]
    pub pmr2t1: PMR2T,
    #[doc = "0x240 - Output Driver Enable Register"]
    pub oder1: ODER,
    #[doc = "0x244 - Output Driver Enable Register - Set"]
    pub oders1: ODERS,
    #[doc = "0x248 - Output Driver Enable Register - Clear"]
    pub oderc1: ODERC,
    #[doc = "0x24c - Output Driver Enable Register - Toggle"]
    pub odert1: ODERT,
    #[doc = "0x250 - Output Value Register"]
    pub ovr1: OVR,
    #[doc = "0x254 - Output Value Register - Set"]
    pub ovrs1: OVRS,
    #[doc = "0x258 - Output Value Register - Clear"]
    pub ovrc1: OVRC,
    #[doc = "0x25c - Output Value Register - Toggle"]
    pub ovrt1: OVRT,
    #[doc = "0x260 - Pin Value Register"]
    pub pvr1: PVR,
    _reserved108: [u8; 12usize],
    #[doc = "0x270 - Pull-up Enable Register"]
    pub puer1: PUER,
    #[doc = "0x274 - Pull-up Enable Register - Set"]
    pub puers1: PUERS,
    #[doc = "0x278 - Pull-up Enable Register - Clear"]
    pub puerc1: PUERC,
    #[doc = "0x27c - Pull-up Enable Register - Toggle"]
    pub puert1: PUERT,
    #[doc = "0x280 - Pull-down Enable Register"]
    pub pder1: PDER,
    #[doc = "0x284 - Pull-down Enable Register - Set"]
    pub pders1: PDERS,
    #[doc = "0x288 - Pull-down Enable Register - Clear"]
    pub pderc1: PDERC,
    #[doc = "0x28c - Pull-down Enable Register - Toggle"]
    pub pdert1: PDERT,
    #[doc = "0x290 - Interrupt Enable Register"]
    pub ier1: IER,
    #[doc = "0x294 - Interrupt Enable Register - Set"]
    pub iers1: IERS,
    #[doc = "0x298 - Interrupt Enable Register - Clear"]
    pub ierc1: IERC,
    #[doc = "0x29c - Interrupt Enable Register - Toggle"]
    pub iert1: IERT,
    #[doc = "0x2a0 - Interrupt Mode Register 0"]
    pub imr01: IMR0,
    #[doc = "0x2a4 - Interrupt Mode Register 0 - Set"]
    pub imr0s1: IMR0S,
    #[doc = "0x2a8 - Interrupt Mode Register 0 - Clear"]
    pub imr0c1: IMR0C,
    #[doc = "0x2ac - Interrupt Mode Register 0 - Toggle"]
    pub imr0t1: IMR0T,
    #[doc = "0x2b0 - Interrupt Mode Register 1"]
    pub imr11: IMR1,
    #[doc = "0x2b4 - Interrupt Mode Register 1 - Set"]
    pub imr1s1: IMR1S,
    #[doc = "0x2b8 - Interrupt Mode Register 1 - Clear"]
    pub imr1c1: IMR1C,
    #[doc = "0x2bc - Interrupt Mode Register 1 - Toggle"]
    pub imr1t1: IMR1T,
    #[doc = "0x2c0 - Glitch Filter Enable Register"]
    pub gfer1: GFER,
    #[doc = "0x2c4 - Glitch Filter Enable Register - Set"]
    pub gfers1: GFERS,
    #[doc = "0x2c8 - Glitch Filter Enable Register - Clear"]
    pub gferc1: GFERC,
    #[doc = "0x2cc - Glitch Filter Enable Register - Toggle"]
    pub gfert1: GFERT,
    #[doc = "0x2d0 - Interrupt Flag Register"]
    pub ifr1: IFR,
    _reserved133: [u8; 4usize],
    #[doc = "0x2d8 - Interrupt Flag Register - Clear"]
    pub ifrc1: IFRC,
    _reserved134: [u8; 4usize],
    #[doc = "0x2e0 - Open Drain Mode Register"]
    pub odmer1: ODMER,
    #[doc = "0x2e4 - Open Drain Mode Register - Set"]
    pub odmers1: ODMERS,
    #[doc = "0x2e8 - Open Drain Mode Register - Clear"]
    pub odmerc1: ODMERC,
    #[doc = "0x2ec - Open Drain Mode Register - Toggle"]
    pub odmert1: ODMERT,
    _reserved138: [u8; 16usize],
    #[doc = "0x300 - Output Driving Capability Register 0"]
    pub odcr01: ODCR0,
    #[doc = "0x304 - Output Driving Capability Register 0 - Set"]
    pub odcr0s1: ODCR0S,
    #[doc = "0x308 - Output Driving Capability Register 0 - Clear"]
    pub odcr0c1: ODCR0C,
    #[doc = "0x30c - Output Driving Capability Register 0 - Toggle"]
    pub odcr0t1: ODCR0T,
    #[doc = "0x310 - Output Driving Capability Register 1"]
    pub odcr11: ODCR1,
    #[doc = "0x314 - Output Driving Capability Register 1 - Set"]
    pub odcr1s1: ODCR1S,
    #[doc = "0x318 - Output Driving Capability Register 1 - Clear"]
    pub odcr1c1: ODCR1C,
    #[doc = "0x31c - Output Driving Capability Register 1 - Toggle"]
    pub odcr1t1: ODCR1T,
    _reserved146: [u8; 16usize],
    #[doc = "0x330 - Output Slew Rate Register 0"]
    pub osrr01: OSRR0,
    #[doc = "0x334 - Output Slew Rate Register 0 - Set"]
    pub osrr0s1: OSRR0S,
    #[doc = "0x338 - Output Slew Rate Register 0 - Clear"]
    pub osrr0c1: OSRR0C,
    #[doc = "0x33c - Output Slew Rate Register 0 - Toggle"]
    pub osrr0t1: OSRR0T,
    _reserved150: [u8; 32usize],
    #[doc = "0x360 - Schmitt Trigger Enable Register"]
    pub ster1: STER,
    #[doc = "0x364 - Schmitt Trigger Enable Register - Set"]
    pub sters1: STERS,
    #[doc = "0x368 - Schmitt Trigger Enable Register - Clear"]
    pub sterc1: STERC,
    #[doc = "0x36c - Schmitt Trigger Enable Register - Toggle"]
    pub stert1: STERT,
    _reserved154: [u8; 16usize],
    #[doc = "0x380 - Event Enable Register"]
    pub ever1: EVER,
    #[doc = "0x384 - Event Enable Register - Set"]
    pub evers1: EVERS,
    #[doc = "0x388 - Event Enable Register - Clear"]
    pub everc1: EVERC,
    #[doc = "0x38c - Event Enable Register - Toggle"]
    pub evert1: EVERT,
    _reserved158: [u8; 16usize],
    #[doc = "0x3a0 - Lock Register"]
    pub lock1: LOCK,
    #[doc = "0x3a4 - Lock Register - Set"]
    pub locks1: LOCKS,
    #[doc = "0x3a8 - Lock Register - Clear"]
    pub lockc1: LOCKC,
    #[doc = "0x3ac - Lock Register - Toggle"]
    pub lockt1: LOCKT,
    _reserved162: [u8; 48usize],
    #[doc = "0x3e0 - Unlock Register"]
    pub unlock1: UNLOCK,
    #[doc = "0x3e4 - Access Status Register"]
    pub asr1: ASR,
    _reserved164: [u8; 16usize],
    #[doc = "0x3f8 - Parameter Register"]
    pub parameter1: PARAMETER,
    #[doc = "0x3fc - Version Register"]
    pub version1: VERSION,
    #[doc = "0x400 - GPIO Enable Register"]
    pub gper2: GPER,
    #[doc = "0x404 - GPIO Enable Register - Set"]
    pub gpers2: GPERS,
    #[doc = "0x408 - GPIO Enable Register - Clear"]
    pub gperc2: GPERC,
    #[doc = "0x40c - GPIO Enable Register - Toggle"]
    pub gpert2: GPERT,
    #[doc = "0x410 - Peripheral Mux Register 0"]
    pub pmr02: PMR0,
    #[doc = "0x414 - Peripheral Mux Register 0 - Set"]
    pub pmr0s2: PMR0S,
    #[doc = "0x418 - Peripheral Mux Register 0 - Clear"]
    pub pmr0c2: PMR0C,
    #[doc = "0x41c - Peripheral Mux Register 0 - Toggle"]
    pub pmr0t2: PMR0T,
    #[doc = "0x420 - Peripheral Mux Register 1"]
    pub pmr12: PMR1,
    #[doc = "0x424 - Peripheral Mux Register 1 - Set"]
    pub pmr1s2: PMR1S,
    #[doc = "0x428 - Peripheral Mux Register 1 - Clear"]
    pub pmr1c2: PMR1C,
    #[doc = "0x42c - Peripheral Mux Register 1 - Toggle"]
    pub pmr1t2: PMR1T,
    #[doc = "0x430 - Peripheral Mux Register 2"]
    pub pmr22: PMR2,
    #[doc = "0x434 - Peripheral Mux Register 2 - Set"]
    pub pmr2s2: PMR2S,
    #[doc = "0x438 - Peripheral Mux Register 2 - Clear"]
    pub pmr2c2: PMR2C,
    #[doc = "0x43c - Peripheral Mux Register 2 - Toggle"]
    pub pmr2t2: PMR2T,
    #[doc = "0x440 - Output Driver Enable Register"]
    pub oder2: ODER,
    #[doc = "0x444 - Output Driver Enable Register - Set"]
    pub oders2: ODERS,
    #[doc = "0x448 - Output Driver Enable Register - Clear"]
    pub oderc2: ODERC,
    #[doc = "0x44c - Output Driver Enable Register - Toggle"]
    pub odert2: ODERT,
    #[doc = "0x450 - Output Value Register"]
    pub ovr2: OVR,
    #[doc = "0x454 - Output Value Register - Set"]
    pub ovrs2: OVRS,
    #[doc = "0x458 - Output Value Register - Clear"]
    pub ovrc2: OVRC,
    #[doc = "0x45c - Output Value Register - Toggle"]
    pub ovrt2: OVRT,
    #[doc = "0x460 - Pin Value Register"]
    pub pvr2: PVR,
    _reserved191: [u8; 12usize],
    #[doc = "0x470 - Pull-up Enable Register"]
    pub puer2: PUER,
    #[doc = "0x474 - Pull-up Enable Register - Set"]
    pub puers2: PUERS,
    #[doc = "0x478 - Pull-up Enable Register - Clear"]
    pub puerc2: PUERC,
    #[doc = "0x47c - Pull-up Enable Register - Toggle"]
    pub puert2: PUERT,
    #[doc = "0x480 - Pull-down Enable Register"]
    pub pder2: PDER,
    #[doc = "0x484 - Pull-down Enable Register - Set"]
    pub pders2: PDERS,
    #[doc = "0x488 - Pull-down Enable Register - Clear"]
    pub pderc2: PDERC,
    #[doc = "0x48c - Pull-down Enable Register - Toggle"]
    pub pdert2: PDERT,
    #[doc = "0x490 - Interrupt Enable Register"]
    pub ier2: IER,
    #[doc = "0x494 - Interrupt Enable Register - Set"]
    pub iers2: IERS,
    #[doc = "0x498 - Interrupt Enable Register - Clear"]
    pub ierc2: IERC,
    #[doc = "0x49c - Interrupt Enable Register - Toggle"]
    pub iert2: IERT,
    #[doc = "0x4a0 - Interrupt Mode Register 0"]
    pub imr02: IMR0,
    #[doc = "0x4a4 - Interrupt Mode Register 0 - Set"]
    pub imr0s2: IMR0S,
    #[doc = "0x4a8 - Interrupt Mode Register 0 - Clear"]
    pub imr0c2: IMR0C,
    #[doc = "0x4ac - Interrupt Mode Register 0 - Toggle"]
    pub imr0t2: IMR0T,
    #[doc = "0x4b0 - Interrupt Mode Register 1"]
    pub imr12: IMR1,
    #[doc = "0x4b4 - Interrupt Mode Register 1 - Set"]
    pub imr1s2: IMR1S,
    #[doc = "0x4b8 - Interrupt Mode Register 1 - Clear"]
    pub imr1c2: IMR1C,
    #[doc = "0x4bc - Interrupt Mode Register 1 - Toggle"]
    pub imr1t2: IMR1T,
    #[doc = "0x4c0 - Glitch Filter Enable Register"]
    pub gfer2: GFER,
    #[doc = "0x4c4 - Glitch Filter Enable Register - Set"]
    pub gfers2: GFERS,
    #[doc = "0x4c8 - Glitch Filter Enable Register - Clear"]
    pub gferc2: GFERC,
    #[doc = "0x4cc - Glitch Filter Enable Register - Toggle"]
    pub gfert2: GFERT,
    #[doc = "0x4d0 - Interrupt Flag Register"]
    pub ifr2: IFR,
    _reserved216: [u8; 4usize],
    #[doc = "0x4d8 - Interrupt Flag Register - Clear"]
    pub ifrc2: IFRC,
    _reserved217: [u8; 4usize],
    #[doc = "0x4e0 - Open Drain Mode Register"]
    pub odmer2: ODMER,
    #[doc = "0x4e4 - Open Drain Mode Register - Set"]
    pub odmers2: ODMERS,
    #[doc = "0x4e8 - Open Drain Mode Register - Clear"]
    pub odmerc2: ODMERC,
    #[doc = "0x4ec - Open Drain Mode Register - Toggle"]
    pub odmert2: ODMERT,
    _reserved221: [u8; 16usize],
    #[doc = "0x500 - Output Driving Capability Register 0"]
    pub odcr02: ODCR0,
    #[doc = "0x504 - Output Driving Capability Register 0 - Set"]
    pub odcr0s2: ODCR0S,
    #[doc = "0x508 - Output Driving Capability Register 0 - Clear"]
    pub odcr0c2: ODCR0C,
    #[doc = "0x50c - Output Driving Capability Register 0 - Toggle"]
    pub odcr0t2: ODCR0T,
    #[doc = "0x510 - Output Driving Capability Register 1"]
    pub odcr12: ODCR1,
    #[doc = "0x514 - Output Driving Capability Register 1 - Set"]
    pub odcr1s2: ODCR1S,
    #[doc = "0x518 - Output Driving Capability Register 1 - Clear"]
    pub odcr1c2: ODCR1C,
    #[doc = "0x51c - Output Driving Capability Register 1 - Toggle"]
    pub odcr1t2: ODCR1T,
    _reserved229: [u8; 16usize],
    #[doc = "0x530 - Output Slew Rate Register 0"]
    pub osrr02: OSRR0,
    #[doc = "0x534 - Output Slew Rate Register 0 - Set"]
    pub osrr0s2: OSRR0S,
    #[doc = "0x538 - Output Slew Rate Register 0 - Clear"]
    pub osrr0c2: OSRR0C,
    #[doc = "0x53c - Output Slew Rate Register 0 - Toggle"]
    pub osrr0t2: OSRR0T,
    _reserved233: [u8; 32usize],
    #[doc = "0x560 - Schmitt Trigger Enable Register"]
    pub ster2: STER,
    #[doc = "0x564 - Schmitt Trigger Enable Register - Set"]
    pub sters2: STERS,
    #[doc = "0x568 - Schmitt Trigger Enable Register - Clear"]
    pub sterc2: STERC,
    #[doc = "0x56c - Schmitt Trigger Enable Register - Toggle"]
    pub stert2: STERT,
    _reserved237: [u8; 16usize],
    #[doc = "0x580 - Event Enable Register"]
    pub ever2: EVER,
    #[doc = "0x584 - Event Enable Register - Set"]
    pub evers2: EVERS,
    #[doc = "0x588 - Event Enable Register - Clear"]
    pub everc2: EVERC,
    #[doc = "0x58c - Event Enable Register - Toggle"]
    pub evert2: EVERT,
    _reserved241: [u8; 16usize],
    #[doc = "0x5a0 - Lock Register"]
    pub lock2: LOCK,
    #[doc = "0x5a4 - Lock Register - Set"]
    pub locks2: LOCKS,
    #[doc = "0x5a8 - Lock Register - Clear"]
    pub lockc2: LOCKC,
    #[doc = "0x5ac - Lock Register - Toggle"]
    pub lockt2: LOCKT,
    _reserved245: [u8; 48usize],
    #[doc = "0x5e0 - Unlock Register"]
    pub unlock2: UNLOCK,
    #[doc = "0x5e4 - Access Status Register"]
    pub asr2: ASR,
    _reserved247: [u8; 16usize],
    #[doc = "0x5f8 - Parameter Register"]
    pub parameter2: PARAMETER,
    #[doc = "0x5fc - Version Register"]
    pub version2: VERSION,
}
#[doc = "Access Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asr](asr) module"]
pub type ASR = crate::Reg<u32, _ASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASR;
#[doc = "`read()` method returns [asr::R](asr::R) reader structure"]
impl crate::Readable for ASR {}
#[doc = "`write(|w| ..)` method takes [asr::W](asr::W) writer structure"]
impl crate::Writable for ASR {}
#[doc = "Access Status Register"]
pub mod asr;
#[doc = "Event Enable Register - Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [everc](everc) module"]
pub type EVERC = crate::Reg<u32, _EVERC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVERC;
#[doc = "`write(|w| ..)` method takes [everc::W](everc::W) writer structure"]
impl crate::Writable for EVERC {}
#[doc = "Event Enable Register - Clear"]
pub mod everc;
#[doc = "Event Enable Register - Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evers](evers) module"]
pub type EVERS = crate::Reg<u32, _EVERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVERS;
#[doc = "`write(|w| ..)` method takes [evers::W](evers::W) writer structure"]
impl crate::Writable for EVERS {}
#[doc = "Event Enable Register - Set"]
pub mod evers;
#[doc = "Event Enable Register - Toggle\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evert](evert) module"]
pub type EVERT = crate::Reg<u32, _EVERT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVERT;
#[doc = "`write(|w| ..)` method takes [evert::W](evert::W) writer structure"]
impl crate::Writable for EVERT {}
#[doc = "Event Enable Register - Toggle"]
pub mod evert;
#[doc = "Event Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ever](ever) module"]
pub type EVER = crate::Reg<u32, _EVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVER;
#[doc = "`read()` method returns [ever::R](ever::R) reader structure"]
impl crate::Readable for EVER {}
#[doc = "`write(|w| ..)` method takes [ever::W](ever::W) writer structure"]
impl crate::Writable for EVER {}
#[doc = "Event Enable Register"]
pub mod ever;
#[doc = "Glitch Filter Enable Register - Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gferc](gferc) module"]
pub type GFERC = crate::Reg<u32, _GFERC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GFERC;
#[doc = "`write(|w| ..)` method takes [gferc::W](gferc::W) writer structure"]
impl crate::Writable for GFERC {}
#[doc = "Glitch Filter Enable Register - Clear"]
pub mod gferc;
#[doc = "Glitch Filter Enable Register - Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gfers](gfers) module"]
pub type GFERS = crate::Reg<u32, _GFERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GFERS;
#[doc = "`write(|w| ..)` method takes [gfers::W](gfers::W) writer structure"]
impl crate::Writable for GFERS {}
#[doc = "Glitch Filter Enable Register - Set"]
pub mod gfers;
#[doc = "Glitch Filter Enable Register - Toggle\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gfert](gfert) module"]
pub type GFERT = crate::Reg<u32, _GFERT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GFERT;
#[doc = "`write(|w| ..)` method takes [gfert::W](gfert::W) writer structure"]
impl crate::Writable for GFERT {}
#[doc = "Glitch Filter Enable Register - Toggle"]
pub mod gfert;
#[doc = "Glitch Filter Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gfer](gfer) module"]
pub type GFER = crate::Reg<u32, _GFER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GFER;
#[doc = "`read()` method returns [gfer::R](gfer::R) reader structure"]
impl crate::Readable for GFER {}
#[doc = "`write(|w| ..)` method takes [gfer::W](gfer::W) writer structure"]
impl crate::Writable for GFER {}
#[doc = "Glitch Filter Enable Register"]
pub mod gfer;
#[doc = "GPIO Enable Register - Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gperc](gperc) module"]
pub type GPERC = crate::Reg<u32, _GPERC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPERC;
#[doc = "`write(|w| ..)` method takes [gperc::W](gperc::W) writer structure"]
impl crate::Writable for GPERC {}
#[doc = "GPIO Enable Register - Clear"]
pub mod gperc;
#[doc = "GPIO Enable Register - Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpers](gpers) module"]
pub type GPERS = crate::Reg<u32, _GPERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPERS;
#[doc = "`write(|w| ..)` method takes [gpers::W](gpers::W) writer structure"]
impl crate::Writable for GPERS {}
#[doc = "GPIO Enable Register - Set"]
pub mod gpers;
#[doc = "GPIO Enable Register - Toggle\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpert](gpert) module"]
pub type GPERT = crate::Reg<u32, _GPERT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPERT;
#[doc = "`write(|w| ..)` method takes [gpert::W](gpert::W) writer structure"]
impl crate::Writable for GPERT {}
#[doc = "GPIO Enable Register - Toggle"]
pub mod gpert;
#[doc = "GPIO Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gper](gper) module"]
pub type GPER = crate::Reg<u32, _GPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPER;
#[doc = "`read()` method returns [gper::R](gper::R) reader structure"]
impl crate::Readable for GPER {}
#[doc = "`write(|w| ..)` method takes [gper::W](gper::W) writer structure"]
impl crate::Writable for GPER {}
#[doc = "GPIO Enable Register"]
pub mod gper;
#[doc = "Interrupt Enable Register - Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ierc](ierc) module"]
pub type IERC = crate::Reg<u32, _IERC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IERC;
#[doc = "`write(|w| ..)` method takes [ierc::W](ierc::W) writer structure"]
impl crate::Writable for IERC {}
#[doc = "Interrupt Enable Register - Clear"]
pub mod ierc;
#[doc = "Interrupt Enable Register - Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iers](iers) module"]
pub type IERS = crate::Reg<u32, _IERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IERS;
#[doc = "`write(|w| ..)` method takes [iers::W](iers::W) writer structure"]
impl crate::Writable for IERS {}
#[doc = "Interrupt Enable Register - Set"]
pub mod iers;
#[doc = "Interrupt Enable Register - Toggle\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iert](iert) module"]
pub type IERT = crate::Reg<u32, _IERT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IERT;
#[doc = "`write(|w| ..)` method takes [iert::W](iert::W) writer structure"]
impl crate::Writable for IERT {}
#[doc = "Interrupt Enable Register - Toggle"]
pub mod iert;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Flag Register - Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifrc](ifrc) module"]
pub type IFRC = crate::Reg<u32, _IFRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFRC;
#[doc = "`write(|w| ..)` method takes [ifrc::W](ifrc::W) writer structure"]
impl crate::Writable for IFRC {}
#[doc = "Interrupt Flag Register - Clear"]
pub mod ifrc;
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifr](ifr) module"]
pub type IFR = crate::Reg<u32, _IFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFR;
#[doc = "`read()` method returns [ifr::R](ifr::R) reader structure"]
impl crate::Readable for IFR {}
#[doc = "Interrupt Flag Register"]
pub mod ifr;
#[doc = "Interrupt Mode Register 0 - Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr0c](imr0c) module"]
pub type IMR0C = crate::Reg<u32, _IMR0C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR0C;
#[doc = "`write(|w| ..)` method takes [imr0c::W](imr0c::W) writer structure"]
impl crate::Writable for IMR0C {}
#[doc = "Interrupt Mode Register 0 - Clear"]
pub mod imr0c;
#[doc = "Interrupt Mode Register 0 - Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr0s](imr0s) module"]
pub type IMR0S = crate::Reg<u32, _IMR0S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR0S;
#[doc = "`write(|w| ..)` method takes [imr0s::W](imr0s::W) writer structure"]
impl crate::Writable for IMR0S {}
#[doc = "Interrupt Mode Register 0 - Set"]
pub mod imr0s;
#[doc = "Interrupt Mode Register 0 - Toggle\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr0t](imr0t) module"]
pub type IMR0T = crate::Reg<u32, _IMR0T>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR0T;
#[doc = "`write(|w| ..)` method takes [imr0t::W](imr0t::W) writer structure"]
impl crate::Writable for IMR0T {}
#[doc = "Interrupt Mode Register 0 - Toggle"]
pub mod imr0t;
#[doc = "Interrupt Mode Register 1 - Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1c](imr1c) module"]
pub type IMR1C = crate::Reg<u32, _IMR1C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR1C;
#[doc = "`write(|w| ..)` method takes [imr1c::W](imr1c::W) writer structure"]
impl crate::Writable for IMR1C {}
#[doc = "Interrupt Mode Register 1 - Clear"]
pub mod imr1c;
#[doc = "Interrupt Mode Register 1 - Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1s](imr1s) module"]
pub type IMR1S = crate::Reg<u32, _IMR1S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR1S;
#[doc = "`write(|w| ..)` method takes [imr1s::W](imr1s::W) writer structure"]
impl crate::Writable for IMR1S {}
#[doc = "Interrupt Mode Register 1 - Set"]
pub mod imr1s;
#[doc = "Interrupt Mode Register 1 - Toggle\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1t](imr1t) module"]
pub type IMR1T = crate::Reg<u32, _IMR1T>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR1T;
#[doc = "`write(|w| ..)` method takes [imr1t::W](imr1t::W) writer structure"]
impl crate::Writable for IMR1T {}
#[doc = "Interrupt Mode Register 1 - Toggle"]
pub mod imr1t;
#[doc = "Interrupt Mode Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr0](imr0) module"]
pub type IMR0 = crate::Reg<u32, _IMR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR0;
#[doc = "`read()` method returns [imr0::R](imr0::R) reader structure"]
impl crate::Readable for IMR0 {}
#[doc = "`write(|w| ..)` method takes [imr0::W](imr0::W) writer structure"]
impl crate::Writable for IMR0 {}
#[doc = "Interrupt Mode Register 0"]
pub mod imr0;
#[doc = "Interrupt Mode Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1](imr1) module"]
pub type IMR1 = crate::Reg<u32, _IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR1;
#[doc = "`read()` method returns [imr1::R](imr1::R) reader structure"]
impl crate::Readable for IMR1 {}
#[doc = "`write(|w| ..)` method takes [imr1::W](imr1::W) writer structure"]
impl crate::Writable for IMR1 {}
#[doc = "Interrupt Mode Register 1"]
pub mod imr1;
#[doc = "Lock Register - Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockc](lockc) module"]
pub type LOCKC = crate::Reg<u32, _LOCKC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKC;
#[doc = "`write(|w| ..)` method takes [lockc::W](lockc::W) writer structure"]
impl crate::Writable for LOCKC {}
#[doc = "Lock Register - Clear"]
pub mod lockc;
#[doc = "Lock Register - Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [locks](locks) module"]
pub type LOCKS = crate::Reg<u32, _LOCKS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKS;
#[doc = "`write(|w| ..)` method takes [locks::W](locks::W) writer structure"]
impl crate::Writable for LOCKS {}
#[doc = "Lock Register - Set"]
pub mod locks;
#[doc = "Lock Register - Toggle\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockt](lockt) module"]
pub type LOCKT = crate::Reg<u32, _LOCKT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKT;
#[doc = "`write(|w| ..)` method takes [lockt::W](lockt::W) writer structure"]
impl crate::Writable for LOCKT {}
#[doc = "Lock Register - Toggle"]
pub mod lockt;
#[doc = "Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Lock Register"]
pub mod lock;
#[doc = "Output Driving Capability Register 0 - Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odcr0c](odcr0c) module"]
pub type ODCR0C = crate::Reg<u32, _ODCR0C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODCR0C;
#[doc = "`read()` method returns [odcr0c::R](odcr0c::R) reader structure"]
impl crate::Readable for ODCR0C {}
#[doc = "`write(|w| ..)` method takes [odcr0c::W](odcr0c::W) writer structure"]
impl crate::Writable for ODCR0C {}
#[doc = "Output Driving Capability Register 0 - Clear"]
pub mod odcr0c;
#[doc = "Output Driving Capability Register 0 - Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odcr0s](odcr0s) module"]
pub type ODCR0S = crate::Reg<u32, _ODCR0S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODCR0S;
#[doc = "`read()` method returns [odcr0s::R](odcr0s::R) reader structure"]
impl crate::Readable for ODCR0S {}
#[doc = "`write(|w| ..)` method takes [odcr0s::W](odcr0s::W) writer structure"]
impl crate::Writable for ODCR0S {}
#[doc = "Output Driving Capability Register 0 - Set"]
pub mod odcr0s;
#[doc = "Output Driving Capability Register 0 - Toggle\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odcr0t](odcr0t) module"]
pub type ODCR0T = crate::Reg<u32, _ODCR0T>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODCR0T;
#[doc = "`read()` method returns [odcr0t::R](odcr0t::R) reader structure"]
impl crate::Readable for ODCR0T {}
#[doc = "`write(|w| ..)` method takes [odcr0t::W](odcr0t::W) writer structure"]
impl crate::Writable for ODCR0T {}
#[doc = "Output Driving Capability Register 0 - Toggle"]
pub mod odcr0t;
#[doc = "Output Driving Capability Register 1 - Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odcr1c](odcr1c) module"]
pub type ODCR1C = crate::Reg<u32, _ODCR1C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODCR1C;
#[doc = "`read()` method returns [odcr1c::R](odcr1c::R) reader structure"]
impl crate::Readable for ODCR1C {}
#[doc = "`write(|w| ..)` method takes [odcr1c::W](odcr1c::W) writer structure"]
impl crate::Writable for ODCR1C {}
#[doc = "Output Driving Capability Register 1 - Clear"]
pub mod odcr1c;
#[doc = "Output Driving Capability Register 1 - Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odcr1s](odcr1s) module"]
pub type ODCR1S = crate::Reg<u32, _ODCR1S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODCR1S;
#[doc = "`read()` method returns [odcr1s::R](odcr1s::R) reader structure"]
impl crate::Readable for ODCR1S {}
#[doc = "`write(|w| ..)` method takes [odcr1s::W](odcr1s::W) writer structure"]
impl crate::Writable for ODCR1S {}
#[doc = "Output Driving Capability Register 1 - Set"]
pub mod odcr1s;
#[doc = "Output Driving Capability Register 1 - Toggle\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odcr1t](odcr1t) module"]
pub type ODCR1T = crate::Reg<u32, _ODCR1T>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODCR1T;
#[doc = "`read()` method returns [odcr1t::R](odcr1t::R) reader structure"]
impl crate::Readable for ODCR1T {}
#[doc = "`write(|w| ..)` method takes [odcr1t::W](odcr1t::W) writer structure"]
impl crate::Writable for ODCR1T {}
#[doc = "Output Driving Capability Register 1 - Toggle"]
pub mod odcr1t;
#[doc = "Output Driving Capability Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odcr0](odcr0) module"]
pub type ODCR0 = crate::Reg<u32, _ODCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODCR0;
#[doc = "`read()` method returns [odcr0::R](odcr0::R) reader structure"]
impl crate::Readable for ODCR0 {}
#[doc = "`write(|w| ..)` method takes [odcr0::W](odcr0::W) writer structure"]
impl crate::Writable for ODCR0 {}
#[doc = "Output Driving Capability Register 0"]
pub mod odcr0;
#[doc = "Output Driving Capability Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odcr1](odcr1) module"]
pub type ODCR1 = crate::Reg<u32, _ODCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODCR1;
#[doc = "`read()` method returns [odcr1::R](odcr1::R) reader structure"]
impl crate::Readable for ODCR1 {}
#[doc = "`write(|w| ..)` method takes [odcr1::W](odcr1::W) writer structure"]
impl crate::Writable for ODCR1 {}
#[doc = "Output Driving Capability Register 1"]
pub mod odcr1;
#[doc = "Output Driver Enable Register - Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oderc](oderc) module"]
pub type ODERC = crate::Reg<u32, _ODERC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODERC;
#[doc = "`write(|w| ..)` method takes [oderc::W](oderc::W) writer structure"]
impl crate::Writable for ODERC {}
#[doc = "Output Driver Enable Register - Clear"]
pub mod oderc;
#[doc = "Output Driver Enable Register - Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oders](oders) module"]
pub type ODERS = crate::Reg<u32, _ODERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODERS;
#[doc = "`write(|w| ..)` method takes [oders::W](oders::W) writer structure"]
impl crate::Writable for ODERS {}
#[doc = "Output Driver Enable Register - Set"]
pub mod oders;
#[doc = "Output Driver Enable Register - Toggle\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odert](odert) module"]
pub type ODERT = crate::Reg<u32, _ODERT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODERT;
#[doc = "`write(|w| ..)` method takes [odert::W](odert::W) writer structure"]
impl crate::Writable for ODERT {}
#[doc = "Output Driver Enable Register - Toggle"]
pub mod odert;
#[doc = "Output Driver Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oder](oder) module"]
pub type ODER = crate::Reg<u32, _ODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODER;
#[doc = "`read()` method returns [oder::R](oder::R) reader structure"]
impl crate::Readable for ODER {}
#[doc = "`write(|w| ..)` method takes [oder::W](oder::W) writer structure"]
impl crate::Writable for ODER {}
#[doc = "Output Driver Enable Register"]
pub mod oder;
#[doc = "Open Drain Mode Register - Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odmerc](odmerc) module"]
pub type ODMERC = crate::Reg<u32, _ODMERC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODMERC;
#[doc = "`write(|w| ..)` method takes [odmerc::W](odmerc::W) writer structure"]
impl crate::Writable for ODMERC {}
#[doc = "Open Drain Mode Register - Clear"]
pub mod odmerc;
#[doc = "Open Drain Mode Register - Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odmers](odmers) module"]
pub type ODMERS = crate::Reg<u32, _ODMERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODMERS;
#[doc = "`write(|w| ..)` method takes [odmers::W](odmers::W) writer structure"]
impl crate::Writable for ODMERS {}
#[doc = "Open Drain Mode Register - Set"]
pub mod odmers;
#[doc = "Open Drain Mode Register - Toggle\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odmert](odmert) module"]
pub type ODMERT = crate::Reg<u32, _ODMERT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODMERT;
#[doc = "`write(|w| ..)` method takes [odmert::W](odmert::W) writer structure"]
impl crate::Writable for ODMERT {}
#[doc = "Open Drain Mode Register - Toggle"]
pub mod odmert;
#[doc = "Open Drain Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odmer](odmer) module"]
pub type ODMER = crate::Reg<u32, _ODMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODMER;
#[doc = "`read()` method returns [odmer::R](odmer::R) reader structure"]
impl crate::Readable for ODMER {}
#[doc = "`write(|w| ..)` method takes [odmer::W](odmer::W) writer structure"]
impl crate::Writable for ODMER {}
#[doc = "Open Drain Mode Register"]
pub mod odmer;
#[doc = "Output Slew Rate Register 0 - Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osrr0c](osrr0c) module"]
pub type OSRR0C = crate::Reg<u32, _OSRR0C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSRR0C;
#[doc = "`read()` method returns [osrr0c::R](osrr0c::R) reader structure"]
impl crate::Readable for OSRR0C {}
#[doc = "`write(|w| ..)` method takes [osrr0c::W](osrr0c::W) writer structure"]
impl crate::Writable for OSRR0C {}
#[doc = "Output Slew Rate Register 0 - Clear"]
pub mod osrr0c;
#[doc = "Output Slew Rate Register 0 - Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osrr0s](osrr0s) module"]
pub type OSRR0S = crate::Reg<u32, _OSRR0S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSRR0S;
#[doc = "`read()` method returns [osrr0s::R](osrr0s::R) reader structure"]
impl crate::Readable for OSRR0S {}
#[doc = "`write(|w| ..)` method takes [osrr0s::W](osrr0s::W) writer structure"]
impl crate::Writable for OSRR0S {}
#[doc = "Output Slew Rate Register 0 - Set"]
pub mod osrr0s;
#[doc = "Output Slew Rate Register 0 - Toggle\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osrr0t](osrr0t) module"]
pub type OSRR0T = crate::Reg<u32, _OSRR0T>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSRR0T;
#[doc = "`read()` method returns [osrr0t::R](osrr0t::R) reader structure"]
impl crate::Readable for OSRR0T {}
#[doc = "`write(|w| ..)` method takes [osrr0t::W](osrr0t::W) writer structure"]
impl crate::Writable for OSRR0T {}
#[doc = "Output Slew Rate Register 0 - Toggle"]
pub mod osrr0t;
#[doc = "Output Slew Rate Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osrr0](osrr0) module"]
pub type OSRR0 = crate::Reg<u32, _OSRR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSRR0;
#[doc = "`read()` method returns [osrr0::R](osrr0::R) reader structure"]
impl crate::Readable for OSRR0 {}
#[doc = "`write(|w| ..)` method takes [osrr0::W](osrr0::W) writer structure"]
impl crate::Writable for OSRR0 {}
#[doc = "Output Slew Rate Register 0"]
pub mod osrr0;
#[doc = "Output Value Register - Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovrc](ovrc) module"]
pub type OVRC = crate::Reg<u32, _OVRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OVRC;
#[doc = "`write(|w| ..)` method takes [ovrc::W](ovrc::W) writer structure"]
impl crate::Writable for OVRC {}
#[doc = "Output Value Register - Clear"]
pub mod ovrc;
#[doc = "Output Value Register - Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovrs](ovrs) module"]
pub type OVRS = crate::Reg<u32, _OVRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OVRS;
#[doc = "`write(|w| ..)` method takes [ovrs::W](ovrs::W) writer structure"]
impl crate::Writable for OVRS {}
#[doc = "Output Value Register - Set"]
pub mod ovrs;
#[doc = "Output Value Register - Toggle\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovrt](ovrt) module"]
pub type OVRT = crate::Reg<u32, _OVRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OVRT;
#[doc = "`write(|w| ..)` method takes [ovrt::W](ovrt::W) writer structure"]
impl crate::Writable for OVRT {}
#[doc = "Output Value Register - Toggle"]
pub mod ovrt;
#[doc = "Output Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovr](ovr) module"]
pub type OVR = crate::Reg<u32, _OVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OVR;
#[doc = "`read()` method returns [ovr::R](ovr::R) reader structure"]
impl crate::Readable for OVR {}
#[doc = "`write(|w| ..)` method takes [ovr::W](ovr::W) writer structure"]
impl crate::Writable for OVR {}
#[doc = "Output Value Register"]
pub mod ovr;
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [parameter](parameter) module"]
pub type PARAMETER = crate::Reg<u32, _PARAMETER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PARAMETER;
#[doc = "`read()` method returns [parameter::R](parameter::R) reader structure"]
impl crate::Readable for PARAMETER {}
#[doc = "Parameter Register"]
pub mod parameter;
#[doc = "Pull-down Enable Register - Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pderc](pderc) module"]
pub type PDERC = crate::Reg<u32, _PDERC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDERC;
#[doc = "`write(|w| ..)` method takes [pderc::W](pderc::W) writer structure"]
impl crate::Writable for PDERC {}
#[doc = "Pull-down Enable Register - Clear"]
pub mod pderc;
#[doc = "Pull-down Enable Register - Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pders](pders) module"]
pub type PDERS = crate::Reg<u32, _PDERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDERS;
#[doc = "`write(|w| ..)` method takes [pders::W](pders::W) writer structure"]
impl crate::Writable for PDERS {}
#[doc = "Pull-down Enable Register - Set"]
pub mod pders;
#[doc = "Pull-down Enable Register - Toggle\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdert](pdert) module"]
pub type PDERT = crate::Reg<u32, _PDERT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDERT;
#[doc = "`write(|w| ..)` method takes [pdert::W](pdert::W) writer structure"]
impl crate::Writable for PDERT {}
#[doc = "Pull-down Enable Register - Toggle"]
pub mod pdert;
#[doc = "Pull-down Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pder](pder) module"]
pub type PDER = crate::Reg<u32, _PDER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDER;
#[doc = "`read()` method returns [pder::R](pder::R) reader structure"]
impl crate::Readable for PDER {}
#[doc = "`write(|w| ..)` method takes [pder::W](pder::W) writer structure"]
impl crate::Writable for PDER {}
#[doc = "Pull-down Enable Register"]
pub mod pder;
#[doc = "Peripheral Mux Register 0 - Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmr0c](pmr0c) module"]
pub type PMR0C = crate::Reg<u32, _PMR0C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMR0C;
#[doc = "`write(|w| ..)` method takes [pmr0c::W](pmr0c::W) writer structure"]
impl crate::Writable for PMR0C {}
#[doc = "Peripheral Mux Register 0 - Clear"]
pub mod pmr0c;
#[doc = "Peripheral Mux Register 0 - Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmr0s](pmr0s) module"]
pub type PMR0S = crate::Reg<u32, _PMR0S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMR0S;
#[doc = "`write(|w| ..)` method takes [pmr0s::W](pmr0s::W) writer structure"]
impl crate::Writable for PMR0S {}
#[doc = "Peripheral Mux Register 0 - Set"]
pub mod pmr0s;
#[doc = "Peripheral Mux Register 0 - Toggle\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmr0t](pmr0t) module"]
pub type PMR0T = crate::Reg<u32, _PMR0T>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMR0T;
#[doc = "`write(|w| ..)` method takes [pmr0t::W](pmr0t::W) writer structure"]
impl crate::Writable for PMR0T {}
#[doc = "Peripheral Mux Register 0 - Toggle"]
pub mod pmr0t;
#[doc = "Peripheral Mux Register 1 - Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmr1c](pmr1c) module"]
pub type PMR1C = crate::Reg<u32, _PMR1C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMR1C;
#[doc = "`write(|w| ..)` method takes [pmr1c::W](pmr1c::W) writer structure"]
impl crate::Writable for PMR1C {}
#[doc = "Peripheral Mux Register 1 - Clear"]
pub mod pmr1c;
#[doc = "Peripheral Mux Register 1 - Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmr1s](pmr1s) module"]
pub type PMR1S = crate::Reg<u32, _PMR1S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMR1S;
#[doc = "`write(|w| ..)` method takes [pmr1s::W](pmr1s::W) writer structure"]
impl crate::Writable for PMR1S {}
#[doc = "Peripheral Mux Register 1 - Set"]
pub mod pmr1s;
#[doc = "Peripheral Mux Register 1 - Toggle\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmr1t](pmr1t) module"]
pub type PMR1T = crate::Reg<u32, _PMR1T>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMR1T;
#[doc = "`write(|w| ..)` method takes [pmr1t::W](pmr1t::W) writer structure"]
impl crate::Writable for PMR1T {}
#[doc = "Peripheral Mux Register 1 - Toggle"]
pub mod pmr1t;
#[doc = "Peripheral Mux Register 2 - Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmr2c](pmr2c) module"]
pub type PMR2C = crate::Reg<u32, _PMR2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMR2C;
#[doc = "`write(|w| ..)` method takes [pmr2c::W](pmr2c::W) writer structure"]
impl crate::Writable for PMR2C {}
#[doc = "Peripheral Mux Register 2 - Clear"]
pub mod pmr2c;
#[doc = "Peripheral Mux Register 2 - Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmr2s](pmr2s) module"]
pub type PMR2S = crate::Reg<u32, _PMR2S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMR2S;
#[doc = "`write(|w| ..)` method takes [pmr2s::W](pmr2s::W) writer structure"]
impl crate::Writable for PMR2S {}
#[doc = "Peripheral Mux Register 2 - Set"]
pub mod pmr2s;
#[doc = "Peripheral Mux Register 2 - Toggle\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmr2t](pmr2t) module"]
pub type PMR2T = crate::Reg<u32, _PMR2T>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMR2T;
#[doc = "`write(|w| ..)` method takes [pmr2t::W](pmr2t::W) writer structure"]
impl crate::Writable for PMR2T {}
#[doc = "Peripheral Mux Register 2 - Toggle"]
pub mod pmr2t;
#[doc = "Peripheral Mux Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmr0](pmr0) module"]
pub type PMR0 = crate::Reg<u32, _PMR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMR0;
#[doc = "`read()` method returns [pmr0::R](pmr0::R) reader structure"]
impl crate::Readable for PMR0 {}
#[doc = "`write(|w| ..)` method takes [pmr0::W](pmr0::W) writer structure"]
impl crate::Writable for PMR0 {}
#[doc = "Peripheral Mux Register 0"]
pub mod pmr0;
#[doc = "Peripheral Mux Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmr1](pmr1) module"]
pub type PMR1 = crate::Reg<u32, _PMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMR1;
#[doc = "`read()` method returns [pmr1::R](pmr1::R) reader structure"]
impl crate::Readable for PMR1 {}
#[doc = "`write(|w| ..)` method takes [pmr1::W](pmr1::W) writer structure"]
impl crate::Writable for PMR1 {}
#[doc = "Peripheral Mux Register 1"]
pub mod pmr1;
#[doc = "Peripheral Mux Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmr2](pmr2) module"]
pub type PMR2 = crate::Reg<u32, _PMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMR2;
#[doc = "`read()` method returns [pmr2::R](pmr2::R) reader structure"]
impl crate::Readable for PMR2 {}
#[doc = "`write(|w| ..)` method takes [pmr2::W](pmr2::W) writer structure"]
impl crate::Writable for PMR2 {}
#[doc = "Peripheral Mux Register 2"]
pub mod pmr2;
#[doc = "Pull-up Enable Register - Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [puerc](puerc) module"]
pub type PUERC = crate::Reg<u32, _PUERC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUERC;
#[doc = "`write(|w| ..)` method takes [puerc::W](puerc::W) writer structure"]
impl crate::Writable for PUERC {}
#[doc = "Pull-up Enable Register - Clear"]
pub mod puerc;
#[doc = "Pull-up Enable Register - Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [puers](puers) module"]
pub type PUERS = crate::Reg<u32, _PUERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUERS;
#[doc = "`write(|w| ..)` method takes [puers::W](puers::W) writer structure"]
impl crate::Writable for PUERS {}
#[doc = "Pull-up Enable Register - Set"]
pub mod puers;
#[doc = "Pull-up Enable Register - Toggle\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [puert](puert) module"]
pub type PUERT = crate::Reg<u32, _PUERT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUERT;
#[doc = "`write(|w| ..)` method takes [puert::W](puert::W) writer structure"]
impl crate::Writable for PUERT {}
#[doc = "Pull-up Enable Register - Toggle"]
pub mod puert;
#[doc = "Pull-up Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [puer](puer) module"]
pub type PUER = crate::Reg<u32, _PUER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUER;
#[doc = "`read()` method returns [puer::R](puer::R) reader structure"]
impl crate::Readable for PUER {}
#[doc = "`write(|w| ..)` method takes [puer::W](puer::W) writer structure"]
impl crate::Writable for PUER {}
#[doc = "Pull-up Enable Register"]
pub mod puer;
#[doc = "Pin Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pvr](pvr) module"]
pub type PVR = crate::Reg<u32, _PVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PVR;
#[doc = "`read()` method returns [pvr::R](pvr::R) reader structure"]
impl crate::Readable for PVR {}
#[doc = "Pin Value Register"]
pub mod pvr;
#[doc = "Schmitt Trigger Enable Register - Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sterc](sterc) module"]
pub type STERC = crate::Reg<u32, _STERC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STERC;
#[doc = "`read()` method returns [sterc::R](sterc::R) reader structure"]
impl crate::Readable for STERC {}
#[doc = "`write(|w| ..)` method takes [sterc::W](sterc::W) writer structure"]
impl crate::Writable for STERC {}
#[doc = "Schmitt Trigger Enable Register - Clear"]
pub mod sterc;
#[doc = "Schmitt Trigger Enable Register - Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sters](sters) module"]
pub type STERS = crate::Reg<u32, _STERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STERS;
#[doc = "`read()` method returns [sters::R](sters::R) reader structure"]
impl crate::Readable for STERS {}
#[doc = "`write(|w| ..)` method takes [sters::W](sters::W) writer structure"]
impl crate::Writable for STERS {}
#[doc = "Schmitt Trigger Enable Register - Set"]
pub mod sters;
#[doc = "Schmitt Trigger Enable Register - Toggle\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stert](stert) module"]
pub type STERT = crate::Reg<u32, _STERT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STERT;
#[doc = "`read()` method returns [stert::R](stert::R) reader structure"]
impl crate::Readable for STERT {}
#[doc = "`write(|w| ..)` method takes [stert::W](stert::W) writer structure"]
impl crate::Writable for STERT {}
#[doc = "Schmitt Trigger Enable Register - Toggle"]
pub mod stert;
#[doc = "Schmitt Trigger Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ster](ster) module"]
pub type STER = crate::Reg<u32, _STER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STER;
#[doc = "`read()` method returns [ster::R](ster::R) reader structure"]
impl crate::Readable for STER {}
#[doc = "`write(|w| ..)` method takes [ster::W](ster::W) writer structure"]
impl crate::Writable for STER {}
#[doc = "Schmitt Trigger Enable Register"]
pub mod ster;
#[doc = "Unlock Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unlock](unlock) module"]
pub type UNLOCK = crate::Reg<u32, _UNLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNLOCK;
#[doc = "`write(|w| ..)` method takes [unlock::W](unlock::W) writer structure"]
impl crate::Writable for UNLOCK {}
#[doc = "Unlock Register"]
pub mod unlock;
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "Version Register"]
pub mod version;
