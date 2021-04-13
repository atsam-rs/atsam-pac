#[doc = "Channel Interrupt Enable Register (chid = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie](cie) module"]
pub type CIE = crate::Reg<u32, _CIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE;
#[doc = "`write(|w| ..)` method takes [cie::W](cie::W) writer structure"]
impl crate::Writable for CIE {}
#[doc = "Channel Interrupt Enable Register (chid = 0)"]
pub mod cie;
#[doc = "Channel Interrupt Disable Register (chid = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid](cid) module"]
pub type CID = crate::Reg<u32, _CID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID;
#[doc = "`write(|w| ..)` method takes [cid::W](cid::W) writer structure"]
impl crate::Writable for CID {}
#[doc = "Channel Interrupt Disable Register (chid = 0)"]
pub mod cid;
#[doc = "Channel Interrupt Mask Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim](cim) module"]
pub type CIM = crate::Reg<u32, _CIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIM;
#[doc = "`read()` method returns [cim::R](cim::R) reader structure"]
impl crate::Readable for CIM {}
#[doc = "Channel Interrupt Mask Register (chid = 0)"]
pub mod cim;
#[doc = "Channel Interrupt Status Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis](cis) module"]
pub type CIS = crate::Reg<u32, _CIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIS;
#[doc = "`read()` method returns [cis::R](cis::R) reader structure"]
impl crate::Readable for CIS {}
#[doc = "Channel Interrupt Status Register (chid = 0)"]
pub mod cis;
#[doc = "Channel Source Address Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa](csa) module"]
pub type CSA = crate::Reg<u32, _CSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSA;
#[doc = "`read()` method returns [csa::R](csa::R) reader structure"]
impl crate::Readable for CSA {}
#[doc = "`write(|w| ..)` method takes [csa::W](csa::W) writer structure"]
impl crate::Writable for CSA {}
#[doc = "Channel Source Address Register (chid = 0)"]
pub mod csa;
#[doc = "Channel Destination Address Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda](cda) module"]
pub type CDA = crate::Reg<u32, _CDA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDA;
#[doc = "`read()` method returns [cda::R](cda::R) reader structure"]
impl crate::Readable for CDA {}
#[doc = "`write(|w| ..)` method takes [cda::W](cda::W) writer structure"]
impl crate::Writable for CDA {}
#[doc = "Channel Destination Address Register (chid = 0)"]
pub mod cda;
#[doc = "Channel Next Descriptor Address Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda](cnda) module"]
pub type CNDA = crate::Reg<u32, _CNDA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDA;
#[doc = "`read()` method returns [cnda::R](cnda::R) reader structure"]
impl crate::Readable for CNDA {}
#[doc = "`write(|w| ..)` method takes [cnda::W](cnda::W) writer structure"]
impl crate::Writable for CNDA {}
#[doc = "Channel Next Descriptor Address Register (chid = 0)"]
pub mod cnda;
#[doc = "Channel Next Descriptor Control Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc](cndc) module"]
pub type CNDC = crate::Reg<u32, _CNDC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDC;
#[doc = "`read()` method returns [cndc::R](cndc::R) reader structure"]
impl crate::Readable for CNDC {}
#[doc = "`write(|w| ..)` method takes [cndc::W](cndc::W) writer structure"]
impl crate::Writable for CNDC {}
#[doc = "Channel Next Descriptor Control Register (chid = 0)"]
pub mod cndc;
#[doc = "Channel Microblock Control Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc](cubc) module"]
pub type CUBC = crate::Reg<u32, _CUBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUBC;
#[doc = "`read()` method returns [cubc::R](cubc::R) reader structure"]
impl crate::Readable for CUBC {}
#[doc = "`write(|w| ..)` method takes [cubc::W](cubc::W) writer structure"]
impl crate::Writable for CUBC {}
#[doc = "Channel Microblock Control Register (chid = 0)"]
pub mod cubc;
#[doc = "Channel Block Control Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc](cbc) module"]
pub type CBC = crate::Reg<u32, _CBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBC;
#[doc = "`read()` method returns [cbc::R](cbc::R) reader structure"]
impl crate::Readable for CBC {}
#[doc = "`write(|w| ..)` method takes [cbc::W](cbc::W) writer structure"]
impl crate::Writable for CBC {}
#[doc = "Channel Block Control Register (chid = 0)"]
pub mod cbc;
#[doc = "Channel Configuration Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](cc) module"]
pub type CC = crate::Reg<u32, _CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC;
#[doc = "`read()` method returns [cc::R](cc::R) reader structure"]
impl crate::Readable for CC {}
#[doc = "`write(|w| ..)` method takes [cc::W](cc::W) writer structure"]
impl crate::Writable for CC {}
#[doc = "Channel Configuration Register (chid = 0)"]
pub mod cc;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp](cds_msp) module"]
pub type CDS_MSP = crate::Reg<u32, _CDS_MSP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDS_MSP;
#[doc = "`read()` method returns [cds_msp::R](cds_msp::R) reader structure"]
impl crate::Readable for CDS_MSP {}
#[doc = "`write(|w| ..)` method takes [cds_msp::W](cds_msp::W) writer structure"]
impl crate::Writable for CDS_MSP {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 0)"]
pub mod cds_msp;
#[doc = "Channel Source Microblock Stride (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus](csus) module"]
pub type CSUS = crate::Reg<u32, _CSUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSUS;
#[doc = "`read()` method returns [csus::R](csus::R) reader structure"]
impl crate::Readable for CSUS {}
#[doc = "`write(|w| ..)` method takes [csus::W](csus::W) writer structure"]
impl crate::Writable for CSUS {}
#[doc = "Channel Source Microblock Stride (chid = 0)"]
pub mod csus;
#[doc = "Channel Destination Microblock Stride (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus](cdus) module"]
pub type CDUS = crate::Reg<u32, _CDUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDUS;
#[doc = "`read()` method returns [cdus::R](cdus::R) reader structure"]
impl crate::Readable for CDUS {}
#[doc = "`write(|w| ..)` method takes [cdus::W](cdus::W) writer structure"]
impl crate::Writable for CDUS {}
#[doc = "Channel Destination Microblock Stride (chid = 0)"]
pub mod cdus;
