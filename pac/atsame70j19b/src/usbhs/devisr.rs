#[doc = "Register `DEVISR` reader"]
pub struct R(crate::R<DEVISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SUSP` reader - Suspend Interrupt"]
pub struct SUSP_R(crate::FieldReader<bool, bool>);
impl SUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSOF` reader - Micro Start of Frame Interrupt"]
pub struct MSOF_R(crate::FieldReader<bool, bool>);
impl MSOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF` reader - Start of Frame Interrupt"]
pub struct SOF_R(crate::FieldReader<bool, bool>);
impl SOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EORST` reader - End of Reset Interrupt"]
pub struct EORST_R(crate::FieldReader<bool, bool>);
impl EORST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EORST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EORST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP` reader - Wake-Up Interrupt"]
pub struct WAKEUP_R(crate::FieldReader<bool, bool>);
impl WAKEUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EORSM` reader - End of Resume Interrupt"]
pub struct EORSM_R(crate::FieldReader<bool, bool>);
impl EORSM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EORSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EORSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPRSM` reader - Upstream Resume Interrupt"]
pub struct UPRSM_R(crate::FieldReader<bool, bool>);
impl UPRSM_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPRSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPRSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_0` reader - Endpoint 0 Interrupt"]
pub struct PEP_0_R(crate::FieldReader<bool, bool>);
impl PEP_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEP_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_1` reader - Endpoint 1 Interrupt"]
pub struct PEP_1_R(crate::FieldReader<bool, bool>);
impl PEP_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEP_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_2` reader - Endpoint 2 Interrupt"]
pub struct PEP_2_R(crate::FieldReader<bool, bool>);
impl PEP_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEP_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_3` reader - Endpoint 3 Interrupt"]
pub struct PEP_3_R(crate::FieldReader<bool, bool>);
impl PEP_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEP_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_4` reader - Endpoint 4 Interrupt"]
pub struct PEP_4_R(crate::FieldReader<bool, bool>);
impl PEP_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEP_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_5` reader - Endpoint 5 Interrupt"]
pub struct PEP_5_R(crate::FieldReader<bool, bool>);
impl PEP_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEP_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_6` reader - Endpoint 6 Interrupt"]
pub struct PEP_6_R(crate::FieldReader<bool, bool>);
impl PEP_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEP_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_7` reader - Endpoint 7 Interrupt"]
pub struct PEP_7_R(crate::FieldReader<bool, bool>);
impl PEP_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEP_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_8` reader - Endpoint 8 Interrupt"]
pub struct PEP_8_R(crate::FieldReader<bool, bool>);
impl PEP_8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEP_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_9` reader - Endpoint 9 Interrupt"]
pub struct PEP_9_R(crate::FieldReader<bool, bool>);
impl PEP_9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEP_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_1` reader - DMA Channel 1 Interrupt"]
pub struct DMA_1_R(crate::FieldReader<bool, bool>);
impl DMA_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_2` reader - DMA Channel 2 Interrupt"]
pub struct DMA_2_R(crate::FieldReader<bool, bool>);
impl DMA_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_3` reader - DMA Channel 3 Interrupt"]
pub struct DMA_3_R(crate::FieldReader<bool, bool>);
impl DMA_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_4` reader - DMA Channel 4 Interrupt"]
pub struct DMA_4_R(crate::FieldReader<bool, bool>);
impl DMA_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_5` reader - DMA Channel 5 Interrupt"]
pub struct DMA_5_R(crate::FieldReader<bool, bool>);
impl DMA_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_6` reader - DMA Channel 6 Interrupt"]
pub struct DMA_6_R(crate::FieldReader<bool, bool>);
impl DMA_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_7` reader - DMA Channel 7 Interrupt"]
pub struct DMA_7_R(crate::FieldReader<bool, bool>);
impl DMA_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Suspend Interrupt"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt"]
    #[inline(always)]
    pub fn msof(&self) -> MSOF_R {
        MSOF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start of Frame Interrupt"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of Reset Interrupt"]
    #[inline(always)]
    pub fn eorst(&self) -> EORST_R {
        EORST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Resume Interrupt"]
    #[inline(always)]
    pub fn eorsm(&self) -> EORSM_R {
        EORSM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt"]
    #[inline(always)]
    pub fn uprsm(&self) -> UPRSM_R {
        UPRSM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint 0 Interrupt"]
    #[inline(always)]
    pub fn pep_0(&self) -> PEP_0_R {
        PEP_0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn pep_1(&self) -> PEP_1_R {
        PEP_1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn pep_2(&self) -> PEP_2_R {
        PEP_2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn pep_3(&self) -> PEP_3_R {
        PEP_3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn pep_4(&self) -> PEP_4_R {
        PEP_4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn pep_5(&self) -> PEP_5_R {
        PEP_5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn pep_6(&self) -> PEP_6_R {
        PEP_6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint 7 Interrupt"]
    #[inline(always)]
    pub fn pep_7(&self) -> PEP_7_R {
        PEP_7_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Endpoint 8 Interrupt"]
    #[inline(always)]
    pub fn pep_8(&self) -> PEP_8_R {
        PEP_8_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Endpoint 9 Interrupt"]
    #[inline(always)]
    pub fn pep_9(&self) -> PEP_9_R {
        PEP_9_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt"]
    #[inline(always)]
    pub fn dma_1(&self) -> DMA_1_R {
        DMA_1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt"]
    #[inline(always)]
    pub fn dma_2(&self) -> DMA_2_R {
        DMA_2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt"]
    #[inline(always)]
    pub fn dma_3(&self) -> DMA_3_R {
        DMA_3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt"]
    #[inline(always)]
    pub fn dma_4(&self) -> DMA_4_R {
        DMA_4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt"]
    #[inline(always)]
    pub fn dma_5(&self) -> DMA_5_R {
        DMA_5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt"]
    #[inline(always)]
    pub fn dma_6(&self) -> DMA_6_R {
        DMA_6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DMA Channel 7 Interrupt"]
    #[inline(always)]
    pub fn dma_7(&self) -> DMA_7_R {
        DMA_7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Device Global Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devisr](index.html) module"]
pub struct DEVISR_SPEC;
impl crate::RegisterSpec for DEVISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devisr::R](R) reader structure"]
impl crate::Readable for DEVISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVISR to value 0"]
impl crate::Resettable for DEVISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
