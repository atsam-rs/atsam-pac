#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SAMPLE` reader - Sample Ready Interrupt Status"]
pub type SAMPLE_R = crate::BitReader<bool>;
#[doc = "Field `INTCH` reader - In-touch Interrupt Status"]
pub type INTCH_R = crate::BitReader<bool>;
#[doc = "Field `OUTTCH` reader - Out-of-Touch Interrupt Status"]
pub type OUTTCH_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Sample Ready Interrupt Status"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - In-touch Interrupt Status"]
    #[inline(always)]
    pub fn intch(&self) -> INTCH_R {
        INTCH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Out-of-Touch Interrupt Status"]
    #[inline(always)]
    pub fn outtch(&self) -> OUTTCH_R {
        OUTTCH_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
