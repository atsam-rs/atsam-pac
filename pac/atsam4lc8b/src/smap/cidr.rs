#[doc = "Register `CIDR` reader"]
pub struct R(crate::R<CIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VERSION` reader - Version of the Device"]
pub struct VERSION_R(crate::FieldReader<u8, u8>);
impl VERSION_R {
    pub(crate) fn new(bits: u8) -> Self {
        VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VERSION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPROC` reader - Embedded Processor"]
pub struct EPROC_R(crate::FieldReader<u8, u8>);
impl EPROC_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPROC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPROC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVPSIZ` reader - Nonvolatile Program Memory Size"]
pub struct NVPSIZ_R(crate::FieldReader<u8, u8>);
impl NVPSIZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        NVPSIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVPSIZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVPSIZ2` reader - Second Nonvolatile Program Memory Size"]
pub struct NVPSIZ2_R(crate::FieldReader<u8, u8>);
impl NVPSIZ2_R {
    pub(crate) fn new(bits: u8) -> Self {
        NVPSIZ2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVPSIZ2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAMSIZ` reader - Internal SRAM Size"]
pub struct SRAMSIZ_R(crate::FieldReader<u8, u8>);
impl SRAMSIZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        SRAMSIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAMSIZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARCH` reader - Architecture Identifier"]
pub struct ARCH_R(crate::FieldReader<u8, u8>);
impl ARCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        ARCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVPTYP` reader - Nonvolatile Program Memory Type"]
pub struct NVPTYP_R(crate::FieldReader<u8, u8>);
impl NVPTYP_R {
    pub(crate) fn new(bits: u8) -> Self {
        NVPTYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVPTYP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT` reader - Extension Flag"]
pub struct EXT_R(crate::FieldReader<bool, bool>);
impl EXT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Version of the Device"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Embedded Processor"]
    #[inline(always)]
    pub fn eproc(&self) -> EPROC_R {
        EPROC_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz(&self) -> NVPSIZ_R {
        NVPSIZ_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Second Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz2(&self) -> NVPSIZ2_R {
        NVPSIZ2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Internal SRAM Size"]
    #[inline(always)]
    pub fn sramsiz(&self) -> SRAMSIZ_R {
        SRAMSIZ_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:27 - Architecture Identifier"]
    #[inline(always)]
    pub fn arch(&self) -> ARCH_R {
        ARCH_R::new(((self.bits >> 21) & 0x7f) as u8)
    }
    #[doc = "Bits 28:30 - Nonvolatile Program Memory Type"]
    #[inline(always)]
    pub fn nvptyp(&self) -> NVPTYP_R {
        NVPTYP_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Extension Flag"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Chip ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr](index.html) module"]
pub struct CIDR_SPEC;
impl crate::RegisterSpec for CIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cidr::R](R) reader structure"]
impl crate::Readable for CIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIDR to value 0"]
impl crate::Resettable for CIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
