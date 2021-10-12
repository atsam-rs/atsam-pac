#[doc = "Register `UDINTE` reader"]
pub struct R(crate::R<UDINTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDINTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDINTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDINTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SUSPE` reader - SUSP Interrupt Enable"]
pub struct SUSPE_R(crate::FieldReader<bool, bool>);
impl SUSPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSOFE` reader - MSOF Interrupt Enable"]
pub struct MSOFE_R(crate::FieldReader<bool, bool>);
impl MSOFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSOFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSOFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFE` reader - SOF Interrupt Enable"]
pub struct SOFE_R(crate::FieldReader<bool, bool>);
impl SOFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EORSTE` reader - EORST Interrupt Enable"]
pub struct EORSTE_R(crate::FieldReader<bool, bool>);
impl EORSTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EORSTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EORSTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPE` reader - WAKEUP Interrupt Enable"]
pub struct WAKEUPE_R(crate::FieldReader<bool, bool>);
impl WAKEUPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EORSME` reader - EORSM Interrupt Enable"]
pub struct EORSME_R(crate::FieldReader<bool, bool>);
impl EORSME_R {
    pub(crate) fn new(bits: bool) -> Self {
        EORSME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EORSME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPRSME` reader - UPRSM Interrupt Enable"]
pub struct UPRSME_R(crate::FieldReader<bool, bool>);
impl UPRSME_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPRSME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPRSME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0INTE` reader - EP0INT Interrupt Enable"]
pub struct EP0INTE_R(crate::FieldReader<bool, bool>);
impl EP0INTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP0INTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0INTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP1INTE` reader - EP1INT Interrupt Enable"]
pub struct EP1INTE_R(crate::FieldReader<bool, bool>);
impl EP1INTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP1INTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP1INTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP2INTE` reader - EP2INT Interrupt Enable"]
pub struct EP2INTE_R(crate::FieldReader<bool, bool>);
impl EP2INTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP2INTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2INTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP3INTE` reader - EP3INT Interrupt Enable"]
pub struct EP3INTE_R(crate::FieldReader<bool, bool>);
impl EP3INTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP3INTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP3INTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP4INTE` reader - EP4INT Interrupt Enable"]
pub struct EP4INTE_R(crate::FieldReader<bool, bool>);
impl EP4INTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP4INTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4INTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP5INTE` reader - EP5INT Interrupt Enable"]
pub struct EP5INTE_R(crate::FieldReader<bool, bool>);
impl EP5INTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP5INTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP5INTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP6INTE` reader - EP6INT Interrupt Enable"]
pub struct EP6INTE_R(crate::FieldReader<bool, bool>);
impl EP6INTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP6INTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP6INTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP7INTE` reader - EP7INT Interrupt Enable"]
pub struct EP7INTE_R(crate::FieldReader<bool, bool>);
impl EP7INTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP7INTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP7INTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - SUSP Interrupt Enable"]
    #[inline(always)]
    pub fn suspe(&self) -> SUSPE_R {
        SUSPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MSOF Interrupt Enable"]
    #[inline(always)]
    pub fn msofe(&self) -> MSOFE_R {
        MSOFE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SOF Interrupt Enable"]
    #[inline(always)]
    pub fn sofe(&self) -> SOFE_R {
        SOFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EORST Interrupt Enable"]
    #[inline(always)]
    pub fn eorste(&self) -> EORSTE_R {
        EORSTE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WAKEUP Interrupt Enable"]
    #[inline(always)]
    pub fn wakeupe(&self) -> WAKEUPE_R {
        WAKEUPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EORSM Interrupt Enable"]
    #[inline(always)]
    pub fn eorsme(&self) -> EORSME_R {
        EORSME_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UPRSM Interrupt Enable"]
    #[inline(always)]
    pub fn uprsme(&self) -> UPRSME_R {
        UPRSME_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EP0INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep0inte(&self) -> EP0INTE_R {
        EP0INTE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EP1INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep1inte(&self) -> EP1INTE_R {
        EP1INTE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EP2INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep2inte(&self) -> EP2INTE_R {
        EP2INTE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EP3INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep3inte(&self) -> EP3INTE_R {
        EP3INTE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - EP4INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep4inte(&self) -> EP4INTE_R {
        EP4INTE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - EP5INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep5inte(&self) -> EP5INTE_R {
        EP5INTE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - EP6INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep6inte(&self) -> EP6INTE_R {
        EP6INTE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - EP7INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep7inte(&self) -> EP7INTE_R {
        EP7INTE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
#[doc = "Device Global Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udinte](index.html) module"]
pub struct UDINTE_SPEC;
impl crate::RegisterSpec for UDINTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [udinte::R](R) reader structure"]
impl crate::Readable for UDINTE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UDINTE to value 0"]
impl crate::Resettable for UDINTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
