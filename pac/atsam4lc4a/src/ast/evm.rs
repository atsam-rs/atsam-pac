#[doc = "Register `EVM` reader"]
pub struct R(crate::R<EVM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVF` reader - Overflow"]
pub type OVF_R = crate::BitReader<bool>;
#[doc = "Field `ALARM0` reader - Alarm 0"]
pub type ALARM0_R = crate::BitReader<bool>;
#[doc = "Field `ALARM1` reader - Alarm 1"]
pub type ALARM1_R = crate::BitReader<bool>;
#[doc = "Field `PER0` reader - Perioidc 0"]
pub type PER0_R = crate::BitReader<bool>;
#[doc = "Field `PER1` reader - Periodic 1"]
pub type PER1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm 0"]
    #[inline(always)]
    pub fn alarm0(&self) -> ALARM0_R {
        ALARM0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm 1"]
    #[inline(always)]
    pub fn alarm1(&self) -> ALARM1_R {
        ALARM1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Perioidc 0"]
    #[inline(always)]
    pub fn per0(&self) -> PER0_R {
        PER0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Periodic 1"]
    #[inline(always)]
    pub fn per1(&self) -> PER1_R {
        PER1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Event Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evm](index.html) module"]
pub struct EVM_SPEC;
impl crate::RegisterSpec for EVM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evm::R](R) reader structure"]
impl crate::Readable for EVM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EVM to value 0"]
impl crate::Resettable for EVM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
