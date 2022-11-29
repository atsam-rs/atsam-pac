#[doc = "Register `EFRS` reader"]
pub struct R(crate::R<EFRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RUD` reader - Register Update"]
pub type RUD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new(self.bits)
    }
}
#[doc = "PTP Event Frame Received Seconds\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efrs](index.html) module"]
pub struct EFRS_SPEC;
impl crate::RegisterSpec for EFRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efrs::R](R) reader structure"]
impl crate::Readable for EFRS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EFRS to value 0"]
impl crate::Resettable for EFRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
