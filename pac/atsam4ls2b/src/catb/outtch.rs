#[doc = "Register `OUTTCH%s` reader"]
pub struct R(crate::R<OUTTCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTTCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTTCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTTCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTTCH` reader - Out-of-Touch"]
pub type OUTTCH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Out-of-Touch"]
    #[inline(always)]
    pub fn outtch(&self) -> OUTTCH_R {
        OUTTCH_R::new(self.bits)
    }
}
#[doc = "Out-of-Touch Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outtch](index.html) module"]
pub struct OUTTCH_SPEC;
impl crate::RegisterSpec for OUTTCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outtch::R](R) reader structure"]
impl crate::Readable for OUTTCH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUTTCH%s to value 0"]
impl crate::Resettable for OUTTCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
