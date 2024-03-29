#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WINDOW` reader - WDT in window"]
pub type WINDOW_R = crate::BitReader<bool>;
#[doc = "Field `CLEARED` reader - WDT cleared"]
pub type CLEARED_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - WDT in window"]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDT cleared"]
    #[inline(always)]
    pub fn cleared(&self) -> CLEARED_R {
        CLEARED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0x03"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
