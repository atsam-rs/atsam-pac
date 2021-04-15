#[doc = "Register `INTCH%s` reader"]
pub struct R(crate::R<INTCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTCH_SPEC>> for R {
    fn from(reader: crate::R<INTCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTCH` reader - In-Touch"]
pub struct INTCH_R(crate::FieldReader<u32, u32>);
impl INTCH_R {
    pub(crate) fn new(bits: u32) -> Self {
        INTCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTCH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - In-Touch"]
    #[inline(always)]
    pub fn intch(&self) -> INTCH_R {
        INTCH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "In-Touch Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intch](index.html) module"]
pub struct INTCH_SPEC;
impl crate::RegisterSpec for INTCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intch::R](R) reader structure"]
impl crate::Readable for INTCH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTCH%s to value 0"]
impl crate::Resettable for INTCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
