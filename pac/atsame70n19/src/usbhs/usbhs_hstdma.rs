#[doc = r"Register block"]
#[repr(C)]
pub struct USBHS_HSTDMA {
    #[doc = "0x00 - Host DMA Channel Next Descriptor Address Register (n = 1)"]
    pub hstdmanxtdsc: HSTDMANXTDSC,
    #[doc = "0x04 - Host DMA Channel Address Register (n = 1)"]
    pub hstdmaaddress: HSTDMAADDRESS,
    #[doc = "0x08 - Host DMA Channel Control Register (n = 1)"]
    pub hstdmacontrol: HSTDMACONTROL,
    #[doc = "0x0c - Host DMA Channel Status Register (n = 1)"]
    pub hstdmastatus: HSTDMASTATUS,
}
#[doc = "HSTDMANXTDSC (rw) register accessor: an alias for `Reg<HSTDMANXTDSC_SPEC>`"]
pub type HSTDMANXTDSC = crate::Reg<hstdmanxtdsc::HSTDMANXTDSC_SPEC>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod hstdmanxtdsc;
#[doc = "HSTDMAADDRESS (rw) register accessor: an alias for `Reg<HSTDMAADDRESS_SPEC>`"]
pub type HSTDMAADDRESS = crate::Reg<hstdmaaddress::HSTDMAADDRESS_SPEC>;
#[doc = "Host DMA Channel Address Register (n = 1)"]
pub mod hstdmaaddress;
#[doc = "HSTDMACONTROL (rw) register accessor: an alias for `Reg<HSTDMACONTROL_SPEC>`"]
pub type HSTDMACONTROL = crate::Reg<hstdmacontrol::HSTDMACONTROL_SPEC>;
#[doc = "Host DMA Channel Control Register (n = 1)"]
pub mod hstdmacontrol;
#[doc = "HSTDMASTATUS (rw) register accessor: an alias for `Reg<HSTDMASTATUS_SPEC>`"]
pub type HSTDMASTATUS = crate::Reg<hstdmastatus::HSTDMASTATUS_SPEC>;
#[doc = "Host DMA Channel Status Register (n = 1)"]
pub mod hstdmastatus;
