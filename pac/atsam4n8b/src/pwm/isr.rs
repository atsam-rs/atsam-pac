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
#[doc = "Field `CHID0` reader - Channel ID"]
pub type CHID0_R = crate::BitReader<bool>;
#[doc = "Field `CHID1` reader - Channel ID"]
pub type CHID1_R = crate::BitReader<bool>;
#[doc = "Field `CHID2` reader - Channel ID"]
pub type CHID2_R = crate::BitReader<bool>;
#[doc = "Field `CHID3` reader - Channel ID"]
pub type CHID3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Channel ID"]
    #[inline(always)]
    pub fn chid0(&self) -> CHID0_R {
        CHID0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel ID"]
    #[inline(always)]
    pub fn chid1(&self) -> CHID1_R {
        CHID1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel ID"]
    #[inline(always)]
    pub fn chid2(&self) -> CHID2_R {
        CHID2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel ID"]
    #[inline(always)]
    pub fn chid3(&self) -> CHID3_R {
        CHID3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "PWM Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
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
