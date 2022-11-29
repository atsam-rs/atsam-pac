#[doc = "Register `UNAME2` reader"]
pub struct R(crate::R<UNAME2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNAME2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNAME2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNAME2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UNAME2` reader - IP Name Part Two"]
pub type UNAME2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IP Name Part Two"]
    #[inline(always)]
    pub fn uname2(&self) -> UNAME2_R {
        UNAME2_R::new(self.bits)
    }
}
#[doc = "IP Name Part Two: HOST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uname2](index.html) module"]
pub struct UNAME2_SPEC;
impl crate::RegisterSpec for UNAME2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uname2::R](R) reader structure"]
impl crate::Readable for UNAME2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UNAME2 to value 0x484f_5354"]
impl crate::Resettable for UNAME2_SPEC {
    const RESET_VALUE: Self::Ux = 0x484f_5354;
}
