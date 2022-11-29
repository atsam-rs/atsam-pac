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
pub type VERSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPROC` reader - Embedded Processor"]
pub type EPROC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NVPSIZ` reader - Nonvolatile Program Memory Size"]
pub type NVPSIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NVPSIZ2` reader - Second Nonvolatile Program Memory Size"]
pub type NVPSIZ2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRAMSIZ` reader - Internal SRAM Size"]
pub type SRAMSIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARCH` reader - Architecture Identifier"]
pub type ARCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NVPTYP` reader - Nonvolatile Program Memory Type"]
pub type NVPTYP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXT` reader - Extension Flag"]
pub type EXT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:4 - Version of the Device"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Embedded Processor"]
    #[inline(always)]
    pub fn eproc(&self) -> EPROC_R {
        EPROC_R::new(((self.bits >> 5) & 7) as u8)
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
        NVPTYP_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Extension Flag"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 31) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
