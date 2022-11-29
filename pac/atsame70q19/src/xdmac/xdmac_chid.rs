#[doc = r"Register block"]
#[repr(C)]
pub struct XDMAC_CHID {
    #[doc = "0x00 - Channel Interrupt Enable Register (chid = 0)"]
    pub cie: CIE,
    #[doc = "0x04 - Channel Interrupt Disable Register (chid = 0)"]
    pub cid: CID,
    #[doc = "0x08 - Channel Interrupt Mask Register (chid = 0)"]
    pub cim: CIM,
    #[doc = "0x0c - Channel Interrupt Status Register (chid = 0)"]
    pub cis: CIS,
    #[doc = "0x10 - Channel Source Address Register (chid = 0)"]
    pub csa: CSA,
    #[doc = "0x14 - Channel Destination Address Register (chid = 0)"]
    pub cda: CDA,
    #[doc = "0x18 - Channel Next Descriptor Address Register (chid = 0)"]
    pub cnda: CNDA,
    #[doc = "0x1c - Channel Next Descriptor Control Register (chid = 0)"]
    pub cndc: CNDC,
    #[doc = "0x20 - Channel Microblock Control Register (chid = 0)"]
    pub cubc: CUBC,
    #[doc = "0x24 - Channel Block Control Register (chid = 0)"]
    pub cbc: CBC,
    #[doc = "0x28 - Channel Configuration Register (chid = 0)"]
    pub cc: CC,
    #[doc = "0x2c - Channel Data Stride Memory Set Pattern (chid = 0)"]
    pub cds_msp: CDS_MSP,
    #[doc = "0x30 - Channel Source Microblock Stride (chid = 0)"]
    pub csus: CSUS,
    #[doc = "0x34 - Channel Destination Microblock Stride (chid = 0)"]
    pub cdus: CDUS,
}
#[doc = "CIE (w) register accessor: an alias for `Reg<CIE_SPEC>`"]
pub type CIE = crate::Reg<cie::CIE_SPEC>;
#[doc = "Channel Interrupt Enable Register (chid = 0)"]
pub mod cie;
#[doc = "CID (w) register accessor: an alias for `Reg<CID_SPEC>`"]
pub type CID = crate::Reg<cid::CID_SPEC>;
#[doc = "Channel Interrupt Disable Register (chid = 0)"]
pub mod cid;
#[doc = "CIM (w) register accessor: an alias for `Reg<CIM_SPEC>`"]
pub type CIM = crate::Reg<cim::CIM_SPEC>;
#[doc = "Channel Interrupt Mask Register (chid = 0)"]
pub mod cim;
#[doc = "CIS (r) register accessor: an alias for `Reg<CIS_SPEC>`"]
pub type CIS = crate::Reg<cis::CIS_SPEC>;
#[doc = "Channel Interrupt Status Register (chid = 0)"]
pub mod cis;
#[doc = "CSA (rw) register accessor: an alias for `Reg<CSA_SPEC>`"]
pub type CSA = crate::Reg<csa::CSA_SPEC>;
#[doc = "Channel Source Address Register (chid = 0)"]
pub mod csa;
#[doc = "CDA (rw) register accessor: an alias for `Reg<CDA_SPEC>`"]
pub type CDA = crate::Reg<cda::CDA_SPEC>;
#[doc = "Channel Destination Address Register (chid = 0)"]
pub mod cda;
#[doc = "CNDA (rw) register accessor: an alias for `Reg<CNDA_SPEC>`"]
pub type CNDA = crate::Reg<cnda::CNDA_SPEC>;
#[doc = "Channel Next Descriptor Address Register (chid = 0)"]
pub mod cnda;
#[doc = "CNDC (rw) register accessor: an alias for `Reg<CNDC_SPEC>`"]
pub type CNDC = crate::Reg<cndc::CNDC_SPEC>;
#[doc = "Channel Next Descriptor Control Register (chid = 0)"]
pub mod cndc;
#[doc = "CUBC (rw) register accessor: an alias for `Reg<CUBC_SPEC>`"]
pub type CUBC = crate::Reg<cubc::CUBC_SPEC>;
#[doc = "Channel Microblock Control Register (chid = 0)"]
pub mod cubc;
#[doc = "CBC (rw) register accessor: an alias for `Reg<CBC_SPEC>`"]
pub type CBC = crate::Reg<cbc::CBC_SPEC>;
#[doc = "Channel Block Control Register (chid = 0)"]
pub mod cbc;
#[doc = "CC (rw) register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Channel Configuration Register (chid = 0)"]
pub mod cc;
#[doc = "CDS_MSP (rw) register accessor: an alias for `Reg<CDS_MSP_SPEC>`"]
pub type CDS_MSP = crate::Reg<cds_msp::CDS_MSP_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 0)"]
pub mod cds_msp;
#[doc = "CSUS (rw) register accessor: an alias for `Reg<CSUS_SPEC>`"]
pub type CSUS = crate::Reg<csus::CSUS_SPEC>;
#[doc = "Channel Source Microblock Stride (chid = 0)"]
pub mod csus;
#[doc = "CDUS (rw) register accessor: an alias for `Reg<CDUS_SPEC>`"]
pub type CDUS = crate::Reg<cdus::CDUS_SPEC>;
#[doc = "Channel Destination Microblock Stride (chid = 0)"]
pub mod cdus;
