#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Type Register"]
    pub gtype: GTYPE,
    #[doc = "0x04 - Global Configuration Register"]
    pub gcfg: GCFG,
    #[doc = "0x08 - Global Weighted Arbiter Configuration Register"]
    pub gwac: GWAC,
    #[doc = "0x0c - Global Interrupt Enable Register"]
    pub gie: GIE,
    #[doc = "0x10 - Global Interrupt Disable Register"]
    pub gid: GID,
    #[doc = "0x14 - Global Interrupt Mask Register"]
    pub gim: GIM,
    #[doc = "0x18 - Global Interrupt Status Register"]
    pub gis: GIS,
    #[doc = "0x1c - Global Channel Enable Register"]
    pub ge: GE,
    #[doc = "0x20 - Global Channel Disable Register"]
    pub gd: GD,
    #[doc = "0x24 - Global Channel Status Register"]
    pub gs: GS,
    #[doc = "0x28 - Global Channel Read Suspend Register"]
    pub grs: GRS,
    #[doc = "0x2c - Global Channel Write Suspend Register"]
    pub gws: GWS,
    #[doc = "0x30 - Global Channel Read Write Suspend Register"]
    pub grws: GRWS,
    #[doc = "0x34 - Global Channel Read Write Resume Register"]
    pub grwr: GRWR,
    #[doc = "0x38 - Global Channel Software Request Register"]
    pub gswr: GSWR,
    #[doc = "0x3c - Global Channel Software Request Status Register"]
    pub gsws: GSWS,
    #[doc = "0x40 - Global Channel Software Flush Request Register"]
    pub gswf: GSWF,
    _reserved17: [u8; 12usize],
    #[doc = "0x50 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid0: XDMAC_CHID,
    _reserved18: [u8; 8usize],
    #[doc = "0x90 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid1: XDMAC_CHID,
    _reserved19: [u8; 8usize],
    #[doc = "0xd0 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid2: XDMAC_CHID,
    _reserved20: [u8; 8usize],
    #[doc = "0x110 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid3: XDMAC_CHID,
    _reserved21: [u8; 8usize],
    #[doc = "0x150 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid4: XDMAC_CHID,
    _reserved22: [u8; 8usize],
    #[doc = "0x190 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid5: XDMAC_CHID,
    _reserved23: [u8; 8usize],
    #[doc = "0x1d0 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid6: XDMAC_CHID,
    _reserved24: [u8; 8usize],
    #[doc = "0x210 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid7: XDMAC_CHID,
    _reserved25: [u8; 8usize],
    #[doc = "0x250 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid8: XDMAC_CHID,
    _reserved26: [u8; 8usize],
    #[doc = "0x290 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid9: XDMAC_CHID,
    _reserved27: [u8; 8usize],
    #[doc = "0x2d0 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid10: XDMAC_CHID,
    _reserved28: [u8; 8usize],
    #[doc = "0x310 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid11: XDMAC_CHID,
    _reserved29: [u8; 8usize],
    #[doc = "0x350 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid12: XDMAC_CHID,
    _reserved30: [u8; 8usize],
    #[doc = "0x390 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid13: XDMAC_CHID,
    _reserved31: [u8; 8usize],
    #[doc = "0x3d0 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid14: XDMAC_CHID,
    _reserved32: [u8; 8usize],
    #[doc = "0x410 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid15: XDMAC_CHID,
    _reserved33: [u8; 8usize],
    #[doc = "0x450 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid16: XDMAC_CHID,
    _reserved34: [u8; 8usize],
    #[doc = "0x490 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid17: XDMAC_CHID,
    _reserved35: [u8; 8usize],
    #[doc = "0x4d0 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid18: XDMAC_CHID,
    _reserved36: [u8; 8usize],
    #[doc = "0x510 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid19: XDMAC_CHID,
    _reserved37: [u8; 8usize],
    #[doc = "0x550 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid20: XDMAC_CHID,
    _reserved38: [u8; 8usize],
    #[doc = "0x590 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid21: XDMAC_CHID,
    _reserved39: [u8; 8usize],
    #[doc = "0x5d0 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid22: XDMAC_CHID,
    _reserved40: [u8; 8usize],
    #[doc = "0x610 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid23: XDMAC_CHID,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct XDMAC_CHID {
    #[doc = "0x00 - Channel Interrupt Enable Register (chid = 0)"]
    pub cie: self::xdmac_chid::CIE,
    #[doc = "0x04 - Channel Interrupt Disable Register (chid = 0)"]
    pub cid: self::xdmac_chid::CID,
    #[doc = "0x08 - Channel Interrupt Mask Register (chid = 0)"]
    pub cim: self::xdmac_chid::CIM,
    #[doc = "0x0c - Channel Interrupt Status Register (chid = 0)"]
    pub cis: self::xdmac_chid::CIS,
    #[doc = "0x10 - Channel Source Address Register (chid = 0)"]
    pub csa: self::xdmac_chid::CSA,
    #[doc = "0x14 - Channel Destination Address Register (chid = 0)"]
    pub cda: self::xdmac_chid::CDA,
    #[doc = "0x18 - Channel Next Descriptor Address Register (chid = 0)"]
    pub cnda: self::xdmac_chid::CNDA,
    #[doc = "0x1c - Channel Next Descriptor Control Register (chid = 0)"]
    pub cndc: self::xdmac_chid::CNDC,
    #[doc = "0x20 - Channel Microblock Control Register (chid = 0)"]
    pub cubc: self::xdmac_chid::CUBC,
    #[doc = "0x24 - Channel Block Control Register (chid = 0)"]
    pub cbc: self::xdmac_chid::CBC,
    #[doc = "0x28 - Channel Configuration Register (chid = 0)"]
    pub cc: self::xdmac_chid::CC,
    #[doc = "0x2c - Channel Data Stride Memory Set Pattern (chid = 0)"]
    pub cds_msp: self::xdmac_chid::CDS_MSP,
    #[doc = "0x30 - Channel Source Microblock Stride (chid = 0)"]
    pub csus: self::xdmac_chid::CSUS,
    #[doc = "0x34 - Channel Destination Microblock Stride (chid = 0)"]
    pub cdus: self::xdmac_chid::CDUS,
}
#[doc = r"Register block"]
#[doc = "Channel Interrupt Enable Register (chid = 0)"]
pub mod xdmac_chid;
#[doc = "Global Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtype](gtype) module"]
pub type GTYPE = crate::Reg<u32, _GTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GTYPE;
#[doc = "`read()` method returns [gtype::R](gtype::R) reader structure"]
impl crate::Readable for GTYPE {}
#[doc = "Global Type Register"]
pub mod gtype;
#[doc = "Global Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcfg](gcfg) module"]
pub type GCFG = crate::Reg<u32, _GCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCFG;
#[doc = "`read()` method returns [gcfg::R](gcfg::R) reader structure"]
impl crate::Readable for GCFG {}
#[doc = "`write(|w| ..)` method takes [gcfg::W](gcfg::W) writer structure"]
impl crate::Writable for GCFG {}
#[doc = "Global Configuration Register"]
pub mod gcfg;
#[doc = "Global Weighted Arbiter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gwac](gwac) module"]
pub type GWAC = crate::Reg<u32, _GWAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GWAC;
#[doc = "`read()` method returns [gwac::R](gwac::R) reader structure"]
impl crate::Readable for GWAC {}
#[doc = "`write(|w| ..)` method takes [gwac::W](gwac::W) writer structure"]
impl crate::Writable for GWAC {}
#[doc = "Global Weighted Arbiter Configuration Register"]
pub mod gwac;
#[doc = "Global Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gie](gie) module"]
pub type GIE = crate::Reg<u32, _GIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GIE;
#[doc = "`write(|w| ..)` method takes [gie::W](gie::W) writer structure"]
impl crate::Writable for GIE {}
#[doc = "Global Interrupt Enable Register"]
pub mod gie;
#[doc = "Global Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gid](gid) module"]
pub type GID = crate::Reg<u32, _GID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GID;
#[doc = "`write(|w| ..)` method takes [gid::W](gid::W) writer structure"]
impl crate::Writable for GID {}
#[doc = "Global Interrupt Disable Register"]
pub mod gid;
#[doc = "Global Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gim](gim) module"]
pub type GIM = crate::Reg<u32, _GIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GIM;
#[doc = "`read()` method returns [gim::R](gim::R) reader structure"]
impl crate::Readable for GIM {}
#[doc = "Global Interrupt Mask Register"]
pub mod gim;
#[doc = "Global Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gis](gis) module"]
pub type GIS = crate::Reg<u32, _GIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GIS;
#[doc = "`read()` method returns [gis::R](gis::R) reader structure"]
impl crate::Readable for GIS {}
#[doc = "Global Interrupt Status Register"]
pub mod gis;
#[doc = "Global Channel Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ge](ge) module"]
pub type GE = crate::Reg<u32, _GE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GE;
#[doc = "`write(|w| ..)` method takes [ge::W](ge::W) writer structure"]
impl crate::Writable for GE {}
#[doc = "Global Channel Enable Register"]
pub mod ge;
#[doc = "Global Channel Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gd](gd) module"]
pub type GD = crate::Reg<u32, _GD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GD;
#[doc = "`write(|w| ..)` method takes [gd::W](gd::W) writer structure"]
impl crate::Writable for GD {}
#[doc = "Global Channel Disable Register"]
pub mod gd;
#[doc = "Global Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gs](gs) module"]
pub type GS = crate::Reg<u32, _GS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GS;
#[doc = "`read()` method returns [gs::R](gs::R) reader structure"]
impl crate::Readable for GS {}
#[doc = "Global Channel Status Register"]
pub mod gs;
#[doc = "Global Channel Read Suspend Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grs](grs) module"]
pub type GRS = crate::Reg<u32, _GRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRS;
#[doc = "`read()` method returns [grs::R](grs::R) reader structure"]
impl crate::Readable for GRS {}
#[doc = "`write(|w| ..)` method takes [grs::W](grs::W) writer structure"]
impl crate::Writable for GRS {}
#[doc = "Global Channel Read Suspend Register"]
pub mod grs;
#[doc = "Global Channel Write Suspend Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gws](gws) module"]
pub type GWS = crate::Reg<u32, _GWS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GWS;
#[doc = "`read()` method returns [gws::R](gws::R) reader structure"]
impl crate::Readable for GWS {}
#[doc = "`write(|w| ..)` method takes [gws::W](gws::W) writer structure"]
impl crate::Writable for GWS {}
#[doc = "Global Channel Write Suspend Register"]
pub mod gws;
#[doc = "Global Channel Read Write Suspend Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grws](grws) module"]
pub type GRWS = crate::Reg<u32, _GRWS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRWS;
#[doc = "`write(|w| ..)` method takes [grws::W](grws::W) writer structure"]
impl crate::Writable for GRWS {}
#[doc = "Global Channel Read Write Suspend Register"]
pub mod grws;
#[doc = "Global Channel Read Write Resume Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grwr](grwr) module"]
pub type GRWR = crate::Reg<u32, _GRWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRWR;
#[doc = "`write(|w| ..)` method takes [grwr::W](grwr::W) writer structure"]
impl crate::Writable for GRWR {}
#[doc = "Global Channel Read Write Resume Register"]
pub mod grwr;
#[doc = "Global Channel Software Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gswr](gswr) module"]
pub type GSWR = crate::Reg<u32, _GSWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GSWR;
#[doc = "`write(|w| ..)` method takes [gswr::W](gswr::W) writer structure"]
impl crate::Writable for GSWR {}
#[doc = "Global Channel Software Request Register"]
pub mod gswr;
#[doc = "Global Channel Software Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gsws](gsws) module"]
pub type GSWS = crate::Reg<u32, _GSWS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GSWS;
#[doc = "`read()` method returns [gsws::R](gsws::R) reader structure"]
impl crate::Readable for GSWS {}
#[doc = "Global Channel Software Request Status Register"]
pub mod gsws;
#[doc = "Global Channel Software Flush Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gswf](gswf) module"]
pub type GSWF = crate::Reg<u32, _GSWF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GSWF;
#[doc = "`write(|w| ..)` method takes [gswf::W](gswf::W) writer structure"]
impl crate::Writable for GSWF {}
#[doc = "Global Channel Software Flush Request Register"]
pub mod gswf;
