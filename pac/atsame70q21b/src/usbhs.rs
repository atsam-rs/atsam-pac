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
    #[doc = "0x100 - Device Endpoint Configuration Register"]
    pub deveptcfg: [DEVEPTCFG; 10],
    _reserved10: [u8; 8usize],
    _reserved_10_deveptisr: [u8; 40usize],
    _reserved11: [u8; 8usize],
    _reserved_11_devepticr: [u8; 40usize],
    _reserved12: [u8; 8usize],
    _reserved_12_deveptifr: [u8; 40usize],
    _reserved13: [u8; 8usize],
    _reserved_13_deveptimr: [u8; 40usize],
    _reserved14: [u8; 8usize],
    _reserved_14_deveptier: [u8; 40usize],
    _reserved15: [u8; 8usize],
    _reserved_15_deveptidr: [u8; 40usize],
    _reserved16: [u8; 200usize],
    #[doc = "0x310 - Device DMA Channel Next Descriptor Address Register"]
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
    _reserved_29_hstpipcfg: [u8; 40usize],
    _reserved30: [u8; 8usize],
    _reserved_30_hstpipisr: [u8; 40usize],
    _reserved31: [u8; 8usize],
    _reserved_31_hstpipicr: [u8; 40usize],
    _reserved32: [u8; 8usize],
    _reserved_32_hstpipifr: [u8; 40usize],
    _reserved33: [u8; 8usize],
    _reserved_33_hstpipimr: [u8; 40usize],
    _reserved34: [u8; 8usize],
    _reserved_34_hstpipier: [u8; 40usize],
    _reserved35: [u8; 8usize],
    _reserved_35_hstpipidr: [u8; 40usize],
    _reserved36: [u8; 8usize],
    #[doc = "0x650 - Host Pipe IN Request Register"]
    pub hstpipinrq: [HSTPIPINRQ; 10],
    _reserved37: [u8; 8usize],
    #[doc = "0x680 - Host Pipe Error Register"]
    pub hstpiperr: [HSTPIPERR; 10],
    _reserved38: [u8; 104usize],
    #[doc = "0x710 - Host DMA Channel Next Descriptor Address Register"]
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
impl RegisterBlock {
    #[doc = "0x130 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn deveptisr_intrpt_mode(&self) -> &[DEVEPTISR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(304usize)
                as *const [DEVEPTISR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x130 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn deveptisr_intrpt_mode_mut(&self) -> &mut [DEVEPTISR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(304usize)
                as *mut [DEVEPTISR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x130 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn deveptisr_blk_mode(&self) -> &[DEVEPTISR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(304usize)
                as *const [DEVEPTISR_BLK_MODE; 10])
        }
    }
    #[doc = "0x130 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn deveptisr_blk_mode_mut(&self) -> &mut [DEVEPTISR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(304usize)
                as *mut [DEVEPTISR_BLK_MODE; 10])
        }
    }
    #[doc = "0x130 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn deveptisr_iso_mode(&self) -> &[DEVEPTISR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(304usize)
                as *const [DEVEPTISR_ISO_MODE; 10])
        }
    }
    #[doc = "0x130 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn deveptisr_iso_mode_mut(&self) -> &mut [DEVEPTISR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(304usize)
                as *mut [DEVEPTISR_ISO_MODE; 10])
        }
    }
    #[doc = "0x130 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn deveptisr_ctrl_mode(&self) -> &[DEVEPTISR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(304usize)
                as *const [DEVEPTISR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x130 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn deveptisr_ctrl_mode_mut(&self) -> &mut [DEVEPTISR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(304usize)
                as *mut [DEVEPTISR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x160 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn devepticr_intrpt_mode(&self) -> &[DEVEPTICR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(352usize)
                as *const [DEVEPTICR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x160 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn devepticr_intrpt_mode_mut(&self) -> &mut [DEVEPTICR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(352usize)
                as *mut [DEVEPTICR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x160 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn devepticr_blk_mode(&self) -> &[DEVEPTICR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(352usize)
                as *const [DEVEPTICR_BLK_MODE; 10])
        }
    }
    #[doc = "0x160 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn devepticr_blk_mode_mut(&self) -> &mut [DEVEPTICR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(352usize)
                as *mut [DEVEPTICR_BLK_MODE; 10])
        }
    }
    #[doc = "0x160 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn devepticr_iso_mode(&self) -> &[DEVEPTICR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(352usize)
                as *const [DEVEPTICR_ISO_MODE; 10])
        }
    }
    #[doc = "0x160 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn devepticr_iso_mode_mut(&self) -> &mut [DEVEPTICR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(352usize)
                as *mut [DEVEPTICR_ISO_MODE; 10])
        }
    }
    #[doc = "0x160 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn devepticr_ctrl_mode(&self) -> &[DEVEPTICR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(352usize)
                as *const [DEVEPTICR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x160 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn devepticr_ctrl_mode_mut(&self) -> &mut [DEVEPTICR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(352usize)
                as *mut [DEVEPTICR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x190 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn deveptifr_intrpt_mode(&self) -> &[DEVEPTIFR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(400usize)
                as *const [DEVEPTIFR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x190 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn deveptifr_intrpt_mode_mut(&self) -> &mut [DEVEPTIFR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(400usize)
                as *mut [DEVEPTIFR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x190 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn deveptifr_blk_mode(&self) -> &[DEVEPTIFR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(400usize)
                as *const [DEVEPTIFR_BLK_MODE; 10])
        }
    }
    #[doc = "0x190 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn deveptifr_blk_mode_mut(&self) -> &mut [DEVEPTIFR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(400usize)
                as *mut [DEVEPTIFR_BLK_MODE; 10])
        }
    }
    #[doc = "0x190 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn deveptifr_iso_mode(&self) -> &[DEVEPTIFR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(400usize)
                as *const [DEVEPTIFR_ISO_MODE; 10])
        }
    }
    #[doc = "0x190 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn deveptifr_iso_mode_mut(&self) -> &mut [DEVEPTIFR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(400usize)
                as *mut [DEVEPTIFR_ISO_MODE; 10])
        }
    }
    #[doc = "0x190 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn deveptifr_ctrl_mode(&self) -> &[DEVEPTIFR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(400usize)
                as *const [DEVEPTIFR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x190 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn deveptifr_ctrl_mode_mut(&self) -> &mut [DEVEPTIFR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(400usize)
                as *mut [DEVEPTIFR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x1c0 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn deveptimr_intrpt_mode(&self) -> &[DEVEPTIMR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(448usize)
                as *const [DEVEPTIMR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x1c0 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn deveptimr_intrpt_mode_mut(&self) -> &mut [DEVEPTIMR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(448usize)
                as *mut [DEVEPTIMR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x1c0 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn deveptimr_blk_mode(&self) -> &[DEVEPTIMR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(448usize)
                as *const [DEVEPTIMR_BLK_MODE; 10])
        }
    }
    #[doc = "0x1c0 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn deveptimr_blk_mode_mut(&self) -> &mut [DEVEPTIMR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(448usize)
                as *mut [DEVEPTIMR_BLK_MODE; 10])
        }
    }
    #[doc = "0x1c0 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn deveptimr_iso_mode(&self) -> &[DEVEPTIMR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(448usize)
                as *const [DEVEPTIMR_ISO_MODE; 10])
        }
    }
    #[doc = "0x1c0 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn deveptimr_iso_mode_mut(&self) -> &mut [DEVEPTIMR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(448usize)
                as *mut [DEVEPTIMR_ISO_MODE; 10])
        }
    }
    #[doc = "0x1c0 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn deveptimr_ctrl_mode(&self) -> &[DEVEPTIMR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(448usize)
                as *const [DEVEPTIMR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x1c0 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn deveptimr_ctrl_mode_mut(&self) -> &mut [DEVEPTIMR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(448usize)
                as *mut [DEVEPTIMR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x1f0 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn deveptier_intrpt_mode(&self) -> &[DEVEPTIER_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(496usize)
                as *const [DEVEPTIER_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x1f0 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn deveptier_intrpt_mode_mut(&self) -> &mut [DEVEPTIER_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(496usize)
                as *mut [DEVEPTIER_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x1f0 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn deveptier_blk_mode(&self) -> &[DEVEPTIER_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(496usize)
                as *const [DEVEPTIER_BLK_MODE; 10])
        }
    }
    #[doc = "0x1f0 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn deveptier_blk_mode_mut(&self) -> &mut [DEVEPTIER_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(496usize)
                as *mut [DEVEPTIER_BLK_MODE; 10])
        }
    }
    #[doc = "0x1f0 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn deveptier_iso_mode(&self) -> &[DEVEPTIER_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(496usize)
                as *const [DEVEPTIER_ISO_MODE; 10])
        }
    }
    #[doc = "0x1f0 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn deveptier_iso_mode_mut(&self) -> &mut [DEVEPTIER_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(496usize)
                as *mut [DEVEPTIER_ISO_MODE; 10])
        }
    }
    #[doc = "0x1f0 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn deveptier_ctrl_mode(&self) -> &[DEVEPTIER_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(496usize)
                as *const [DEVEPTIER_CTRL_MODE; 10])
        }
    }
    #[doc = "0x1f0 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn deveptier_ctrl_mode_mut(&self) -> &mut [DEVEPTIER_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(496usize)
                as *mut [DEVEPTIER_CTRL_MODE; 10])
        }
    }
    #[doc = "0x220 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn deveptidr_intrpt_mode(&self) -> &[DEVEPTIDR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(544usize)
                as *const [DEVEPTIDR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x220 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn deveptidr_intrpt_mode_mut(&self) -> &mut [DEVEPTIDR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(544usize)
                as *mut [DEVEPTIDR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x220 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn deveptidr_blk_mode(&self) -> &[DEVEPTIDR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(544usize)
                as *const [DEVEPTIDR_BLK_MODE; 10])
        }
    }
    #[doc = "0x220 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn deveptidr_blk_mode_mut(&self) -> &mut [DEVEPTIDR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(544usize)
                as *mut [DEVEPTIDR_BLK_MODE; 10])
        }
    }
    #[doc = "0x220 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn deveptidr_iso_mode(&self) -> &[DEVEPTIDR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(544usize)
                as *const [DEVEPTIDR_ISO_MODE; 10])
        }
    }
    #[doc = "0x220 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn deveptidr_iso_mode_mut(&self) -> &mut [DEVEPTIDR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(544usize)
                as *mut [DEVEPTIDR_ISO_MODE; 10])
        }
    }
    #[doc = "0x220 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn deveptidr_ctrl_mode(&self) -> &[DEVEPTIDR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(544usize)
                as *const [DEVEPTIDR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x220 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn deveptidr_ctrl_mode_mut(&self) -> &mut [DEVEPTIDR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(544usize)
                as *mut [DEVEPTIDR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x500 - Host Pipe Configuration Register"]
    #[inline(always)]
    pub fn hstpipcfg_ctrl_bulk_mode(&self) -> &[HSTPIPCFG_CTRL_BULK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1280usize)
                as *const [HSTPIPCFG_CTRL_BULK_MODE; 10])
        }
    }
    #[doc = "0x500 - Host Pipe Configuration Register"]
    #[inline(always)]
    pub fn hstpipcfg_ctrl_bulk_mode_mut(&self) -> &mut [HSTPIPCFG_CTRL_BULK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1280usize)
                as *mut [HSTPIPCFG_CTRL_BULK_MODE; 10])
        }
    }
    #[doc = "0x500 - Host Pipe Configuration Register"]
    #[inline(always)]
    pub fn hstpipcfg(&self) -> &[HSTPIPCFG; 10] {
        unsafe { &*(((self as *const Self) as *const u8).add(1280usize) as *const [HSTPIPCFG; 10]) }
    }
    #[doc = "0x500 - Host Pipe Configuration Register"]
    #[inline(always)]
    pub fn hstpipcfg_mut(&self) -> &mut [HSTPIPCFG; 10] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1280usize) as *mut [HSTPIPCFG; 10]) }
    }
    #[doc = "0x530 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn hstpipisr_intrpt_mode(&self) -> &[HSTPIPISR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize)
                as *const [HSTPIPISR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x530 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn hstpipisr_intrpt_mode_mut(&self) -> &mut [HSTPIPISR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1328usize)
                as *mut [HSTPIPISR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x530 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn hstpipisr_blk_mode(&self) -> &[HSTPIPISR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize)
                as *const [HSTPIPISR_BLK_MODE; 10])
        }
    }
    #[doc = "0x530 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn hstpipisr_blk_mode_mut(&self) -> &mut [HSTPIPISR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1328usize)
                as *mut [HSTPIPISR_BLK_MODE; 10])
        }
    }
    #[doc = "0x530 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn hstpipisr_iso_mode(&self) -> &[HSTPIPISR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize)
                as *const [HSTPIPISR_ISO_MODE; 10])
        }
    }
    #[doc = "0x530 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn hstpipisr_iso_mode_mut(&self) -> &mut [HSTPIPISR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1328usize)
                as *mut [HSTPIPISR_ISO_MODE; 10])
        }
    }
    #[doc = "0x530 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn hstpipisr_ctrl_mode(&self) -> &[HSTPIPISR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize)
                as *const [HSTPIPISR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x530 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn hstpipisr_ctrl_mode_mut(&self) -> &mut [HSTPIPISR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1328usize)
                as *mut [HSTPIPISR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn hstpipicr_intrpt_mode(&self) -> &[HSTPIPICR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const [HSTPIPICR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn hstpipicr_intrpt_mode_mut(&self) -> &mut [HSTPIPICR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1376usize)
                as *mut [HSTPIPICR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn hstpipicr_blk_mode(&self) -> &[HSTPIPICR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const [HSTPIPICR_BLK_MODE; 10])
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn hstpipicr_blk_mode_mut(&self) -> &mut [HSTPIPICR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1376usize)
                as *mut [HSTPIPICR_BLK_MODE; 10])
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn hstpipicr_iso_mode(&self) -> &[HSTPIPICR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const [HSTPIPICR_ISO_MODE; 10])
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn hstpipicr_iso_mode_mut(&self) -> &mut [HSTPIPICR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1376usize)
                as *mut [HSTPIPICR_ISO_MODE; 10])
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn hstpipicr_ctrl_mode(&self) -> &[HSTPIPICR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const [HSTPIPICR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn hstpipicr_ctrl_mode_mut(&self) -> &mut [HSTPIPICR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1376usize)
                as *mut [HSTPIPICR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x590 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn hstpipifr_intrpt_mode(&self) -> &[HSTPIPIFR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const [HSTPIPIFR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x590 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn hstpipifr_intrpt_mode_mut(&self) -> &mut [HSTPIPIFR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1424usize)
                as *mut [HSTPIPIFR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x590 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn hstpipifr_blk_mode(&self) -> &[HSTPIPIFR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const [HSTPIPIFR_BLK_MODE; 10])
        }
    }
    #[doc = "0x590 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn hstpipifr_blk_mode_mut(&self) -> &mut [HSTPIPIFR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1424usize)
                as *mut [HSTPIPIFR_BLK_MODE; 10])
        }
    }
    #[doc = "0x590 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn hstpipifr_iso_mode(&self) -> &[HSTPIPIFR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const [HSTPIPIFR_ISO_MODE; 10])
        }
    }
    #[doc = "0x590 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn hstpipifr_iso_mode_mut(&self) -> &mut [HSTPIPIFR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1424usize)
                as *mut [HSTPIPIFR_ISO_MODE; 10])
        }
    }
    #[doc = "0x590 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn hstpipifr_ctrl_mode(&self) -> &[HSTPIPIFR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const [HSTPIPIFR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x590 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn hstpipifr_ctrl_mode_mut(&self) -> &mut [HSTPIPIFR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1424usize)
                as *mut [HSTPIPIFR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn hstpipimr_intrpt_mode(&self) -> &[HSTPIPIMR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const [HSTPIPIMR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn hstpipimr_intrpt_mode_mut(&self) -> &mut [HSTPIPIMR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1472usize)
                as *mut [HSTPIPIMR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn hstpipimr_blk_mode(&self) -> &[HSTPIPIMR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const [HSTPIPIMR_BLK_MODE; 10])
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn hstpipimr_blk_mode_mut(&self) -> &mut [HSTPIPIMR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1472usize)
                as *mut [HSTPIPIMR_BLK_MODE; 10])
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn hstpipimr_iso_mode(&self) -> &[HSTPIPIMR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const [HSTPIPIMR_ISO_MODE; 10])
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn hstpipimr_iso_mode_mut(&self) -> &mut [HSTPIPIMR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1472usize)
                as *mut [HSTPIPIMR_ISO_MODE; 10])
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn hstpipimr_ctrl_mode(&self) -> &[HSTPIPIMR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const [HSTPIPIMR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn hstpipimr_ctrl_mode_mut(&self) -> &mut [HSTPIPIMR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1472usize)
                as *mut [HSTPIPIMR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn hstpipier_intrpt_mode(&self) -> &[HSTPIPIER_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const [HSTPIPIER_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn hstpipier_intrpt_mode_mut(&self) -> &mut [HSTPIPIER_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1520usize)
                as *mut [HSTPIPIER_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn hstpipier_blk_mode(&self) -> &[HSTPIPIER_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const [HSTPIPIER_BLK_MODE; 10])
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn hstpipier_blk_mode_mut(&self) -> &mut [HSTPIPIER_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1520usize)
                as *mut [HSTPIPIER_BLK_MODE; 10])
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn hstpipier_iso_mode(&self) -> &[HSTPIPIER_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const [HSTPIPIER_ISO_MODE; 10])
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn hstpipier_iso_mode_mut(&self) -> &mut [HSTPIPIER_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1520usize)
                as *mut [HSTPIPIER_ISO_MODE; 10])
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn hstpipier_ctrl_mode(&self) -> &[HSTPIPIER_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const [HSTPIPIER_CTRL_MODE; 10])
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn hstpipier_ctrl_mode_mut(&self) -> &mut [HSTPIPIER_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1520usize)
                as *mut [HSTPIPIER_CTRL_MODE; 10])
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn hstpipidr_intrpt_mode(&self) -> &[HSTPIPIDR_INTRPT_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1568usize)
                as *const [HSTPIPIDR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn hstpipidr_intrpt_mode_mut(&self) -> &mut [HSTPIPIDR_INTRPT_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1568usize)
                as *mut [HSTPIPIDR_INTRPT_MODE; 10])
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn hstpipidr_blk_mode(&self) -> &[HSTPIPIDR_BLK_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1568usize)
                as *const [HSTPIPIDR_BLK_MODE; 10])
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn hstpipidr_blk_mode_mut(&self) -> &mut [HSTPIPIDR_BLK_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1568usize)
                as *mut [HSTPIPIDR_BLK_MODE; 10])
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn hstpipidr_iso_mode(&self) -> &[HSTPIPIDR_ISO_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1568usize)
                as *const [HSTPIPIDR_ISO_MODE; 10])
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn hstpipidr_iso_mode_mut(&self) -> &mut [HSTPIPIDR_ISO_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1568usize)
                as *mut [HSTPIPIDR_ISO_MODE; 10])
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn hstpipidr_ctrl_mode(&self) -> &[HSTPIPIDR_CTRL_MODE; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1568usize)
                as *const [HSTPIPIDR_CTRL_MODE; 10])
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn hstpipidr_ctrl_mode_mut(&self) -> &mut [HSTPIPIDR_CTRL_MODE; 10] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1568usize)
                as *mut [HSTPIPIDR_CTRL_MODE; 10])
        }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct USBHS_DEVDMA {
    #[doc = "0x00 - Device DMA Channel Next Descriptor Address Register"]
    pub devdmanxtdsc: self::usbhs_devdma::DEVDMANXTDSC,
    #[doc = "0x04 - Device DMA Channel Address Register"]
    pub devdmaaddress: self::usbhs_devdma::DEVDMAADDRESS,
    #[doc = "0x08 - Device DMA Channel Control Register"]
    pub devdmacontrol: self::usbhs_devdma::DEVDMACONTROL,
    #[doc = "0x0c - Device DMA Channel Status Register"]
    pub devdmastatus: self::usbhs_devdma::DEVDMASTATUS,
}
#[doc = r"Register block"]
#[doc = "Device DMA Channel Next Descriptor Address Register"]
pub mod usbhs_devdma;
#[doc = r"Register block"]
#[repr(C)]
pub struct USBHS_HSTDMA {
    #[doc = "0x00 - Host DMA Channel Next Descriptor Address Register"]
    pub hstdmanxtdsc: self::usbhs_hstdma::HSTDMANXTDSC,
    #[doc = "0x04 - Host DMA Channel Address Register"]
    pub hstdmaaddress: self::usbhs_hstdma::HSTDMAADDRESS,
    #[doc = "0x08 - Host DMA Channel Control Register"]
    pub hstdmacontrol: self::usbhs_hstdma::HSTDMACONTROL,
    #[doc = "0x0c - Host DMA Channel Status Register"]
    pub hstdmastatus: self::usbhs_hstdma::HSTDMASTATUS,
}
#[doc = r"Register block"]
#[doc = "Host DMA Channel Next Descriptor Address Register"]
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
#[doc = "Device Endpoint Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptcfg](deveptcfg) module"]
pub type DEVEPTCFG = crate::Reg<u32, _DEVEPTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTCFG;
#[doc = "`read()` method returns [deveptcfg::R](deveptcfg::R) reader structure"]
impl crate::Readable for DEVEPTCFG {}
#[doc = "`write(|w| ..)` method takes [deveptcfg::W](deveptcfg::W) writer structure"]
impl crate::Writable for DEVEPTCFG {}
#[doc = "Device Endpoint Configuration Register"]
pub mod deveptcfg;
#[doc = "Device Endpoint Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptisr_ctrl_mode](deveptisr_ctrl_mode) module"]
pub type DEVEPTISR_CTRL_MODE = crate::Reg<u32, _DEVEPTISR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTISR_CTRL_MODE;
#[doc = "`read()` method returns [deveptisr_ctrl_mode::R](deveptisr_ctrl_mode::R) reader structure"]
impl crate::Readable for DEVEPTISR_CTRL_MODE {}
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod deveptisr_ctrl_mode;
#[doc = "Device Endpoint Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptisr_iso_mode](deveptisr_iso_mode) module"]
pub type DEVEPTISR_ISO_MODE = crate::Reg<u32, _DEVEPTISR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTISR_ISO_MODE;
#[doc = "`read()` method returns [deveptisr_iso_mode::R](deveptisr_iso_mode::R) reader structure"]
impl crate::Readable for DEVEPTISR_ISO_MODE {}
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod deveptisr_iso_mode;
#[doc = "Device Endpoint Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptisr_blk_mode](deveptisr_blk_mode) module"]
pub type DEVEPTISR_BLK_MODE = crate::Reg<u32, _DEVEPTISR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTISR_BLK_MODE;
#[doc = "`read()` method returns [deveptisr_blk_mode::R](deveptisr_blk_mode::R) reader structure"]
impl crate::Readable for DEVEPTISR_BLK_MODE {}
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod deveptisr_blk_mode;
#[doc = "Device Endpoint Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptisr_intrpt_mode](deveptisr_intrpt_mode) module"]
pub type DEVEPTISR_INTRPT_MODE = crate::Reg<u32, _DEVEPTISR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTISR_INTRPT_MODE;
#[doc = "`read()` method returns [deveptisr_intrpt_mode::R](deveptisr_intrpt_mode::R) reader structure"]
impl crate::Readable for DEVEPTISR_INTRPT_MODE {}
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod deveptisr_intrpt_mode;
#[doc = "Device Endpoint Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devepticr_ctrl_mode](devepticr_ctrl_mode) module"]
pub type DEVEPTICR_CTRL_MODE = crate::Reg<u32, _DEVEPTICR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTICR_CTRL_MODE;
#[doc = "`write(|w| ..)` method takes [devepticr_ctrl_mode::W](devepticr_ctrl_mode::W) writer structure"]
impl crate::Writable for DEVEPTICR_CTRL_MODE {}
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod devepticr_ctrl_mode;
#[doc = "Device Endpoint Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devepticr_iso_mode](devepticr_iso_mode) module"]
pub type DEVEPTICR_ISO_MODE = crate::Reg<u32, _DEVEPTICR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTICR_ISO_MODE;
#[doc = "`write(|w| ..)` method takes [devepticr_iso_mode::W](devepticr_iso_mode::W) writer structure"]
impl crate::Writable for DEVEPTICR_ISO_MODE {}
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod devepticr_iso_mode;
#[doc = "Device Endpoint Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devepticr_blk_mode](devepticr_blk_mode) module"]
pub type DEVEPTICR_BLK_MODE = crate::Reg<u32, _DEVEPTICR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTICR_BLK_MODE;
#[doc = "`write(|w| ..)` method takes [devepticr_blk_mode::W](devepticr_blk_mode::W) writer structure"]
impl crate::Writable for DEVEPTICR_BLK_MODE {}
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod devepticr_blk_mode;
#[doc = "Device Endpoint Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devepticr_intrpt_mode](devepticr_intrpt_mode) module"]
pub type DEVEPTICR_INTRPT_MODE = crate::Reg<u32, _DEVEPTICR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTICR_INTRPT_MODE;
#[doc = "`write(|w| ..)` method takes [devepticr_intrpt_mode::W](devepticr_intrpt_mode::W) writer structure"]
impl crate::Writable for DEVEPTICR_INTRPT_MODE {}
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod devepticr_intrpt_mode;
#[doc = "Device Endpoint Interrupt Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptifr_ctrl_mode](deveptifr_ctrl_mode) module"]
pub type DEVEPTIFR_CTRL_MODE = crate::Reg<u32, _DEVEPTIFR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIFR_CTRL_MODE;
#[doc = "`write(|w| ..)` method takes [deveptifr_ctrl_mode::W](deveptifr_ctrl_mode::W) writer structure"]
impl crate::Writable for DEVEPTIFR_CTRL_MODE {}
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod deveptifr_ctrl_mode;
#[doc = "Device Endpoint Interrupt Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptifr_iso_mode](deveptifr_iso_mode) module"]
pub type DEVEPTIFR_ISO_MODE = crate::Reg<u32, _DEVEPTIFR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIFR_ISO_MODE;
#[doc = "`write(|w| ..)` method takes [deveptifr_iso_mode::W](deveptifr_iso_mode::W) writer structure"]
impl crate::Writable for DEVEPTIFR_ISO_MODE {}
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod deveptifr_iso_mode;
#[doc = "Device Endpoint Interrupt Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptifr_blk_mode](deveptifr_blk_mode) module"]
pub type DEVEPTIFR_BLK_MODE = crate::Reg<u32, _DEVEPTIFR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIFR_BLK_MODE;
#[doc = "`write(|w| ..)` method takes [deveptifr_blk_mode::W](deveptifr_blk_mode::W) writer structure"]
impl crate::Writable for DEVEPTIFR_BLK_MODE {}
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod deveptifr_blk_mode;
#[doc = "Device Endpoint Interrupt Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptifr_intrpt_mode](deveptifr_intrpt_mode) module"]
pub type DEVEPTIFR_INTRPT_MODE = crate::Reg<u32, _DEVEPTIFR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIFR_INTRPT_MODE;
#[doc = "`write(|w| ..)` method takes [deveptifr_intrpt_mode::W](deveptifr_intrpt_mode::W) writer structure"]
impl crate::Writable for DEVEPTIFR_INTRPT_MODE {}
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod deveptifr_intrpt_mode;
#[doc = "Device Endpoint Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptimr_ctrl_mode](deveptimr_ctrl_mode) module"]
pub type DEVEPTIMR_CTRL_MODE = crate::Reg<u32, _DEVEPTIMR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIMR_CTRL_MODE;
#[doc = "`read()` method returns [deveptimr_ctrl_mode::R](deveptimr_ctrl_mode::R) reader structure"]
impl crate::Readable for DEVEPTIMR_CTRL_MODE {}
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod deveptimr_ctrl_mode;
#[doc = "Device Endpoint Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptimr_iso_mode](deveptimr_iso_mode) module"]
pub type DEVEPTIMR_ISO_MODE = crate::Reg<u32, _DEVEPTIMR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIMR_ISO_MODE;
#[doc = "`read()` method returns [deveptimr_iso_mode::R](deveptimr_iso_mode::R) reader structure"]
impl crate::Readable for DEVEPTIMR_ISO_MODE {}
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod deveptimr_iso_mode;
#[doc = "Device Endpoint Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptimr_blk_mode](deveptimr_blk_mode) module"]
pub type DEVEPTIMR_BLK_MODE = crate::Reg<u32, _DEVEPTIMR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIMR_BLK_MODE;
#[doc = "`read()` method returns [deveptimr_blk_mode::R](deveptimr_blk_mode::R) reader structure"]
impl crate::Readable for DEVEPTIMR_BLK_MODE {}
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod deveptimr_blk_mode;
#[doc = "Device Endpoint Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptimr_intrpt_mode](deveptimr_intrpt_mode) module"]
pub type DEVEPTIMR_INTRPT_MODE = crate::Reg<u32, _DEVEPTIMR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIMR_INTRPT_MODE;
#[doc = "`read()` method returns [deveptimr_intrpt_mode::R](deveptimr_intrpt_mode::R) reader structure"]
impl crate::Readable for DEVEPTIMR_INTRPT_MODE {}
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod deveptimr_intrpt_mode;
#[doc = "Device Endpoint Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptier_ctrl_mode](deveptier_ctrl_mode) module"]
pub type DEVEPTIER_CTRL_MODE = crate::Reg<u32, _DEVEPTIER_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIER_CTRL_MODE;
#[doc = "`write(|w| ..)` method takes [deveptier_ctrl_mode::W](deveptier_ctrl_mode::W) writer structure"]
impl crate::Writable for DEVEPTIER_CTRL_MODE {}
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod deveptier_ctrl_mode;
#[doc = "Device Endpoint Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptier_iso_mode](deveptier_iso_mode) module"]
pub type DEVEPTIER_ISO_MODE = crate::Reg<u32, _DEVEPTIER_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIER_ISO_MODE;
#[doc = "`write(|w| ..)` method takes [deveptier_iso_mode::W](deveptier_iso_mode::W) writer structure"]
impl crate::Writable for DEVEPTIER_ISO_MODE {}
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod deveptier_iso_mode;
#[doc = "Device Endpoint Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptier_blk_mode](deveptier_blk_mode) module"]
pub type DEVEPTIER_BLK_MODE = crate::Reg<u32, _DEVEPTIER_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIER_BLK_MODE;
#[doc = "`write(|w| ..)` method takes [deveptier_blk_mode::W](deveptier_blk_mode::W) writer structure"]
impl crate::Writable for DEVEPTIER_BLK_MODE {}
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod deveptier_blk_mode;
#[doc = "Device Endpoint Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptier_intrpt_mode](deveptier_intrpt_mode) module"]
pub type DEVEPTIER_INTRPT_MODE = crate::Reg<u32, _DEVEPTIER_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIER_INTRPT_MODE;
#[doc = "`write(|w| ..)` method takes [deveptier_intrpt_mode::W](deveptier_intrpt_mode::W) writer structure"]
impl crate::Writable for DEVEPTIER_INTRPT_MODE {}
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod deveptier_intrpt_mode;
#[doc = "Device Endpoint Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptidr_ctrl_mode](deveptidr_ctrl_mode) module"]
pub type DEVEPTIDR_CTRL_MODE = crate::Reg<u32, _DEVEPTIDR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIDR_CTRL_MODE;
#[doc = "`write(|w| ..)` method takes [deveptidr_ctrl_mode::W](deveptidr_ctrl_mode::W) writer structure"]
impl crate::Writable for DEVEPTIDR_CTRL_MODE {}
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod deveptidr_ctrl_mode;
#[doc = "Device Endpoint Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptidr_iso_mode](deveptidr_iso_mode) module"]
pub type DEVEPTIDR_ISO_MODE = crate::Reg<u32, _DEVEPTIDR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIDR_ISO_MODE;
#[doc = "`write(|w| ..)` method takes [deveptidr_iso_mode::W](deveptidr_iso_mode::W) writer structure"]
impl crate::Writable for DEVEPTIDR_ISO_MODE {}
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod deveptidr_iso_mode;
#[doc = "Device Endpoint Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptidr_blk_mode](deveptidr_blk_mode) module"]
pub type DEVEPTIDR_BLK_MODE = crate::Reg<u32, _DEVEPTIDR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIDR_BLK_MODE;
#[doc = "`write(|w| ..)` method takes [deveptidr_blk_mode::W](deveptidr_blk_mode::W) writer structure"]
impl crate::Writable for DEVEPTIDR_BLK_MODE {}
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod deveptidr_blk_mode;
#[doc = "Device Endpoint Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptidr_intrpt_mode](deveptidr_intrpt_mode) module"]
pub type DEVEPTIDR_INTRPT_MODE = crate::Reg<u32, _DEVEPTIDR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIDR_INTRPT_MODE;
#[doc = "`write(|w| ..)` method takes [deveptidr_intrpt_mode::W](deveptidr_intrpt_mode::W) writer structure"]
impl crate::Writable for DEVEPTIDR_INTRPT_MODE {}
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod deveptidr_intrpt_mode;
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
#[doc = "Host Pipe Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipcfg](hstpipcfg) module"]
pub type HSTPIPCFG = crate::Reg<u32, _HSTPIPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPCFG;
#[doc = "`read()` method returns [hstpipcfg::R](hstpipcfg::R) reader structure"]
impl crate::Readable for HSTPIPCFG {}
#[doc = "`write(|w| ..)` method takes [hstpipcfg::W](hstpipcfg::W) writer structure"]
impl crate::Writable for HSTPIPCFG {}
#[doc = "Host Pipe Configuration Register"]
pub mod hstpipcfg;
#[doc = "Host Pipe Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipcfg_ctrl_bulk_mode](hstpipcfg_ctrl_bulk_mode) module"]
pub type HSTPIPCFG_CTRL_BULK_MODE = crate::Reg<u32, _HSTPIPCFG_CTRL_BULK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPCFG_CTRL_BULK_MODE;
#[doc = "`read()` method returns [hstpipcfg_ctrl_bulk_mode::R](hstpipcfg_ctrl_bulk_mode::R) reader structure"]
impl crate::Readable for HSTPIPCFG_CTRL_BULK_MODE {}
#[doc = "`write(|w| ..)` method takes [hstpipcfg_ctrl_bulk_mode::W](hstpipcfg_ctrl_bulk_mode::W) writer structure"]
impl crate::Writable for HSTPIPCFG_CTRL_BULK_MODE {}
#[doc = "Host Pipe Configuration Register"]
pub mod hstpipcfg_ctrl_bulk_mode;
#[doc = "Host Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipisr_ctrl_mode](hstpipisr_ctrl_mode) module"]
pub type HSTPIPISR_CTRL_MODE = crate::Reg<u32, _HSTPIPISR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPISR_CTRL_MODE;
#[doc = "`read()` method returns [hstpipisr_ctrl_mode::R](hstpipisr_ctrl_mode::R) reader structure"]
impl crate::Readable for HSTPIPISR_CTRL_MODE {}
#[doc = "Host Pipe Status Register"]
pub mod hstpipisr_ctrl_mode;
#[doc = "Host Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipisr_iso_mode](hstpipisr_iso_mode) module"]
pub type HSTPIPISR_ISO_MODE = crate::Reg<u32, _HSTPIPISR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPISR_ISO_MODE;
#[doc = "`read()` method returns [hstpipisr_iso_mode::R](hstpipisr_iso_mode::R) reader structure"]
impl crate::Readable for HSTPIPISR_ISO_MODE {}
#[doc = "Host Pipe Status Register"]
pub mod hstpipisr_iso_mode;
#[doc = "Host Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipisr_blk_mode](hstpipisr_blk_mode) module"]
pub type HSTPIPISR_BLK_MODE = crate::Reg<u32, _HSTPIPISR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPISR_BLK_MODE;
#[doc = "`read()` method returns [hstpipisr_blk_mode::R](hstpipisr_blk_mode::R) reader structure"]
impl crate::Readable for HSTPIPISR_BLK_MODE {}
#[doc = "Host Pipe Status Register"]
pub mod hstpipisr_blk_mode;
#[doc = "Host Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipisr_intrpt_mode](hstpipisr_intrpt_mode) module"]
pub type HSTPIPISR_INTRPT_MODE = crate::Reg<u32, _HSTPIPISR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPISR_INTRPT_MODE;
#[doc = "`read()` method returns [hstpipisr_intrpt_mode::R](hstpipisr_intrpt_mode::R) reader structure"]
impl crate::Readable for HSTPIPISR_INTRPT_MODE {}
#[doc = "Host Pipe Status Register"]
pub mod hstpipisr_intrpt_mode;
#[doc = "Host Pipe Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipicr_ctrl_mode](hstpipicr_ctrl_mode) module"]
pub type HSTPIPICR_CTRL_MODE = crate::Reg<u32, _HSTPIPICR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPICR_CTRL_MODE;
#[doc = "`write(|w| ..)` method takes [hstpipicr_ctrl_mode::W](hstpipicr_ctrl_mode::W) writer structure"]
impl crate::Writable for HSTPIPICR_CTRL_MODE {}
#[doc = "Host Pipe Clear Register"]
pub mod hstpipicr_ctrl_mode;
#[doc = "Host Pipe Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipicr_iso_mode](hstpipicr_iso_mode) module"]
pub type HSTPIPICR_ISO_MODE = crate::Reg<u32, _HSTPIPICR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPICR_ISO_MODE;
#[doc = "`write(|w| ..)` method takes [hstpipicr_iso_mode::W](hstpipicr_iso_mode::W) writer structure"]
impl crate::Writable for HSTPIPICR_ISO_MODE {}
#[doc = "Host Pipe Clear Register"]
pub mod hstpipicr_iso_mode;
#[doc = "Host Pipe Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipicr_blk_mode](hstpipicr_blk_mode) module"]
pub type HSTPIPICR_BLK_MODE = crate::Reg<u32, _HSTPIPICR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPICR_BLK_MODE;
#[doc = "`write(|w| ..)` method takes [hstpipicr_blk_mode::W](hstpipicr_blk_mode::W) writer structure"]
impl crate::Writable for HSTPIPICR_BLK_MODE {}
#[doc = "Host Pipe Clear Register"]
pub mod hstpipicr_blk_mode;
#[doc = "Host Pipe Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipicr_intrpt_mode](hstpipicr_intrpt_mode) module"]
pub type HSTPIPICR_INTRPT_MODE = crate::Reg<u32, _HSTPIPICR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPICR_INTRPT_MODE;
#[doc = "`write(|w| ..)` method takes [hstpipicr_intrpt_mode::W](hstpipicr_intrpt_mode::W) writer structure"]
impl crate::Writable for HSTPIPICR_INTRPT_MODE {}
#[doc = "Host Pipe Clear Register"]
pub mod hstpipicr_intrpt_mode;
#[doc = "Host Pipe Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipifr_ctrl_mode](hstpipifr_ctrl_mode) module"]
pub type HSTPIPIFR_CTRL_MODE = crate::Reg<u32, _HSTPIPIFR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIFR_CTRL_MODE;
#[doc = "`write(|w| ..)` method takes [hstpipifr_ctrl_mode::W](hstpipifr_ctrl_mode::W) writer structure"]
impl crate::Writable for HSTPIPIFR_CTRL_MODE {}
#[doc = "Host Pipe Set Register"]
pub mod hstpipifr_ctrl_mode;
#[doc = "Host Pipe Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipifr_iso_mode](hstpipifr_iso_mode) module"]
pub type HSTPIPIFR_ISO_MODE = crate::Reg<u32, _HSTPIPIFR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIFR_ISO_MODE;
#[doc = "`write(|w| ..)` method takes [hstpipifr_iso_mode::W](hstpipifr_iso_mode::W) writer structure"]
impl crate::Writable for HSTPIPIFR_ISO_MODE {}
#[doc = "Host Pipe Set Register"]
pub mod hstpipifr_iso_mode;
#[doc = "Host Pipe Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipifr_blk_mode](hstpipifr_blk_mode) module"]
pub type HSTPIPIFR_BLK_MODE = crate::Reg<u32, _HSTPIPIFR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIFR_BLK_MODE;
#[doc = "`write(|w| ..)` method takes [hstpipifr_blk_mode::W](hstpipifr_blk_mode::W) writer structure"]
impl crate::Writable for HSTPIPIFR_BLK_MODE {}
#[doc = "Host Pipe Set Register"]
pub mod hstpipifr_blk_mode;
#[doc = "Host Pipe Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipifr_intrpt_mode](hstpipifr_intrpt_mode) module"]
pub type HSTPIPIFR_INTRPT_MODE = crate::Reg<u32, _HSTPIPIFR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIFR_INTRPT_MODE;
#[doc = "`write(|w| ..)` method takes [hstpipifr_intrpt_mode::W](hstpipifr_intrpt_mode::W) writer structure"]
impl crate::Writable for HSTPIPIFR_INTRPT_MODE {}
#[doc = "Host Pipe Set Register"]
pub mod hstpipifr_intrpt_mode;
#[doc = "Host Pipe Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipimr_ctrl_mode](hstpipimr_ctrl_mode) module"]
pub type HSTPIPIMR_CTRL_MODE = crate::Reg<u32, _HSTPIPIMR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIMR_CTRL_MODE;
#[doc = "`read()` method returns [hstpipimr_ctrl_mode::R](hstpipimr_ctrl_mode::R) reader structure"]
impl crate::Readable for HSTPIPIMR_CTRL_MODE {}
#[doc = "Host Pipe Mask Register"]
pub mod hstpipimr_ctrl_mode;
#[doc = "Host Pipe Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipimr_iso_mode](hstpipimr_iso_mode) module"]
pub type HSTPIPIMR_ISO_MODE = crate::Reg<u32, _HSTPIPIMR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIMR_ISO_MODE;
#[doc = "`read()` method returns [hstpipimr_iso_mode::R](hstpipimr_iso_mode::R) reader structure"]
impl crate::Readable for HSTPIPIMR_ISO_MODE {}
#[doc = "Host Pipe Mask Register"]
pub mod hstpipimr_iso_mode;
#[doc = "Host Pipe Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipimr_blk_mode](hstpipimr_blk_mode) module"]
pub type HSTPIPIMR_BLK_MODE = crate::Reg<u32, _HSTPIPIMR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIMR_BLK_MODE;
#[doc = "`read()` method returns [hstpipimr_blk_mode::R](hstpipimr_blk_mode::R) reader structure"]
impl crate::Readable for HSTPIPIMR_BLK_MODE {}
#[doc = "Host Pipe Mask Register"]
pub mod hstpipimr_blk_mode;
#[doc = "Host Pipe Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipimr_intrpt_mode](hstpipimr_intrpt_mode) module"]
pub type HSTPIPIMR_INTRPT_MODE = crate::Reg<u32, _HSTPIPIMR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIMR_INTRPT_MODE;
#[doc = "`read()` method returns [hstpipimr_intrpt_mode::R](hstpipimr_intrpt_mode::R) reader structure"]
impl crate::Readable for HSTPIPIMR_INTRPT_MODE {}
#[doc = "Host Pipe Mask Register"]
pub mod hstpipimr_intrpt_mode;
#[doc = "Host Pipe Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipier_ctrl_mode](hstpipier_ctrl_mode) module"]
pub type HSTPIPIER_CTRL_MODE = crate::Reg<u32, _HSTPIPIER_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIER_CTRL_MODE;
#[doc = "`write(|w| ..)` method takes [hstpipier_ctrl_mode::W](hstpipier_ctrl_mode::W) writer structure"]
impl crate::Writable for HSTPIPIER_CTRL_MODE {}
#[doc = "Host Pipe Enable Register"]
pub mod hstpipier_ctrl_mode;
#[doc = "Host Pipe Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipier_iso_mode](hstpipier_iso_mode) module"]
pub type HSTPIPIER_ISO_MODE = crate::Reg<u32, _HSTPIPIER_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIER_ISO_MODE;
#[doc = "`write(|w| ..)` method takes [hstpipier_iso_mode::W](hstpipier_iso_mode::W) writer structure"]
impl crate::Writable for HSTPIPIER_ISO_MODE {}
#[doc = "Host Pipe Enable Register"]
pub mod hstpipier_iso_mode;
#[doc = "Host Pipe Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipier_blk_mode](hstpipier_blk_mode) module"]
pub type HSTPIPIER_BLK_MODE = crate::Reg<u32, _HSTPIPIER_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIER_BLK_MODE;
#[doc = "`write(|w| ..)` method takes [hstpipier_blk_mode::W](hstpipier_blk_mode::W) writer structure"]
impl crate::Writable for HSTPIPIER_BLK_MODE {}
#[doc = "Host Pipe Enable Register"]
pub mod hstpipier_blk_mode;
#[doc = "Host Pipe Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipier_intrpt_mode](hstpipier_intrpt_mode) module"]
pub type HSTPIPIER_INTRPT_MODE = crate::Reg<u32, _HSTPIPIER_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIER_INTRPT_MODE;
#[doc = "`write(|w| ..)` method takes [hstpipier_intrpt_mode::W](hstpipier_intrpt_mode::W) writer structure"]
impl crate::Writable for HSTPIPIER_INTRPT_MODE {}
#[doc = "Host Pipe Enable Register"]
pub mod hstpipier_intrpt_mode;
#[doc = "Host Pipe Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipidr_ctrl_mode](hstpipidr_ctrl_mode) module"]
pub type HSTPIPIDR_CTRL_MODE = crate::Reg<u32, _HSTPIPIDR_CTRL_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIDR_CTRL_MODE;
#[doc = "`write(|w| ..)` method takes [hstpipidr_ctrl_mode::W](hstpipidr_ctrl_mode::W) writer structure"]
impl crate::Writable for HSTPIPIDR_CTRL_MODE {}
#[doc = "Host Pipe Disable Register"]
pub mod hstpipidr_ctrl_mode;
#[doc = "Host Pipe Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipidr_iso_mode](hstpipidr_iso_mode) module"]
pub type HSTPIPIDR_ISO_MODE = crate::Reg<u32, _HSTPIPIDR_ISO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIDR_ISO_MODE;
#[doc = "`write(|w| ..)` method takes [hstpipidr_iso_mode::W](hstpipidr_iso_mode::W) writer structure"]
impl crate::Writable for HSTPIPIDR_ISO_MODE {}
#[doc = "Host Pipe Disable Register"]
pub mod hstpipidr_iso_mode;
#[doc = "Host Pipe Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipidr_blk_mode](hstpipidr_blk_mode) module"]
pub type HSTPIPIDR_BLK_MODE = crate::Reg<u32, _HSTPIPIDR_BLK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIDR_BLK_MODE;
#[doc = "`write(|w| ..)` method takes [hstpipidr_blk_mode::W](hstpipidr_blk_mode::W) writer structure"]
impl crate::Writable for HSTPIPIDR_BLK_MODE {}
#[doc = "Host Pipe Disable Register"]
pub mod hstpipidr_blk_mode;
#[doc = "Host Pipe Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipidr_intrpt_mode](hstpipidr_intrpt_mode) module"]
pub type HSTPIPIDR_INTRPT_MODE = crate::Reg<u32, _HSTPIPIDR_INTRPT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIDR_INTRPT_MODE;
#[doc = "`write(|w| ..)` method takes [hstpipidr_intrpt_mode::W](hstpipidr_intrpt_mode::W) writer structure"]
impl crate::Writable for HSTPIPIDR_INTRPT_MODE {}
#[doc = "Host Pipe Disable Register"]
pub mod hstpipidr_intrpt_mode;
#[doc = "Host Pipe IN Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipinrq](hstpipinrq) module"]
pub type HSTPIPINRQ = crate::Reg<u32, _HSTPIPINRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPINRQ;
#[doc = "`read()` method returns [hstpipinrq::R](hstpipinrq::R) reader structure"]
impl crate::Readable for HSTPIPINRQ {}
#[doc = "`write(|w| ..)` method takes [hstpipinrq::W](hstpipinrq::W) writer structure"]
impl crate::Writable for HSTPIPINRQ {}
#[doc = "Host Pipe IN Request Register"]
pub mod hstpipinrq;
#[doc = "Host Pipe Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpiperr](hstpiperr) module"]
pub type HSTPIPERR = crate::Reg<u32, _HSTPIPERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPERR;
#[doc = "`read()` method returns [hstpiperr::R](hstpiperr::R) reader structure"]
impl crate::Readable for HSTPIPERR {}
#[doc = "`write(|w| ..)` method takes [hstpiperr::W](hstpiperr::W) writer structure"]
impl crate::Writable for HSTPIPERR {}
#[doc = "Host Pipe Error Register"]
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
