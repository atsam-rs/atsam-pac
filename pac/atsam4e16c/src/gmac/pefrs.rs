#[doc = "Register `PEFRS` reader"]
pub struct R(crate::R<PEFRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEFRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEFRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEFRS_SPEC>) -> Self {
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
#[doc = "PTP Peer Event Frame Received Seconds\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pefrs](index.html) module"]
pub struct PEFRS_SPEC;
impl crate::RegisterSpec for PEFRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pefrs::R](R) reader structure"]
impl crate::Readable for PEFRS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PEFRS to value 0"]
impl crate::Resettable for PEFRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
