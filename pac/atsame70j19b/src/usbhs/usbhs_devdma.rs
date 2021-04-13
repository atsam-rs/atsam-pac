#[doc = "Device DMA Channel Next Descriptor Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devdmanxtdsc](devdmanxtdsc) module"]
pub type DEVDMANXTDSC = crate::Reg<u32, _DEVDMANXTDSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMANXTDSC;
#[doc = "`read()` method returns [devdmanxtdsc::R](devdmanxtdsc::R) reader structure"]
impl crate::Readable for DEVDMANXTDSC {}
#[doc = "`write(|w| ..)` method takes [devdmanxtdsc::W](devdmanxtdsc::W) writer structure"]
impl crate::Writable for DEVDMANXTDSC {}
#[doc = "Device DMA Channel Next Descriptor Address Register"]
pub mod devdmanxtdsc;
#[doc = "Device DMA Channel Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devdmaaddress](devdmaaddress) module"]
pub type DEVDMAADDRESS = crate::Reg<u32, _DEVDMAADDRESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMAADDRESS;
#[doc = "`read()` method returns [devdmaaddress::R](devdmaaddress::R) reader structure"]
impl crate::Readable for DEVDMAADDRESS {}
#[doc = "`write(|w| ..)` method takes [devdmaaddress::W](devdmaaddress::W) writer structure"]
impl crate::Writable for DEVDMAADDRESS {}
#[doc = "Device DMA Channel Address Register"]
pub mod devdmaaddress;
#[doc = "Device DMA Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devdmacontrol](devdmacontrol) module"]
pub type DEVDMACONTROL = crate::Reg<u32, _DEVDMACONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMACONTROL;
#[doc = "`read()` method returns [devdmacontrol::R](devdmacontrol::R) reader structure"]
impl crate::Readable for DEVDMACONTROL {}
#[doc = "`write(|w| ..)` method takes [devdmacontrol::W](devdmacontrol::W) writer structure"]
impl crate::Writable for DEVDMACONTROL {}
#[doc = "Device DMA Channel Control Register"]
pub mod devdmacontrol;
#[doc = "Device DMA Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devdmastatus](devdmastatus) module"]
pub type DEVDMASTATUS = crate::Reg<u32, _DEVDMASTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMASTATUS;
#[doc = "`read()` method returns [devdmastatus::R](devdmastatus::R) reader structure"]
impl crate::Readable for DEVDMASTATUS {}
#[doc = "`write(|w| ..)` method takes [devdmastatus::W](devdmastatus::W) writer structure"]
impl crate::Writable for DEVDMASTATUS {}
#[doc = "Device DMA Channel Status Register"]
pub mod devdmastatus;
