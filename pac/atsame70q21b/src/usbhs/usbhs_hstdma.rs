#[doc = "Host DMA Channel Next Descriptor Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstdmanxtdsc](hstdmanxtdsc) module"]
pub type HSTDMANXTDSC = crate::Reg<u32, _HSTDMANXTDSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMANXTDSC;
#[doc = "`read()` method returns [hstdmanxtdsc::R](hstdmanxtdsc::R) reader structure"]
impl crate::Readable for HSTDMANXTDSC {}
#[doc = "`write(|w| ..)` method takes [hstdmanxtdsc::W](hstdmanxtdsc::W) writer structure"]
impl crate::Writable for HSTDMANXTDSC {}
#[doc = "Host DMA Channel Next Descriptor Address Register"]
pub mod hstdmanxtdsc;
#[doc = "Host DMA Channel Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstdmaaddress](hstdmaaddress) module"]
pub type HSTDMAADDRESS = crate::Reg<u32, _HSTDMAADDRESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMAADDRESS;
#[doc = "`read()` method returns [hstdmaaddress::R](hstdmaaddress::R) reader structure"]
impl crate::Readable for HSTDMAADDRESS {}
#[doc = "`write(|w| ..)` method takes [hstdmaaddress::W](hstdmaaddress::W) writer structure"]
impl crate::Writable for HSTDMAADDRESS {}
#[doc = "Host DMA Channel Address Register"]
pub mod hstdmaaddress;
#[doc = "Host DMA Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstdmacontrol](hstdmacontrol) module"]
pub type HSTDMACONTROL = crate::Reg<u32, _HSTDMACONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMACONTROL;
#[doc = "`read()` method returns [hstdmacontrol::R](hstdmacontrol::R) reader structure"]
impl crate::Readable for HSTDMACONTROL {}
#[doc = "`write(|w| ..)` method takes [hstdmacontrol::W](hstdmacontrol::W) writer structure"]
impl crate::Writable for HSTDMACONTROL {}
#[doc = "Host DMA Channel Control Register"]
pub mod hstdmacontrol;
#[doc = "Host DMA Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstdmastatus](hstdmastatus) module"]
pub type HSTDMASTATUS = crate::Reg<u32, _HSTDMASTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMASTATUS;
#[doc = "`read()` method returns [hstdmastatus::R](hstdmastatus::R) reader structure"]
impl crate::Readable for HSTDMASTATUS {}
#[doc = "`write(|w| ..)` method takes [hstdmastatus::W](hstdmastatus::W) writer structure"]
impl crate::Writable for HSTDMASTATUS {}
#[doc = "Host DMA Channel Status Register"]
pub mod hstdmastatus;
