#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SAMPLE` reader - Sample Ready Interrupt Mask"]
pub type SAMPLE_R = crate::BitReader<bool>;
#[doc = "Field `INTCH` reader - In-touch Interrupt Mask"]
pub type INTCH_R = crate::BitReader<bool>;
#[doc = "Field `OUTTCH` reader - Out-of-Touch Interrupt Mask"]
pub type OUTTCH_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Sample Ready Interrupt Mask"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - In-touch Interrupt Mask"]
    #[inline(always)]
    pub fn intch(&self) -> INTCH_R {
        INTCH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Out-of-Touch Interrupt Mask"]
    #[inline(always)]
    pub fn outtch(&self) -> OUTTCH_R {
        OUTTCH_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
