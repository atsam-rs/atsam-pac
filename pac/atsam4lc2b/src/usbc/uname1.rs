#[doc = "Register `UNAME1` reader"]
pub struct R(crate::R<UNAME1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNAME1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNAME1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNAME1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UNAME1` reader - IP Name Part One"]
pub type UNAME1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IP Name Part One"]
    #[inline(always)]
    pub fn uname1(&self) -> UNAME1_R {
        UNAME1_R::new(self.bits)
    }
}
#[doc = "IP Name Part One: HUSB\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uname1](index.html) module"]
pub struct UNAME1_SPEC;
impl crate::RegisterSpec for UNAME1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uname1::R](R) reader structure"]
impl crate::Readable for UNAME1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UNAME1 to value 0x4855_5342"]
impl crate::Resettable for UNAME1_SPEC {
    const RESET_VALUE: Self::Ux = 0x4855_5342;
}
