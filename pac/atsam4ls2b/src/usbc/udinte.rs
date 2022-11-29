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
pub type SUSPE_R = crate::BitReader<bool>;
#[doc = "Field `MSOFE` reader - MSOF Interrupt Enable"]
pub type MSOFE_R = crate::BitReader<bool>;
#[doc = "Field `SOFE` reader - SOF Interrupt Enable"]
pub type SOFE_R = crate::BitReader<bool>;
#[doc = "Field `EORSTE` reader - EORST Interrupt Enable"]
pub type EORSTE_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUPE` reader - WAKEUP Interrupt Enable"]
pub type WAKEUPE_R = crate::BitReader<bool>;
#[doc = "Field `EORSME` reader - EORSM Interrupt Enable"]
pub type EORSME_R = crate::BitReader<bool>;
#[doc = "Field `UPRSME` reader - UPRSM Interrupt Enable"]
pub type UPRSME_R = crate::BitReader<bool>;
#[doc = "Field `EP0INTE` reader - EP0INT Interrupt Enable"]
pub type EP0INTE_R = crate::BitReader<bool>;
#[doc = "Field `EP1INTE` reader - EP1INT Interrupt Enable"]
pub type EP1INTE_R = crate::BitReader<bool>;
#[doc = "Field `EP2INTE` reader - EP2INT Interrupt Enable"]
pub type EP2INTE_R = crate::BitReader<bool>;
#[doc = "Field `EP3INTE` reader - EP3INT Interrupt Enable"]
pub type EP3INTE_R = crate::BitReader<bool>;
#[doc = "Field `EP4INTE` reader - EP4INT Interrupt Enable"]
pub type EP4INTE_R = crate::BitReader<bool>;
#[doc = "Field `EP5INTE` reader - EP5INT Interrupt Enable"]
pub type EP5INTE_R = crate::BitReader<bool>;
#[doc = "Field `EP6INTE` reader - EP6INT Interrupt Enable"]
pub type EP6INTE_R = crate::BitReader<bool>;
#[doc = "Field `EP7INTE` reader - EP7INT Interrupt Enable"]
pub type EP7INTE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - SUSP Interrupt Enable"]
    #[inline(always)]
    pub fn suspe(&self) -> SUSPE_R {
        SUSPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MSOF Interrupt Enable"]
    #[inline(always)]
    pub fn msofe(&self) -> MSOFE_R {
        MSOFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SOF Interrupt Enable"]
    #[inline(always)]
    pub fn sofe(&self) -> SOFE_R {
        SOFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EORST Interrupt Enable"]
    #[inline(always)]
    pub fn eorste(&self) -> EORSTE_R {
        EORSTE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WAKEUP Interrupt Enable"]
    #[inline(always)]
    pub fn wakeupe(&self) -> WAKEUPE_R {
        WAKEUPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EORSM Interrupt Enable"]
    #[inline(always)]
    pub fn eorsme(&self) -> EORSME_R {
        EORSME_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UPRSM Interrupt Enable"]
    #[inline(always)]
    pub fn uprsme(&self) -> UPRSME_R {
        UPRSME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - EP0INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep0inte(&self) -> EP0INTE_R {
        EP0INTE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EP1INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep1inte(&self) -> EP1INTE_R {
        EP1INTE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EP2INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep2inte(&self) -> EP2INTE_R {
        EP2INTE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EP3INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep3inte(&self) -> EP3INTE_R {
        EP3INTE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - EP4INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep4inte(&self) -> EP4INTE_R {
        EP4INTE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - EP5INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep5inte(&self) -> EP5INTE_R {
        EP5INTE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - EP6INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep6inte(&self) -> EP6INTE_R {
        EP6INTE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - EP7INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep7inte(&self) -> EP7INTE_R {
        EP7INTE_R::new(((self.bits >> 19) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
