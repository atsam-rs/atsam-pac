#[doc = "Register `BUSY` reader"]
pub struct R(crate::R<BUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - Channel Status"]
pub type BUSY_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(self.bits)
    }
}
#[doc = "Channel / User Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busy](index.html) module"]
pub struct BUSY_SPEC;
impl crate::RegisterSpec for BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [busy::R](R) reader structure"]
impl crate::Readable for BUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUSY to value 0"]
impl crate::Resettable for BUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
